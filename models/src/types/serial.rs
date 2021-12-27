use std::ops::{
    Deref,
    DerefMut,
};

use models_parser::ast::DataType;
#[cfg(feature = "serde")]
use serde::*;

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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(feature = "sqlx", sqlx(transparent))]
#[derive(WrapperStruct, Debug, Clone, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Serial(pub i32);

impl IntoSQL for Serial {
    fn into_sql() -> Result<DataType> {
        Ok(DataType::Serial)
    }
}
