
/// The following table shows the type correspondances across all supported databases. 
/// These are the same as the ones in the [`sqlx`](https://docs.rs/sqlx/latest/sqlx/types/index.html) crate. 
/// | Rust type                  | Postgres          | MySQL               | SQLite                 |
/// |----------------------------|-------------------|---------------------|------------------------|
/// | `chrono::DateTime<Utc>`    | TIMESTAMPTZ       | TIMESTAMP           | DATE
/// | `chrono::DateTime<Local>`  | TIMESTAMPTZ       | TIMESTAMP           | DATE
/// | `chrono::NaiveDateTime`    | TIMESTAMP         | TIMESTAMP           | DATETIME
/// | `chrono::NaiveDate`        | DATE              | DATE                | DATE
/// | `chrono::NaiveTime`        | TIME              | TIME                | TIME
/// | `chrono::Utc`              | TIMESTAMPTZ       | TIMESTAMP           | DATETIME

use chrono::{
    DateTime,
    Local,
    NaiveDate,
    NaiveDateTime,
    NaiveTime,
    Utc,
};
use models_parser::ast::DataType;

use super::*;

impl IntoSQL for DateTime<Utc> {
    fn into_sql() -> Result<DataType> {
        Ok(match *DIALECT {
            | PostgreSQL => DataType::custom("TIMESTAMPTZ"),
            | SQLite => DataType::custom("DATETIME"),
            | _ => DataType::Timestamp,
        })
    }
}
impl IntoSQL for DateTime<Local> {
    fn into_sql() -> Result<DataType> {
        Ok(match *DIALECT {
            | PostgreSQL => DataType::custom("TIMESTAMPTZ"),
            | SQLite => DataType::custom("DATETIME"),
            | _ => DataType::Timestamp,
        })
    }
}

impl IntoSQL for NaiveDateTime {
    fn into_sql() -> Result<DataType> {
        Ok(match *DIALECT {
            | PostgreSQL => DataType::Timestamp,
            | _ => DataType::custom("DATETIME"),
        })
    }
}

impl IntoSQL for NaiveDate {
    fn into_sql() -> Result<DataType> {
        Ok(match *DIALECT {
            | SQLite => DataType::custom("DATETIME"),
            | _ => DataType::Date,
        })
    }
}

impl IntoSQL for NaiveTime {
    fn into_sql() -> Result<DataType> {
        Ok(match *DIALECT {
            | SQLite => DataType::custom("DATETIME"),
            | _ => DataType::Time,
        })
    }
}
