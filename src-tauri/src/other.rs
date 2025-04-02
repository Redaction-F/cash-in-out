use std::{fmt::{Debug, Display}, error};
use serde::Serialize;

pub type ThisResult<T> = Result<T, Error>;

// エラー型
#[derive(Debug)]
pub struct Error {
    error_msg: ErrorMsg, 
    error_kind: ErrorKinds
}

impl Error {
    // 自分で定義したメッセージからエラー型を作成
    pub fn from_msg(kind: ErrorKinds, msg_for_developer: &str, msg_for_user: &str) -> Error {
        Error { 
            error_msg: ErrorMsg::new(msg_for_developer, msg_for_user), 
            error_kind: kind 
        }
    }

    // 自分で定義したメッセージ+エラーの原因からエラー型を作成
    pub fn from_into_string<T>(kind: ErrorKinds, msg_for_developer: &str, msg_for_user: &str, into_string: T) -> Error
        where
            T: Display {
        let msg_for_developer: String = format!("{} ({})", msg_for_developer, into_string.to_string());
        Error::from_msg(kind, &msg_for_developer, msg_for_user)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}: {}", self.error_kind, self.error_msg)
    }
}

impl error::Error for Error {}

// エラーをjsonデータに変換
impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        serializer.serialize_str(format!("{}", self.error_msg.for_user).as_str())
    }
}

// エラー種
// DevepolerError
// A: 日付処理
//      01: CashRecord::read_by_month
// B: カテゴリ処理
//      01: Category::new
#[derive(Debug)]
pub enum ErrorKinds {
    ReadCsvError, 
    DataBaseError, 
    TypeError, 
    CategoryError, 
    DeveloperError
}

impl Display for ErrorKinds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            ErrorKinds::ReadCsvError => "ReadCsvError", 
            ErrorKinds::DataBaseError => "DataBaseError", 
            ErrorKinds::TypeError => "TypeError", 
            ErrorKinds::CategoryError => "CategoryError", 
            ErrorKinds::DeveloperError => "DeveloperError"
        })
    }
}

#[derive(Debug)]
pub struct ErrorMsg {
    for_developer: String, 
    for_user: String
}

impl ErrorMsg {
    fn new(msg_for_developer: &str, msg_for_user: &str) -> ErrorMsg {
        ErrorMsg { 
            for_developer: String::from(msg_for_developer), 
            for_user: String::from(msg_for_user) 
        }
    }
}

impl Display for ErrorMsg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.for_developer)
    }
}
