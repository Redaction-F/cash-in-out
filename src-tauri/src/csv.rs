//! The mod for read and write a csv file.

// import for file control
use std::fs::File;
// import for date
use chrono::Local;
// import for csv
use csv::{Reader, Writer};
// this crate
use crate::{
    // CashIO
    cash_io::CashIORecord,
    // other
    other::{err_with_msg, ErrorKinds, ThisResult},
};

/// Write records of `CashIO` on a csv file.
pub fn write_in_csv(records: Vec<CashIORecord>, path: String) -> ThisResult<()> {
    // create a file
    let mut writer: Writer<File> =
        csv::Writer::from_writer(File::create(path).map_err(|e| {
            err_with_msg!(
                ErrorKinds::FileError,
                "Failed to open a file.",
                "ファイルの書き出しに失敗しました。",
                e
            )
        })?);

    for v in records.into_iter() {
        // write data on the file
        writer.serialize(v).map_err(|e| {
            err_with_msg!(
                ErrorKinds::FileError,
                "Failed to write in a file.",
                "ファイルの書き出しに失敗しました。",
                e
            )
        })?;
    }
    Ok(())
}

/// Read records of `CashIO from a csv file.
pub fn read_from_csv(file_name: String) -> ThisResult<Vec<CashIORecord>> {
    let mut reader: Reader<File> =
        csv::Reader::from_reader(File::open(file_name).map_err(|e| {
            err_with_msg!(
                ErrorKinds::FileError,
                "Failed to open a file.",
                "ファイルのアクセスに失敗しました。",
                e
            )
        })?);
    reader
        .deserialize()
        .collect::<Result<Vec<CashIORecord>, csv::Error>>()
        .map_err(|e| {
            err_with_msg!(
                ErrorKinds::FileError,
                "Failed to read a file as the data.",
                "ファイルの読み込みに失敗しました。",
                e
            )
        })
}
