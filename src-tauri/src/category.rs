//! The mod for category `CashIO`.

use std::{
    // import for hash map
    collections::HashMap,
    // import for debug
    fmt::{Debug, Display},
};
// import for date
use chrono::NaiveDateTime;
// import for log
use log::warn;
// import for Serialize
use serde::Serialize;
// import for database
use sqlx::{FromRow, MySql, Pool, Row};
// this crate
use crate::{
    // database
    database::RemoveSpecialChars,
    other::{err_with_msg, err_with_msg_with_non_error, Error, ErrorKinds, ThisResult},
};

/// The full category of `CashIO`. This is the pair of main category and sub category.
#[derive(Debug)]
pub struct Category {
    main: MainCategory,
    sub: SubCategory,
}

#[allow(dead_code)]
impl Category {
    /// SQL statement for select Category
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

    /// Create a full category from main category and sub category.
    fn new(main_category: MainCategory, sub_category: SubCategory) -> ThisResult<Category> {
        if main_category.get_id() == sub_category.get_super_category() {
            Ok(Category {
                main: main_category,
                sub: sub_category,
            })
        } else {
            Err(Error::from_msg(
                ErrorKinds::DeveloperError,
                "The main_category doesn't have the sub_category.",
                "予期せぬエラーが発生しました(E004)",
            ))
        }
    }

    /// Get an id of sub category.
    fn get_id(&self) -> usize {
        self.sub.get_id()
    }

    /// Get a main category.
    fn get_main(&self) -> &MainCategory {
        &self.main
    }

    /// Get a sub category.
    fn get_sub(&self) -> &SubCategory {
        &self.sub
    }

    /// Select all full categorys from the database.
    async fn select_all(pool: &Pool<MySql>) -> ThisResult<Vec<Category>> {
        sqlx::query_as::<_, Category>(format!(r#"{}"#, Category::SELECT_SQL).as_str())
            .fetch_all(pool)
            .await
            .map_err(|e| {
                err_with_msg!(
                    ErrorKinds::DataBaseError,
                    "Failed to read Category from databsae",
                    "データの取得に失敗しました。",
                    e
                )
            })
    }

    /// Select a full category by a name of main category and sub category from the database.
    pub async fn select_by_name(
        pool: &Pool<MySql>,
        main_category_name: &String,
        sub_category_name: &String,
    ) -> ThisResult<Category> {
        sqlx::query_as::<_, Category>(
            format!(
                r#"{} WHERE main_category.name="{}" AND sub_category.name="{}";"#,
                Category::SELECT_SQL,
                main_category_name
                    .remove_special_chars()
                    .unwrap_or_else(|e| {
                        warn!(
                            r#"MainCategory({}) contains '"', ';', '-'"#,
                            main_category_name
                        );
                        e
                    }),
                sub_category_name
                    .remove_special_chars()
                    .unwrap_or_else(|e| {
                        warn!(
                            r#"SubCategory({}) contains '"', ';', '-'"#,
                            sub_category_name
                        );
                        e
                    }),
            )
            .as_str(),
        )
        .fetch_one(pool)
        .await
        .map_or_else(
            |e| {
                Err(err_with_msg!(
                    sqlx::Error::RowNotFound =>
                        ErrorKinds::TypeError,
                        "The category does not exist.",
                        "そのカテゴリは存在しません。";;
                    ErrorKinds::DataBaseError,
                    "Failed to get Category from database.",
                    "データの取得に失敗しました。",
                    e
                ))
            },
            |v| Ok(v),
        )
    }

    /// Select a full category with default sub category by a name of main category from the database.
    pub async fn select_by_name_default(
        pool: &Pool<MySql>,
        main_category_name: &String,
    ) -> ThisResult<Category> {
        Category::select_by_name(
            pool,
            main_category_name,
            &SubCategory::DEFUALT_NAME.to_string(),
        )
        .await
    }

    /// Insert a main category to the database.
    pub async fn insert_main(
        pool: &Pool<MySql>,
        new_main_category_name: &String,
    ) -> ThisResult<()> {
        MainCategory::insert(pool, new_main_category_name).await
    }

    /// Delete a main category of the database.
    pub async fn delete_main(&self, pool: &Pool<MySql>) -> ThisResult<()> {
        self.main.delete(pool).await
    }

    /// Insert a sub category to the database.
    pub async fn insert_sub(
        &self,
        pool: &Pool<MySql>,
        new_sub_category_name: &String,
    ) -> ThisResult<()> {
        SubCategory::insert(pool, new_sub_category_name, &self.main).await
    }

    /// Delete a sub category of the database.
    pub async fn delete_sub(&self, pool: &Pool<MySql>) -> ThisResult<()> {
        self.sub.delete(pool).await
    }
}

// into (MainCategory, Subcategory)
impl Into<(MainCategory, SubCategory)> for Category {
    fn into(self) -> (MainCategory, SubCategory) {
        (self.main, self.sub)
    }
}

// display
impl Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.get_main(), self.get_sub())
    }
}

