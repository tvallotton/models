use super::{error::ORMError, queries::Queries, DATABASE_CONNECTION, DATABASE_URL};
use crate::prelude::*;
use sqlx::{
    any::{Any, AnyPool, AnyRow},
    Database, Encode, Executor, FromRow, Type,
};

pub struct Connection {
    queries: Queries,
    pool: AnyPool,
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

    pub async fn query_key<'q, T, O>(
        &'q self,
        table: &'static str,
        key_name: &'static str,
        key_val: T,
    ) -> Result<O, ORMError>
    where
        T: 'q + Send + Encode<'q, Any> + Type<Any>,
        O: for<'r> FromRow<'r, AnyRow> + Send + Unpin,
    {
        let query = self.queries.query_key(table, key_name);
        Ok(sqlx::query_as(query)
            .bind(key_val)
            .fetch_one(&self.pool)
            .await?)
    }

    pub async fn query_foreign_key<'q, T, O>(
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
