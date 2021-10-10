use crate::{prelude::*, types::IntoSQL};
use models_parser::ast::DataType;
use std::{
    convert::AsMut,
    ops::{Deref, DerefMut},
};

#[cfg(feature = "serde")]
use serde::*;

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

#[cfg(feature = "sqlx")]
mod sqlx_impl {
    use super::*;
    use sqlx::{
        database::{HasArguments, HasValueRef},
        encode::IsNull,
        Database, Decode, Encode, Type,
    };

    impl<DB, const N: u64> Type<DB> for VarBinary<N>
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
    impl<'q, DB, const N: u64> Encode<'q, DB> for VarBinary<N>
    where
        DB: Database,
        Vec<u8>: Encode<'q, DB>,
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

    impl<'r, DB, const N: u64> Decode<'r, DB> for VarBinary<N>
    where
        DB: Database,
        Vec<u8>: Decode<'r, DB>,
    {
        fn decode(
            value: <DB as HasValueRef<'r>>::ValueRef,
        ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
            let string_value = <Vec<u8> as Decode<DB>>::decode(value)?;
            Ok(VarBinary(string_value))
        }
    }
}
