use super::IntoSQL;
use crate::prelude::*;

impl IntoSQL for i32 {
    fn into_sql(_dialect: Dialect) -> DataType {
        DataType::Int(None)
    }
    fn null_option() -> ColumnOptionDef {
        ColumnOptionDef {
            name: None,
            option: ColumnOption::NotNull,
        }
    }
}
impl IntoSQL for i16 {
    fn into_sql(_dialect: Dialect) -> DataType {
        DataType::Int(None)
    }
}
impl IntoSQL for i8 {
    fn into_sql(_dialect: Dialect) -> DataType {
        DataType::Int(None)
    }
}

impl IntoSQL for u32 {
    fn into_sql(_dialect: Dialect) -> DataType {
        DataType::Int(None)
    }
}
impl IntoSQL for u16 {
    fn into_sql(_dialect: Dialect) -> DataType {
        DataType::Int(None)
    }
}
impl IntoSQL for u8 {
    fn into_sql(_dialect: Dialect) -> DataType {
        DataType::Int(None)
    }
}

impl IntoSQL for u64 {
    fn into_sql(_dialect: Dialect) -> DataType {
        DataType::BigInt(None)
    }
}
impl IntoSQL for i64 {
    fn into_sql(_dialect: Dialect) -> DataType {
        DataType::BigInt(None)
    }
}
impl IntoSQL for f64 {
    fn into_sql(_dialect: Dialect) -> DataType {
        DataType::Real
    }
}
impl IntoSQL for f32 {
    fn into_sql(_dialect: Dialect) -> DataType {
        DataType::Real
    }
}

impl IntoSQL for String {
    fn into_sql(_dialect: Dialect) -> DataType {
        DataType::Text
    }
}
impl<const N: usize> IntoSQL for [u8; N] {
    fn into_sql(_dialect: Dialect) -> DataType {
        DataType::Blob(Some(N as u64))
    }
}
impl IntoSQL for Vec<u8> {
    fn into_sql(_dialect: Dialect) -> DataType {
        DataType::Blob(None)
    }
}

impl<T: IntoSQL> IntoSQL for Option<T> {
    fn into_sql(_dialect: Dialect) -> DataType {
        T::into_sql(_dialect)
    }
    fn null_option() -> ColumnOptionDef {
        ColumnOptionDef {
            name: None,
            option: ColumnOption::Null,
        }
    }
}
impl IntoSQL for bool {
    fn into_sql(_dialect: Dialect) -> DataType {
        DataType::Boolean
    }
}