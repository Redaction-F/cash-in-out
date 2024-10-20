use std::fs::File;
use csv::{Reader, ReaderBuilder, DeserializeRecordsIter};
use serde::{ser::{Serialize, SerializeStruct}, Deserialize};
use regex::{Regex, Captures};
use crate::other::Error;

#[tauri::command]
pub fn read_csv(data_title: String) -> Result<Vec<CsvData>, Error> {
    CsvData::read_csv(format!("data/{}.csv", data_title))
}

pub struct CsvData {
    id: usize, 
    date: Option<DateInLocal>, 
    category: String, 
    title: String, 
    amount: usize, 
    memo: String
}

impl CsvData {
    fn read_csv(data_title: String) -> Result<Vec<CsvData>, Error> {
        let file: File = match File::open(data_title) {
            Ok(o_) => o_, 
            Err(e_) => return Err(Error::from_into_string(Error::READ_CSV_ERROR, "can't read file", e_))
        };
        let mut rdr: Reader<File> = ReaderBuilder::new().from_reader(file);
        let iter: DeserializeRecordsIter<'_, File, RawCsvData> = rdr.deserialize::<RawCsvData>();
        iter.map(
            |v| match v {
                Ok(o_) => o_.into(), 
                Err(e_) => Err(Error::from_into_string(Error::READ_CSV_ERROR, "fail to parse csv data", e_))
            }
        ).collect::<Result<Vec<CsvData>, Error>>()
    }
}

impl Serialize for CsvData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        let mut s = serializer.serialize_struct("CsvData", 6)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("date", &self.date)?;
        s.serialize_field("category", &self.category)?;
        s.serialize_field("title", &self.title)?;
        s.serialize_field("amount", &self.amount)?;
        s.serialize_field("memo", &self.memo)?;
        s.end()
    }
}

struct DateInLocal {
    year: u16, 
    month: u8, 
    date: u8
}

impl DateInLocal {
    fn parse_from_string(string: String) -> Result<DateInLocal, Error> {
        const ERROR_MSG: &str = r#"fail to parse date"#;
        let re: Regex = match Regex::new(r"(\d{4})/(\d{1,2})/(\d{1,2})") {
            Ok(o_) => o_, 
            Err(e_) => return Err(Error::from_into_string(Error::READ_CSV_ERROR, ERROR_MSG, e_))
        };
        let cap: Captures = re.captures(&string).ok_or(Error::from_msg(Error::READ_CSV_ERROR, ERROR_MSG))?;
        Ok(
            DateInLocal { 
                year: match cap.get(1).ok_or(Error::from_msg(Error::READ_CSV_ERROR, ERROR_MSG))?.as_str().parse() {
                    Ok(o_) if 2000 <= o_ && o_ <= 2100 => o_, 
                    Ok(o_) => return Err(Error::from_into_string(Error::READ_CSV_ERROR, ERROR_MSG, o_)), 
                    Err(e_) => return Err(Error::from_into_string(Error::READ_CSV_ERROR, ERROR_MSG, e_))
                }, 
                month: match cap.get(2).ok_or(Error::from_msg(Error::READ_CSV_ERROR, ERROR_MSG))?.as_str().parse() {
                    Ok(o_) if 1 <= o_ && o_ <= 12 => o_, 
                    Ok(o_) => return Err(Error::from_into_string(Error::READ_CSV_ERROR, ERROR_MSG, o_)), 
                    Err(e_) => return Err(Error::from_into_string(Error::READ_CSV_ERROR, ERROR_MSG, e_))
                }, 
                date: match cap.get(3).ok_or(Error::from_msg(Error::READ_CSV_ERROR, ERROR_MSG))?.as_str().parse() {
                    Ok(o_) if 1 <= o_ && o_ <= 31 => o_, 
                    Ok(o_) => return Err(Error::from_into_string(Error::READ_CSV_ERROR, ERROR_MSG, o_)), 
                    Err(e_) => return Err(Error::from_into_string(Error::READ_CSV_ERROR, ERROR_MSG, e_))
                }
            }
        )
    }
}

impl Serialize for DateInLocal {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        serializer.serialize_str(&format!("{}/{}/{}", self.year, self.month, self.date))
    }
}

#[derive(Deserialize)]
struct RawCsvData {
    id: String, 
    date: String, 
    category: String, 
    title: String, 
    amount: String, 
    memo: String
}

impl Into<Result<CsvData, Error>> for RawCsvData {
    fn into(self) -> Result<CsvData, Error> {
        Ok(CsvData { 
            id: match self.id.parse() {
                Ok(o_) => o_, 
                Err(e_) => return Err(Error::from_into_string(Error::READ_CSV_ERROR, "fail to parse index", e_))
            }, 
            date: DateInLocal::parse_from_string(self.date).ok(), 
            category: self.category, 
            title: self.title, 
            amount: match self.amount.parse() {
                Ok(o_) => o_, 
                Err(e_) => return Err(Error::from_into_string(Error::READ_CSV_ERROR, "fail to parse amount", e_))
            }, 
            memo: self.memo
        })
    }
}