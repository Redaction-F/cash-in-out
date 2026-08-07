//! The mod for payment and deposit data.

// import for debug
use std::fmt::Debug;
use cash_in_out_derive::{MyDeserialize, MySerialize};
// import for date
use chrono::{Datelike, NaiveDate, NaiveDateTime};
// import for logging
use log::warn;
// import for decimal
use rust_decimal::Decimal;
// import for database
use sqlx::{mysql::MySql, FromRow, Pool, Row};
// this crate
use crate::{
    // database
    database::RemoveSpecialChars,
    // other
    other::{err_with_msg, err_with_msg_with_non_error, ErrorKinds, ThisResult},
};

/// Payment or deposit data.
#[derive(Debug, MySerialize, MyDeserialize)]
// #[serde(rename_all="camelCase")]
pub struct CashIORecord {
    id: usize,
    date: NaiveDate,
    main_category: String,
    sub_category: String,
    title: String,
    amount: isize,
    memo: String,
    #[allow(dead_code)]
    #[MyDeserialize(default=None)]
    created_at: Option<NaiveDateTime>,
    #[allow(dead_code)]
    #[MyDeserialize(default=None)]
    updated_at: Option<NaiveDateTime>,
}

impl CashIORecord {
    /// SQL statement for selecting.
    const SELECT_SQL_BASE: &'static str = "SELECT 
    cash_record.id, 
    cash_record.record_date, 
    main_category.name AS main_category_name, 
    sub_category.id AS sub_category_id, 
    sub_category.name AS sub_category_name, 
    cash_record.title, 
    cash_record.amount, 
    cash_record.memo, 
    cash_record.created_at, 
    cash_record.updated_at 
FROM cash_record 
    INNER JOIN sub_category ON cash_record.category=sub_category.id 
    INNER JOIN main_category ON sub_category.super_category=main_category.id";

    fn sql_select(where_sql: Option<String>, order_sql: Option<String>) -> String {
        format!(
            "{}{}{}", 
            Self::SELECT_SQL_BASE, 
            match where_sql {
                Some(v) => format!(" WHERE {}", v),
                None => String::new()
            },
            match order_sql {
                Some(v) => format!(" ORDER BY {}", v),
                None => String::new()
            }
        )
    }

    fn sql_between_month(date: NaiveDate) -> ThisResult<String> {
        // first day of the month
        let first_day_in_month: NaiveDate = NaiveDate::from_ymd_opt(date.year(), date.month(), 1)
            .ok_or_else(|| {
            err_with_msg!(
                ErrorKinds::DeveloperError,
                "Failed to get first day in the month.",
                "予期せぬエラーが発生しました。(E001)"
            )
        })?;
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
                .ok_or_else(|| {
                    err_with_msg!(
                        ErrorKinds::DeveloperError,
                        "Failed to get last day in the month.",
                        "予期せぬエラーが発生しました。(E001)"
                    )
                })?
        };

