//! All private ORM related functionality

use crate::prelude::{
    Result,
    *,
};
use dotenv::*;
pub use error::Error;
use tokio::sync::OnceCell;

use sqlx::{
    any::AnyPool,
    ConnectOptions,
    Database,
    Encode,
    Executor,
    FromRow,
    Type,
};
use std::{
    env,
    sync::RwLock,
};

use url::Url;
mod error;
mod traits;
pub struct Connection {
    pub dialect: Dialect,
    pub pool: AnyPool,
}

#[throws(Error)]
pub fn get_database_url() -> Url {
    dotenv::dotenv().ok();
    env::var("DATABASE_URL")
        .or_else(|_| var("DATABASE_URL"))
        .map_err(|_| Error::NoDatabaseUrl)
        .map(|url| Url::parse(&url))?
        .map_err(|_| Error::InvalidDatabaseUrl)?
}

#[throws(Error)]
pub async fn connect() -> Connection {
    let url = get_database_url()?;
    let dialect = get_dialect(&url)?;
    let conn = Connection {
        dialect,
        pool: AnyPool::connect(&url.to_string()).await?,
    };
    conn
}
#[throws(Error)]
fn get_dialect(url: &Url) -> crate::dialect::Dialect {
    match url.scheme() {
        | "sqlite" => SQLite,
        | "postgres" => PostgreSQL,
        | "mysql" => MySQL,
        | scheme => throw!(Error::UnsupportedScheme(scheme.into())),
    }
}
static DATABASE_CONNECTION: OnceCell<Connection> = tokio::sync::OnceCell::const_new();

#[throws(Error)]
pub async fn get_connection() -> &'static Connection {
    DATABASE_CONNECTION.get_or_try_init(connect).await?
}
