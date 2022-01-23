use std::{
    fmt::{
        write,
        Debug,
        Display,
    },
    ops::{
        Deref,
        DerefMut,
    },
};

use models_parser::ast::DataType;
#[cfg(feature = "serde")]
use serde::*;
use sqlx::Postgres;

use crate::prelude::*;

/// PostgreSQL `SERIAL` type. It enables autoincrementing functionality.
/// Example:
/// ```
/// struct Profile {
///     id: Serial,
/// }
/// ```
/// The previous structure would generate the following SQL:
/// ```sql
/// -- PostgreSQL
/// CREATE TABLE profile (
///     id SERIAL NOT NULL
/// );
/// 
/// -- SQLite 
/// CREATE TABLE profile (
///     id INTEGER NOT NULL, 
/// ); 
/// 
///
/// While using SQLite, `Serial` is translated to `INTEGER`. W
/// ```
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Serial {
    AutoIncrement,
    Number(i32),
}

impl Debug for Serial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for Serial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            | Self::AutoIncrement => write!(f, "AUTO_INCREMENT"),
            | Self::Number(number) => write!(f, "{}", number),
        }
    }
}

impl From<i32> for Serial {
    fn from(number: i32) -> Self {
        Self::Number(number)
    }
}

impl IntoSQL for Serial {
    fn into_sql() -> Result<DataType> {
        match *DIALECT {
            | PostgreSQL => Ok(DataType::Serial),
            | SQLite => Ok(DataType::Int(None)),
            | dialect => Err(Error::UnsupportedDatatype {
                ty: DataType::Serial,
                dialect,
            }),
        }
    }
}

#[cfg(feature = "sqlx")]
impl<DB: sqlx::Database> sqlx::Type<DB> for Serial
where
    i32: sqlx::Type<DB>,
{
    fn type_info() -> DB::TypeInfo {
        i32::type_info()
    }
}
