use crate::prelude::{Error, Result, *};
use connection::Connection;
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
mod connection;
mod error;
mod queries;

static DATABASE_URL: Lazy<Result<Url, ORMError>> = Lazy::new(|| {
    dotenv::dotenv().ok();
    env::var("DATABASE_URL")
        .or_else(|_| var("DATABASE_URL"))
        .map_err(|_| ORMError::NoDatabaseUrl)
        .map(|url| Url::parse(&url))
        .and_then(|result| result.map_err(|_| ORMError::InvalidDatabaseUrl))
});

pub static DATABASE_CONNECTION: Lazy<Result<Connection, ORMError>> =
    Lazy::new(|| futures::executor::block_on(Connection::new()));
