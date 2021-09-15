use crate::prelude::*;
mod primitives;

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
        matches!(self, Dialect::Any | Dialect::Sqlite)
    }
}

use sqlx_models_parser::dialect::*;
impl sqlx_models_parser::dialect::Dialect for Dialect {
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

pub trait IntoSQL {
    fn into_sql() -> DataType;
    fn null_option() -> ColumnOptionDef {
        ColumnOptionDef {
            name: None,
            option: ColumnOption::NotNull,
        }
    }
}
// #[cfg(feature = "serde_json")]
// use json::Json;

// #[cfg(feature = "serde_json")]
// mod json {

//     use serde::*;
//     use serde_json::*;

//     #[derive(Debug, Serialize, Deserialize, Clone, Copy, Hash)]
//     pub struct Json<T>(pub T);

//     impl<T> IntoSQL for Json<T> {
//         fn into_sql() -> DataType {
//             DataType::Custom(ObjectName(vec![Ident::new("JSON")]))
//         }
//     }
// }

// #[cfg(feature = "json")]
// use json::Json;

// #[cfg(feature = "blob")]
// use binary::Binary;

// #[cfg(feature = "blob")]
// mod binary {
//     use serde::*;

//     #[derive(Debug, Serialize, Deserialize, Clone, Copy, Hash)]
//     pub struct Binary<T>(pub T);
//     impl<T> IntoSQL for Binary<T> {
//         fn into_sql() -> DataType {
//             match dialect {
//                 Postgres => DataType::Bytea,
//                 _ => DataType::Blob(None),
//             }
            
//         }
//     }
// }
