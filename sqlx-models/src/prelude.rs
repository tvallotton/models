pub(crate) use crate::error::Error;
pub(crate) use crate::model::{Dialect, Dialect::*};
pub(crate) use crate::scheduler::Table;
pub use ::std::{fs, io, *};
pub(crate) use ast::*;
pub(crate) use collections::HashMap;

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

pub static DATABASE_URL: Lazy<Result<Url, Error>> = Lazy::new(|| {
    let url = env::var("DATABASE_URL").unwrap();
    Url::parse(&url).map_err(|_| Error::InvalidDatabaseUrl)
});
pub(crate) static MIGRATIONS_DIR: Lazy<String> = Lazy::new(|| env::var("MIGRATIONS_DIR").unwrap());

pub static DIALECT: Lazy<Result<Dialect, Error>> = Lazy::new(|| {
    let url = (*DATABASE_URL).clone()?;
    Ok(match url.scheme() {
        "sqlite" => Sqlite,
        "postgres" => Postgres,
        "mysql" => Mysql,
        "mssql" => Mssql,
        "any" => Any,
        _ => return Err(error!("scheme \"{}\" is not supported", url.scheme())),
    })
});

use sqlformat::{FormatOptions, Indent};

pub static FORMAT_OPTIONS: FormatOptions = FormatOptions {
    indent: Indent::Spaces(4),
    uppercase: true,
    lines_between_queries: 2,
};
