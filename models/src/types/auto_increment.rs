use super::IntoSQL;
use crate::{
    dialect::Dialect,
    prelude::*,
};
use models_parser::ast::DataType;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AutoIncrement {
    AutoIncrement,
    Number(i32),
}

impl Default for AutoIncrement {
    fn default() -> Self {
        Self::AutoIncrement
    }
}

impl IntoSQL for AutoIncrement {
    fn into_sql() -> Result<DataType> {
        match *DIALECT {
            | PostgreSQL => Ok(DataType::Serial),
            | MySQL => Ok(DataType::custom("INT AUTO_INCREMENT")),
            | SQLite => Ok(DataType::Int(None)),
            | dialect => Err(Error::UnsupportedDatatype {
                ty: DataType::Serial,
                dialect,
            }),
        }
    }
}

#[cfg(feature = "sqlx")]
mod sqlx_impl {
    use super::*;
    use sqlx::{
        Database,
        Type,
    };

    impl<DB: Database> Type<DB> for AutoIncrement
    where
        i32: Type<DB>,
    {
        fn type_info() -> DB::TypeInfo {
            i32::type_info()
        }
    }
}
