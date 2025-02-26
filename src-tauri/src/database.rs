// 環境変数管理
use std::env;
// Debug表示トレイト
use std::fmt::Debug;
// 時刻管理
use chrono::{Datelike, NaiveDate};
// Serialize(Frontendとの通信用)
use serde::{de::{self, Deserialize, Visitor}, ser::{Serialize, SerializeStruct}};
// DB操作用
use sqlx::{
    // MySQLとの通信用
    mysql::{MySql, MySqlPool}, 
    FromRow, Pool, Row
};
// logging用
use log::{error, info, warn};
// このcrate
use crate::other::{ThisResult, Error, ErrorKinds};

// データベースと通信確立
pub async fn connect_db() -> ThisResult<Pool<MySql>> {
    // 環境変数からデータベースURLを取得
    let database_url: String = env::var("DATABASE_URL")
        .map_err(|e| {
            let e: Error = Error::from_into_string(
                ErrorKinds::DataBaseError, 
                "Failed to find env var \"DATABASE_URL\"", 
                "データベースが設定されていません。開発者にお問い合わせください。", 
                e
            );
            error!("{:?}", e);
            e
        })?;
    // データベースと通信
    let pool: Pool<MySql> = MySqlPool::connect(&database_url)
        .await
        .map_err(|e| {
            let e: Error = Error::from_into_string(
                ErrorKinds::DataBaseError, 
                "Failed to connect database", 
                "データベースと通信できませんでした。データベースの状態を確認してください。", 
                e
            );
            error!("{:?}", e);
            e
        })?;
    // logging
    info!("Succeed in connecting database.");
    Ok(pool)
}

// データベース一行分のデータ
#[derive(Debug)]
pub struct CashRecord {
    id: usize, 
    date: NaiveDate, 
    category: String, 
    title: String, 
    amount: usize, 
    memo: Option<String>
}

impl CashRecord {
    const FIELDS: [&str; 6] = ["id", "date", "category", "title", "amount", "memo"];

    // データベースをすべて読む
    #[allow(dead_code)]
    pub async fn read_db_all(pool: &Pool<MySql>) -> ThisResult<Vec<CashRecord>> {
        sqlx::query_as::<_, CashRecord>(r#"SELECT * FROM cash_record;"#)
            .fetch_all(pool)
            .await
            .map_err(|e| {
                let e: Error = Error::from_into_string(
                    ErrorKinds::DataBaseError, 
                    "Failed to get data from db.", 
                    "データの取得に失敗しました。", 
                    e
                );
                error!("{:?}", e);
                e
            })
    }

    pub async fn read_db_from_id(pool: &Pool<MySql>, id: usize) -> ThisResult<Option<CashRecord>> {
        sqlx::query_as::<_, CashRecord>(format!(
                r#"SELECT * FROM cash_record WHERE id={};"#, 
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
                            "Failed to get data from db.", 
                            "データの取得に失敗しました。", 
                            e
                        );
                        error!("{:?}", e);
                        Err(e)
                    }
                }
            }, |v| Ok(Some(v)))
    }

    pub async fn read_db_from_month(pool: &Pool<MySql>, date: NaiveDate) -> ThisResult<Vec<CashRecord>> {
        let first_day_in_month: NaiveDate = NaiveDate::from_ymd_opt(date.year(), date.month(), 1).ok_or_else(|| {
            let e: Error = Error::from_msg(
                ErrorKinds::DeveloperError, 
                "Failed to get first day in the month.", 
                "日付の処理に失敗しました。開発者にお問い合わせください。"
            );
            error!("{:?}", e);
            e
        })?;
        let last_day_in_month: NaiveDate = {
            let (y, m): (i32, u32) = if date.month() == 12 { (date.year() + 1, date.month()) } else { (date.year(), date.month() + 1) };
            NaiveDate::from_ymd_opt(y, m, 1).map(|v| v.pred_opt()).flatten().ok_or_else(|| {
                let e: Error = Error::from_msg(
                    ErrorKinds::DeveloperError, 
                    "Failed to get first day in the month.", 
                    "日付の処理に失敗しました。開発者にお問い合わせください。"
                );
                error!("{:?}", e);
                e
            })?
        };
        sqlx::query_as::<_, CashRecord>(format!(
                r#"SELECT * FROM cash_record WHERE record_date BETWEEN "{}" AND "{}";"#, 
                first_day_in_month, 
                last_day_in_month
            ).as_str())
            .fetch_all(pool)
            .await
            .map_err(|e| {
                let e: Error = Error::from_into_string(
                    ErrorKinds::DataBaseError, 
                    "Failed to get data from db.", 
                    "データの取得に失敗しました。", 
                    e
                );
                error!("{:?}", e);
                e
            })
    }

    // インジェクションの防止
    pub async fn update_db_one(pool: &Pool<MySql>, changed_record: CashRecord) -> ThisResult<()> {
        sqlx::query(format!(
            r#"UPDATE cash_record SET record_date="{}", category="{}", title="{}", amount={}, memo="{}" WHERE id={}"#, 
            changed_record.date, 
            changed_record.category, 
            changed_record.title, 
            changed_record.amount, 
            changed_record.memo.unwrap_or_default(), 
            changed_record.id
        ).as_str())
        .execute(pool)
        .await
        .map_err(|e| {
            let e: Error = Error::from_into_string(
                ErrorKinds::DataBaseError, 
                "Failed to update data from db.", 
                "データの更新に失敗しました。", 
                e
            );
            error!("{:?}", e);
            e
        })?;
        Ok(())
    }
}

