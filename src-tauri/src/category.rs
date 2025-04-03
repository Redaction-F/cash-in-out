// Debug, Display表示トレイト
use std::{collections::HashMap, fmt::{Debug, Display}};
// 時刻管理
use chrono::NaiveDateTime;
// FrontEnd通信用
use serde::{ser::SerializeStruct, Serialize};
// DB操作用
use sqlx::{FromRow, MySql, Pool, Row};
// logging用
use log::{error, warn};
// このクレート
use crate::{
    // データベース関連
    database::remove_special_chars, 
    // その他
    other::{Error, ErrorKinds, ThisResult}
};

// カテゴリ
#[derive(Debug)]
pub struct Category {
    main: MainCategory, 
    sub: SubCategory
}

#[allow(dead_code)]
impl Category {
    const SELECT_SQL: &str = "SELECT 
            main_category.id As main_id, 
            main_category.name As main_name, 
            main_category.created_at As main_created_at, 
            main_category.updated_at As main_updated_at, 
            sub_category.id As sub_id, 
            sub_category.name As sub_name, 
            sub_category.created_at As sub_created_at, 
            sub_category.updated_at As sub_updated_at 
        FROM sub_category 
        INNER JOIN main_category ON sub_category.super_category=main_category.id";

    // sub_categoryのidを取得
    fn get_id(&self) -> usize {
        self.sub.get_id()
    }

    // メインカテゴリを取得
    fn get_main(&self) -> &MainCategory {
        &self.main
    }

    // サブカテゴリを取得
    fn get_sub(&self) -> &SubCategory {
        &self.sub
    }

    // 所有権を奪ってmainとsubを取得
    fn get_tuple(self) -> (MainCategory, SubCategory) {
        (self.main, self.sub)
    }

    // 親子関係を確認して新規作成
    fn new(main_category: MainCategory, sub_category: SubCategory) -> ThisResult<Category> {
        if main_category.get_id() == sub_category.get_super_category() {
            Ok(Category { 
                main: main_category, 
                sub: sub_category 
            })
        } else {
            Err(Error::from_msg(
                ErrorKinds::DeveloperError, 
                "The main_category doesn't have the sub_category.", 
                "エラー: B-01"
            ))
        }
    }

    // すべてのカテゴリを取得
    async fn read_all(pool: &Pool<MySql>) -> ThisResult<Vec<Category>> {
        sqlx::query_as::<_, Category>(format!(
            r#"{}"#, 
            Category::SELECT_SQL
        ).as_str())
            .fetch_all(pool)
            .await
            .map_err(|e| {
                let e: Error = Error::from_into_string(
                    ErrorKinds::DataBaseError, 
                    "Failed to read Category from databsae", 
                    "データの取得に失敗しました。", 
                    e
                );
                error!("{}", e);
                e
            })
    }

