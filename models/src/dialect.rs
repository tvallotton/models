use dialect::*;
use models_parser::dialect;

use self::Dialect::*;
#[derive(Clone, Copy, Debug)]
pub enum Dialect {
    SQLite,
    PostgreSQL,
    MySQL,
    // Any,
}

impl Dialect {
    pub(crate) fn requires_move(&self) -> bool {
        matches!(self, Dialect::SQLite)
    }

    pub(crate) fn _has_default_constr_name(&self) -> bool {
        matches!(self, Dialect::PostgreSQL)
    }

    pub(crate) fn supports_cascade(&self) -> bool {
        !matches!(self, SQLite)
    }
}

impl dialect::Dialect for Dialect {
    fn is_delimited_identifier_start(&self, ch: char) -> bool {
        match self {
            | SQLite => SQLiteDialect {}.is_delimited_identifier_start(ch),
            | PostgreSQL => PostgreSqlDialect {}.is_delimited_identifier_start(ch),
            | MySQL => MySqlDialect {}.is_delimited_identifier_start(ch),
            // | Any => GenericDialect {}.is_delimited_identifier_start(ch),
        }
    }

    fn is_identifier_start(&self, ch: char) -> bool {
        match self {
            | SQLite => SQLiteDialect {}.is_identifier_start(ch),
            | PostgreSQL => PostgreSqlDialect {}.is_identifier_start(ch),
            | MySQL => MySqlDialect {}.is_identifier_start(ch),
            // | Any => GenericDialect {}.is_identifier_start(ch),
        }
    }

    fn is_identifier_part(&self, ch: char) -> bool {
        match self {
            | SQLite => SQLiteDialect {}.is_identifier_part(ch),
            | PostgreSQL => PostgreSqlDialect {}.is_identifier_part(ch),
            | MySQL => MySqlDialect {}.is_identifier_part(ch),
            // | Any => GenericDialect {}.is_identifier_part(ch),
        }
    }
}
