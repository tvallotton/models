use crate::prelude::*;
use sqlx_models_parser::parser::ParserError;
use thiserror::Error;

use std::collections::HashSet;

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
    CycleError(HashSet<String>),
    #[error(
        "The environment variable DATABASE_URL is not set. Set it or store it in an `.env` file."
    )]
    DatabaseUrlNotSet,
    #[error("The DATABASE_URL environment variable could not be parsed.")]
    InvalidDatabaseUrl,
}

impl Error {
    pub(crate) fn kind(&self) -> &'static str {
        match self {
            &Self::CycleError(_) => "CycleError",
            &Self::Message(_) => "Error",
            &Self::IOError => "IOError",
            &Self::DatabaseUrlNotSet => "Database URL error",
            &Self::InvalidDatabaseUrl => "Database URL error",
            &Self::SyntaxError(_) => "SyntaxError"
        }
    }
}
