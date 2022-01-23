use std::ops::{
    Deref,
    DerefMut,
};

#[cfg(feature = "serde")]
use serde::*;

use super::*;

/// Wrapper for date related types.
/// The generated SQL for `Date<T>` will always be `DATE`.
/// For example:
/// ```
/// struct Person {
///     birthday: Date<String>
/// }
/// ```
/// The generated SQL for the previous struct would be:
/// ```sql
/// CREATE TABLE person (
///     DATE birthday NOT NULL,
/// );
/// ```
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(feature = "sqlx", sqlx(transparent))]
#[derive(WrapperStruct, Debug, Clone, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Date<T>(pub T);

impl<T> IntoSQL for Date<T> {
    fn into_sql() -> Result<DataType> {
        Ok(DataType::Date)
    }
}

/// Wrapper for datetime related types.
/// The generated SQL for `DateTime<T>` will always be `DATETIME`,
/// no matter if the SQL dialect supports such type or not.
/// For example:
/// ```
/// struct Post {
///     creation_date: DateTime<u8>
/// }
/// ```
/// The generated SQL for the previous struct would be:
/// ```sql
/// CREATE TABLE post (
///     DATETIME creation_date NOT NULL,
/// );
/// ```
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(feature = "sqlx", sqlx(transparent))]
#[derive(WrapperStruct, Debug, Clone, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DateTime<T>(pub T);

impl<T> IntoSQL for DateTime<T> {
    fn into_sql() -> Result<DataType> {
        Ok(DataType::custom("DATETIME"))
    }
}
/// Wrapper for timestamps related types.
/// The generated SQL for `Timestamp<T>` will always be `TIMESTAMP`,
/// no matter if the SQL dialect supports such type or not.
/// For example:
/// ```
/// struct Post {
///     creation_date: Timestamp<u8>
/// }
/// ```
/// The generated SQL for the previous struct would be:
/// ```sql
/// CREATE TABLE post (
///     TIMESTAMP creation_date NOT NULL,
/// );
/// ```
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(feature = "sqlx", sqlx(transparent))]
#[derive(WrapperStruct, Debug, Clone, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Timestamp<T>(pub T);

impl<T> IntoSQL for Timestamp<T> {
    fn into_sql() -> Result<DataType> {
        Ok(DataType::Timestamp)
    }
}
/// Wrapper for types related to PostgreSQL's `TIMESTAMPTZ` type.
/// The generated SQL for `TimestampTz<T>` will always be `TIMESTAMP`,
/// no matter if the SQL dialect supports such type or not.
/// For example:
/// ```
/// struct Post {
///     creation_date: TimestampTz<u8>
/// }
/// ```
/// The generated SQL for the previous struct would be:
/// ```sql
/// CREATE TABLE post (
///     TIMESTAMPTZ creation_date NOT NULL,
/// );
/// ```
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(feature = "sqlx", sqlx(transparent))]
#[derive(WrapperStruct, Debug, Clone, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TimestampTz<T>(pub T);

impl<T> IntoSQL for TimestampTz<T> {
    fn into_sql() -> Result<DataType> {
        Ok(DataType::custom("TIMESTAMPTZ"))
    }
}
/// Wrapper for types related to PostgreSQL's `TIMETZ` type.
/// The generated SQL for `TimeTz<T>` will always be `TIMETZ`,
/// no matter if the SQL dialect supports such type or not.
/// ```
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(feature = "sqlx", sqlx(transparent))]
#[derive(WrapperStruct, Debug, Clone, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TimeTz<T>(pub T);

impl<T> IntoSQL for TimeTz<T> {
    fn into_sql() -> Result<DataType> {
        Ok(DataType::custom("TIMETZ"))
    }
}