        Ok(format!(
            "cash_record.record_date BETWEEN \"{}\" AND \"{}\"",
            first_day_in_month,
            last_day_in_month
        ))
    }

    /// Select all records.
    pub async fn select_all(pool: &Pool<MySql>) -> ThisResult<Vec<CashIORecord>> {
        sqlx::query_as::<_, CashIORecord>(CashIORecord::sql_select(
            None, 
            None)
        .as_str())
            .fetch_all(pool)
            .await
            .map_err(|e| {
                err_with_msg!(
                    ErrorKinds::DataBaseError,
                    "Failed to get CashIORecords from the database.",
                    "データの取得に失敗しました。",
                    e
                )
            })
    }

    /// Select a record by an id from the database.
    pub async fn select_by_id(pool: &Pool<MySql>, id: usize) -> ThisResult<Option<CashIORecord>> {
        use sqlx::Error as SqlxError;

        sqlx::query_as::<_, CashIORecord>(CashIORecord::sql_select(
            Some(format!("cash_record.id={}", id)), 
            None
        ).as_str())
        .fetch_one(pool)
        .await
        .map_or_else(
            |e| {
                err_with_msg_with_non_error!(
                    SqlxError::RowNotFound => None;
                    ErrorKinds::DataBaseError,
                    "Failed to get CashIORecords from the database.",
                    "データの取得に失敗しました。",
                    e
                )
            },
            |v| Ok(Some(v)),
        )
    }

    /// Select a record by a month from the database.
    pub async fn select_by_month(
        pool: &Pool<MySql>,
        date: NaiveDate,
    ) -> ThisResult<Vec<CashIORecord>> {
        sqlx::query_as::<_, CashIORecord>(&CashIORecord::sql_select(
            Some(CashIORecord::sql_between_month(date)?), 
            Some("cash_record.record_date".to_string())
        ).as_str())
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
    pub async fn select_by_option(
        pool: &Pool<MySql>,
        option: CashIORecordOption,
    ) -> ThisResult<Vec<CashIORecord>> {
        sqlx::query_as::<_, CashIORecord>(CashIORecord::sql_select(
            if option.is_all_none() { 
                None 
            } else {
                Some([
                    ("id", option.id.map(|v| v.to_string())),
                    ("date", option.date.map(|v| v.to_string())),
                    ("main_category_name", option.main_category.map(|v| v.to_string())),
                    ("sub_category_name", option.sub_category.map(|v| v.to_string())),
                    ("title", option.title.map(|v| v.to_string())),
                    ("amount", option.amount.map(|v| v.to_string())),
                    ("memo", option.memo.map(|v| v.to_string())),
                ]
                    .map(|(key, v)| v.map_or_else(
                        || String::new(),
                        |v| format!(r#" cash_record.{}="{}""#, key, v)
                    ))
                    .join(" ")
            )
        },
            None
        ).as_str())
            .fetch_all(pool)
            .await
            .map_err(|e| {
                err_with_msg!(
                    ErrorKinds::DataBaseError,
                    "Failed to get CashIORecord from database.",
                    "データの取得に失敗しました。",
                    e
                )
            })
    }

    pub async fn sum_by_month(
        pool: &Pool<MySql>,
        date: NaiveDate,
    ) -> ThisResult<isize> {
        sqlx::query_scalar::<_, Option<Decimal>>(format!(
                r#"SELECT sum(amount) FROM cash_record WHERE {};"#, 
                CashIORecord::sql_between_month(date)?
        ).as_str())
            .fetch_one(pool)
            .await
            .map(|v| 
                v.map(|v| <Decimal as TryInto<isize>>::try_into(v).ok()).flatten().unwrap_or_default()
            )
            .map_err(|e| err_with_msg!(
                ErrorKinds::DataBaseError, 
                "Failed to get CashIORecords from the database.", 
                "データの取得に失敗しました。", 
                e
            ))
    }

    pub async fn sum_by_month_group_by_main_category(
        pool: &Pool<MySql>,
        date: NaiveDate
    ) -> ThisResult<Vec<(String, isize)>> {
        struct SumGroupByMainCategory {
            #[allow(dead_code)]
            id: usize,
            category_name: String,
            sum: isize
        }
        impl<'r, R> FromRow<'r, R> for SumGroupByMainCategory
        where
            R: Row,
            &'r str: sqlx::ColumnIndex<R>,
            i32: sqlx::Type<R::Database> + sqlx::Decode<'r, R::Database>,
            Decimal: sqlx::Type<R::Database> + sqlx::Decode<'r, R::Database>,
            NaiveDate: sqlx::Type<R::Database> + sqlx::Decode<'r, R::Database>,
            String: sqlx::Type<R::Database> + sqlx::Decode<'r, R::Database>,
            Option<String>: sqlx::Type<R::Database> + sqlx::Decode<'r, R::Database>,
            NaiveDateTime: sqlx::Type<R::Database> + sqlx::Decode<'r, R::Database>,
        {
            fn from_row(row: &'r R) -> Result<Self, sqlx::Error> {
                let id: usize = <usize as TryFrom<i32>>::try_from(row.try_get::<'_, i32, _>("id")?)
                    .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
                let category_name: String = row.try_get::<'_, String, _>("main_category_name")?;
                let sum: isize = <isize as TryFrom<Decimal>>::try_from({
                    let a: Decimal = row.try_get("sum")?;
                    a
                })
                    .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
                Ok(SumGroupByMainCategory { 
                    id,
                    category_name,
                    sum
                })
            }
        }
        
        sqlx::query_as::<_, SumGroupByMainCategory>(
            format!(
                "SELECT 
    main_category.id AS id, 
    main_category.name AS main_category_name, 
    sum(cash_record.amount) AS sum
FROM 
    cash_record 
    INNER JOIN sub_category ON cash_record.category = sub_category.id 
    INNER JOIN main_category ON sub_category.super_category = main_category.id 
WHERE {} 
GROUP BY main_category.id 
ORDER BY main_category.id;",
            CashIORecord::sql_between_month(date)?
        ).as_str())
            .fetch_all(pool)
            .await
            .map(|v| v
                .into_iter()
                .map(|v| (v.category_name, v.sum))
                .collect::<Vec<(String, isize)>>())
            .map_err(|e| err_with_msg!(
                ErrorKinds::DataBaseError, 
                "Failed to get CashIORecords from the database.", 
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
                    .unwrap_or_else(|e| { warn!(r#"Memo of the record({}) contains '"', ';', or '-'"#, self.title); e }), 
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
        sqlx::query(
            format!(
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
                (&self.title).remove_special_chars().unwrap_or_else(|e| {
                    warn!(
                        r#"Title of the record({}) contains '"', ';', '-'"#,
                        self.title
                    );
                    e
                }),
                self.amount,
                (&self.memo).remove_special_chars().unwrap_or_else(|e| {
                    warn!(
                        r#"Title of the record({}) contains '"', ';', or '-'"#,
                        self.title
                    );
                    e
                }),
                self.main_category,
                self.sub_category
            )
            .as_str(),
        )
        .execute(pool)
        .await
        .map_err(|e| {
            err_with_msg!(
                ErrorKinds::DataBaseError,
                "Failed to insert a CashIORecord on the database.",
                "データの作成に失敗しました。",
                e
            )
        })?;
        Ok(())
    }

    /// Delete a record of the database.
    pub async fn delete(self, pool: &Pool<MySql>) -> ThisResult<()> {
        sqlx::query(format!(r#"DELETE FROM cash_record WHERE id={}"#, self.id).as_str())
            .execute(pool)
            .await
            .map_err(|e| {
                err_with_msg!(
                    ErrorKinds::DataBaseError,
                    "Failed to delete a CashIORecord on the database.",
                    "データの削除に失敗しました。",
                    e
                )
            })?;
        Ok(())
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
            updated_at: Some(updated_at),
        })
    }
}

#[derive(MySerialize, MyDeserialize)]
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