// convert a database row to a Category
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
            sub: sub_category,
        })
    }
}

/// A main category of `CashIO`.
#[derive(Debug)]
struct MainCategory {
    id: usize,
    name: String,
    #[allow(dead_code)]
    created_at: NaiveDateTime,
    #[allow(dead_code)]
    updated_at: NaiveDateTime,
}

impl MainCategory {
    /// Get an id.
    fn get_id(&self) -> usize {
        self.id
    }

    /// Get a name
    fn get_name_ref(&self) -> &String {
        &self.name
    }

    /// Convert from a `Category` database row.
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
            updated_at,
        })
    }

    /// Insert a record to the database.
    async fn insert(pool: &Pool<MySql>, new_main_category_name: &String) -> ThisResult<()> {
        let new_main_category_name: String = new_main_category_name
            .remove_special_chars()
            .unwrap_or_else(|err| {
                warn!("MainCategory({}) contains \'\"\'", new_main_category_name);
                err
            });
        sqlx::query(
            format!(
                r#"INSERT INTO main_category (name) VALUES ("{}");"#,
                new_main_category_name,
            )
            .as_str(),
        )
        .execute(pool)
        .await
        .map_err(|e| {
            err_with_msg!(
                sqlx::Error::Database(v) if v.is_unique_violation() =>
                    ErrorKinds::DataBaseError,
                    "The new MainCategory already has inserted.",
                    "このメインカテゴリは既に存在します。",
                    v;;
                ErrorKinds::DataBaseError,
                "Failed to create MainCategory on database.",
                "カテゴリの作成に失敗しました。",
                e
            )
        })?;
        sqlx::query(format!(
            r#"INSERT INTO sub_category (name, super_category) SELECT "{}", id FROM main_category WHERE name="{}";"#, 
            SubCategory::DEFUALT_NAME, 
            new_main_category_name, 
        ).as_str())
            .execute(pool)
            .await
            .map_err(|e| err_with_msg!(
                ErrorKinds::DataBaseError, 
                "Failed to create SubCategory on database.", 
                "カテゴリの作成に失敗しました。",
                e
            ))?;
        Ok(())
    }

    /// Delete a record of the database.
    async fn delete(&self, pool: &Pool<MySql>) -> ThisResult<()> {
        sqlx::query_as::<_, SubCategory>(
            format!(
                r#"SELECT * FROM sub_category WHERE super_category={} AND name!="{}" LIMIT 1;"#,
                self.get_id(),
                SubCategory::DEFUALT_NAME
            )
            .as_str(),
        )
        .fetch_one(pool)
        .await
        .map_or_else(
            |e| {
                err_with_msg_with_non_error!(
                    sqlx::Error::RowNotFound => ();
                    ErrorKinds::DataBaseError,
                    "Failed to remove MainCategory from database.",
                    "カテゴリの削除に失敗しました。",
                    e
                )
            },
            |_| {
                Err(Error::from_msg(
                    ErrorKinds::CategoryError,
                    "The MainCategory has some SubCategorys.",
                    "サブカテゴリを持つメインカテゴリは削除できません。",
                ))
            },
        )?;
        sqlx::query(
            format!(
                r#"DELETE FROM sub_category WHERE super_category={} AND name="{}";"#,
                self.get_id(),
                SubCategory::DEFUALT_NAME
            )
            .as_str(),
        )
        .execute(pool)
        .await
        .map_err(|e| {
            err_with_msg!(
                sqlx::Error::Database(v) if v.is_foreign_key_violation() =>
                    ErrorKinds::DataBaseError,
                    "There are some records which have this MainCategory.",
                    "このメインカテゴリを持つデータがあります。",
                    v;;
                ErrorKinds::DataBaseError,
                "Failed to create SubCategory from database.",
                "カテゴリの削除に失敗しました。",
                e
            )
        })?;
        sqlx::query(format!(r#"DELETE FROM main_category WHERE id={};"#, self.get_id()).as_str())
            .execute(pool)
            .await
            .map_err(|e| {
                err_with_msg!(
                    ErrorKinds::DataBaseError,
                    "Failed to create MainCategory from database.",
                    "カテゴリの削除に失敗しました。",
                    e
                )
            })?;
        Ok(())
    }
}

// display
impl Display for MainCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_name_ref())
    }
}

