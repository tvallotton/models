//! All private ORM related functionality

use crate::prelude::{Result, *};

use dotenv::*;
pub use error::Error;
use futures::{executor::block_on, TryFutureExt};

use sqlx::{
    any::{Any, AnyPool, AnyRow},
    Database, Encode, Executor, FromRow, Type,
};
use std::{env, sync::RwLock};
use url::Url;
mod error;

pub struct Connection {
    pub dialect: Dialect,
    pub pool: AnyPool,
}

pub static DATABASE_URL: Lazy<Result<Url, Error>> = Lazy::new(|| {
    dotenv::dotenv().ok();
    env::var("DATABASE_URL")
        .or_else(|_| var("DATABASE_URL"))
        .map_err(|_| Error::NoDatabaseUrl)
        .map(|url| Url::parse(&url))
        .and_then(|result| result.map_err(|_| Error::InvalidDatabaseUrl))
});

pub static DATABASE_CONNECTION: Lazy<Result<Connection, Error>> = Lazy::new(|| {
    futures::executor::block_on(async {
        let url = DATABASE_URL.as_ref().map_err(Clone::clone)?;
        let dialect = match url.scheme() {
            "sqlite" => Ok(SQLite),
            "postgres" => Ok(PostgreSQL),
            "mysql" => Ok(MySQL),
            scheme => Err(Error::UnsupportedScheme(scheme.into())),
        }?;

        let pool = AnyPool::connect(&url.to_string()).await?;
     
        Ok(Connection { dialect, pool })
    })
});
