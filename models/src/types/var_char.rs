use crate::{prelude::*, types::IntoSQL};
use models_parser::ast::DataType;
#[cfg(feature = "serde")]
use serde::*;
use std::{
    convert::AsMut,
    ops::{Deref, DerefMut},
};
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
            SQLite => DataType::Text,
            _ => DataType::Varchar(Some(N)),
        }
    }
}
#[cfg(feature = "sqlx")]
mod sqlx_impl {
    use super::*;
    use sqlx::{
        database::{HasArguments, HasValueRef},
        encode::IsNull,
        Database, Decode, Encode, Type,
    };

    impl<DB, const N: u64> Type<DB> for VarChar<N>
    where
        DB: Database,
        String: Type<DB>,
    {
        fn type_info() -> DB::TypeInfo {
            String::type_info()
        }
        fn compatible(ty: &DB::TypeInfo) -> bool {
            String::compatible(ty)
        }
    }
    impl<'q, DB, const N: u64> Encode<'q, DB> for VarChar<N>
    where
        DB: Database,
        String: Encode<'q, DB>,
    {
        fn encode_by_ref(&self, buf: &mut <DB as HasArguments<'q>>::ArgumentBuffer) -> IsNull {
            self.0.encode_by_ref(buf)
        }
        fn encode(self, buf: &mut <DB as HasArguments<'q>>::ArgumentBuffer) -> IsNull
        where
            Self: Sized,
        {
            self.0.encode(buf)
        }
        fn size_hint(&self) -> usize {
            self.0.size_hint()
        }
    }

    impl<'r, DB, const N: u64> Decode<'r, DB> for VarChar<N>
    where
        DB: Database,
        String: Decode<'r, DB>,
    {
        fn decode(
            value: <DB as HasValueRef<'r>>::ValueRef,
        ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
            let string_value = <String as Decode<DB>>::decode(value)?;
            Ok(VarChar(string_value))
        }
    }
}
