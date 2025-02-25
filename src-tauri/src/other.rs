use std::{fmt::{Debug, Display}, error};
use serde::Serialize;

pub type ThisResult<T> = Result<T, Error>;

// エラー型
pub enum Error {
    ReadCsvError(String), 
    DataBaseError(String), 
    DeveloperError(String)
}

impl Error {
    // 自分で定義したメッセージからエラー型を作成
    pub fn from_msg(kind: ErrorKinds, msg: &str) -> Error {
        match kind {
            ErrorKinds::ReadCsvError => Error::ReadCsvError(String::from(msg)), 
            ErrorKinds::DataBaseError => Error::DataBaseError(String::from(msg)), 
            ErrorKinds::DeveloperError => Error::DeveloperError(String::from(msg)), 
        }
    }

    // 自分で定義したメッセージ+エラーの原因からエラー型を作成
    pub fn from_into_string<T>(kind: ErrorKinds, msg: &str, into_string: T) -> Error
        where
            T: Display {
        let msg: String = format!("{} ({})", msg, into_string.to_string());
        Error::from_msg(kind, &msg)
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Error as Display>::fmt(self, f)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg: String = match self {
            Error::ReadCsvError(e) => format!("ReadCsvError: {}", e), 
            Error::DataBaseError(e) => format!("DataBaseError: {}", e), 
            Error::DeveloperError(e) => format!("DeveloperError: {}", e), 
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for Error {}

// エラーを文字列データに変換
impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        match self {
            Error::ReadCsvError(e) => serializer.serialize_newtype_variant("RustError", 0, "ReadCsvError", e), 
            Error::DataBaseError(e) => serializer.serialize_newtype_variant("RustError", 0, "DataBaseError", e), 
            Error::DeveloperError(e) => serializer.serialize_newtype_variant("RustError", 0, "DeveloperError", e), 
        }
    }
}

// エラー型(中身なし)
pub enum ErrorKinds {
    ReadCsvError, 
    DataBaseError, 
    DeveloperError
}