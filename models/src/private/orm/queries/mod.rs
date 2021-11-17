use static_key::{Key, StaticPtr};
mod static_key;

use std::{collections::HashMap, ops::Deref, sync::RwLock};

use sqlx::query_with;

use crate::{
    dialect::{self, Dialect},
    prelude::*,
};
pub struct Queries {
    dialect: Dialect,
    key_getters: RwLock<HashMap<Key, &'static str>>,
    foreign_getters: RwLock<HashMap<(StaticPtr<str>, StaticPtr<str>), &'static str>>,
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
        let key = Key::new(table, columns);
        let mut query: Option<&'static str> = {
            let read = self.key_getters.read();
            read.unwrap().get(&key).map(|x| *x)
        };
        if query.is_none() {
            let sql = self.sql(table, columns);
            query = Some(sql);
            self.key_getters.write().unwrap().insert(key, sql);
        }
        query.unwrap()
    }

    pub fn query_foreign_key(
        &self,
        table: &'static str,
        local_column: &'static str,
        foreign_table: &'static str,
        foreign_key: &'static str,
    ) -> &'static str {
        let key = (StaticPtr(table), StaticPtr(local_column));
        let mut query: Option<&'static str> =
            { self.foreign_getters.read().unwrap().get(&key).map(|x| *x) };
        if query.is_none() {
            let sql = format!(
                "
                select {foreign_table}.* from {foreign_table} 
                inner join {table} 
                on {table}.{loca_column} = {foreign_table}.{foreign_key} 
                LIMIT 1;",
                table = table,
                loca_column = local_column,
                foreign_key = foreign_key,
                foreign_table = foreign_table
            );
            let sql = Box::leak(sql.into_boxed_str());
            query = Some(sql);
            self.foreign_getters.write().unwrap().insert(key, sql);
        }
        query.unwrap()
    }
}