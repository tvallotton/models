use crate::prelude::{Error, Result, *};
use dotenv::*;
pub use error::ORMError;
use futures::{executor::block_on, TryFutureExt};
use models_parser::dialect::keywords::{DATABASE, SQL};
use queries::Queries;
use sqlx::{
    any::{Any, AnyPool, AnyRow},
    Database, Encode, Executor, FromRow, Type,
};
use std::{env, sync::RwLock};
use url::Url;

mod error;
mod queries;

static DATABASE_URL: Lazy<Result<Url, ORMError>> = Lazy::new(|| {
    dotenv().ok();
    env::var("DATABASE_URL")
        .or_else(|_| var("DATABASE_URL"))
        .map_err(|_| ORMError::NoDatabaseUrl)
        .map(|url| Url::parse(&url))
        .and_then(|result| result.map_err(|_| ORMError::InvalidDatabaseUrl))
});

pub static DATABASE_CONNECTION: Lazy<Result<DatabaseConnection, ORMError>> =
    Lazy::new(|| futures::executor::block_on(DatabaseConnection::new()));

pub struct DatabaseConnection {
    queries: Queries,
    pool: AnyPool,
}

impl DatabaseConnection {
    async fn new() -> Result<Self, ORMError> {
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
    ) -> Result<O>
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
    ) -> Result<O>
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