// CashRecordを文字列データに変換(backendからfrontendへの通信用)
impl Serialize for CashRecord {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        let mut s = serializer.serialize_struct("CsvData", 6)?;
        s.serialize_field("id", &self.id).map_err(|e| { error!("{:?}", e); e })?;
        s.serialize_field("date", &<NaiveDateWrapper as From<&NaiveDate>>::from(&self.date)).map_err(|e| { error!("{:?}", e); e })?;
        s.serialize_field("category", &self.category).map_err(|e| { error!("{:?}", e); e })?;
        s.serialize_field("title", &self.title).map_err(|e| { error!("{:?}", e); e })?;
        s.serialize_field("amount", &self.amount).map_err(|e| { error!("{:?}", e); e })?;
        s.serialize_field("memo", &self.memo).map_err(|e| { error!("{:?}", e); e })?;
        s.end()
    }
}

impl<'de> Deserialize<'de> for CashRecord {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: de::Deserializer<'de> {
        deserializer.deserialize_struct("CashRecord", &CashRecord::FIELDS, CashRecordVisitor)
    }
}

struct CashRecordVisitor;

impl<'de> Visitor<'de> for CashRecordVisitor {
    type Value = CashRecord;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "fields: {}", Self::Value::FIELDS.join(", "))
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::MapAccess<'de>, {
        let mut map: A = map;
        let mut id: Option<usize> = None;
        let mut date: Option<NaiveDate> = None;
        let mut category: Option<String> = None;
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
                        let date = map.next_value::<NaiveDateWrapper>().map_err(|e| { error!("{:?}", e); e })?;
                        Some(<NaiveDateWrapper as Into<NaiveDate>>::into(date))
                    };
                }, 
                "category" => {
                    if category.is_some() {
                        let e = de::Error::duplicate_field("category");
                        error!("{:?}", e);
                        return Err(e);
                    }
                    category = Some(map.next_value::<String>().map_err(|e| { error!("{:?}", e); e })?)
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
        let id: usize = id.ok_or_else(|| {
            let e = de::Error::missing_field("id");
            error!("{:?}", e);
            e
        })?;
        let date: NaiveDate = date.ok_or_else(|| {
            let e = de::Error::missing_field("date");
            error!("{:?}", e);
            e
        })?;
        let category: String = category.ok_or_else(|| {
            let e = de::Error::missing_field("category");
            error!("{:?}", e);
            e
        })?;
        let title: String = title.ok_or_else(|| {
            let e = de::Error::missing_field("title");
            error!("{:?}", e);
            e
        })?;
        let amount: usize = amount.ok_or_else(|| {
            let e = de::Error::missing_field("amount");
            error!("{:?}", e);
            e
        })?;
        let memo: Option<String> = memo.ok_or_else(|| {
            let e = de::Error::missing_field("memo");
            error!("{:?}", e);
            e
        })?;
        Ok(CashRecord { 
            id, 
            date, 
            category, 
            title, 
            amount, 
            memo 
        })
    }
}

