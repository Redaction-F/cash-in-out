use std::{fmt::{Debug, Display}, error};
use serde::Serialize;

// エラー型
pub enum Error {
    ReadCsvError(String)
}

impl Error {
    // 自分で定義したメッセージからエラー型を作成
    pub fn from_msg(kind: ErrorKinds, msg: &str) -> Error {
        match kind {
            ErrorKinds::ReadCsvError => Error::ReadCsvError(format!("{}", msg))
        }
    }

    // 自分で定義したメッセージ+エラーの原因からエラー型を作成
    pub fn from_into_string<T>(kind: ErrorKinds, msg: &str, into_string: T) -> Error
        where
            T: Display {
        match kind {
            ErrorKinds::ReadCsvError => Error::ReadCsvError(format!("{} ({})", msg, into_string.to_string()))
        }
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Error::ReadCsvError(e) => format!("ReadCsvError: {}", e)
        };
        write!(f, "{}", msg)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Error::ReadCsvError(e) => format!("ReadCsvError: {}", e)
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
            Error::ReadCsvError(rce_) => serializer.serialize_newtype_variant("RustError", 0, "ReadCsvError", rce_)
        }
    }
}

// エラー型(中身なし)
pub enum ErrorKinds {
    ReadCsvError
}