pub(crate) use crate::dialect::Dialect;
pub use crate::error::Error;
pub(crate) use crate::private::*;
pub(crate) use convert::{TryFrom, TryInto};
pub(crate) use models_parser::{ast::*, *};
pub(crate) use once_cell::sync::Lazy;
pub(crate) use std::{collections::HashMap, sync::Mutex, *};
pub(crate) use Dialect::*;
pub(crate) type Result<T = (), E = Error> = std::result::Result<T, E>;

use url::Url;

pub(crate) static DATABASE_URL: Lazy<Url> = Lazy::new(|| {
    let database_url = env::var("DATABASE_URL").unwrap();
    Url::parse(&database_url).unwrap()
});
pub(crate) static MIGRATIONS_DIR: Lazy<String> = Lazy::new(|| {
    let dir = env::var("MIGRATION_DIR");
    dir.unwrap()
});
pub(crate) static DIALECT: Lazy<Dialect> = Lazy::new(|| match DATABASE_URL.scheme() {
    "sqlite" => SQLite,
    "postgres" => PostgreSQL,
    "mysql" => MySQL,
    "mssql" => MsSQL,
    _ => Any,
});
#[cfg(feature = "sqlformat")]
use sqlformat::{FormatOptions, Indent};
#[cfg(feature = "sqlformat")]
pub static FORMAT_OPTIONS: FormatOptions = FormatOptions {
    indent: Indent::Spaces(4),
    uppercase: true,
    lines_between_queries: 2,
};

pub(crate) fn parse_sql(sql: &str) -> Result<Vec<Statement>> {
    let stmts = parser::Parser::parse_sql(&*DIALECT, sql)?;
    Ok(stmts)
}
