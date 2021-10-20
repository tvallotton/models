use std::{
    env,
    sync::RwLock,
};

use dotenv::*;
use futures::{
    executor::block_on,
    TryFutureExt,
};
use models_parser::dialect::keywords::{
    DATABASE,
    SQL,
};
use queries::Queries;
use sqlx::{
    any::{
        Any,
        AnyPool,
        AnyRow,
    },
    Database,
    Encode,
    Executor,
    FromRow,
    Type,
};
use url::Url;

use crate::prelude::{
    Error,
    Result,
    *,
};
mod queries;

static DATABASE_URL: Lazy<Result<Url>> = Lazy::new(|| {
    dotenv().ok();
    env::var("DATABASE_URL")
        .or_else(|_| var("DATABASE_URL"))
        .map(|url| Url::parse(&url))
        .map_err(|_| Error::DatabaseUrl)
        .and_then(|result| result.map_err(|_| Error::DatabaseUrl))
        .map_err(|_| Error::DatabaseUrl)
});

pub static DATABASE_CONNECTION: Lazy<Result<DatabaseConnection>> =
    Lazy::new(|| block_on(DatabaseConnection::new()));

pub struct DatabaseConnection {
    queries: Queries,
    pool: AnyPool,
}

impl DatabaseConnection {
    async fn new() -> Result<Self> {
        let url = DATABASE_URL.as_ref().map_err(Clone::clone)?;
        let dialect = match url.scheme() {
            | "sqlite" => Ok(SQLite),
            | "postgres" => Ok(PostgreSQL),
            | "mysql" => Ok(MySQL),
            | scheme => Err(Error::UnsupportedScheme(scheme.into())),
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
}