    // 名前からカテゴリを取得
    pub async fn read_by_name(pool: &Pool<MySql>, main_category_name: &String, sub_category_name: &String) -> ThisResult<Category> {
        sqlx::query_as::<_, Category>(format!(
            r#"{} WHERE main_category.name="{}" AND sub_category.name="{}";"#, 
            Category::SELECT_SQL, 
            remove_special_chars(main_category_name)
                .unwrap_or_else(|e| { warn!(r#"MainCategory({}) contains '"', ';', '-'"#, main_category_name); e }), 
            remove_special_chars(sub_category_name)
                .unwrap_or_else(|e| { warn!(r#"SubCategory({}) contains '"', ';', '-'"#, sub_category_name); e }), 
        ).as_str())
            .fetch_one(pool)
            .await
            .map_or_else(|e| {
                let e: Error = match e {
                    sqlx::Error::RowNotFound => Error::from_into_string(
                        ErrorKinds::TypeError, 
                        "The category does not exist.", 
                        "そのカテゴリは存在しません。", 
                        e
                    ), 
                    e => Error::from_into_string(
                        ErrorKinds::DataBaseError, 
                        "Failed to get Category from database.", 
                        "データの取得に失敗しました。", 
                        e
                    )
                };
                error!("{:?}", e);
                Err(e)
            }, |v| Ok(v))
    }

    // 名前からカテゴリを取得(サブカテゴリは標準)
    pub async fn read_by_name_default(pool: &Pool<MySql>, main_category_name: &String) -> ThisResult<Category> {
        Category::read_by_name(pool, main_category_name, &SubCategory::DEFUALT_NAME.to_string()).await
    }

    // メインカテゴリを追加
    pub async fn add_main(pool: &Pool<MySql>, new_main_category_name: &String) -> ThisResult<()> {
        MainCategory::add(pool, new_main_category_name).await
    }

    // メインカテゴリを削除
    pub async fn remove_main(&self, pool: &Pool<MySql>) -> ThisResult<()> {
        self.main.remove(pool).await
    }

    // サブカテゴリを追加
    pub async fn add_sub(&self, pool: &Pool<MySql>, new_sub_category_name: &String) -> ThisResult<()> {
        SubCategory::add(pool, new_sub_category_name, &self.main).await
    }

    // サブカテゴリを削除
    pub async fn remove_sub(&self, pool: &Pool<MySql>) -> ThisResult<()> {
        self.sub.remove(pool).await
    }
}

impl Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.get_main(), self.get_sub())
    }
}

// データベースからCategoryに変換
impl<'r, R> FromRow<'r, R> for Category
    where
        R: Row,
        &'r str: sqlx::ColumnIndex<R>,
        i32: sqlx::Type<R::Database> + sqlx::Decode<'r, R::Database>,
        String: sqlx::Type<R::Database> + sqlx::Decode<'r, R::Database>,
        NaiveDateTime: sqlx::Type<R::Database> + sqlx::Decode<'r, R::Database>,
{
    fn from_row(row: &'r R) -> Result<Self, sqlx::Error> {
        let main_category: MainCategory = MainCategory::from_category_row(row)?;
        let sub_category: SubCategory = SubCategory::from_category_row(row)?;
        Ok(Category { 
            main: main_category, 
            sub: sub_category 
        })
    }
}

// メインカテゴリ
#[derive(Debug)]
struct MainCategory {
    id: usize, 
    name: String, 
    #[allow(dead_code)]
    created_at: NaiveDateTime, 
    #[allow(dead_code)]
    updated_at: NaiveDateTime
}

impl MainCategory {
    // idを取得
    fn get_id(&self) -> usize {
        self.id
    }

    // カテゴリ名を取得
    fn get_name(self) -> String {
        self.name
    }

    // カテゴリ名の参照を取得
    fn get_name_ref(&self) -> &String {
        &self.name
    }

    // CategoryのFromRow用
    fn from_category_row<'r, R>(row: &'r R) -> Result<MainCategory, sqlx::Error>
        where
            R: Row,
            &'r str: sqlx::ColumnIndex<R>,
            i32: sqlx::Type<R::Database> + sqlx::Decode<'r, R::Database>,
            String: sqlx::Type<R::Database> + sqlx::Decode<'r, R::Database>,
            NaiveDateTime: sqlx::Type<R::Database> + sqlx::Decode<'r, R::Database>,
    {
        let id: usize = <usize as TryFrom<i32>>::try_from(row.try_get::<'_, i32, _>("main_id")?)
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
        let name: String = row.try_get::<'_, String, _>("main_name")?;
        let created_at: NaiveDateTime = row.try_get::<'_, NaiveDateTime, _>("main_created_at")?;
        let updated_at: NaiveDateTime = row.try_get::<'_, NaiveDateTime, _>("main_updated_at")?;
        Ok(MainCategory { 
            id, 
            name, 
            created_at, 
            updated_at 
        })
    }

    // 追加
    async fn add(pool: &Pool<MySql>, new_main_category_name: &String) -> ThisResult<()> {
        let new_main_category_name: String = remove_special_chars(&new_main_category_name)
            .unwrap_or_else(|err| { warn!("MainCategory({}) contains \'\"\'", new_main_category_name); err });
        sqlx::query(format!(
            r#"INSERT INTO main_category (name) VALUES ("{}");"#, 
            new_main_category_name, 
        ).as_str())
            .execute(pool)
            .await
            .map_err(|e| {
                let e = match e {
                    sqlx::Error::Database(e) if e.is_unique_violation() => Error::from_into_string(
                        ErrorKinds::DataBaseError, 
                        "The new MainCategory already has inserted.", 
                        "このメインカテゴリは既に存在します。", 
                        e
                    ), 
                    e => Error::from_into_string(
                        ErrorKinds::DataBaseError, 
                        "Failed to create MainCategory on database.", 
                        "カテゴリの作成に失敗しました。", 
                        e
                    )
                };
                error!("{:?}", e);
                e
            })?;
        sqlx::query(format!(
            r#"INSERT INTO sub_category (name, super_category) SELECT "{}", id FROM main_category WHERE name="{}";"#, 
            SubCategory::DEFUALT_NAME, 
            new_main_category_name, 
        ).as_str())
            .execute(pool)
            .await
            .map_err(|e| {
                let e = Error::from_into_string(
                    ErrorKinds::DataBaseError, 
                    "Failed to create SubCategory on database.", 
                    "カテゴリの作成に失敗しました。", 
                    e
                );
                error!("{:?}", e);
                e
            })?;
        Ok(())
    }

    // 削除
    async fn remove(&self, pool: &Pool<MySql>) -> ThisResult<()> {
        sqlx::query_as::<_, SubCategory>(format!(
            r#"SELECT * FROM sub_category WHERE super_category={} AND name!="{}" LIMIT 1;"#, 
            self.get_id(), 
            SubCategory::DEFUALT_NAME
        ).as_str())
            .fetch_one(pool)
            .await
            .map_or_else(|e| {
                match e {
                    sqlx::Error::RowNotFound => Ok(()), 
                    e => {                        
                        let e: Error = Error::from_into_string(
                            ErrorKinds::DataBaseError, 
                            "Failed to remove MainCategory from database.", 
                            "カテゴリの削除に失敗しました。", 
                            e
                        );
                        error!("{:?}", e);
                        Err(e)
                    }
                }
            }, |_| Err(Error::from_msg(
                ErrorKinds::CategoryError, 
                "The MainCategory has some SubCategorys.",
                "サブカテゴリを持つメインカテゴリは削除できません。"
            )))?;
        sqlx::query(format!(
            r#"DELETE FROM sub_category WHERE super_category={} AND name="{}";"#, 
            self.get_id(), 
            SubCategory::DEFUALT_NAME
        ).as_str())
            .execute(pool)
            .await
            .map_err(|e| {
                let e: Error = Error::from_into_string(
                    ErrorKinds::DataBaseError, 
                    "Failed to create SubCategory from databaseb.", 
                    "カテゴリの削除に失敗しました。", 
                    e
                );
                error!("{:?}", e);
                e
            })?;
        sqlx::query(format!(
            r#"DELETE FROM main_category WHERE id={};"#, 
            self.get_id()
        ).as_str())
            .execute(pool)
            .await
            .map_err(|e| {
                let e: Error = Error::from_into_string(
                    ErrorKinds::DataBaseError, 
                    "Failed to create MainCategory from database.", 
                    "カテゴリの削除に失敗しました。", 
                    e
                );
                error!("{:?}", e);
                e
            })?;
        Ok(())
    }
}

impl Display for MainCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_name_ref())
    }
}

