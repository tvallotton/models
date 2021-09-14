pub(crate) use crate::error::Error;
pub(crate) use crate::model::{Dialect, Dialect::*};
pub(crate) use crate::scheduler::{table::Column, Table};
pub use ::std::{fs, io, *};
pub(crate) use ast::*;
pub(crate) use collections::HashMap;
pub(crate) use dotenv::*;
pub(crate) use fehler::*;
pub(crate) use once_cell::sync::Lazy;
pub(crate) use parser::Parser;
pub(crate) use result::Result;
pub(crate) use sqlx_models_parser::*;
pub(crate) use url::*;

pub(crate) fn parse_sql(
    dialect: &dyn dialect::Dialect,
    sql: &str,
) -> Result<Vec<Statement>, parser::ParserError> {
    Parser::parse_sql(dialect, sql)
}

pub static DATABASE_URL: Lazy<Result<Url, Error>> = Lazy::new(get_uri);

fn get_uri() -> Result<Url, Error> {
    dotenv().ok();
    let database_url = if let Ok(url) = var("DATABASE_URL") {
        Ok(url)
    } else {
        env::var("DATABASE_URL").map_err(|_| Error::DatabaseUrlNotSet)
    };
    Url::parse(&database_url?).map_err(|_| Error::InvalidDatabaseUrl)
}
fn get_migrations_dir() -> String {
    var("MIGRATIONS_DIR").unwrap_or_else(|_| "migrations/".into())
}


pub(crate) static MIGRATIONS_DIR: Lazy<String> = Lazy::new(get_migrations_dir);

use sqlformat::{FormatOptions, Indent};

pub static FORMAT_OPTIONS: FormatOptions = FormatOptions {
    indent: Indent::Spaces(4),
    uppercase: true,
    lines_between_queries: 2,
};