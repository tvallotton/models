pub use crate::migration::{table::Column, Table};

pub(crate) use crate::model::{Dialect, Dialect::*, Model, SqlType};
pub(crate) use ast::*;
pub(crate) use collections::HashMap;
pub(crate) use dotenv::*;
// pub(crate) use fehler::*;
pub(crate) use lazy_static::lazy_static;
pub(crate) use lazy_static::*;
 
pub(crate) use parser::Parser;
pub(crate) use result::Result;
pub(crate) use sqlparser::*;
pub(crate) use std::{collections::*, *};
pub(crate) use url::*;
pub(crate) fn parse_sql(
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
const fn get_migrations_dir() -> &'static str {
    "migrations/"
}

static MIGRATIONS_DIR: &str = get_migrations_dir();

use sqlformat::{FormatOptions, Indent};

pub static FORMAT_OPTIONS: FormatOptions = FormatOptions {
    indent: Indent::Spaces(4),
    uppercase: true,
    lines_between_queries: 2,
};
