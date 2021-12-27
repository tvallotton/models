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
#[derive(WrapperStruct, Clone, Default, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct VarChar<const SIZE: u64>(pub String);

impl<const SIZE: u64> VarChar<SIZE> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<const N: u64> IntoSQL for VarChar<N> {
    fn into_sql() -> Result<DataType> {
        Ok(match *DIALECT {
            | SQLite => DataType::Text,
            | _ => DataType::Varchar(Some(N)),
        })
    }
}
