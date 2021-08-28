#![allow(dead_code)]
mod migration;
mod model;
mod prelude;

use std::hash::Hash;

use prelude::*;
use sqlx::{database::HasArguments, Column};

pub use model::{Model, SqlType};

#[test]
fn test() {
    let sql = "create table user (asd BLOB);";
    let ast = parse_sql(&Dialect::Sqlite, sql).unwrap();
    println!("{:?}", ast[0]);
}


