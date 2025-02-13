use std::fs::File;
use log::{error, warn};
use csv::{Reader, ReaderBuilder};
use serde::{de::{self, Visitor}, ser::{Serialize, SerializeStruct}, Deserialize};
use regex::{Regex, Captures};
use crate::other::{Error, ErrorKinds};

// csv読み込み
#[tauri::command]
pub fn read_csv(data_title: String) -> Result<Vec<CsvData>, Error> {
    CsvData::read_csv(format!("data/{}.csv", data_title))
}

// csvデータ
pub struct CsvData {
    id: usize, 
    date: Option<DateInLocal>, 
    category: String, 
    title: String, 
    amount: usize, 
    memo: String
}

impl CsvData {
    const FIELDS: [&'static str; 6] = ["id", "date", "category", "title", "amount", "memo"];

    // csv読み込み
    fn read_csv(data_path: String) -> Result<Vec<CsvData>, Error> {
        let file: File = File::open(data_path)
            .map_err(|e| {
                let e: Error = Error::from_into_string(ErrorKinds::ReadCsvError, "can't read file", e);
                error!("{}", e);
                e
            })?;
        let mut rdr: Reader<File> = ReaderBuilder::new().from_reader(file);
        let mut data: Vec<CsvData> = rdr
            .deserialize::<CsvData>()
            .collect::<Result<Vec<CsvData>, csv::Error>>()
            .map_err(|e| {
                let e: Error = Error::from_into_string(ErrorKinds::ReadCsvError, "fail to parse csv", e);
                error!("{}", e);
                e
            })?;
        data.sort_by_key(|v| v.id);
        Ok(data)
    }
}

// CsvDataを文字列データに変換
impl Serialize for CsvData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        let mut s = serializer.serialize_struct("CsvData", 6)?;
        s.serialize_field("id", &self.id).map_err(|e| { error!("{}", e); e })?;
        s.serialize_field("date", &self.date).map_err(|e| { error!("{}", e); e })?;
        s.serialize_field("category", &self.category).map_err(|e| { error!("{}", e); e })?;
        s.serialize_field("title", &self.title).map_err(|e| { error!("{}", e); e })?;
        s.serialize_field("amount", &self.amount).map_err(|e| { error!("{}", e); e })?;
        s.serialize_field("memo", &self.memo).map_err(|e| { error!("{}", e); e })?;
        s.end()
    }
}

// 文字列データからCsvDataに変換
impl<'de> Deserialize<'de> for CsvData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: de::Deserializer<'de> {
        deserializer.deserialize_struct("Row2", &CsvData::FIELDS, CsvDataVistor)
    }

}

// CsvData as Deserialize用
struct CsvDataVistor;

impl<'de> Visitor<'de> for CsvDataVistor {
    type Value = CsvData;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "fields: id, date, category, title, amount, memo")
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::MapAccess<'de>, {
        let mut map: A = map;
        let mut id: Option<usize> = None;
        let mut date: Option<Option<DateInLocal>> = None;
        let mut category: Option<String> = None;
        let mut title: Option<String> = None;
        let mut amount: Option<usize> = None;
        let mut memo: Option<String> = None;
        while let Some(key) = map.next_key::<String>()? {
            match key.to_lowercase().as_str() {
                "id" => {
                    if id.is_some() {
                        let e = de::Error::duplicate_field("id");
                        warn!("{}", e);
                        return Err(e);
                    };
                    id = Some(map.next_value::<usize>().map_err(|e| { error!("{}", e); e })?)
                }, 
                "date" => {
                    if date.is_some() {
                        let e = de::Error::duplicate_field("date");
                        warn!("{}", e);
                        return Err(e);
                    };
                    date = match map.next_value::<String>().map_err(|e| { error!("{}", e); e })?.try_into() {
                        Ok(v) => Some(Some(v)), 
                        Err(e) => {
                            warn!("{}", e);
                            Some(None)
                        }
                    }
                }, 
                "category" => {
                    if category.is_some() {
                        let e = de::Error::duplicate_field("category");
                        warn!("{}", e);
                        return Err(e);
                    };
                    category = Some(map.next_value::<String>().map_err(|e| { error!("{}", e); e })?)
                }, 
                "title" => {
                    if title.is_some() {
                        let e = de::Error::duplicate_field("title");
                        warn!("{}", e);
                        return Err(e);
                    };
                    title = Some(map.next_value::<String>().map_err(|e| { error!("{}", e); e })?)
                }, 
                "amount" => {
                    if amount.is_some() {
                        let e = de::Error::duplicate_field("amount");
                        warn!("{}", e);
                        return Err(e);
                    };
                    amount = Some(map.next_value::<usize>().map_err(|e| { error!("{}", e); e })?)
                }, 
                "memo" => {
                    if memo.is_some() {
                        let e = de::Error::duplicate_field("memo");
                        warn!("{}", e);
                        return Err(e);
                    };
                    memo = Some(map.next_value::<String>().map_err(|e| { error!("{}", e); e })?)
                }, 
                v => {
                    let e = de::Error::unknown_field(v, &CsvData::FIELDS);
                    warn!("{}", e);
                    return Err(e);
                }
            }
        };
        let id: usize = id.ok_or_else(|| {
            let e = de::Error::missing_field("id");
            warn!("{}", e);
            e
        })?;
        let date: Option<DateInLocal> = date.ok_or_else(|| {
            let e = de::Error::missing_field("date");
            warn!("{}", e);
            e
        })?;
        let category: String = category.ok_or_else(|| {
            let e = de::Error::missing_field("category");
            warn!("{}", e);
            e
        })?;
        let title: String = title.ok_or_else(|| {
            let e = de::Error::missing_field("title");
            warn!("{}", e);
            e
        })?;
        let amount: usize = amount.ok_or_else(|| {
            let e = de::Error::missing_field("amount");
            warn!("{}", e);
            e
        })?;
        let memo: String = memo.ok_or_else(|| {
            let e = de::Error::missing_field("memo");
            warn!("{}", e);
            e
        })?;
        Ok(CsvData { 
            id, 
            date, 
            category, 
            title, 
            amount, 
            memo 
        })
    }
}

