use models_parser::ast::DataType;
#[cfg(feature = "serde")]
use serde::*;
use std::ops::{Deref, DerefMut};

use crate::prelude::*;

/// PostgreSQL `SERIAL` type. It enables autoincrementing functionality.
/// Example:
/// ```
/// struct Profile {
///     id: Serial,
/// }
/// ```
/// The previous structure would generate:
/// ```sql
/// CREATE TABLE profile (
///     id SERIAL NOT NULL
/// );
/// ```
///
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(feature = "sqlx", sqlx(transparent))]
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Serial(pub i32);

impl<T> From<T> for Serial
where
    T: Into<i32>,
{
    fn from(obj: T) -> Self {
        Self(obj.into())
    }
}

impl Deref for Serial {
    type Target = i32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Serial {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl AsMut<i32> for Serial {
    fn as_mut(&mut self) -> &mut i32 {
        &mut self.0
    }
}

impl AsRef<i32> for Serial {
    fn as_ref(&self) -> &i32 {
        &self.0
    }
}

impl IntoSQL for Serial {
    fn into_sql() -> DataType {
        DataType::Serial
    }
}
