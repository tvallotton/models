//!
//! # Types
//!
//! The following table shows the type correspondances across all supported databases.
//! For compatibility purposes, these are the same as the ones in
//! the [`sqlx`](https://docs.rs/sqlx/latest/sqlx/types/index.html) crate.
//!
//!
//! | Rust          | PostgreSQL               | MySQL               | SQLiten    |
//! |---------------|--------------------------|---------------------|------------|
//! | `bool`        | BOOLEAN                  | BOOLEAN             | BOOLEAN    |          
//! | `i8`          | -                        | TINYINT             | INTEGER    |
//! | `u8`          | -                        | TINYINT UNSIGNED    | INTEGER    |
//! | `i16`         | SMALLINT                 | SMALLINT            | INTEGER    |
//! | `u16`         | -                        | SMALLINT UNSIGNED   | INTEGER    |
//! | `i32`         | INTEGER, SERIAL, INT4    | INTEGER             | INTEGER    |
//! | `u32`         | -                        | INTEGER UNSIGNED    | INTEGER    |
//! | `i64`         | BIGINT, BIGSERIAL, INT8  | BIGINT              | REAL       |
//! | `u64`         | -                        | BIGINT UNSIGNED     | REAL       |
//! | `f32`         | FLOAT                    | FLOAT               | REAL       |
//! | `f64`         | DOUBLE PRECISION, FLOAT8 | REAL                | REAL       |
//! | `String`      | TEXT                     | TEXT                | TEXT       |
//! | `Vec<u8>`     | BYTEA                    | BLOB                | BLOB       |

//! ## Native Wappers
//! | Rust              | PostgreSQL             | MySQL               | SQLite     |
//! |-------------------|------------------------|---------------------|----------- |
//! | `Json<T>`         | JSON                   | JSON                | JSON       |
//! | `Serial`          | SERIAL                 | -                   | INTEGER    |
//! | `VarChar<SIZE>`   | VARCHAR(SIZE)          | VARCHAR(SIZE)       | TEXT       |
//! | `VarBinary<SIZE>` | VARBINARY(SIZE)        | VARBINARY(SIZE)     | BLOB       |
//! | `Date<T>`         | DATE                   | DATE                | DATE       |
//! | `DateTime<T>`     | DATETIME               | DATETIME            | DATETIME   |
//! | `Time<T>`         | TIME                   | TIME                | TIME       |
//! | `Timestamp`       | TIMESTAMP              | TIMESTAMP           | TIMESTAMP  |
//!

//! ### [`chrono`](https://crates.io/crates/chrono)
//! The following table shows the type correspondances across all supported databases.
//! Alternatively, you can use the Wrappers `
//! | Rust type                  | Postgres          | MySQL               | SQLite                 |
//! |----------------------------|-------------------|---------------------|------------------------|
//! | `chrono::DateTime<Utc>`    | TIMESTAMPTZ       | TIMESTAMP           | DATE                   |
//! | `chrono::DateTime<Local>`  | TIMESTAMPTZ       | TIMESTAMP           | DATE                   |
//! | `chrono::NaiveDateTime`    | TIMESTAMP         | TIMESTAMP           | DATETIME               |
//! | `chrono::NaiveDate`        | DATE              | DATE                | DATE                   |
//! | `chrono::NaiveTime`        | TIME              | TIME                | TIME                   |
//! | `PgTimeTz`                 | TIMETZ            | -                   | DATETIME               |



#[cfg(feature = "chrono")]
mod chrono_impl;
#[cfg(feature = "json")]
mod json;
mod serial;
mod time;
mod var_binary;
mod var_char;

#[cfg(feature = "json")]
pub use json::*;
use models_parser::ast::DataType;
pub use serial::Serial;
pub use time::*;
pub use var_binary::VarBinary;
pub use var_char::VarChar;

use crate::prelude::*;

/// Do not use this trait in your production code.
/// Its intended use is for migration generation only.
/// It will panic if used outside its intended API.
#[doc(hidden)]
pub trait IntoSQL {
    fn into_sql() -> Result<DataType>;
    const IS_NULLABLE: bool = false;
}

impl IntoSQL for i32 {
    fn into_sql() -> Result<DataType> {
        Ok(DataType::Int(None))
    }
}
impl IntoSQL for i16 {
    fn into_sql() -> Result<DataType> {
        Ok(match *DIALECT {
            | SQLite => DataType::Int(None),
            | PostgreSQL => DataType::SmallInt(None),
            | _ => DataType::SmallInt(None),
        })
    }
}
impl IntoSQL for i8 {
    fn into_sql() -> Result<DataType> {
        Ok(match *DIALECT {
            | SQLite => DataType::Int(None),
            | PostgreSQL => DataType::SmallInt(None),
            | _ => DataType::TinyInt(None),
        })
    }
}

impl IntoSQL for u32 {
    fn into_sql() -> Result<DataType> {
        Ok(match *DIALECT {
            | MySQL => DataType::BigInt(None),
            | PostgreSQL => DataType::BigInt(None),
            | _ => DataType::Int(None),
        })
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
    fn into_sql() -> Result<DataType> {
        Ok(match *DIALECT {
            | SQLite => DataType::Int(None),
            | _ => DataType::BigInt(None),
        })
    }
}
impl IntoSQL for f64 {
    fn into_sql() -> Result<DataType> {
        Ok(match *DIALECT {
            | PostgreSQL => DataType::Double,
            | _ => DataType::Real,
        })
    }
}
impl IntoSQL for f32 {
    fn into_sql() -> Result<DataType> {
        Ok(match *DIALECT {
            | MySQL => DataType::Real,
            | _ => DataType::Real,
        })
    }
}

impl IntoSQL for String {
    fn into_sql() -> Result<DataType> {
        Ok(DataType::Text)
    }
}
impl<const N: usize> IntoSQL for [u8; N] {
    fn into_sql() -> Result<DataType> {
        Ok(match *DIALECT {
            | PostgreSQL => DataType::Bytea,
            | SQLite => DataType::Blob(None),
            | _ => DataType::Blob(Some(N as u64)),
        })
    }
}
impl IntoSQL for Vec<u8> {
    fn into_sql() -> Result<DataType> {
        Ok(match *DIALECT {
            | PostgreSQL => DataType::Bytea,
            | _ => DataType::Blob(None),
        })
    }
}

impl<T: IntoSQL> IntoSQL for Option<T> {
    const IS_NULLABLE: bool = false;

    fn into_sql() -> Result<DataType> {
        T::into_sql()
    }
}
impl IntoSQL for bool {
    fn into_sql() -> Result<DataType> {
        Ok(DataType::Boolean)
    }
}

#[test]
fn func() {
    let x = &models_parser::parser::Parser::parse_sql(
        &models_parser::dialect::GenericDialect {},
        "
    
    CREATE TABLE Persons (
    Personid int NOT NULL AUTO_INCREMENT,
    LastName varchar(255) NOT NULL,
    FirstName varchar(255),
    Age int,
    PRIMARY KEY (Personid)
); 
    ",
    )
    .unwrap()[0];

    println!("{}", x);
}
