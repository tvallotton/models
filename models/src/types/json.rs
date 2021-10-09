use super::*;
use models_parser::ast::DataType;
use serde::*;
use std::ops::{Deref, DerefMut};

#[derive(Serialize, Deserialize, Clone, Default, Hash)]
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
    use sqlx::database::HasArguments;
    use sqlx::decode::Decode;
    use sqlx::encode::{Encode, IsNull};
    use sqlx::error::BoxDynError;
    use sqlx::types::Type;
    use sqlx::Database;
    use std::io::Write;
    // impl<T, DB> Type<DB> for Json<T>
    // where
    //     DB: Database,
    //     sqlx::types::Json<T>: Type<DB>,
    // {
    //     fn type_info() -> <DB as Database>::TypeInfo {
    //         sqlx::types::Json::type_info()
    //     }

    //     fn compatible(ty: &<DB as Database>::TypeInfo) -> bool {
    //         sqlx::types::Json::compatible(ty)
    //     }
    // }

    // impl<T, DB> Type<DB> for Json<T>
    // where
    //     DB: Database,
    //     sqlx::types::Json<T>: Type<DB>,
    // {
    //     fn type_info() -> <DB as Database>::TypeInfo {
    //         sqlx::types::Json::type_info()
    //     }

    //     fn compatible(ty: &<DB as Database>::TypeInfo) -> bool {
    //         sqlx::types::Json::compatible(ty)
    //     }
    // }

    impl<'q, T, DB> Encode<'q, DB> for Json<T>
    where
        DB: Database,
        T: Serialize,
        <DB as HasArguments<'q>>::ArgumentBuffer: Write,
    {
        fn encode_by_ref(&self, buf: &mut <DB as HasArguments<'q>>::ArgumentBuffer) -> IsNull {
            use std::io::Write;
            serde_json::to_writer(buf, self).ok();
            IsNull::No
        }
    }

 
    impl<'r, DB: Database> Decode<'r, DB> for MyType
    where
        &'r str: Decode<'r, DB>,
    {
        fn decode(
            value: <DB as HasValueRef<'r>>::ValueRef,
        ) -> Result<MyType, Box<dyn std::error::Error + 'static + Send + Sync>> {
            let string_value = <&str as Decode<DB>>::decode(value)?;
            serde_json::from_str(string_value)
                .map(Json)
                .map_err(Into::into)
        }
    }
}
