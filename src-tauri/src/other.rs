use std::{fmt::{Debug, Display}, error};
use serde::Serialize;

pub type ThisResult<T> = Result<T, Error>;

// エラー型
pub enum Error {
    ReadCsvError(ErrorMsg), 
    DataBaseError(ErrorMsg), 
    DeveloperError(ErrorMsg)
}

impl Error {
    // 自分で定義したメッセージからエラー型を作成
    pub fn from_msg(kind: ErrorKinds, msg_for_developer: &str, msg_for_user: &str) -> Error {
        match kind {
            ErrorKinds::ReadCsvError => Error::ReadCsvError(ErrorMsg::new(msg_for_developer, msg_for_user)), 
            ErrorKinds::DataBaseError => Error::DataBaseError(ErrorMsg::new(msg_for_developer, msg_for_user)), 
            ErrorKinds::DeveloperError => Error::DeveloperError(ErrorMsg::new(msg_for_developer, msg_for_user)), 
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

impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg: String = match self {
            Error::ReadCsvError(e) => format!("ReadCsvError: {:?}", e), 
            Error::DataBaseError(e) => format!("DataBaseError: {:?}", e), 
            Error::DeveloperError(e) => format!("DeveloperError: {:?}", e), 
        };
        write!(f, "{}", msg)
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
            Error::DataBaseError(e) => serializer.serialize_newtype_variant("RustError", 1, "DataBaseError", e), 
            Error::DeveloperError(e) => serializer.serialize_newtype_variant("RustError", 2, "DeveloperError", e), 
        }
    }
}

// エラー型(中身なし)
pub enum ErrorKinds {
    ReadCsvError, 
    DataBaseError, 
    DeveloperError
}

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

impl Debug for ErrorMsg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}(for user: {})", self.for_developer, self.for_user)
    }
}

impl Display for ErrorMsg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.for_developer)
    }
}

impl Serialize for ErrorMsg {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        serializer.serialize_str(&self.for_user)
    }
}