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

use crate::prelude::*;

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
#[derive(WrapperStruct, Debug, Clone, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VarBinary<const SIZE: u64>(pub Vec<u8>);

impl<const N: u64> IntoSQL for VarBinary<N> {
    fn into_sql() -> Result<DataType> {
        if let SQLite = *DIALECT {
            Ok(DataType::Blob(None))
        } else {
            Ok(DataType::Varbinary(Some(N)))
        }
    }
}
