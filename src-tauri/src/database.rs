// 環境変数管理
use std::env;
// DB操作用
use sqlx::{
    // MySQLとの通信用
    mysql::{MySql, MySqlPool}, 
    Pool
};
// logging用
use log::{error, info};
// このcrate
use crate::other::{Error, ErrorKinds, ThisResult};

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

// インジェクション防止用等に特定の文字を除く
pub fn remove_special_chars(value: &String) -> Result<String, String> {
    let res: String = value.chars().filter(|c| {
        !['\"', ';', '-'].contains(c)
    }).collect::<String>();
    if res.len() == value.len() {
        Ok(res)
    } else {
        Err(res)
    }
}
