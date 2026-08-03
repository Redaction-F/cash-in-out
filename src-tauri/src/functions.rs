//! The mod for API functions.

// import for date
use chrono::NaiveDate;
// import for database
use sqlx::{MySql, Pool};
// this crate
use crate::{
    // database record
    cash_io::CashIORecord,
    // category
    category::{Category, MainCategoryWithSubs},
    // for reading and writing csv
    csv::{read_from_csv as read_from_csv_simple, write_in_csv as write_in_csv_simple},
    // database
    database::connect_db,
    // other
    other::{err_with_msg, ErrorKinds, ThisResult},
};

/// Get all records.
#[tauri::command]
pub async fn get_all_categorys() -> ThisResult<Vec<MainCategoryWithSubs>> {
    // connect the database
    let pool: Pool<MySql> = connect_db().await?;

    // select all records on the database
    MainCategoryWithSubs::select_all(&pool).await
}

/// Get all records for a month.
/// # Error
/// Parameter `year` or `month` is invaid.
#[tauri::command]
pub async fn get_records_by_month(year: usize, month: usize) -> ThisResult<Vec<CashIORecord>> {
    // connect the database
    let pool: Pool<MySql> = connect_db().await?;

    // select records for the month on the database
    CashIORecord::select_by_month(
        &pool,
        NaiveDate::from_ymd_opt(year as i32, month as u32, 1).ok_or_else(|| {
            err_with_msg!(
                ErrorKinds::DeveloperError,
                "Invaid year or month",
                "予期せぬエラーが発生しました。(E003)"
            )
        })?,
    )
    .await
}

/// Get a record with an id.
#[tauri::command]
pub async fn get_record_by_id(id: usize) -> ThisResult<Option<CashIORecord>> {
    // connect the database
    let pool: Pool<MySql> = connect_db().await?;

    // select a record with the id on the database
    CashIORecord::select_by_id(&pool, id).await
}

/// Get a sum for a month.
#[tauri::command]
pub async fn get_sum_by_month(year: usize, month: usize) -> ThisResult<isize> {
    // connect the database
    let pool: Pool<MySql> = connect_db().await?;

    // select a record with the id on the database
    CashIORecord::sum_by_month(
        &pool,
        NaiveDate::from_ymd_opt(year as i32, month as u32, 1).ok_or_else(|| {
            err_with_msg!(
                ErrorKinds::DeveloperError,
                "Invaid year or month",
                "予期せぬエラーが発生しました。(E003)"
            )
        })?,).await
}

/// Update a record.
#[tauri::command]
pub async fn update_record(changed_record: CashIORecord) -> ThisResult<()> {
    // connect the database
    let pool: Pool<MySql> = connect_db().await?;

    // update a record on the database
    changed_record.update(&pool).await
}

/// Create a record.
#[tauri::command]
pub async fn create_record(new_record: CashIORecord) -> ThisResult<()> {
    // connect the database
    let pool: Pool<MySql> = connect_db().await?;

    // insert a record on the database
    new_record.insert(&pool).await
}

/// Delete a record by an
///
///
///  id.
#[tauri::command]
pub async fn delete_record_by_id(id: usize) -> ThisResult<()> {
    // connect the database
    let pool: Pool<MySql> = connect_db().await?;

    // delete a record on the database
    CashIORecord::select_by_id(&pool, id)
        .await?
        .ok_or_else(|| {
            err_with_msg!(
                ErrorKinds::DataBaseError,
                "Failed to get a CashIORecord which has the id.",
                "そのIdのデータは既に存在しません。"
            )
        })?
        .delete(&pool)
        .await
}

/// Create a main category.
#[tauri::command]
pub async fn create_main_category(new_main_category_name: String) -> ThisResult<()> {
    // connect the database
    let pool: Pool<MySql> = connect_db().await?;

    // insert a main category on the database
    Category::insert_main(&pool, &new_main_category_name).await
}

/// Delete a main category.
#[tauri::command]
pub async fn delete_main_category(main_category_name: String) -> ThisResult<()> {
    // connect the database
    let pool: Pool<MySql> = connect_db().await?;

    // delete a main category on the database
    Category::select_by_name_default(&pool, &main_category_name)
        .await?
        .delete_main(&pool)
        .await
}

/// Create a sub category.
#[tauri::command]
pub async fn create_sub_category(
    new_sub_category_name: String,
    main_category_name: String,
) -> ThisResult<()> {
    // connect the database
    let pool: Pool<MySql> = connect_db().await?;

    // insert a sub category on the database
    Category::select_by_name_default(&pool, &main_category_name)
        .await?
        .insert_sub(&pool, &new_sub_category_name)
        .await
}

/// Delete a sub category.
#[tauri::command]
pub async fn delete_sub_category(
    sub_category_name: String,
    main_category_name: String,
) -> ThisResult<()> {
    // connect the database
    let pool: Pool<MySql> = connect_db().await?;

    // delete a sub category on the database
    Category::select_by_name(&pool, &main_category_name, &sub_category_name)
        .await?
        .delete_sub(&pool)
        .await
}

/// Write data of the datatbase on a csv file.
#[tauri::command]
pub async fn write_in_csv() -> ThisResult<()> {
    // connect the database
    let pool: Pool<MySql> = connect_db().await?;

    // select all records
    let records: Vec<CashIORecord> = CashIORecord::select_all(&pool).await?;

    // write on csv
    write_in_csv_simple(records)
}

/// Read data a csv file and create the data on the database.
#[tauri::command]
pub async fn read_from_csv(file_name: String) -> ThisResult<()> {
    // connect the database
    let pool: Pool<MySql> = connect_db().await?;

    // read csv
    let reader: Vec<CashIORecord> = read_from_csv_simple(file_name)?;

    // insert data on the database
    for v in reader.into_iter() {
        v.insert(&pool).await?;
    }
    Ok(())
}