// データベースからメインカテゴリに変換
impl<'r, R> FromRow<'r, R> for MainCategory 
    where 
        R: Row, 
        &'r str: sqlx::ColumnIndex<R>,
        i32: sqlx::Type<R::Database> + sqlx::Decode<'r, R::Database>,
        String: sqlx::Type<R::Database> + sqlx::Decode<'r, R::Database>,
        NaiveDateTime: sqlx::Type<R::Database> + sqlx::Decode<'r, R::Database>
{
    fn from_row(row: &'r R) -> Result<Self, sqlx::Error> {
        let id: usize = <usize as TryFrom<i32>>::try_from(row.try_get::<'_, i32, _>("id")?)
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
        let name: String = row.try_get::<'_, String, _>("name")?;
        let created_at: NaiveDateTime = row.try_get::<'_, NaiveDateTime, _>("created_at")?;
        let updated_at: NaiveDateTime = row.try_get::<'_, NaiveDateTime, _>("updated_at")?;
        Ok(MainCategory {
            id, 
            name, 
            created_at, 
            updated_at
        })
    }
}

// サブカテゴリ
#[derive(Debug)]
struct SubCategory {
    id: usize, 
    name: String, 
    super_category: usize, 
    #[allow(dead_code)]
    created_at: NaiveDateTime, 
    #[allow(dead_code)]
    updated_at: NaiveDateTime, 
}

impl SubCategory {
    const DEFUALT_NAME: &str = "その他";

    // idを取得
    fn get_id(&self) -> usize {
        self.id
    }

    // カテゴリ名を取得
    fn get_name(self) -> String {
        self.name
    }

    // カテゴリ名の参照を取得
    fn get_name_ref(&self) -> &String {
        &self.name
    }

    // 親idを取得
    fn get_super_category(&self) -> usize {
        self.super_category
    }

    // CategoryのFromRow用
    fn from_category_row<'r, R>(row: &'r R) -> Result<SubCategory, sqlx::Error>
        where
            R: Row,
            &'r str: sqlx::ColumnIndex<R>,
            i32: sqlx::Type<R::Database> + sqlx::Decode<'r, R::Database>,
            String: sqlx::Type<R::Database> + sqlx::Decode<'r, R::Database>,
            NaiveDateTime: sqlx::Type<R::Database> + sqlx::Decode<'r, R::Database>,
    {
        let id: usize = <usize as TryFrom<i32>>::try_from(row.try_get::<'_, i32, _>("sub_id")?)
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
        let name: String = row.try_get::<'_, String, _>("sub_name")?;
        let super_category: usize = <usize as TryFrom<i32>>::try_from(row.try_get::<'_, i32, _>("main_id")?)
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
        let created_at: NaiveDateTime = row.try_get::<'_, NaiveDateTime, _>("sub_created_at")?;
        let updated_at: NaiveDateTime = row.try_get::<'_, NaiveDateTime, _>("sub_updated_at")?;
        Ok(SubCategory { 
            id, 
            name, 
            super_category, 
            created_at, 
            updated_at 
        })
    }

    // 標準サブカテゴリか判定
    fn is_default(&self) -> bool {
        self.name.as_str() == SubCategory::DEFUALT_NAME
    }

    // 追加
    async fn add(pool: &Pool<MySql>, new_sub_category_name: &String, main_category: &MainCategory) -> ThisResult<()> {
        sqlx::query(format!(
            r#"INSERT INTO sub_category (name, super_category) VALUES ("{}", {});"#, 
            remove_special_chars(&new_sub_category_name).unwrap_or_else(|err| { warn!(r#"SubCategory({}) contains '"', ';', '-'"#, new_sub_category_name); err }), 
            main_category.get_id()
        ).as_str())
            .execute(pool)
            .await
            .map_err(|e| {
                let e = match e {
                    sqlx::Error::Database(e) if e.is_unique_violation() => Error::from_into_string(
                        ErrorKinds::DataBaseError, 
                        "The new SubCategory already has inserted.", 
                        "このサブカテゴリは既に存在します。", 
                        e
                    ), 
                    sqlx::Error::Database(e) if e.is_foreign_key_violation() => Error::from_into_string(
                        ErrorKinds::DataBaseError, 
                        "The MainCategory already has deleted.", 
                        "このメインカテゴリは既に削除されています。", 
                        e
                    ), 
                    e => Error::from_into_string(
                        ErrorKinds::DataBaseError, 
                        "Failed to create SubCategory on database.", 
                        "カテゴリの作成に失敗しました。", 
                        e
                    )
                };
                error!("{:?}", e);
                e
            })?;
        Ok(())
    }

    // 削除
    async fn remove(&self, pool: &Pool<MySql>) -> ThisResult<()> {
        if self.is_default() {
            return Err(Error::from_msg(
                ErrorKinds::CategoryError, 
                "Can't remove default SubCategory.", 
                "標準のサブカテゴリは削除できません。"
            ));
        };
        sqlx::query(format!(
            r#"DELETE FROM sub_category WHERE id={};"#, 
            self.get_id()
        ).as_str())
            .execute(pool)
            .await
            .map_err(|e| {
                let e: Error = Error::from_into_string(
                    ErrorKinds::DataBaseError, 
                    "Failed to remove SubCategory from database.", 
                    "カテゴリの削除に失敗しました。", 
                    e
                );
                error!("{:?}", e);
                e
            })?;
        Ok(())
    }
}

impl Display for SubCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_name_ref())
    }
}

// データベースからサブカテゴリに変換
impl<'r, R> FromRow<'r, R> for SubCategory
    where 
        R: Row, 
        &'r str: sqlx::ColumnIndex<R>,
        i32: sqlx::Type<R::Database> + sqlx::Decode<'r, R::Database>,
        String: sqlx::Type<R::Database> + sqlx::Decode<'r, R::Database>,
        NaiveDateTime: sqlx::Type<R::Database> + sqlx::Decode<'r, R::Database>
{
    fn from_row(row: &'r R) -> Result<Self, sqlx::Error> {
        let id: usize = <usize as TryFrom<i32>>::try_from(row.try_get::<'_, i32, _>("id")?)
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
        let name: String = row.try_get::<'_, String, _>("name")?;
        let super_category: usize = <usize as TryFrom<i32>>::try_from(row.try_get::<'_, i32, _>("super_category")?)
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
        let created_at: NaiveDateTime = row.try_get::<'_, NaiveDateTime, _>("created_at")?;
        let updated_at: NaiveDateTime = row.try_get::<'_, NaiveDateTime, _>("updated_at")?;
        Ok(SubCategory {
            id, 
            name, 
            super_category, 
            created_at, 
            updated_at
        })
    }
}

pub struct MainCategoryWithSubs {
    name: String, 
    subs: Vec<String>
}

impl MainCategoryWithSubs {
    fn new(main_name: String) -> MainCategoryWithSubs {
        MainCategoryWithSubs { 
            name: main_name, 
            subs: Vec::new() 
        }
    }

    pub async fn read_all(pool: &Pool<MySql>) -> ThisResult<Vec<MainCategoryWithSubs>> {
        let mut all_category: Vec<Category> = Category::read_all(pool).await?;
        all_category.sort_by_key(|v| v.get_id());
        let mut categorys_hashmap: HashMap<usize, MainCategoryWithSubs> = HashMap::new();
        for (main, sub) in all_category.into_iter().map(|v| v.get_tuple()) {
            let main_with_subs: &mut MainCategoryWithSubs = categorys_hashmap
                .entry(main.get_id())
                .or_insert(<MainCategoryWithSubs as From<MainCategory>>::from(main));
            main_with_subs.push(sub);
        }
        let mut categorys: Vec<(usize, MainCategoryWithSubs)> = categorys_hashmap.into_iter().collect::<Vec<(usize, MainCategoryWithSubs)>>();
        categorys.sort_by_key(|&(i, _)| i);
        Ok(
            categorys
                .into_iter()
                .map(|(_, v)| v)
                .collect::<Vec<MainCategoryWithSubs>>()
        )
    }

    fn push(&mut self, value: SubCategory) {
        self.subs.push(value.get_name());
    }
}

impl Serialize for MainCategoryWithSubs {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        let mut s: <S as serde::Serializer>::SerializeStruct = serializer.serialize_struct("MainCategoryWithSubs", 2)?;
        s.serialize_field("name", &self.name)?;
        s.serialize_field("subs", &self.subs)?;
        s.end()
    }
}

impl From<MainCategory> for MainCategoryWithSubs {
    fn from(value: MainCategory) -> Self {
        MainCategoryWithSubs::new(value.get_name())
    }
}
