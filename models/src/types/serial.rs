use models_parser::ast::DataType;
#[cfg(feature = "serde")]
use serde::*;
use std::ops::{Deref, DerefMut};

use super::IntoSQL;

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

#[cfg(feature = "sqlx/postgres")]
mod sqlx_impl {
    use sqlx::{PgTypeInfo, Postgres, Type, postgres::PgTypeInfo};
    impl Type<Postgres> for Serial {
        type TypeInfo = PgTypeInfo; 
        fn type_info() -> PgTypeInfo {
            PgTypeInfo::INT4
        }
    }
}