// convert a database row to a MainCategory
impl<'r, R> FromRow<'r, R> for MainCategory
where
    R: Row,
    &'r str: sqlx::ColumnIndex<R>,
    i32: sqlx::Type<R::Database> + sqlx::Decode<'r, R::Database>,
    String: sqlx::Type<R::Database> + sqlx::Decode<'r, R::Database>,
    NaiveDateTime: sqlx::Type<R::Database> + sqlx::Decode<'r, R::Database>,
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
            updated_at,
        })
    }
}

/// A sub category of `CashIO`
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

    /// Get an id.
    fn get_id(&self) -> usize {
        self.id
    }

    /// Get a name
    fn get_name_ref(&self) -> &String {
        &self.name
    }

    /// Get an id of super main category.
    fn get_super_category(&self) -> usize {
        self.super_category
    }

    /// Convert from a `Category` database row.
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
        let super_category: usize =
            <usize as TryFrom<i32>>::try_from(row.try_get::<'_, i32, _>("main_id")?)
                .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
        let created_at: NaiveDateTime = row.try_get::<'_, NaiveDateTime, _>("sub_created_at")?;
        let updated_at: NaiveDateTime = row.try_get::<'_, NaiveDateTime, _>("sub_updated_at")?;
        Ok(SubCategory {
            id,
            name,
            super_category,
            created_at,
            updated_at,
        })
    }

    /// Check default sub category.
    fn is_default(&self) -> bool {
        self.name.as_str() == SubCategory::DEFUALT_NAME
    }

    /// Insert a record to the database.
    async fn insert(
        pool: &Pool<MySql>,
        new_sub_category_name: &String,
        main_category: &MainCategory,
    ) -> ThisResult<()> {
        sqlx::query(
            format!(
                r#"INSERT INTO sub_category (name, super_category) VALUES ("{}", {});"#,
                new_sub_category_name
                    .remove_special_chars()
                    .unwrap_or_else(|err| {
                        warn!(
                            r#"SubCategory({}) contains '"', ';', '-'"#,
                            new_sub_category_name
                        );
                        err
                    }),
                main_category.get_id()
            )
            .as_str(),
        )
        .execute(pool)
        .await
        .map_err(|e| {
            err_with_msg!(
                sqlx::Error::Database(v) if v.is_unique_violation() =>
                    ErrorKinds::DataBaseError,
                    "The new SubCategory already has inserted.",
                    "このサブカテゴリは既に存在します。",
                    v;
                sqlx::Error::Database(v) if v.is_foreign_key_violation() =>
                    ErrorKinds::DataBaseError,
                    "The MainCategory already has deleted.",
                    "このメインカテゴリは既に削除されています。",
                    v;;
                ErrorKinds::DataBaseError,
                "Failed to create SubCategory on database.",
                "カテゴリの作成に失敗しました。",
                e
            )
        })?;
        Ok(())
    }

    /// Delete a record of the database.
    async fn delete(&self, pool: &Pool<MySql>) -> ThisResult<()> {
        if self.is_default() {
            return Err(err_with_msg!(
                ErrorKinds::CategoryError,
                "Can't remove default SubCategory.",
                "標準のサブカテゴリは削除できません。"
            ));
        };
        sqlx::query(format!(r#"DELETE FROM sub_category WHERE id={};"#, self.get_id()).as_str())
            .execute(pool)
            .await
            .map_err(|e| {
                err_with_msg!(
                    sqlx::Error::Database(v) if v.is_foreign_key_violation() =>
                        ErrorKinds::DataBaseError,
                        "There are some records which have this SubCategory.",
                        "このサブカテゴリを持つデータがあります。",
                        v;;
                    ErrorKinds::DataBaseError,
                    "Failed to remove SubCategory from database.",
                    "カテゴリの削除に失敗しました。",
                    e
                )
            })?;
        Ok(())
    }
}

// display
impl Display for SubCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_name_ref())
    }
}

