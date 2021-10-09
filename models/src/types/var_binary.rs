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

impl<const N: u64> From<&[u8]> for VarBinary<N> {
    fn from(obj: &[u8]) -> Self {
        VarBinary(obj.to_vec())
    }
}

impl<const N: u64> IntoSQL for VarBinary<N> {
    const IS_NULLABLE: bool = false;
    fn into_sql() -> DataType {
        if !matches!(*DIALECT, SQLite) {
            DataType::Varbinary(N)
        } else {
            DataType::Blob(None)
        }
    }
}
