use super::{Dialect, SqlType};
use crate::prelude::*;

fn named(name: &str) -> DataType {
    DataType::Custom(ObjectName(vec![Ident::new(name)]))
}

impl SqlType for i32 {
    fn as_sql(dialect: Dialect) -> DataType {
        match dialect {
            Sqlite => named("INTEGER"),
            _ => DataType::Int,
        }
    }
}
impl SqlType for i16 {
    fn as_sql(dialect: Dialect) -> DataType {
        match dialect {
            Sqlite => named("INTEGER"),
            _ => DataType::Int,
        }
    }
}
impl SqlType for i8 {
    fn as_sql(dialect: Dialect) -> DataType {
        match dialect {
            Sqlite => named("INTEGER"),
            _ => DataType::Int,
        }
    }
}

impl SqlType for u32 {
    fn as_sql(dialect: Dialect) -> DataType {
        match dialect {
            Sqlite => named("INTEGER"),
            _ => DataType::Int,
        }
    }
}
impl SqlType for u16 {
    fn as_sql(dialect: Dialect) -> DataType {
        match dialect {
            Sqlite => named("INTEGER"),
            _ => DataType::Int,
        }
    }
}
impl SqlType for u8 {
    fn as_sql(dialect: Dialect) -> DataType {
        match dialect {
            Sqlite => named("INTEGER"),
            _ => DataType::Int,
        }
    }
}

impl SqlType for u64 {
    fn as_sql(_: Dialect) -> DataType {
        DataType::BigInt
    }
}
impl SqlType for i64 {
    fn as_sql(_: Dialect) -> DataType {
        DataType::BigInt
    }
}
impl SqlType for f64 {
    fn as_sql(_: Dialect) -> DataType {
        DataType::Real
    }
}
impl SqlType for f32 {
    fn as_sql(_: Dialect) -> DataType {
        DataType::Real
    }
}

impl SqlType for String {
    fn as_sql(_: Dialect) -> DataType {
        DataType::Text
    }
}
impl<const N: usize> SqlType for [u8; N] {
    fn as_sql(dialect: Dialect) -> DataType {
        match dialect {
            Sqlite => named("BLOB"),
            _ => DataType::Blob(N as u64),
        }
    }
}
impl SqlType for Vec<u8> {
    fn as_sql(_: Dialect) -> DataType {
        named("BLOB")
    }
}
