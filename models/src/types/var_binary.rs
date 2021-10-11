use crate::prelude::*;
use models_parser::ast::DataType;
use std::{
    convert::AsMut,
    ops::{Deref, DerefMut},
};

#[cfg(feature = "serde")]
use serde::*;

/// Used for MySQL when to specify that the datatype should be
/// a `VARBINARY(N)`. The database will make sure the field does not
/// go over the specified length.
/// ```
/// use models::{Model, VarChar};
/// #[derive(Model)]
/// struct Example {
///     bin_data: VarBinary<255>
/// }
/// ```
/// The previous structure would generate:
/// ```sql
/// CREATE TABLE example (
///     bin_data VarBinary(255) NOT NULL
/// );
/// ```

#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(feature = "sqlx", sqlx(transparent))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VarBinary<const SIZE: u64>(pub Vec<u8>);

impl<const SIZE: u64> VarBinary<SIZE> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<const N: u64> Deref for VarBinary<N> {
    type Target = Vec<u8>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const N: u64> DerefMut for VarBinary<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<const N: u64> AsRef<Vec<u8>> for VarBinary<N> {
    fn as_ref(&self) -> &Vec<u8> {
        &self.0
    }
}

impl<const N: u64> AsMut<Vec<u8>> for VarBinary<N> {
    fn as_mut(&mut self) -> &mut Vec<u8> {
        &mut self.0
    }
}

impl<const N: u64> IntoSQL for VarBinary<N> {
    const IS_NULLABLE: bool = false;
    fn into_sql() -> DataType {
        if !matches!(*DIALECT, SQLite) {
            DataType::Varbinary(Some(N))
        } else {
            DataType::Blob(None)
        }
    }
}

impl<T, const N: u64> From<T> for VarBinary<N>
where
    T: Into<Vec<u8>>,
{
    fn from(obj: T) -> Self {
        VarBinary(obj.into())
    }
}
