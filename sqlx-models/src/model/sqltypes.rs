use super::SqlType;
use crate::prelude::*;

impl SqlType for i32 {
    fn as_sql() -> DataType {
        DataType::Int(None)
    }
    fn null_option() -> ColumnOptionDef {
        ColumnOptionDef {
            name: None,
            option: ColumnOption::NotNull,
        }
    }
}
impl SqlType for i16 {
    fn as_sql() -> DataType {
        DataType::Int(None)
    }
}
impl SqlType for i8 {
    fn as_sql() -> DataType {
        DataType::Int(None)
    }
}

impl SqlType for u32 {
    fn as_sql() -> DataType {
        DataType::Int(None)
    }
}
impl SqlType for u16 {
    fn as_sql() -> DataType {
        DataType::Int(None)
    }
}
impl SqlType for u8 {
    fn as_sql() -> DataType {
        DataType::Int(None)
    }
}

impl SqlType for u64 {
    fn as_sql() -> DataType {
        DataType::BigInt(None)
    }
}
impl SqlType for i64 {
    fn as_sql() -> DataType {
        DataType::BigInt(None)
    }
}
impl SqlType for f64 {
    fn as_sql() -> DataType {
        DataType::Real
    }
}
impl SqlType for f32 {
    fn as_sql() -> DataType {
        DataType::Real
    }
}

impl SqlType for String {
    fn as_sql() -> DataType {
        DataType::Text
    }
}
impl<const N: usize> SqlType for [u8; N] {
    fn as_sql() -> DataType {
        DataType::Blob(Some(N as u64))
    }
}
impl SqlType for Vec<u8> {
    fn as_sql() -> DataType {
        DataType::Blob(None)
    }
}

impl<T: SqlType> SqlType for Option<T> {
    fn as_sql() -> DataType {
        T::as_sql()
    }
    fn null_option() -> ColumnOptionDef {
        ColumnOptionDef {
            name: None,
            option: ColumnOption::Null,
        }
    }
}
impl SqlType for bool {
    fn as_sql() -> DataType {
        DataType::Boolean
    }
}