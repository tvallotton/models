//! All private ORM related functionality

use crate::prelude::*;
use dotenv::*;
pub use error::Error;
use tokio::sync::OnceCell;

use sqlx::any::AnyPool;
use std::env;
mod query; 
use url::Url;
mod error;
mod traits;
pub struct Connection {
    pub dialect: Dialect,
    pub pool: AnyPool,
    _priv: (),
}

#[throws(Error)]
fn get_database_url() -> Url {
    dotenv().ok();
    env::var("DATABASE_URL")
        .or_else(|_| dotenv::var("DATABASE_URL"))
        .map_err(|_| Error::NoDatabaseUrl)
        .map(|url| Url::parse(&url))?
        .map_err(|_| Error::InvalidDatabaseUrl)?
}

#[throws(Error)]
async fn connect() -> Connection {
    let url = get_database_url()?;
    let uri = url.to_string(); 
    Connection {
        dialect: get_dialect(&url)?,
        pool: AnyPool::connect(&uri).await?,
        _priv: (),
    }
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
