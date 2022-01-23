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
    #[error("{0}")]
    Syntax(#[from] ParserError),
    #[error("{0}.\n       found at file \"{1}\".")]
    SyntaxAtFile(ParserError, path::PathBuf),
    #[error("{0}")]
    Message(String),
    #[error("could not read or create migration file. {0}")]
    IO(#[from] Arc<io::Error>),
    #[error("dependency cycle detected invlonving the tables: {0:?}. help: consider removing redundant foreign key constraints.")]
    Cycle(Vec<String>),
    #[error("the database scheme {0:?} is not supported")]
    UnsupportedScheme(String),
    #[cfg(feature = "dotenv")]
    #[error("{0}")]
    Dotenv(Arc<dotenv::Error>),
    #[error("{0}")]
    SQLx(Arc<sqlx::Error>),
    #[error("the datatype {ty} is not supported by {dialect:?}.")]
    UnsupportedDatatype { ty: DataType, dialect: Dialect },
}

impl Error {
    pub(crate) fn kind(&self) -> &'static str {
        match self {
            | Self::Syntax(_) => "syntax",
            | Self::SyntaxAtFile(_, _) => "syntax",
            | Self::Cycle(_) => "cycle",
            | Self::IO(_) => "io",
            | Self::UnsupportedScheme(_) => "database url",
            | Self::Dotenv(_) => "doetenv",
            | Self::SQLx(_) => "sqlx",
            | Self::UnsupportedDatatype { .. } => "datatype",
            | _ => "error",
        }
    }

    pub(crate) fn log(&self) {
        print!(
            r#"<SQLX-MODELS-OUTPUT>{{"kind":{kind:?},"message":{message:?}}}</SQLX-MODELS-OUTPUT>"#,
            kind = self.kind(),
            message = self
        );
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
