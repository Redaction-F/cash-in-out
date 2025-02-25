use chrono::{Local, NaiveDate};
use sqlx::{MySql, Pool};
use crate::{
    database::{connect_db, CashRecord, NaiveDateWrapper}, 
    other::ThisResult
};

// Vec<bool>型の値のtrueの数を数える
#[tauri::command]
pub fn count_true(vec: Vec<bool>) -> usize {
    vec.into_iter().filter(|&v| v).count()
}

#[tauri::command]
pub async fn first_get_from_db() -> ThisResult<Vec<CashRecord>> {
    // データベースと通信確立
    let pool: Pool<MySql> = connect_db().await?;
    // データベースを今月分読む
    CashRecord::read_db_from_month(&pool, Local::now().date_naive()).await
}

#[tauri::command]
pub async fn get_in_month_from_db(date: NaiveDateWrapper<'_>) -> ThisResult<Vec<CashRecord>> {
    // データベースと通信確立
    let pool: Pool<MySql> = connect_db().await?;
    // データベースを今月分読む
    CashRecord::read_db_from_month(&pool, <NaiveDateWrapper as Into<NaiveDate>>::into(date)).await
}

#[tauri::command]
pub async fn get_one_from_db(id: usize) -> ThisResult<Option<CashRecord>> {
    // データベースと通信確立
    let pool: Pool<MySql> = connect_db().await?;
    // idからデータを選択
    CashRecord::read_db_from_id(&pool, id).await
}

#[tauri::command]
pub async fn update_one_from_db(changed_record: CashRecord) -> ThisResult<()> {
    // データベースと通信確立
    let pool: Pool<MySql> = connect_db().await?;
    CashRecord::update_db_one(&pool, changed_record).await
}