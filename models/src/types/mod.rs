
//!
//! # Types
//!
//! | Rust             | PostgreSQL    | MySQL                    | SQLite              |
//! |------------      |---------------|--------------------------|---------------------|
//! | `bool`           | BOOLEAN       | BOOLEAN                  | BOOLEAN             |
//! | `i8`             | SMALLINT      | TINYINT                  | INTEGER             |
//! | `i16`            | SMALLINT      | SMALLINT                 | INTEGER             |
//! | `i32`            | INT           | INT                      | INTEGER             |
//! | `i64`            | BIGINT        | BIGINT                   | INTEGER             |
//! | `f32`            | REAL          | FLOAT                    | REAL                |
//! | `f64`            | REAL          | REAL                     | REAL                |
//! | `String`         | TEXT          | TEXT                     | TEXT                |
//! | `VarChar<SIZE>`  | VARCHAR(SIZE) | VARCHAR(SIZE)            | TEXT                |
//! | `VarBinary<SIZE>`| BYTEA         | VARBINARY(SIZE)          | BLOB                |
//! | `Vec<u8>`        | BYTEA         | BLOB                     | BLOB                |
//! | `[u8; SIZE]`     | BYTEA         | BLOB(SIZE)               | BLOB                |
//! |
//!
//! ### [`chrono`](https://crates.io/crates/chrono)
//!
//! Requires the `chrono` Cargo feature flag.
//!
//! | Rust type                    | Postgres         | MySQL            | SQLite             |
//! |------------------------------|------------------|------------------|--------------------|
//! | `chrono::DateTime<Utc>`      | TIMESTAMPTZ      | TIMESTAMP        | DATETIME           |
//! | `chrono::DateTime<Local>`    | TIMESTAMPTZ      | TIMESTAMP        | DATETIME           |
//! | `chrono::NaiveDateTime`      | TIMESTAMP        | DATETIME         | DATETIME           |
//! | `chrono::NaiveDate`          | DATE             | DATE             | DATETIME           |
//! | `chrono::NaiveTime`          | TIME             | TIME             | DATETIME           |
//!
#[cfg(feature = "chrono")]
mod chrono_impl;
#[cfg(feature = "json")]
mod json;
mod serial;
mod time;
mod var_binary;
mod var_char;

use models_parser::ast::DataType;
pub use time::*; 
pub use json::*; 
pub use serial::Serial;
pub use var_binary::VarBinary;
pub use var_char::VarChar;

use crate::prelude::*;

/// Do not use this trait in your production code.
/// Its intended use is for migration generation only.
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
        match *DIALECT {
            SQLite => DataType::Int(None),
            PostgreSQL => DataType::SmallInt(None),
            _ => DataType::SmallInt(None),
        }
    }
}
impl IntoSQL for i8 {
    fn into_sql() -> DataType {
        match *DIALECT {
            SQLite => DataType::Int(None),
            PostgreSQL => DataType::SmallInt(None),
            _ => DataType::TinyInt(None),
        }
    }
}

impl IntoSQL for u32 {
    fn into_sql() -> DataType {
        match *DIALECT {
            MySQL => DataType::BigInt(None),
            PostgreSQL => DataType::BigInt(None),
            _ => DataType::Int(None),
        }
    }
}
// impl IntoSQL for u16 {
//     fn into_sql() -> DataType {
//         match *DIALECT {
//             MySQL => DataType::Int(None),
//             _ => DataType::Int(None),
//         }
//     }
// }
// impl IntoSQL for u8 {
//     fn into_sql() -> DataType {
//         match *DIALECT {
//             MySQL => DataType::Int(None),
//             PostgreSQL => DataType::custom("SMALLINT"),
//             _ => DataType::Int(None),
//         }
//     }
// }

// impl IntoSQL for u64 {
//     fn into_sql() -> DataType {
//         DataType::BigInt(None)
//     }
// }
impl IntoSQL for i64 {
    fn into_sql() -> DataType {
        match *DIALECT {
            SQLite => DataType::Int(None),
            _ => DataType::BigInt(None),
        }
    }
}
impl IntoSQL for f64 {
    fn into_sql() -> DataType {
        match *DIALECT {
            PostgreSQL => DataType::Double,
            _ => DataType::Real,
        }
    }
}
impl IntoSQL for f32 {
    fn into_sql() -> DataType {
        match *DIALECT {
            MySQL => DataType::Real,
            _ => DataType::Real,
        }
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
            SQLite => DataType::Blob(None),
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

#[test]
fn func() {
    let x = &models_parser::parser::Parser::parse_sql(
        &models_parser::dialect::GenericDialect {},
        "
    
    create table foo (bar INT); 
    ",
    )
    .unwrap()[0];

    println!("{}", x);
}
