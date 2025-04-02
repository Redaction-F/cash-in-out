use chrono::NaiveDate;
use sqlx::{MySql, Pool};
use log::error;
use crate::{
    cash_io::CashIORecord, category::{Category, MainCategoryWithSubs}, database::connect_db, other::{Error, ErrorKinds, ThisResult}
};

#[tauri::command]
pub async fn get_records_by_month(year: usize, month: usize) -> ThisResult<Vec<CashIORecord>> {
    // データベースと通信確立
    let pool: Pool<MySql> = connect_db().await?;
    // データベースを指定月分読む
    CashIORecord::read_by_month(&pool, NaiveDate::from_ymd_opt(year as i32, month as u32, 1)
        .ok_or_else(|| {
            let e: Error = Error::from_msg(ErrorKinds::DeveloperError, "Invaid year or month", "日付の解析に失敗しました。開発者にお問い合わせください。");
            error!("{}", e);
            e
        })?)
        .await
}

#[tauri::command]
pub async fn get_record_by_id(id: usize) -> ThisResult<Option<CashIORecord>> {
    // データベースと通信確立
    let pool: Pool<MySql> = connect_db().await?;
    // idからデータを選択
    CashIORecord::read_by_id(&pool, id).await
}

#[tauri::command]
pub async fn update_record(changed_record: CashIORecord) -> ThisResult<()> {
    // データベースと通信確立
    let pool: Pool<MySql> = connect_db().await?;
    // データを更新
    changed_record.update(&pool).await
}

#[tauri::command]
pub async fn create_record(new_record: CashIORecord) -> ThisResult<()> {
    // データベースと通信確立
    let pool: Pool<MySql> = connect_db().await?;
    // データを作成
    new_record.create(&pool).await
}

#[tauri::command]
pub async fn get_all_categorys() -> ThisResult<Vec<MainCategoryWithSubs>> {
    // データベースと通信確立
    let pool: Pool<MySql> = connect_db().await?;
    // カテゴリをすべて読む
    MainCategoryWithSubs::read_all(&pool).await
}

#[tauri::command]
pub async fn add_main_category(new_main_category_name: String) -> ThisResult<()> {
    // データベースと通信確立
    let pool: Pool<MySql> = connect_db().await?;
    // メインカテゴリを追加
    Category::add_main(&pool, &new_main_category_name).await
}

#[tauri::command]
pub async fn remove_main_category(main_category_name: String) -> ThisResult<()> {
    // データベースと通信確立
    let pool: Pool<MySql> = connect_db().await?;
    // メインカテゴリを削除
    Category::read_by_name_default(&pool, &main_category_name)
        .await?
        .remove_main(&pool)
        .await
}

#[tauri::command]
pub async fn add_sub_category(new_sub_category_name: String, main_category_name: String) -> ThisResult<()> {
    // データベースと通信確立
    let pool: Pool<MySql> = connect_db().await?;
    // サブカテゴリを追加
    Category::read_by_name_default(&pool, &main_category_name)
        .await?
        .add_sub(&pool, &new_sub_category_name)
        .await
}

#[tauri::command]
pub async fn remove_sub_category(sub_category_name: String, main_category_name: String) -> ThisResult<()> {
    // データベースと通信確立
    let pool: Pool<MySql> = connect_db().await?;
    // サブカテゴリを削除
    Category::read_by_name(&pool, &main_category_name, &sub_category_name)
        .await?
        .remove_sub(&pool)
        .await
}
