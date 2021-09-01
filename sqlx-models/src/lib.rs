// #![allow(dead_code)]
mod migration;
mod model;
mod prelude;
use prelude::*;

pub use migration::{
    table::{constraint, Column},
    Migration, Table,
};

pub use model::{Dialect, Model, SqlType};

pub use sqlx_models_proc_macro::Model;

use std::sync::Mutex;
lazy_static! {
    static ref MIGRATIONS_LOCK: Mutex<()> = std::sync::Mutex::new(());
}
use sqlparser::ast::ColumnOptionDef; 
#[test]
fn func() {
    let ast = parse_sql(&Dialect::Sqlite, "alter table x alter column y int not null;"); 
    println!("\n\n{}", dbg!(&ast.unwrap()[0])); 
}