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

#[cfg(feature = "bincode")]
pub use binary::Binary;

#[cfg(feature = "bincode")]
mod binary {
    use super::*;
    use crate::private::Dialect;
    use serde::*;
    use sqlx::{database::HasValueRef, *};
    use std::error::Error;

    #[derive(Debug, Serialize, Deserialize, Clone, Copy, Hash)]
    pub struct Binary<T>(pub T);

    impl<T> IntoSQL for Binary<T> {
        fn into_sql() -> DataType {
            match &*DIALECT {
                Ok(Dialect::Postgres) => DataType::Bytea,
                _ => DataType::Blob(None),
            }
        }
    }

    impl<'r, DB, T> Decode<'r, DB> for Binary<T>
    where
        &'r str: Decode<'r, DB>,
        T: Deserialize<'r>,
        DB: Database,
        &'r [u8]: sqlx::Decode<'r, DB>,
    {
        fn decode(
            value: <DB as HasValueRef<'r>>::ValueRef,
        ) -> std::result::Result<Binary<T>, Box<dyn Error + 'static + Send + Sync>> {
            let bytes = <&[u8] as Decode<DB>>::decode(value)?;
            let value = bincode::deserialize(bytes)?;
            Ok(value)
        }
    }

    impl<'r, DB, T> Encode<'r, DB> for Binary<T>
    where
        DB: Database,
        T: Serialize,
        Vec<u8>: Encode<'r, DB>,
    {
        fn encode_by_ref(
            &self,
            buf: &mut <DB as sqlx::database::HasArguments<'r>>::ArgumentBuffer,
        ) -> encode::IsNull {
            let bytes: Vec<u8> = bincode::serialize(&self).unwrap();
            <Vec<u8> as Encode<'r, DB>>::encode(bytes, buf)
        }
    }
}
