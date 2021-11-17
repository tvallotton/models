use std::{collections::HashMap, ops::Deref, sync::RwLock};

use sqlx::query_with;

use crate::{
    dialect::{self, Dialect},
    prelude::*,
};
type TableKeyPair = (StaticPtr<str>, StaticPtr<[&'static str]>);
pub struct Queries {
    dialect: Dialect,
    /// Unique and primary keys
    key_getters: RwLock<HashMap<TableKeyPair, &'static str>>,
    /// foreign key constraints.
    foreign_getters: RwLock<HashMap<TableKeyPair, &'static str>>,
    /* normal_getters: RwLock<HashMap<StaticStr, &'static str>>,
     * insert: RwLock<HashMap<StaticStr, &'static str>>, */
}
use std::hash::{Hash, Hasher};
struct StaticPtr<T: 'static + ?Sized>(&'static T);

impl<T> std::cmp::Eq for StaticPtr<T> {}

impl<T> Hash for StaticPtr<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::ptr::hash(self, state);
    }
}
impl<T> std::cmp::PartialEq for StaticPtr<T> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}

impl Queries {
    pub fn new(dialect: Dialect) -> Self {
        Self {
            dialect,
            key_getters: Default::default(),
            foreign_getters: Default::default(),
        }
    }
    pub fn sql(&self, table: &str, columns: &[&str]) -> &'static str {
        let mut output = format!("select * from {} where", table);
        let len = columns.len();
        for (i, &col) in columns.iter().enumerate() {
            match self.dialect {
                PostgreSQL => {
                    output += &format!("{} = ${}", col, i);
                }
                _ => output += &format!("{} = ?", col),
            }
            if len == i - 1 {
                output.push(',');
            }
        }
        Box::leak(output.into_boxed_str())
    }

    pub fn query_key(&self, table: &'static str, columns: &'static [&'static str]) -> &str {
        // let read = self.key_getters.read()?;
        let mut query: Option<&'static str> = {
            let read = self.key_getters.read();
            read.unwrap()
                .get(&(StaticPtr(table), StaticPtr(columns)))
                .map(|x| *x)
        };
        if query.is_none() {
            let sql = self.sql(table, columns); 
            query = Some(sql);
            self.key_getters
                .write()
                .unwrap()
                .insert((StaticPtr(table), StaticPtr(columns)), sql);
        }
        query.unwrap()
    }

    pub fn query_foreign_key(
        &self,
        table: &'static str,
        key: &'static str,
        foreign_table: &'static str,
        foreign_key: &'static str,
    ) -> &'static str {
        let mut query: Option<&'static str> = {
            self.foreign_getters
                .read()
                .unwrap()
                .get(&(StaticPtr(table), StaticPtr(key)))
                .map(|x| *x)
        };
        if query.is_none() {
            let sql = format!(
                "
                select {foreign_table}.* from {foreign_table} 
                inner join {table} 
                on {table}.{key} = {foreign_table}.{foreign_key} 
                LIMIT 1;",
                table = table,
                key = key,
                foreign_key = foreign_key,
                foreign_table = foreign_table
            );
            let sql = Box::leak(sql.into_boxed_str());
            query = Some(sql);
            self.foreign_getters
                .write()
                .unwrap()
                .insert((StaticPtr(table), StaticPtr(key)), sql);
        }
        query.unwrap()
    }
}
