use std::sync::Arc;
use thiserror::Error;
#[derive(Error, Debug)]
pub enum Error {
    #[error("SQLxError: {0}")]
    SQLx(#[from] sqlx::Error),
    #[error("NoDatabaseUrl: clould not read DATABASE_URL.")]
    NoDatabaseUrl,
    #[error("InvalidDatabaseUrl: clould not parse DATABASE_URL.")]
    InvalidDatabaseUrl,
    #[error("database url error: the database scheme {0:?} is not supported")]
    UnsupportedScheme(String),
}

