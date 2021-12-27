use std::ops::{
    Deref,
    DerefMut,
};
use models_proc_macro::WrapperStruct; 
use models_parser::ast::DataType;
use serde::*;

use super::*;


    /// Wrapper type used to hold serializable data. The type generated is `JSON`.
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
    #[derive(WrapperStruct, Serialize, Deserialize, Clone, Default, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
    pub struct Json<T>(pub T);


impl<T> IntoSQL for Json<T> {
    const IS_NULLABLE: bool = false;

    fn into_sql() -> Result<DataType> {
        Ok(DataType::Json)
    }
}
#[allow(unused_imports)]
#[cfg(all(feature = "sqlx"))]
mod sqlx_impl {
    use std::io::Write;

    use serde::{
        Deserialize,
        Serialize,
    };
    #[cfg(feature = "sqlx-mysql")]
    use sqlx::mysql::{
        MySql,
        MySqlTypeInfo,
    };
    #[cfg(feature = "sqlx-postgres")]
    use sqlx::postgres::{
        PgTypeInfo,
        Postgres,
    };
    #[cfg(feature = "sqlx-mysql")]
    use sqlx::sqlite::{
        Sqlite,
        SqliteTypeInfo,
    };
    use sqlx::{
        database::{
            HasArguments,
            HasValueRef,
        },
        decode::Decode,
        encode::{
            Encode,
            IsNull,
        },
        Database,
        Type,
    };

    use super::*;

    impl<T, DB> Type<DB> for Json<T>
    where
        DB: Database,
        sqlx::types::Json<T>: Type<DB>,
    {
        fn type_info() -> DB::TypeInfo {
            sqlx::types::Json::type_info()
        }

        fn compatible(ty: &DB::TypeInfo) -> bool {
            sqlx::types::Json::compatible(ty)
        }
    }
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
