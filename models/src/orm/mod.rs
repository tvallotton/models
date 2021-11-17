//! All private ORM related functionality

use crate::prelude::{Result, *};
use connection::Connection;
use dotenv::*;
pub use error::Error;
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

pub static DATABASE_URL: Lazy<Result<Url, Error>> = Lazy::new(|| {
    dotenv::dotenv().ok();
    env::var("DATABASE_URL")
        .or_else(|_| var("DATABASE_URL"))
        .map_err(|_| Error::NoDatabaseUrl)
        .map(|url| Url::parse(&url))
        .and_then(|result| result.map_err(|_| Error::InvalidDatabaseUrl))
});

pub static DATABASE_CONNECTION: Lazy<Result<Connection, Error>> =
    Lazy::new(|| futures::executor::block_on(Connection::new()));
