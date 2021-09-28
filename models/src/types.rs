//
// I took some of these correspondances from sqlx. It would be good to review it.
// # Types
//
// | Rust                 | PostgreSQL               | MySQL                    | SQLite                   |
// |----------------------|--------------------------|--------------------------|--------------------------|
// | `bool`               | BOOLEAN                  | BOOLEAN                  | BOOLEAN                  |
// | `i8`                 | SMALLINT                 | TINYINT                  | TINYINT                  |
// | `i16`                | SMALLINT                 | SMALLINT                 | SMALLINT                 |
// | `i32`                | INT                      | INT                      | INTEGER                  |
// | `i64`                | BIGINT                   | BIGINT                   | BIGINT                   |
// | `u8`                 | SMALLINT                 | TINYINT UNSIGNED         | TINYINT UNSIGNED         |
// | `u16`                | INT                      | SMALLINT UNSIGNED        | SMALLINT UNSIGNED        |
// | `u32`                | INT                      | INT UNSIGNED             | INT UNSIGNED             |
// | `u64`                | BIGINT                   | BIGINT UNSIGNED          | BIGINT UNSIGNED          |
// | `f32`                | REAL                     | FLOAT                    | REAL                     |
// | `f64`                | DOUBLE                   | DOUBLE                   | REAL                     |
// | `String`             | TEXT                     | TEXT                     | TEXT                     |
// | `Vec<u8>`            | BYTEA                   | VARBINARY, BINARY, BLOB  | VARBINARY, BINARY, BLOB  |
//
// ### [`chrono`](https://crates.io/crates/chrono)
//
// Requires the `chrono` Cargo feature flag.
//
// | Rust type                             | MySQL type(s)                                        |
// |---------------------------------------|------------------------------------------------------|
// | `chrono::DateTime<Utc>`               | TIMESTAMP                                            |
// | `chrono::DateTime<Local>`             | TIMESTAMP                                            |
// | `chrono::NaiveDateTime`               | DATETIME                                             |
// | `chrono::NaiveDate`                   | DATE                                                 |
// | `chrono::NaiveTime`                   | TIME                                                 |
//
// TODO:
// Fix DataType in models-parser so it preserves the original type name.
// e.g. INTEGER is parsed to INT which is not recognized by SQLite.

use models_parser::ast::DataType;
// pub struct VarChar<const SIZE: usize>(pub String);
// pub struct VarBinary<const SIZE: usize>(pub Vec<u8>);
// #[cfg(feature = "binary")]
// pub struct Bytes<T>(pub T);
// #[cfg(feature = "json")]
// pub struct Json<T>(pub T);
// #[cfg(feature = "chrono")]
// pub use chrono::DateTime;

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
        DataType::Blob(Some(N as u64))
    }
}
impl IntoSQL for Vec<u8> {
    fn into_sql() -> DataType {
        DataType::Blob(None)
    }
}

impl<T: IntoSQL> IntoSQL for Option<T> {
    fn into_sql() -> DataType {
        T::into_sql()
    }
}
impl IntoSQL for bool {
    fn into_sql() -> DataType {
        DataType::Boolean
    }
}

// #[cfg(feature = "json")]
// impl<T> IntoSQL for Json<T> {
//     fn into_sql() -> DataType {
//         DataType::Custom(ObjectName(vec![Ident::new("JSON")]))
//     }
// }
