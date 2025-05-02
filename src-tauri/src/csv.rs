// ファイル関連
use std::fs::File;
use chrono::Local;
// csv関連
use csv::{Reader, Writer};
// logging用
use log::error;
// このcraet
use crate::{
    cash_io::CashIORecord, 
    other::{Error, ErrorKinds, ThisResult}
};

pub fn write_in_csv(records: Vec<CashIORecord>) -> ThisResult<()> {
    let file_name: String = format!("{}.csv", Local::now().format("%Y_%m_%d_%H%M%S").to_string());
    let mut writer: Writer<File> = csv::Writer::from_writer(File::create(file_name)
        .map_err(|e| {
            let e = Error::from_into_string(
                ErrorKinds::FileError, 
                "Failed to open file.", 
                "ファイルの書き出しに失敗しました。", 
                e
            );
            error!("{}", e);
            e
        })?);
    for v in records.into_iter() {
        writer.serialize(v)
            .map_err(|e| {
                let e = Error::from_into_string(
                    ErrorKinds::FileError, 
                    "Failed to write in file.", 
                    "ファイルの書き出しに失敗しました。", 
                    e
                );
                error!("{}", e);
                e
            })?;
    };
    Ok(())
}

pub fn read_from_csv(file_name: String) -> ThisResult<Vec<CashIORecord>> {
    let mut reader: Reader<File> = csv::Reader::from_reader(File::open(file_name).map_err(|e| {
        let e: Error = Error::from_into_string(
            ErrorKinds::FileError, 
            "Failed to open a file.", 
            "ファイルのアクセスに失敗しました。", 
            e
        );
        error!("{}", e);
        e
    })?);
    reader.deserialize().collect::<Result<Vec<CashIORecord>, csv::Error>>().map_err(|e| {
        let e: Error = Error::from_into_string(
            ErrorKinds::FileError, 
            "Failed to read a file as the data.", 
            "ファイルの読み込みに失敗しました。", 
            e
        );
        error!("{}", e);
        e
    })
}