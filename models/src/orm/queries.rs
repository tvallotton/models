use std::{
    collections::HashMap,
    ops::Deref,
    sync::RwLock,
};

use sqlx::query_with;

use crate::{
    dialect::{
        self,
        Dialect,
    },
    prelude::*,
};

pub struct Queries {
    dialect: Dialect,
    key_getters: RwLock<HashMap<(StaticStr, StaticStr), &'static str>>,
    foreign_getters: RwLock<HashMap<StaticStr, &'static str>>,
    normal_getters: RwLock<HashMap<StaticStr, &'static str>>,
    insert: RwLock<HashMap<StaticStr, &'static str>>,
}
use std::hash::{
    Hash,
    Hasher,
};
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
            normal_getters: Default::default(),
            foreign_getters: Default::default(),
            insert: Default::default(),
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
            if matches!(self.dialect, PostgreSQL) {
                let sql = match self.dialect {
                    | PostgreSQL => format!("select * from {} where {} = $1;", table, key_name),
                    | _ => format!("select * from {} where {} = ?;", table, key_name),
                };
                let sql = Box::leak(sql.into_boxed_str());
                query = Some(sql);
                self.key_getters
                    .write()
                    .unwrap()
                    .insert((StaticStr(table), StaticStr(key_name)), sql);
            }
        }
        query.unwrap()
    }
}
