// Debug表示トレイト
use std::fmt::Debug;
// 時刻管理
use chrono::{Datelike, NaiveDate, NaiveDateTime};
// Serialize(Frontendとの通信用)
use serde::{de::{self, Deserialize, Visitor}, ser::{Serialize, SerializeStruct}};
// DB操作用
use sqlx::{mysql::MySql, FromRow, Pool, Row};
// logging用
use log::{error, warn};
// このcrate
use crate::{
    // データベース関連
    database::remove_special_chars, 
    // その他
    other::{Error, ErrorKinds, ThisResult}
};

// 出入金データベース一行分のデータ
#[derive(Debug)]
pub struct CashIORecord {
    id: usize, 
    date: NaiveDate, 
    main_category: String, 
    sub_category: String, 
    title: String, 
    amount: usize, 
    memo: Option<String>, 
    #[allow(dead_code)]
    created_at: Option<NaiveDateTime>, 
    #[allow(dead_code)]
    updated_at: Option<NaiveDateTime>, 
}

impl CashIORecord {
    // field群
    const FIELDS: [&'static str; 9] = ["id", "date", "main_category", "sub_category", "title", "amount", "memo", "created_at", "updated_at"];
    // CashIORecordを取得するSQL文
    const SELECT_SQL: &'static str = "SELECT 
            cash_record.id, 
            cash_record.record_date, 
            main_category.name As main_category_name, 
            sub_category.id As sub_category_id, 
            sub_category.name As sub_category_name, 
            cash_record.title, 
            cash_record.amount, 
            cash_record.memo, 
            cash_record.created_at, 
            cash_record.updated_at 
        FROM cash_record 
            INNER JOIN sub_category ON cash_record.category=sub_category.id 
            INNER JOIN main_category ON sub_category.super_category=main_category.id";

    // データベースからidで検索し取得
    pub async fn read_by_id(pool: &Pool<MySql>, id: usize) -> ThisResult<Option<CashIORecord>> {
        sqlx::query_as::<_, CashIORecord>(format!(
            r#"{} WHERE cash_record.id={};"#, 
            CashIORecord::SELECT_SQL, 
            id
        ).as_str())
            .fetch_one(pool)
            .await
            .map_or_else(|e| {
                match e {
                    sqlx::Error::RowNotFound => Ok(None), 
                    e => {
                        let e: Error = Error::from_into_string(
                            ErrorKinds::DataBaseError, 
                            "Failed to get CashIORecord from database.", 
                            "データの取得に失敗しました。", 
                            e
                        );
                        error!("{:?}", e);
                        Err(e)
                    }
                }
            }, |v| Ok(Some(v)))
    }

    // データベースから月で検索し取得
    pub async fn read_by_month(pool: &Pool<MySql>, date: NaiveDate) -> ThisResult<Vec<CashIORecord>> {
        let first_day_in_month: NaiveDate = NaiveDate::from_ymd_opt(date.year(), date.month(), 1).ok_or_else(|| {
            let e: Error = Error::from_msg(
                ErrorKinds::DeveloperError, 
                "Failed to get first day in the month.", 
                "エラー: A-01"
            );
            error!("{:?}", e);
            e
        })?;
        let last_day_in_month: NaiveDate = {
            let (y, m): (i32, u32) = if date.month() == 12 { (date.year() + 1, date.month()) } else { (date.year(), date.month() + 1) };
            NaiveDate::from_ymd_opt(y, m, 1).map(|v| v.pred_opt()).flatten().ok_or_else(|| {
                let e: Error = Error::from_msg(
                    ErrorKinds::DeveloperError, 
                    "Failed to get last day in the month.", 
                    "エラー: A-01"
                );
                error!("{:?}", e);
                e
            })?
        };
        sqlx::query_as::<_, CashIORecord>(format!(
            r#"{} WHERE cash_record.record_date BETWEEN "{}" AND "{}";"#, 
            CashIORecord::SELECT_SQL, 
            first_day_in_month, 
            last_day_in_month
        ).as_str())
            .fetch_all(pool)
            .await
            .map_err(|e| {
                let e: Error = Error::from_into_string(
                    ErrorKinds::DataBaseError, 
                    "Failed to get CashIORecord from database.", 
                    "データの取得に失敗しました。", 
                    e
                );
                error!("{:?}", e);
                e
            })
    }

    // データベースを更新
    pub async fn update(self, pool: &Pool<MySql>) -> ThisResult<()> {
        sqlx::query(format!(
            r#"UPDATE cash_record 
            INNER JOIN sub_category ON sub_category.name="{}" 
            INNER JOIN main_category ON main_category.name="{}" AND sub_category.super_category=main_category.id
            SET 
                cash_record.record_date="{}", 
                cash_record.category=sub_category.id, 
                cash_record.title="{}", 
                cash_record.amount={}, 
                cash_record.memo="{}" 
            WHERE cash_record.id={}"#, 
            self.sub_category, 
            self.main_category, 
            self.date, 
            remove_special_chars(&self.title)
                .unwrap_or_else(|e| { warn!(r#"Title of the record({}) contains '"', ';', '-'"#, self.title); e }), 
            self.amount, 
            (&self.memo)
                .as_ref()
                .map(|memo| remove_special_chars(memo)
                    .unwrap_or_else(|e| { warn!(r#"Memo of the record({}) contains '"', ';', '-'"#, memo); e })
                )
                .unwrap_or_default(), 
            self.id
        ).as_str())
            .execute(pool)
            .await
            .map_err(|e| {
                let e: Error = Error::from_into_string(
                    ErrorKinds::DataBaseError, 
                    "Failed to update CashIORecord on database.", 
                    "データの更新に失敗しました。", 
                    e
                );
                error!("{:?}", e);
                e
            })?;
        Ok(())
    }

    // データベースに新規作成
    pub async fn create(self, pool: &Pool<MySql>) -> ThisResult<()> {
        sqlx::query(format!(
            r#"INSERT 
            INTO cash_record (record_date, category, title, amount, memo) 
            SELECT 
                "{}", 
                sub_category.id, 
                "{}", 
                "{}", 
                "{}" 
            FROM sub_category 
                INNER JOIN main_category ON sub_category.super_category=main_category.id 
            WHERE main_category.name="{}" AND sub_category.name="{}";"#, 
            self.date, 
            remove_special_chars(&self.title)
                .unwrap_or_else(|e| { warn!(r#"Title of the record({}) contains '"', ';', '-'"#, self.title); e }), 
            self.amount, 
            (&self.memo)
                .as_ref()
                .map(|v| remove_special_chars(v)
                    .unwrap_or_else(|e| { warn!(r#"Memo of the record({}) contains '"', ';', '-'"#, v); e })
                )
                .unwrap_or_default(), 
            self.main_category, 
            self.sub_category
        ).as_str())
            .execute(pool)
            .await
            .map_err(|e| {
                let e: Error = Error::from_into_string(
                    ErrorKinds::DataBaseError, 
                    "Failed to create CashIORecord on database.", 
                    "データの作成に失敗しました。", 
                    e
                );
                error!("{:?}", e);
                e
            })?;
        Ok(())
    }

    pub async fn delete(self, pool: &Pool<MySql>) -> ThisResult<()> {
        sqlx::query(format!(
            r#"DELETE FROM cash_record WHERE id={}"#, 
            self.id
        ).as_str())
            .execute(pool)
            .await
            .map_err(|e| {
                let e = Error::from_into_string(
                    ErrorKinds::DataBaseError, 
                    "Failed to delete CashIORecord on database.", 
                    "データの削除に失敗しました。", 
                    e
                );
                error!("{:?}", e);
                e
            })?;
        Ok(())
    }
}

// CashIORecordをjsonデータに変換(backendからfrontendへの通信用)
impl Serialize for CashIORecord {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        let mut s: <S as serde::Serializer>::SerializeStruct = serializer.serialize_struct("CashIORecord", 7)?;
        s.serialize_field("id", &self.id).map_err(|e| { error!("{:?}", e); e })?;
        s.serialize_field("date", &self.date).map_err(|e| { error!("{:?}", e); e })?;
        s.serialize_field("mainCategory", &self.main_category).map_err(|e| { error!("{:?}", e); e })?;
        s.serialize_field("subCategory", &self.sub_category).map_err(|e| { error!("{:?}", e); e })?;
        s.serialize_field("title", &self.title).map_err(|e| { error!("{:?}", e); e })?;
        s.serialize_field("amount", &self.amount).map_err(|e| { error!("{:?}", e); e })?;
        s.serialize_field("memo", &self.memo).map_err(|e| { error!("{:?}", e); e })?;
        s.end().map_err(|e| { error!("{:?}", e); e })
    }
}

// jsonデータをCashIORecordに変換(frontendからbackendへの通信用)
impl<'de> Deserialize<'de> for CashIORecord {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: de::Deserializer<'de> {
        deserializer.deserialize_struct("CashIORecord", &CashIORecord::FIELDS, CashIORecordVisitor)
    }
}

struct CashIORecordVisitor;

impl<'de> Visitor<'de> for CashIORecordVisitor {
    type Value = CashIORecord;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "fields: {}", Self::Value::FIELDS.join(", "))
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::MapAccess<'de>, {
        let mut map: A = map;
        let mut id: Option<usize> = None;
        let mut date: Option<NaiveDate> = None;
        let mut main_category: Option<String> = None;
        let mut sub_category: Option<String> = None;
        let mut title: Option<String> = None;
        let mut amount: Option<usize> = None;
        let mut memo: Option<Option<String>> = None;
        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "id" => {
                    if id.is_some() {
                        let e = de::Error::duplicate_field("id");
                        error!("{:?}", e);
                        return Err(e);
                    }
                    id = Some(map.next_value::<usize>().map_err(|e| { error!("{:?}", e); e })?)
                }, 
                "date" => {
                    if date.is_some() {
                        let e = de::Error::duplicate_field("date");
                        error!("{:?}", e);
                        return Err(e);
                    }
                    date = {
                        let date = map.next_value::<NaiveDate>().map_err(|e| { error!("{:?}", e); e })?;
                        Some(date)
                    };
                }, 
                "mainCategory" => {
                    if main_category.is_some() {
                        let e = de::Error::duplicate_field("main_category");
                        error!("{:?}", e);
                        return Err(e);
                    }
                    main_category = Some(map.next_value::<String>().map_err(|e| { error!("{:?}", e); e })?)
                }, 
                "subCategory" => {
                    if sub_category.is_some() {
                        let e = de::Error::duplicate_field("sub_category");
                        error!("{:?}", e);
                        return Err(e);
                    }
                    sub_category = Some(map.next_value::<String>().map_err(|e| { error!("{:?}", e); e })?)
                }, 
                "title" => {
                    if title.is_some() {
                        let e = de::Error::duplicate_field("title");
                        error!("{:?}", e);
                        return Err(e);
                    }
                    title = Some(map.next_value::<String>().map_err(|e| { error!("{:?}", e); e })?)
                }, 
                "amount" => {
                    if amount.is_some() {
                        let e = de::Error::duplicate_field("amount");
                        error!("{:?}", e);
                        return Err(e);
                    }
                    amount = Some(map.next_value::<usize>().map_err(|e| { error!("{:?}", e); e })?)
                }, 
                "memo" => {
                    if memo.is_some() {
                        let e = de::Error::duplicate_field("memo");
                        error!("{:?}", e);
                        return Err(e);
                    }
                    memo = {
                        let memo = map.next_value::<String>().map_err(|e| { error!("{:?}", e); e })?;
                        Some(if memo.len() == 0 { None } else { Some(memo) })
                    }
                }, 
                v => {
                    let e = de::Error::unknown_field(v, &Self::Value::FIELDS);
                    error!("{:?}", e);
                    return Err(e);
                }
            }
        };
        Ok(CashIORecord { 
            id: id.ok_or_else(|| {
                let e = de::Error::missing_field("id");
                error!("{:?}", e);
                e
            })?, 
            date: date.ok_or_else(|| {
                let e = de::Error::missing_field("date");
                error!("{:?}", e);
                e
            })?, 
            main_category: main_category.ok_or_else(|| {
                let e = de::Error::missing_field("main_category");
                error!("{:?}", e);
                e
            })?, 
            sub_category: sub_category.ok_or_else(|| {
                let e = de::Error::missing_field("sub_category");
                error!("{:?}", e);
                e
            })?, 
            title: title.ok_or_else(|| {
                let e = de::Error::missing_field("title");
                error!("{:?}", e);
                e
            })?, 
            amount: amount.ok_or_else(|| {
                let e = de::Error::missing_field("amount");
                error!("{:?}", e);
                e
            })?, 
            memo: memo.ok_or_else(|| {
                let e = de::Error::missing_field("memo");
                error!("{:?}", e);
                e
            })?, 
            created_at: None, 
            updated_at: None 
        })
    }
}

// データベースからCashRecordに変換
impl<'r, R> FromRow<'r, R> for CashIORecord 
    where
        R: Row,
        &'r str: sqlx::ColumnIndex<R>,
        i32: sqlx::Type<R::Database> + sqlx::Decode<'r, R::Database>,
        NaiveDate: sqlx::Type<R::Database> + sqlx::Decode<'r, R::Database>,
        String: sqlx::Type<R::Database> + sqlx::Decode<'r, R::Database>,
        Option<String>: sqlx::Type<R::Database> + sqlx::Decode<'r, R::Database>,
        NaiveDateTime: sqlx::Type<R::Database> + sqlx::Decode<'r, R::Database>,
{
    fn from_row(row: &'r R) -> Result<Self, sqlx::Error> {
        let id: usize = <usize as TryFrom<i32>>::try_from(row.try_get::<'_, i32, _>("id")?)
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
        let date: NaiveDate = row.try_get::<'_, NaiveDate, _>("record_date")?;
        let main_category: String = row.try_get::<'_, String, _>("main_category_name")?;
        let sub_category: String = row.try_get::<'_, String, _>("sub_category_name")?;
        let title: String = row.try_get::<'_, String, _>("title")?;
        let amount: usize = <usize as TryFrom<i32>>::try_from(row.try_get::<'_, i32, _>("amount")?)
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
        let memo: Option<String> = row.try_get::<'_, Option<String>, _>("memo")?;
        let created_at: NaiveDateTime = row.try_get::<'_, NaiveDateTime, _>("created_at")?;
        let updated_at: NaiveDateTime = row.try_get::<'_, NaiveDateTime, _>("updated_at")?;
        Ok(CashIORecord { 
            id, 
            date, 
            main_category,
            sub_category,  
            title, 
            amount, 
            memo, 
            created_at: Some(created_at), 
            updated_at: Some(updated_at)
        })
    }
}
