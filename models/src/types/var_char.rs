use crate::{prelude::*, types::IntoSQL};
use models_parser::ast::DataType;
#[cfg(feature = "serde")]
use serde::*;
use std::{
    convert::AsMut,
    ops::{Deref, DerefMut},
};
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Default)]
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
