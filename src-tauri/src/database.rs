//! The mod for database controls.

// import for env
use std::env;
// import for database
use sqlx::{
    mysql::{MySql, MySqlPool},
    Pool,
};
// import for logging
use log::info;
// this crete
use crate::{
    // other
    other::{err_with_msg, ErrorKinds, ThisResult},
};

/// Connect a database which has `CashIO` records.
pub async fn connect_db() -> ThisResult<Pool<MySql>> {
    // get database url
    let database_url: String = env::var("DATABASE_URL").map_err(|e: env::VarError| {
        err_with_msg!(
            ErrorKinds::DeveloperError,
            r#"Failed to find env var "DATABASE_URL""#,
            "予期せぬエラーが発生しました。(E002)",
            e
        )
    })?;

    // connect the database
    let pool: Pool<MySql> = MySqlPool::connect(&database_url).await.map_err(|e| {
        err_with_msg!(
            ErrorKinds::DataBaseError,
            "Failed to connect the database",
            "データベースと通信できませんでした。データベースの状態を確認してください。",
            e
        )
    })?;

    // logging
    info!("Succeed in connecting database.");

    Ok(pool)
}

/// The trait for remove special chars for preventing SQL injections.
pub trait RemoveSpecialChars: Sized {
    /// Remove special chars which may occur a SQL injection.
    fn remove_special_chars(&self) -> Result<Self, Self>;
}

impl RemoveSpecialChars for String {
    fn remove_special_chars(&self) -> Result<Self, Self> {
        let res: String = self
            .chars()
            .filter(|c| !['\"', ';', '-'].contains(c))
            .collect::<String>();
        if res.len() == self.len() {
            Ok(res)
        } else {
            Err(res)
        }
    }
}
