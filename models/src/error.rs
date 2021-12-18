use std::sync::Arc;

use models_parser::parser::ParserError;
use thiserror::Error;

use crate::prelude::*;

macro_rules! error {
    ($($args:expr),+) => {
        Error::Message(format!($($args),*))
    };
}
#[non_exhaustive]
#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("syntax error: {0}")]
    Syntax(#[from] ParserError),
    #[error("syntax error: {0}.\n       found at file \"{1}\".")]
    SyntaxAtFile(ParserError, path::PathBuf),
    #[error("{0}")]
    Message(String),
    #[error("could not read or create migration file. {0}")]
    IO(#[from] Arc<io::Error>),
    #[error("dependency cycle detected invlonving the tables: {0:?}. help: consider removing redundant foreign key constraints. ")]
    Cycle(Vec<String>),
    #[error("database url error: the database scheme {0:?} is not supported")]
    UnsupportedScheme(String),
    #[cfg(feature = "dotenv")]
    #[error("DontenvError: {0}")]
    Dotenv(Arc<dotenv::Error>),
    #[error("SQLxError: {0}")]
    SQLx(Arc<sqlx::Error>),
}

impl Error {
    pub(crate) fn kind(&self) -> &'static str {
        match self {
            | Self::Cycle(_) => "CycleError",
            | Self::IO(_) => "IOError",
            | Self::Syntax(_) => "SyntaxError",
            | Self::SyntaxAtFile(_, _) => "SyntaxAtFile",
            | Self::UnsupportedScheme(_) => "UnsupportedSchemeError",
            | _ => "error",
        }
    }

    pub(crate) fn as_json(&self) -> String {
        let err_msg = format!("{}", self);
        let kind = self.kind();

        format!(
            r#"{{"kind":{kind:?},"message":{message:?}}}"#,
            kind = kind,
            message = err_msg
        )
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IO(Arc::new(err))
    }
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Error {
        Error::SQLx(Arc::new(err))
    }
}
