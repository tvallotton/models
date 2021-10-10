use super::*;
use models_parser::ast::DataType;
use serde::*;
use std::ops::{Deref, DerefMut};

/// Wrapper type used to hold serilizable data. The type generated is `JSON`. 
/// ```rust
/// struct Author {
///     books: Json<Vec<String>>
/// }
/// ```
/// The previous structure would generate: 
/// ```sql
/// CREATE TABLE author (
///     books JSON NOT NULL,
/// );
/// ```



#[derive(Serialize, Deserialize, Clone, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Json<T>(pub T);

impl<T> Deref for Json<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Json<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> AsRef<T> for Json<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> AsMut<T> for Json<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> IntoSQL for Json<T> {
    const IS_NULLABLE: bool = false;
    fn into_sql() -> DataType {
        DataType::Json
    }
}

#[cfg(feature = "sqlx")]
mod sqlx_impl {
    use super::*;
    use serde::{Deserialize, Serialize};
    use sqlx::{
        database::{HasArguments, HasValueRef},
        decode::Decode,
        encode::{Encode, IsNull},
        Database,
    };
    use std::io::Write;

    impl<'q, T, DB> Encode<'q, DB> for Json<T>
    where
        DB: Database,
        T: Serialize,
        <DB as HasArguments<'q>>::ArgumentBuffer: Write,
    {
        fn encode_by_ref(&self, buf: &mut <DB as HasArguments<'q>>::ArgumentBuffer) -> IsNull {
            serde_json::to_writer(buf, self).ok();
            IsNull::No
        }
    }

    impl<'r, DB, T> Decode<'r, DB> for Json<T>
    where
        &'r str: Decode<'r, DB>,
        DB: Database,
        T: Deserialize<'r>,
    {
        fn decode(
            value: <DB as HasValueRef<'r>>::ValueRef,
        ) -> Result<Json<T>, Box<dyn std::error::Error + 'static + Send + Sync>> {
            let string_value = <&str as Decode<DB>>::decode(value)?;
            serde_json::from_str(string_value)
                .map(Json)
                .map_err(Into::into)
        }
    }
}
