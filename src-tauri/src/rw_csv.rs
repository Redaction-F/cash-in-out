use std::fs::File;
use csv::{Reader, ReaderBuilder};
use serde::{de::{self, Visitor}, ser::{Serialize, SerializeStruct}, Deserialize};
use regex::{Regex, Captures};
use crate::other::{Error, ErrorDiscriminants};

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
            .map_err(|e| Error::from_into_string(ErrorDiscriminants::ReadCsvError, "can't read file", e))?;
        let mut rdr: Reader<File> = ReaderBuilder::new().from_reader(file);
        let mut data: Vec<CsvData> = rdr
            .deserialize::<CsvData>()
            .collect::<Result<Vec<CsvData>, csv::Error>>()
            .map_err(|e| Error::from_into_string(ErrorDiscriminants::ReadCsvError, "fail to parse csv", e))?;
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
        s.serialize_field("id", &self.id)?;
        s.serialize_field("date", &self.date)?;
        s.serialize_field("category", &self.category)?;
        s.serialize_field("title", &self.title)?;
        s.serialize_field("amount", &self.amount)?;
        s.serialize_field("memo", &self.memo)?;
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
        let mut date: Option<Result<DateInLocal, Error>> = None;
        let mut category: Option<String> = None;
        let mut title: Option<String> = None;
        let mut amount: Option<usize> = None;
        let mut memo: Option<String> = None;
        while let Some(key) = map.next_key::<String>()? {
            match key.to_lowercase().as_str() {
                "id" => {
                    if id.is_some() {
                        return Err(de::Error::duplicate_field("id"));
                    };
                    id = Some(map.next_value::<usize>()?)
                }, 
                "date" => {
                    if date.is_some() {
                        return Err(de::Error::duplicate_field("date"));
                    };
                    date = Some(map.next_value::<String>()?.try_into())
                }, 
                "category" => {
                    if category.is_some() {
                        return Err(de::Error::duplicate_field("category"));
                    };
                    category = Some(map.next_value::<String>()?)
                }, 
                "title" => {
                    if title.is_some() {
                        return Err(de::Error::duplicate_field("title"));
                    };
                    title = Some(map.next_value::<String>()?)
                }, 
                "amount" => {
                    if amount.is_some() {
                        return Err(de::Error::duplicate_field("amount"));
                    };
                    amount = Some(map.next_value::<usize>()?)
                }, 
                "memo" => {
                    if memo.is_some() {
                        return Err(de::Error::duplicate_field("memo"));
                    };
                    memo = Some(map.next_value::<String>()?)
                }, 
                v => return Err(de::Error::unknown_field(v, &CsvData::FIELDS))
            }
        };
        let id: usize = id.ok_or_else(|| de::Error::missing_field("id"))?;
        let date: Option<DateInLocal> = date.ok_or_else(|| de::Error::missing_field("date"))?.ok();
        let category: String = category.ok_or_else(|| de::Error::missing_field("category"))?;
        let title: String = title.ok_or_else(|| de::Error::missing_field("title"))?;
        let amount: usize = amount.ok_or_else(|| de::Error::missing_field("amount"))?;
        let memo: String = memo.ok_or_else(|| de::Error::missing_field("memo"))?;
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
            Err(e) => return Err(Error::from_into_string(ErrorDiscriminants::ReadCsvError, ERROR_MSG, e))
        };
        let cap: Captures = re.captures(&value).ok_or(Error::from_msg(ErrorDiscriminants::ReadCsvError, ERROR_MSG))?;
        Ok(
            DateInLocal { 
                year: match cap.get(1).ok_or(Error::from_msg(ErrorDiscriminants::ReadCsvError, ERROR_MSG))?.as_str().parse() {
                    Ok(o) if (2000 <= o) && (o <= 2100) => o, 
                    Ok(o) => return Err(Error::from_into_string(ErrorDiscriminants::ReadCsvError, ERROR_MSG, o)), 
                    Err(e) => return Err(Error::from_into_string(ErrorDiscriminants::ReadCsvError, ERROR_MSG, e))
                }, 
                month: match cap.get(2).ok_or(Error::from_msg(ErrorDiscriminants::ReadCsvError, ERROR_MSG))?.as_str().parse() {
                    Ok(o) if (1 <= o) && (o <= 12) => o, 
                    Ok(o) => return Err(Error::from_into_string(ErrorDiscriminants::ReadCsvError, ERROR_MSG, o)), 
                    Err(e) => return Err(Error::from_into_string(ErrorDiscriminants::ReadCsvError, ERROR_MSG, e))
                }, 
                date: match cap.get(3).ok_or(Error::from_msg(ErrorDiscriminants::ReadCsvError, ERROR_MSG))?.as_str().parse() {
                    Ok(o) if (1 <= o) && (o <= 31) => o, 
                    Ok(o) => return Err(Error::from_into_string(ErrorDiscriminants::ReadCsvError, ERROR_MSG, o)), 
                    Err(e) => return Err(Error::from_into_string(ErrorDiscriminants::ReadCsvError, ERROR_MSG, e))
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