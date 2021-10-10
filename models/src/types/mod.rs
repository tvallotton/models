//
// I took some of these correspondances from sqlx. It would be good to review it.
// # Types
//
// | Rust         | PostgreSQL    | MySQL                    | SQLite              |
// |--------------|---------------|--------------------------|---------------------|
// | `bool`       | BOOLEAN       | BOOLEAN                  | BOOLEAN             |
// | `i8`         | SMALLINT      | TINYINT                  | INTEGER             |
// | `i16`        | SMALLINT      | SMALLINT                 | SMALLINT            |
// | `i32`        | INT           | INT                      | INTEGER             |
// | `i64`        | BIGINT        | BIGINT                   | BIGINT              |
// | `u8`         | SMALLINT      | TINYINT UNSIGNED         | TINYINT UNSIGNED    |
// | `u16`        | INT           | SMALLINT UNSIGNED        | SMALLINT UNSIGNED   |
// | `u32`        | INT           | INT UNSIGNED             | INT UNSIGNED        |
// | `u64`        | BIGINT        | BIGINT UNSIGNED          | BIGINT UNSIGNED     |
// | `f32`        | REAL          | FLOAT                    | REAL                |
// | `f64`        | DOUBLE        | DOUBLE                   | REAL                |
// | `String`     | TEXT          | TEXT                     | TEXT                |
// | `Vec<u8>`    | BYTEA         | VARBINARY, BINARY, BLOB  | BLOB                |
//
// ### [`chrono`](https://crates.io/crates/chrono)
//
// Requires the `chrono` Cargo feature flag.
//
// | Rust type                     | MySQL                   | Postgres           | SQLite             |
// |-------------------------------|-------------------------|--------------------|--------------------|
// | `chrono::DateTime<Utc>`       | TIMESTAMP               | TIMESTAMPTZ        | DATETIME           |
// | `chrono::DateTime<Local>`     | TIMESTAMP               | TIMESTAMPTZ        | DATETIME           |
// | `chrono::NaiveDateTime`       | DATETIME                | TIMESTAMP          | DATETIME           |
// | `chrono::NaiveDate`           | DATE                    | DATE               | DATETIME           |
// | `chrono::NaiveTime`           | TIME                    | TIME               | DATETIME           |
//

//! ### [`chrono`](https://crates.io/crates/chrono)
//!
//! Requires the `chrono` Cargo feature flag.
//!
//! | Rust type                             | Postgres type(s)                                     |
//! |---------------------------------------
//! | `chrono::DateTime<Utc>`               
//! | `chrono::DateTime<Local>`             
//! | `chrono::NaiveDateTime`               
//! | `chrono::NaiveDate`                   
//! | `chrono::NaiveTime`                   
//! | [`PgTimeTz`]                          | TIMETZ                                               |
//!

#[cfg(feature = "json")]
mod json;
mod serial;
mod var_binary;
mod var_char;
use models_parser::ast::DataType;
pub use serial::Serial;
pub use var_binary::VarBinary;
pub use var_char::VarChar;

use crate::prelude::*;

// #[cfg(feature = "binary")]
// pub struct Bytes<T>(pub T);
// #[cfg(feature = "json")]
// pub struct Json<T>(pub T);
// #[cfg(feature = "chrono")]
// pub use chrono::DateTime;

/// Do not use this trait in your production code.
/// Its intended use is for migration management only.
/// It will panic if used outside its intended API.
pub trait IntoSQL {
    fn into_sql() -> DataType;
    const IS_NULLABLE: bool = false;
}

impl IntoSQL for i32 {
    fn into_sql() -> DataType {
        DataType::Int(None)
    }
}
impl IntoSQL for i16 {
    fn into_sql() -> DataType {
        DataType::Int(None)
    }
}
impl IntoSQL for i8 {
    fn into_sql() -> DataType {
        DataType::Int(None)
    }
}

impl IntoSQL for u32 {
    fn into_sql() -> DataType {
        DataType::Int(None)
    }
}
impl IntoSQL for u16 {
    fn into_sql() -> DataType {
        DataType::Int(None)
    }
}
impl IntoSQL for u8 {
    fn into_sql() -> DataType {
        DataType::Int(None)
    }
}

impl IntoSQL for u64 {
    fn into_sql() -> DataType {
        DataType::BigInt(None)
    }
}
impl IntoSQL for i64 {
    fn into_sql() -> DataType {
        DataType::BigInt(None)
    }
}
impl IntoSQL for f64 {
    fn into_sql() -> DataType {
        DataType::Real
    }
}
impl IntoSQL for f32 {
    fn into_sql() -> DataType {
        DataType::Real
    }
}

impl IntoSQL for String {
    fn into_sql() -> DataType {
        DataType::Text
    }
}
impl<const N: usize> IntoSQL for [u8; N] {
    fn into_sql() -> DataType {
        match *DIALECT {
            PostgreSQL => DataType::Bytea,
            _ => DataType::Blob(Some(N as u64)),
        }
    }
}
impl IntoSQL for Vec<u8> {
    fn into_sql() -> DataType {
        match *DIALECT {
            PostgreSQL => DataType::Bytea,
            _ => DataType::Blob(None),
        }
    }
}

impl<T: IntoSQL> IntoSQL for Option<T> {
    fn into_sql() -> DataType {
        T::into_sql()
    }
    const IS_NULLABLE: bool = false; 
}
impl IntoSQL for bool {
    fn into_sql() -> DataType {
        DataType::Boolean
    }
}
