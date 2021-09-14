use crate::prelude::*;
use sqlx_models_parser::parser::ParserError;
use thiserror::Error;
use std::sync::Arc; 
#[macro_use]
macro_rules! error {
    ($($args:expr),+) => {
        Err(Error::Message(format!($($args),*)))?
    };
}

#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("{0}")]
    SyntaxError(#[from] ParserError),
    #[error("{0}")]
    Message(String),
    #[error("Could not read or create migration file.")]
    IOError,
    #[error("Dependency cycle detected invlonving the tables: {0:?}.")]
    CycleError(Vec<String>),
    #[error("The environment variable DATABASE_URL is not set. Set it or store it in an `.env` file.")]
    DatabaseUrlNotSet,
    #[error("The DATABASE_URL environment variable could not be parsed.")]
    InvalidDatabaseUrl,
}