// 日付型
struct DateInLocal {
    year: u16, 
    month: u8, 
    date: u8
}

impl TryFrom<String> for DateInLocal {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        const ERROR_MSG: &str = r#"fail to parse this date data"#;
        let re: Regex = match Regex::new(r"(\d{4})/(\d{1,2})/(\d{1,2})") {
            Ok(o) => o, 
            Err(e) => {
                let e: Error = Error::from_into_string(ErrorKinds::ReadCsvError, ERROR_MSG, e);
                warn!("{}", e);
                return Err(e);
            }
        };
        let cap: Captures = re.captures(&value).ok_or(Error::from_msg(ErrorKinds::ReadCsvError, ERROR_MSG))?;
        Ok(
            DateInLocal { 
                year: match cap.get(1).ok_or(Error::from_msg(ErrorKinds::ReadCsvError, ERROR_MSG))?.as_str().parse() {
                    Ok(o) if (2000 <= o) && (o <= 2100) => o, 
                    Ok(o) => {
                        let e: Error = Error::from_into_string(ErrorKinds::ReadCsvError, ERROR_MSG, o);
                        warn!("{}", e);
                        return Err(e);
                    }, 
                    Err(e) => {
                        let e: Error = Error::from_into_string(ErrorKinds::ReadCsvError, ERROR_MSG, e);
                        warn!("{}", e);
                        return Err(e);
                    }
                }, 
                month: match cap.get(2).ok_or(Error::from_msg(ErrorKinds::ReadCsvError, ERROR_MSG))?.as_str().parse() {
                    Ok(o) if (1 <= o) && (o <= 12) => o, 
                    Ok(o) => {
                        let e: Error = Error::from_into_string(ErrorKinds::ReadCsvError, ERROR_MSG, o);
                        warn!("{}", e);
                        return Err(e);
                    }, 
                    Err(e) => {
                        let e: Error = Error::from_into_string(ErrorKinds::ReadCsvError, ERROR_MSG, e);
                        warn!("{}", e);
                        return Err(e);
                    }
                }, 
                date: match cap.get(3).ok_or(Error::from_msg(ErrorKinds::ReadCsvError, ERROR_MSG))?.as_str().parse() {
                    Ok(o) if (1 <= o) && (o <= 31) => o, 
                    Ok(o) => {
                        let e: Error = Error::from_into_string(ErrorKinds::ReadCsvError, ERROR_MSG, o);
                        warn!("{}", e);
                        return Err(e);
                    }, 
                    Err(e) => {
                        let e: Error = Error::from_into_string(ErrorKinds::ReadCsvError, ERROR_MSG, e);
                        warn!("{}", e);
                        return Err(e);
                    }
                }
            }
        )
    }
}

// DateInLocalを文字列データに変換
impl Serialize for DateInLocal {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        serializer.serialize_str(&format!("{}/{}/{}", self.year, self.month, self.date))
    }
}