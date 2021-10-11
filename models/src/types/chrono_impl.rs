// | Rust type                     | MySQL                   | Postgres           | SQLite             |
// |-------------------------------|-------------------------|--------------------|--------------------|
// | `chrono::DateTime<Utc>`       | TIMESTAMP               | TIMESTAMPTZ        | DATETIME           |
// | `chrono::DateTime<Local>`     | TIMESTAMP               | TIMESTAMPTZ        | DATETIME           |
// | `chrono::NaiveDateTime`       | DATETIME                | TIMESTAMP          | DATETIME           |
// | `chrono::NaiveDate`           | DATE                    | DATE               | DATETIME           |
// | `chrono::NaiveTime`           | TIME                    | TIME               | DATETIME           |
//
use super::*;
use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use models_parser::ast::DataType;

impl IntoSQL for DateTime<Utc> {
    fn into_sql() -> DataType {
        match *DIALECT {
            PostgreSQL => DataType::custom("TIMESTAMPTZ"),
            SQLite => DataType::custom("DATETIME"),
            _ => DataType::Timestamp,
        }
    }
}
impl IntoSQL for DateTime<Local> {
    fn into_sql() -> DataType {
        match *DIALECT {
            PostgreSQL => DataType::custom("TIMESTAMPTZ"),
            SQLite => DataType::custom("DATETIME"),
            _ => DataType::Timestamp,
        }
    }
}

impl IntoSQL for NaiveDateTime {
    fn into_sql() -> DataType {
        match *DIALECT {
            PostgreSQL => DataType::Timestamp,
            _ => DataType::custom("DATETIME"),
        }
    }
}

impl IntoSQL for NaiveDate {
    fn into_sql() -> DataType {
        match *DIALECT {
            SQLite => DataType::custom("DATETIME"),
            _ => DataType::Date,
        }
    }
}

impl IntoSQL for NaiveTime {
    fn into_sql() -> DataType {
        match *DIALECT {
            SQLite => DataType::custom("DATETIME"),
            _ => DataType::Time,
        }
    }
}
