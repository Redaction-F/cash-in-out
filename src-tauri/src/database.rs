// 環境変数管理
use std::env;
// Debug表示トレイト
use std::fmt::Debug;
// 時刻管理
use chrono::{Datelike, NaiveDate};
// Serialize(Frontendとの通信用)
use serde::ser::{Serialize, SerializeStruct};
// DB操作用
use sqlx::{
    // MySQLとの通信用
    mysql::{MySql, MySqlPool}, 
    FromRow, Pool, Row
};
// logging用
use log::{error, info};
// このcrate
use crate::other::{Error, ErrorKinds};

#[tauri::command]
pub async fn first_get_from_db() -> Result<Vec<CashRecord>, Error> {
    // データベースと通信確立
    let pool: Pool<MySql> = connect_db().await?;
    // データベースをすべて読む
    CashRecord::read_db_all(&pool).await
}

// データベースと通信確立
async fn connect_db() -> Result<Pool<MySql>, Error> {
    // 環境変数からデータベースURLを取得
    let database_url: String = env::var("DATABASE_URL")
        .map_err(|e| {
            let e: Error = Error::from_into_string(ErrorKinds::DataBaseError, "Failed to find env var \"DATABASE_URL\"", e);
            error!("{}", e);
            e
        })?;
    // データベースと通信
    let pool: Pool<MySql> = MySqlPool::connect(&database_url)
        .await
        .map_err(|e| {
            let e: Error = Error::from_into_string(ErrorKinds::DataBaseError, "Failed to connect database", e);
            error!("{}", e);
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
    memo: String
}

impl CashRecord {
    // データベースをすべて読む
    async fn read_db_all(pool: &Pool<MySql>) -> Result<Vec<CashRecord>, Error> {
        sqlx::query_as::<_, CashRecord>(r#"SELECT * from cash_record"#)
            .fetch_all(pool)
            .await
            .map_err(|e| {
                let e: Error = Error::from_into_string(ErrorKinds::DataBaseError, "Failed to get data from db.", e);
                error!("{}", e);
                e
            })
    }
}

// CashRecordを文字列データに変換(backendからfrontendへの通信用)
impl Serialize for CashRecord {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        let mut s = serializer.serialize_struct("CsvData", 6)?;
        s.serialize_field("id", &self.id).map_err(|e| { error!("{}", e); e })?;
        s.serialize_field("date", &<NaiveDateWrapper as From<&NaiveDate>>::from(&self.date)).map_err(|e| { error!("{}", e); e })?;
        s.serialize_field("category", &self.category).map_err(|e| { error!("{}", e); e })?;
        s.serialize_field("title", &self.title).map_err(|e| { error!("{}", e); e })?;
        s.serialize_field("amount", &self.amount).map_err(|e| { error!("{}", e); e })?;
        s.serialize_field("memo", &self.memo).map_err(|e| { error!("{}", e); e })?;
        s.end()
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
        let memo: String = row.try_get::<'_, Option<String>, _>("memo")?.unwrap_or_default();
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

// Serialize用
struct NaiveDateWrapper<'a> {
    value: &'a NaiveDate
}

impl NaiveDateWrapper<'_> {
    fn as_ref(&self) -> &NaiveDate {
        &self.value
    }
}

impl<'a> From<&'a NaiveDate> for NaiveDateWrapper<'a> {
    fn from(value: &'a NaiveDate) -> Self {
        NaiveDateWrapper { value }
    }
}

impl<'a> Serialize for NaiveDateWrapper<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        let value: &NaiveDate = self.as_ref();
        serializer.serialize_str(&format!("{}-{}-{}", value.year(), value.month(), value.day()))
    }
}