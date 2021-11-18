use super::{error::Error as ORMError, queries::Queries, DATABASE_CONNECTION, DATABASE_URL};
use crate::prelude::*;
use sqlx::{
    any::{Any, AnyPool, AnyRow},
    Database, Encode, Executor, FromRow, Type,
};

pub struct Connection {
    queries: Queries,
   pub pool: AnyPool,
}

impl Connection {
    pub(crate) async fn new() -> Result<Self, ORMError> {
        let url = DATABASE_URL.as_ref().map_err(Clone::clone)?;
        let dialect = match url.scheme() {
            "sqlite" => Ok(SQLite),
            "postgres" => Ok(PostgreSQL),
            "mysql" => Ok(MySQL),
            scheme => Err(ORMError::UnsupportedScheme(scheme.into())),
        }?;

        let pool = AnyPool::connect(&url.to_string()).await?;
        Ok(Self {
            queries: Queries::new(dialect),
            pool,
        })
    }

    pub async fn query<'q, T, O, const N: usize>(
        &'q self,
        query: &'static str,
       
        values: [T; N],
    ) -> Result<O, ORMError>

    {
        todo!()
    }
    #[doc(hidden)]
    pub async fn _query_key<'q, T, O, const N: usize>(
        &'q self,
        table: &'static str,
        columns: &'static [&'static str],
        values: [T; N],
    ) -> Result<O, ORMError>
    where
        T: 'q + Send + Encode<'q, Any> + Type<Any>,
        O: for<'r> FromRow<'r, AnyRow> + Send + Unpin,
    {
        let query = self.queries.query_key(table, columns);
        let mut query = sqlx::query_as(query);
        for value in values {
            query = query.bind(value);
        }
        Ok(query.fetch_one(&self.pool).await?)
    }
    #[doc(hidden)]
    pub async fn _query_foreign_key<'q, T, O>(
        &'q self,
        table: &'static str,
        key: &'static str,
        foreign_table: &'static str,
        foreign_key: &'static str,
        key_val: T,
    ) -> Result<O, ORMError>
    where
        T: 'q + Send + Encode<'q, Any> + Type<Any>,
        O: for<'r> FromRow<'r, AnyRow> + Send + Unpin,
    {
        let query = self
            .queries
            .query_foreign_key(table, key, foreign_table, foreign_key);

        Ok(sqlx::query_as(query)
            .bind(key_val)
            .fetch_one(&self.pool)
            .await?)
    }
}
