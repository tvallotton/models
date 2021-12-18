use std::sync::Arc;
use thiserror::Error;
#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("SQLxError: {0}")]
    SQLx(Arc<sqlx::Error>),
    #[error("NoDatabaseUrl: clould not read DATABASE_URL.")]
    NoDatabaseUrl,
    #[error("InvalidDatabaseUrl: clould not parse DATABASE_URL.")]
    InvalidDatabaseUrl,
    #[error("database url error: the database scheme {0:?} is not supported")]
    UnsupportedScheme(String),
}

impl From<sqlx::Error> for Error {
    fn from(error: sqlx::Error) -> Self {
        Error::SQLx(Arc::new(error))
    }
}
