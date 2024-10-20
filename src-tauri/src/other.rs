use std::fmt::Display;
use serde::Serialize;

#[derive(Debug)]
pub enum Error {
    ReadCsvError(String)
}

impl Error {
    pub const READ_CSV_ERROR: Error = Error::ReadCsvError(String::new());

    pub fn from_msg(kind: Error, msg: &str) -> Error {
        match kind {
            Error::ReadCsvError(_) => Error::ReadCsvError(format!("Read Csv Error: {}", msg))
        }
    }

    pub fn from_into_string<T>(kind: Error, msg: &str, into_string: T) -> Error
        where
            T: Display {
        match kind {
            Error::ReadCsvError(_) => Error::ReadCsvError(format!("Read Csv Error: {} ({})", msg, into_string.to_string()))
        }
    }
}

// impl Display for Error {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", match self {
//             Error::ReadCsvError(rce_) => rce_
//         })
//     }
// }

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        match self {
            Error::ReadCsvError(rce_) => serializer.serialize_newtype_variant("RustError", 0, "ReadCsvError", rce_)
        }
    }
}