// convert a database row to a SubCategory
impl<'r, R> FromRow<'r, R> for SubCategory
where
    R: Row,
    &'r str: sqlx::ColumnIndex<R>,
    i32: sqlx::Type<R::Database> + sqlx::Decode<'r, R::Database>,
    String: sqlx::Type<R::Database> + sqlx::Decode<'r, R::Database>,
    NaiveDateTime: sqlx::Type<R::Database> + sqlx::Decode<'r, R::Database>,
{
    fn from_row(row: &'r R) -> Result<Self, sqlx::Error> {
        let id: usize = <usize as TryFrom<i32>>::try_from(row.try_get::<'_, i32, _>("id")?)
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
        let name: String = row.try_get::<'_, String, _>("name")?;
        let super_category: usize =
            <usize as TryFrom<i32>>::try_from(row.try_get::<'_, i32, _>("super_category")?)
                .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
        let created_at: NaiveDateTime = row.try_get::<'_, NaiveDateTime, _>("created_at")?;
        let updated_at: NaiveDateTime = row.try_get::<'_, NaiveDateTime, _>("updated_at")?;
        Ok(SubCategory {
            id,
            name,
            super_category,
            created_at,
            updated_at,
        })
    }
}

/// A main category and all sub categorys under the main category.
#[derive(Serialize)]
pub struct MainCategoryWithSubs {
    name: String,
    subs: Vec<String>,
}

impl MainCategoryWithSubs {
    /// Empty value with main category name.
    fn new(main_name: String) -> MainCategoryWithSubs {
        MainCategoryWithSubs {
            name: main_name,
            subs: Vec::new(),
        }
    }

    /// Select all main categorys with all subcategorys under the main category.
    /// A return value is sorted by an id of main category.
    pub async fn select_all(pool: &Pool<MySql>) -> ThisResult<Vec<MainCategoryWithSubs>> {
        let all_category: Vec<Category> = Category::select_all(pool).await?;
        // all_category.sort_by_key(|v| v.get_id());
        let mut categorys_hashmap: HashMap<usize, MainCategoryWithSubs> = HashMap::new();

        for (main, sub) in all_category
            .into_iter()
            .map(|v| <Category as Into<(MainCategory, SubCategory)>>::into(v))
        {
            let main_with_subs: &mut MainCategoryWithSubs = categorys_hashmap
                .entry(main.get_id())
                .or_insert(MainCategoryWithSubs::new(main.to_string()));
            main_with_subs.push(sub);
        }

        let mut categorys: Vec<(usize, MainCategoryWithSubs)> = categorys_hashmap
            .into_iter()
            .collect::<Vec<(usize, MainCategoryWithSubs)>>();
        categorys.sort_by_key(|&(i, _)| i);

        Ok(categorys
            .into_iter()
            .map(|(_, v)| v)
            .collect::<Vec<MainCategoryWithSubs>>())
    }

    /// Add a sub category.
    fn push(&mut self, value: SubCategory) {
        self.subs.push(value.to_string());
    }
}