// データベースからCashRecordに変換
impl<'r, R> FromRow<'r, R> for CashRecord 
    where
        R: Row,
        &'r str: sqlx::ColumnIndex<R>,
        i32: sqlx::Type<R::Database> + sqlx::Decode<'r, R::Database>,
        NaiveDate: sqlx::Type<R::Database> + sqlx::Decode<'r, R::Database>,
        String: sqlx::Type<R::Database> + sqlx::Decode<'r, R::Database>,
        Option<String>: sqlx::Type<R::Database> + sqlx::Decode<'r, R::Database>,
{
    fn from_row(row: &'r R) -> Result<Self, sqlx::Error> {
        let id: usize = {
            let id: i32 = row.try_get::<'_, i32, _>("id")?;
            <usize as TryFrom<i32>>::try_from(id).map_err(|e| sqlx::Error::Decode(Box::new(e)))?
        };
        let date: NaiveDate = row.try_get::<'_, NaiveDate, _>("record_date")?;
        let category: String = row.try_get::<'_, String, _>("category")?;
        let title: String = row.try_get::<'_, String, _>("title")?;
        let amount: usize = {
            let amount: i32 = row.try_get::<'_, i32, _>("amount")?;
            <usize as TryFrom<i32>>::try_from(amount).map_err(|e| sqlx::Error::Decode(Box::new(e)))?
        };
        let memo: Option<String> = row.try_get::<'_, Option<String>, _>("memo")?;
        Ok(CashRecord { 
            id, 
            date, 
            category, 
            title, 
            amount, 
            memo 
        })
    }
}

pub enum NaiveDateWrapper<'a> {
    ForSerialize(&'a NaiveDate), 
    ForDeserialize(NaiveDate)
}

impl NaiveDateWrapper<'_> {
    const FORMATTER: &'static str = "%Y-%m-%d";
}

impl<'a> From<&'a NaiveDate> for NaiveDateWrapper<'a> {
    fn from(value: &'a NaiveDate) -> Self {
        NaiveDateWrapper::ForSerialize(value)
    }
}

impl<'a> Into<NaiveDate> for NaiveDateWrapper<'a> {
    fn into(self) -> NaiveDate {
        match self {
            NaiveDateWrapper::ForSerialize(&v) => {
                warn!("Called <NaiveDateWrapper as Into<NaiveDate>>::into by ForSerializer.");
                v
            }, 
            NaiveDateWrapper::ForDeserialize(v) => v
        }
    }
}

impl Serialize for NaiveDateWrapper<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        let value = match self {
            NaiveDateWrapper::ForSerialize(v) => v, 
            NaiveDateWrapper::ForDeserialize(v) => v
        };
        serializer.serialize_str(&value.format(NaiveDateWrapper::FORMATTER).to_string())
    }
}

impl<'de> Deserialize<'de> for NaiveDateWrapper<'_> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: de::Deserializer<'de> {
        deserializer.deserialize_string(NaiveDateWrapperVisitor)
    }
}

struct NaiveDateWrapperVisitor;

impl<'de> Visitor<'de> for NaiveDateWrapperVisitor {
    type Value = NaiveDateWrapper<'static>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "NaiveDate")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error, {
        let value: NaiveDate = NaiveDate::parse_from_str(v, "%Y-%m-%d").map_err(|e| {
            let e = de::Error::custom(&format!("Failed to deserialize string.({})", e));
            error!("{:?}", e);
            e
        })?;
        Ok(NaiveDateWrapper::ForDeserialize(value))
    }
}