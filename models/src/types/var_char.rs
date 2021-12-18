use std::{
    convert::AsMut,
    ops::{
        Deref,
        DerefMut,
    },
};

use models_parser::ast::DataType;
#[cfg(feature = "serde")]
use serde::*;

use crate::{
    prelude::*,
    types::IntoSQL,
};

/// Used for MySQL when to specify that the datatype should be
/// a `VARCHAR(N)`. The database will make sure the field does not
/// go over the specified length.
/// ```
/// use models::{Model, VarChar};
/// #[derive(Model)]
/// struct Profile {
///     email: VarChar<255>
/// }
/// ```
/// The previous structure would generate:
/// ```sql
/// CREATE TABLE profile (
///     email VARCHAR(255) NOT NULL
/// );
/// ```
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(feature = "sqlx", sqlx(transparent))]
#[derive(Clone, Default, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct VarChar<const SIZE: u64>(pub String);

impl<const SIZE: u64> VarChar<SIZE> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<const N: u64> Deref for VarChar<N> {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const N: u64> DerefMut for VarChar<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<const N: u64> AsRef<String> for VarChar<N> {
    fn as_ref(&self) -> &String {
        &self.0
    }
}

impl<const N: u64> AsMut<String> for VarChar<N> {
    fn as_mut(&mut self) -> &mut String {
        &mut self.0
    }
}

impl<T, const N: u64> From<T> for VarChar<N>
where
    T: Into<String>,
{
    fn from(obj: T) -> Self {
        let string: String = obj.into();
        VarChar(string)
    }
}

impl<const N: u64> IntoSQL for VarChar<N> {
    const IS_NULLABLE: bool = false;

    fn into_sql() -> DataType {
        match *DIALECT {
            | SQLite => DataType::Text,
            | _ => DataType::Varchar(Some(N)),
        }
    }
}
