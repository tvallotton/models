pub use crate::migration::{
    table::{Column, Constraint},
    Table,
};

pub use crate::model::{Dialect, Dialect::*, Model, SqlType};
pub use collections::HashMap;
pub use dotenv::*;
pub use lazy_static::lazy_static;

pub use sqlparser::*;
pub use std::{collections::*, *};

pub use ast::*;
// pub use dialect::*;
pub use parser::Parser;
pub use result::Result;
pub use url::*;

pub fn parse_sql(
    dialect: &dyn dialect::Dialect,
    sql: &str,
) -> Result<Vec<Statement>, parser::ParserError> {
    Parser::parse_sql(dialect, sql)
}
lazy_static! {
    pub static ref DATABASE_URL: Url = get_uri();
}

fn get_uri() -> Url {
    dotenv().ok();
    let database_url = if let Ok(url) = var("DATABASE_URL") {
        url
    } else {
        env::var("DATABASE_URL").expect("The DATABASE_URL environment variable must be set")
    };
    Url::parse(&database_url).expect("The DATABASE_URL environment variable could not be parsed.")
}
