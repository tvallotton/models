use crate::prelude::*;
mod sqltypes;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dialect {
    Sqlite,
    Postgres,
    Mysql,
    Mssql,
    Any,
}

impl Dialect {
    pub(crate) fn requires_move(self) -> bool {
        match self {
            Dialect::Any | Dialect::Sqlite => true,
            _ => false,
        }
    }
}

use sqlparser::dialect::*;
impl sqlparser::dialect::Dialect for Dialect {
    fn is_delimited_identifier_start(&self, ch: char) -> bool {
        match self {
            Sqlite => SQLiteDialect {}.is_delimited_identifier_start(ch),
            Postgres => PostgreSqlDialect {}.is_delimited_identifier_start(ch),
            Mysql => MySqlDialect {}.is_delimited_identifier_start(ch),
            Mssql => MsSqlDialect {}.is_delimited_identifier_start(ch),
            Any => GenericDialect {}.is_delimited_identifier_start(ch),
        }
    }
    fn is_identifier_part(&self, ch: char) -> bool {
        match self {
            Sqlite => SQLiteDialect {}.is_identifier_part(ch),
            Postgres => PostgreSqlDialect {}.is_identifier_part(ch),
            Mysql => MySqlDialect {}.is_identifier_part(ch),
            Mssql => MsSqlDialect {}.is_identifier_part(ch),
            Any => GenericDialect {}.is_identifier_part(ch),
        }
    }
    fn is_identifier_start(&self, ch: char) -> bool {
        match self {
            Sqlite => SQLiteDialect {}.is_identifier_start(ch),
            Postgres => PostgreSqlDialect {}.is_identifier_start(ch),
            Mysql => MySqlDialect {}.is_identifier_start(ch),
            Mssql => MsSqlDialect {}.is_identifier_start(ch),
            Any => GenericDialect {}.is_identifier_start(ch),
        }
    }
}

pub trait Model {
    fn target() -> Table;
}

pub trait SqlType {
    fn as_sql(dialect: Dialect) -> DataType;
}

// impl SqlType for i64 {

// }
