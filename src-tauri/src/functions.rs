use chrono::NaiveDate;
use sqlx::{MySql, Pool};
use log::error;
use crate::{
    database::{connect_db, CashRecord}, 
    other::{Error, ErrorKinds, ThisResult}
};

#[tauri::command]
pub async fn get_records_by_month(year: usize, month: usize) -> ThisResult<Vec<CashRecord>> {
    // データベースと通信確立
    let pool: Pool<MySql> = connect_db().await?;
    // データベースを指定分読む
    CashRecord::read_records_by_month(&pool, NaiveDate::from_ymd_opt(year as i32, month as u32, 1).ok_or_else(|| {
        let e: Error = Error::from_msg(ErrorKinds::DeveloperError, "Invaid year or month", "日付の解析に失敗しました。開発者にお問い合わせください。");
        error!("{}", e);
        e
    })?).await
}

#[tauri::command]
pub async fn get_record_by_id(id: usize) -> ThisResult<Option<CashRecord>> {
    // データベースと通信確立
    let pool: Pool<MySql> = connect_db().await?;
    // idからデータを選択
    CashRecord::read_record_by_id(&pool, id).await
}

#[tauri::command]
pub async fn update_record(changed_record: CashRecord) -> ThisResult<()> {
    // データベースと通信確立
    let pool: Pool<MySql> = connect_db().await?;
    changed_record.update_record(&pool).await
}

#[tauri::command]
pub async fn create_record(new_record: CashRecord) -> ThisResult<()> {
    // データベースと通信確立
    let pool: Pool<MySql> = connect_db().await?;
    new_record.create_record(&pool).await
}