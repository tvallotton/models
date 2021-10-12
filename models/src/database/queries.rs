use sqlx::query_with;

use crate::dialect::{self, Dialect};
use crate::prelude::*;
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::RwLock;

pub struct Queries {
    dialect: Dialect,
    key_getters: RwLock<HashMap<StaticStr, &'static str>>,
    foreign_getters: RwLock<HashMap<StaticStr, &'static str>>,
    normal_getters: RwLock<HashMap<StaticStr, &'static str>>,
    insert: RwLock<HashMap<StaticStr, &'static str>>,
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
    fn new(dialect: Dialect) -> Self {
        Self {
            dialect,
            key_getters: Default::default(),
            normal_getters: Default::default(),
            foreign_getters: Default::default(),
            insert: Default::default(),
        }
    }

    pub fn query_key(&self, table: &'static str) -> &'static str {
        // let read = self.key_getters.read()?;
        let mut query: Option<&'static str> = {
            let read = self.key_getters.read();
            read.unwrap().get(&StaticStr(table)).map(|x| *x)
        };
        if query.is_none() {
            if matches!(self.dialect, PostgreSQL) {
                let sql = format!("select * from {} where $1;", &table);
                let sql = Box::leak(sql.into_boxed_str());
                query = Some(sql);
                self.key_getters
                    .write()
                    .unwrap()
                    .insert(StaticStr(table), sql);
            } else {
                let sql = format!("select * from {} where ?;", &table);
                let sql = Box::leak(sql.into_boxed_str());
                query = Some(sql);
                self.key_getters
                    .write()
                    .unwrap()
                    .insert(StaticStr(table), sql);
            }
        }
        query.unwrap()
    }
}
