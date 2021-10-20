use std::{collections::HashMap, ops::Deref, sync::RwLock};

use sqlx::query_with;

use crate::{
    dialect::{self, Dialect},
    prelude::*,
};
type TableKeyPair = (StaticStr, StaticStr);
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
#[derive(Eq)]
struct StaticStr(&'static str);

impl Hash for StaticStr {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::ptr::hash(self, state);
    }
}
impl PartialEq for StaticStr {
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

    pub fn query_key(&self, table: &'static str, key_name: &'static str) -> &'static str {
        // let read = self.key_getters.read()?;
        let mut query: Option<&'static str> = {
            let read = self.key_getters.read();
            read.unwrap()
                .get(&(StaticStr(table), StaticStr(key_name)))
                .map(|x| *x)
        };
        if query.is_none() {
            let sql = match self.dialect {
                PostgreSQL => format!("select * from {} where {} = $1;", table, key_name),
                _ => format!("select * from {} where {} = ?;", table, key_name),
            };
            let sql = Box::leak(sql.into_boxed_str());
            query = Some(sql);
            self.key_getters
                .write()
                .unwrap()
                .insert((StaticStr(table), StaticStr(key_name)), sql);
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
                .get(&(StaticStr(table), StaticStr(key)))
                .map(|x| *x)
        };
        if query.is_none() {
            let sql = format!(
                "
                select {table}.* from {table} 
                inner join {foreign_table} 
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
                .insert((StaticStr(table), StaticStr(key)), sql);
        }
        query.unwrap()
    }
}
