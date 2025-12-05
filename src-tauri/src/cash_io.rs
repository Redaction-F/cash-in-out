//! The mod for payment and deposit data.

// import for debug
use std::fmt::Debug;
// import for date
use chrono::{Datelike, NaiveDate, NaiveDateTime};
// import for logging
use log::{error, warn};
// import for Serialize and Dederialize(data for frontend)
use serde::{de::{self, Deserialize, Visitor}, ser::{Serialize, SerializeStruct}};
// import for database
use sqlx::{mysql::MySql, FromRow, Pool, Row};
// this crate
use crate::{
    // database
    database::RemoveSpecialChars, 
    // other
    other::{err_with_msg, err_with_msg_with_non_error, ErrorKinds, ThisResult}
};

macro_rules! key_match {
    ( $map:expr, $key:expr, $( $p:pat => $field:ident ),*$(,)?) => {
        match $key {
            $(
                $p => {
                    if $field.is_some() {
                        let e = ::serde::de::Error::duplicate_field(stringify!($field));
                        error!("{:?}", e);
                        return Err(e);
                    }
                    $field = Some($map.next_value().map_err(|e| { error!("{:?}", e); e })?)
                },
            )*
            v => {
                let e = ::serde::de::Error::unknown_field(v, &Self::Value::FIELDS);
                error!("{:?}", e);
                return Err(e);
            }
        }
    };
}
macro_rules! field_check {
    ( $target_struct:ident, $( $field:ident ),* $( ; $( $default_field:ident: $default_value:expr ),* )? ) => {
        $target_struct {
            $(
                $field: $field.ok_or_else(|| {
                    let e = ::serde::de::Error::missing_field(stringify!($field));
                    error!("{:?}", e);
                    e
                })?,
            )*
            $($(
                $default_field: $default_value,
            )*)*
        }
    };
}

/// Payment or deposit data.
#[derive(Debug)]
pub struct CashIORecord {
    id: usize, 
    date: NaiveDate, 
    main_category: String, 
    sub_category: String, 
    title: String, 
    amount: isize, 
    memo: String, 
    #[allow(dead_code)]
    created_at: Option<NaiveDateTime>, 
    #[allow(dead_code)]
    updated_at: Option<NaiveDateTime>, 
}

