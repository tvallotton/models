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
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Date<T>(pub T);
impl<T> Deref for Date<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> DerefMut for Date<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl<T> AsRef<T> for Date<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}
impl<T> AsMut<T> for Date<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }
}
impl<T> IntoSQL for Date<T> {
    const IS_NULLABLE: bool = false;

    fn into_sql() -> DataType {
        DataType::Date
    }
}

/// Wrapper for datetime related types.
/// The generated SQL for `DateTime<T>` will always be `DATETIME`.
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
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DateTime<T>(pub T);
impl<T> Deref for DateTime<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> DerefMut for DateTime<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl<T> AsRef<T> for DateTime<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}
impl<T> AsMut<T> for DateTime<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }
}
impl<T> IntoSQL for DateTime<T> {
    const IS_NULLABLE: bool = false;

    fn into_sql() -> DataType {
        DataType::custom("DATETIME")
    }
}
/// Wrapper for datetime related types.
/// The generated SQL for `Timestamp<T>` will always be `TIMESTAMP`.
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
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Timestamp<T>(pub T);
impl<T> Deref for Timestamp<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> DerefMut for Timestamp<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl<T> AsRef<T> for Timestamp<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}
impl<T> AsMut<T> for Timestamp<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }
}
impl<T> IntoSQL for Timestamp<T> {
    const IS_NULLABLE: bool = false;

    fn into_sql() -> DataType {
        DataType::Timestamp
    }
}