impl CashIORecord {
    /// Fields of this sturct.
    const FIELDS: [&'static str; 7] = ["id", "date", "mainCategory", "subCategory", "title", "amount", "memo"];
    /// SQL statement for select `CashIO`.
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

    /// Select all records.
    pub async fn select_all(pool: &Pool<MySql>) -> ThisResult<Vec<CashIORecord>> {
        sqlx::query_as::<_, CashIORecord>(
            format!(
                r#"{};"#, 
                CashIORecord::SELECT_SQL
            ).as_str()
        )
            .fetch_all(pool)
            .await
            .map_err(|e| err_with_msg!(
                ErrorKinds::DataBaseError, 
                "Failed to get CashIORecords from the database.", 
                "データの取得に失敗しました。", 
                e
            ))
    }

    /// Select a record by an id from the database.
    pub async fn select_by_id(pool: &Pool<MySql>, id: usize) -> ThisResult<Option<CashIORecord>> {
        use sqlx::Error as SqlxError;

        sqlx::query_as::<_, CashIORecord>(
            format!(
                r#"{} WHERE cash_record.id={};"#, 
                CashIORecord::SELECT_SQL, 
                id
            ).as_str()
        )
            .fetch_one(pool)
            .await
            .map_or_else(
                |e| err_with_msg_with_non_error!(
                    SqlxError::RowNotFound => None;
                    ErrorKinds::DataBaseError, 
                    "Failed to get CashIORecords from the database.", 
                    "データの取得に失敗しました。", 
                    e
                ), 
                |v| Ok(Some(v))
            )
    }

    /// Select a record by a month from the database.
    pub async fn select_by_month(pool: &Pool<MySql>, date: NaiveDate) -> ThisResult<Vec<CashIORecord>> {
        // first day of the month
        let first_day_in_month: NaiveDate = NaiveDate::from_ymd_opt(date.year(), date.month(), 1)
            .ok_or_else(|| err_with_msg!(
                ErrorKinds::DeveloperError, 
                "Failed to get first day in the month.", 
                "予期せぬエラーが発生しました。(E001)"
            ))?;
        // last day of the month
        let last_day_in_month: NaiveDate = {
            let (next_y, next_m): (i32, u32) = if date.month() == 12 { 
                (date.year() + 1, date.month()) 
            } else { 
                (date.year(), date.month() + 1) 
            };
            NaiveDate::from_ymd_opt(next_y, next_m, 1)
                .map(|v| v.pred_opt())
                .flatten()
                .ok_or_else(|| err_with_msg!(
                    ErrorKinds::DeveloperError, 
                    "Failed to get last day in the month.", 
                    "予期せぬエラーが発生しました。(E001)"
                ))?
        };
        sqlx::query_as::<_, CashIORecord>(
            format!(
                r#"{} WHERE cash_record.record_date BETWEEN "{}" AND "{}" ORDER BY cash_record.record_date;"#, 
                CashIORecord::SELECT_SQL, 
                first_day_in_month, 
                last_day_in_month
            ).as_str()
        )
            .fetch_all(pool)
            .await
            .map_err(|e| err_with_msg!(
                ErrorKinds::DataBaseError, 
                "Failed to get CashIORecords from the database.", 
                "データの取得に失敗しました。", 
                e
            ))
    }

    /// Select records which match an option.
    pub async fn select_by_option(pool: &Pool<MySql>, option: CashIORecordOption) -> ThisResult<Vec<CashIORecord>> {
        sqlx::query_as::<_, CashIORecord>(format!(
            r#"{}{}{}{}{}{}{}{}{};"#, 
            CashIORecord::SELECT_SQL, 
            if option.is_all_none() { " WHERE" } else { "" }, 
            option.id.map_or_else(|| String::new(), |v| format!(r#" cash_record.id={}"#, v)), 
            option.date.map_or_else(|| String::new(), |v| format!(r#" cash_record.date="{}""#, v)), 
            option.main_category.map_or_else(|| String::new(), |v| format!(r#" cash_record.main_category_name="{}""#, v)), 
            option.sub_category.map_or_else(|| String::new(), |v| format!(r#" cash_record.sub_category_name="{}""#, v)), 
            option.title.map_or_else(|| String::new(), |v| format!(r#" cash_record.title="{}""#, v)), 
            option.amount.map_or_else(|| String::new(), |v| format!(r#" cash_record.amount={}"#, v)), 
            option.memo.map_or_else(|| String::new(), |v| format!(r#" cash_record.memo="{}""#, v)), 
        ).as_str())
            .fetch_all(pool)
            .await
            .map_err(|e| err_with_msg!(
                ErrorKinds::DataBaseError, 
                "Failed to get CashIORecord from database.", 
                "データの取得に失敗しました。", 
                e
            ))
    }

    /// Update a record of the database.
    pub async fn update(self, pool: &Pool<MySql>) -> ThisResult<()> {
        sqlx::query(
            format!(
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
                (&self.title)
                    .remove_special_chars()
                    .unwrap_or_else(|e| { warn!(r#"Title of the record({}) contains '"', ';', or '-'"#, self.title); e }), 
                self.amount, 
                (&self.memo)
                    .remove_special_chars()
                    .unwrap_or_else(|e| { warn!(r#"Title of the record({}) contains '"', ';', or '-'"#, self.title); e }), 
                self.id
            ).as_str()
        )
            .execute(pool)
            .await
            .map_err(|e| err_with_msg!(
                ErrorKinds::DataBaseError, 
                "Failed to update a CashIORecord on the database.", 
                "データの更新に失敗しました。", 
                e
            ))?;
        Ok(())
    }

    /// Insert a record to the database.
    pub async fn insert(self, pool: &Pool<MySql>) -> ThisResult<()> {
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
            (&self.title)
                .remove_special_chars()
                .unwrap_or_else(|e| { warn!(r#"Title of the record({}) contains '"', ';', '-'"#, self.title); e }), 
            self.amount, 
            (&self.memo)
                .remove_special_chars()
                .unwrap_or_else(|e| { warn!(r#"Title of the record({}) contains '"', ';', or '-'"#, self.title); e }), 
            self.main_category, 
            self.sub_category
        ).as_str())
            .execute(pool)
            .await
            .map_err(|e| err_with_msg!(
                ErrorKinds::DataBaseError, 
                "Failed to insert a CashIORecord on the database.", 
                "データの作成に失敗しました。", 
                e
            ))?;
        Ok(())
    }

    /// Delete a record of the database.
    pub async fn delete(self, pool: &Pool<MySql>) -> ThisResult<()> {
        sqlx::query(format!(
            r#"DELETE FROM cash_record WHERE id={}"#, 
            self.id
        ).as_str())
            .execute(pool)
            .await
            .map_err(|e| err_with_msg!(
                ErrorKinds::DataBaseError, 
                "Failed to delete a CashIORecord on the database.", 
                "データの削除に失敗しました。", 
                e
            ))?;
        Ok(())
    }
}

// convert a `CashIORecord` to a frontend data
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

// convert a frontend data to a CashIORecord
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
        let mut amount: Option<isize> = None;
        let mut memo: Option<String> = None;


        while let Some(key) = map.next_key::<String>()? {
            key_match!(
                map,
                key.as_str(),
                "id" | "_id" => id,
                "date" | "_date" => date,
                "mainCategory" | "_mainCategory" => main_category,
                "subCategory" | "_subCategory" => sub_category,
                "title" | "_title" => title,
                "amount" | "_amount" => amount,
                "memo" | "_memo" => memo
            );
        };

        Ok(field_check!(CashIORecord, id, date, main_category, sub_category, title, amount, memo; created_at: None, updated_at: None))
    }
}

// convert a database row to a CashIORecord
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
        let amount: isize = <isize as TryFrom<i32>>::try_from(row.try_get::<'_, i32, _>("amount")?)
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
        let memo: String = row.try_get::<'_, String, _>("memo")?;
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

pub struct CashIORecordOption {
    id: Option<usize>, 
    date: Option<NaiveDate>, 
    main_category: Option<String>, 
    sub_category: Option<String>, 
    title: Option<String>, 
    amount: Option<isize>, 
    memo: Option<String>, 
}

impl CashIORecordOption {
    const FIELDS: [&'static str; 7] = ["id", "date", "main_category", "sub_category", "title", "amount", "memo"];

    fn is_all_none(&self) -> bool {
        self.id.is_none()
            && self.date.is_none()
            && self.main_category.is_none()
            && self.sub_category.is_none()
            && self.title.is_none()
            && self.amount.is_none()
            && self.memo.is_none()
    }
}

impl Serialize for CashIORecordOption {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        let mut s: <S as serde::Serializer>::SerializeStruct = serializer.serialize_struct("CashIORecordOption", 7)?;
        s.serialize_field("id", &self.id).map_err(|e| { error!("{}", e); e })?;
        s.serialize_field("date", &self.id).map_err(|e| { error!("{}", e); e })?;
        s.serialize_field("main_category", &self.id).map_err(|e| { error!("{}", e); e })?;
        s.serialize_field("sub_category", &self.id).map_err(|e| { error!("{}", e); e })?;
        s.serialize_field("title", &self.id).map_err(|e| { error!("{}", e); e })?;
        s.serialize_field("amount", &self.id).map_err(|e| { error!("{}", e); e })?;
        s.serialize_field("memo", &self.id).map_err(|e| { error!("{}", e); e })?;
        s.end().map_err(|e| { error!("{}", e); e })
    }
}

impl<'de> Deserialize<'de> for CashIORecordOption {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: de::Deserializer<'de> {
        deserializer.deserialize_struct("CashIORecordOption", &CashIORecordOption::FIELDS, CashIORecordOptionVisitor)
    }
}

struct CashIORecordOptionVisitor;

impl<'de> Visitor<'de> for CashIORecordOptionVisitor {
    type Value = CashIORecordOption;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "fields: {}", Self::Value::FIELDS.join(", "))
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
        where
            A: de::MapAccess<'de>, {
        let mut map: A = map;
        let mut id: Option<Option<usize>> = None;
        let mut date: Option<Option<NaiveDate>> = None;
        let mut main_category: Option<Option<String>> = None;
        let mut sub_category: Option<Option<String>> = None;
        let mut title: Option<Option<String>> = None;
        let mut amount: Option<Option<isize>> = None;
        let mut memo: Option<Option<String>> = None;
        while let Some(key) = map.next_key::<String>()? {
            key_match!(
                map, 
                key.as_str(),
                "id" | "_id" => id,
                "date" | "_date" => date,
                "mainCategory" | "_mainCategory" => main_category,
                "subCategory" | "_subCategory" => sub_category,
                "title" | "_title" => title,
                "amount" | "_amount" => amount,
                "memo" | "_memo" => memo,
            );
        }

        Ok(field_check!(CashIORecordOption, id, date, main_category, sub_category, title, amount, memo))
    }
}
