use crate::prelude::*;

use models_parser::parser::ParserError;
use std::collections::HashSet;
use std::sync::Arc;
use thiserror::Error;

macro_rules! error {
    ($($args:expr),+) => {
        Error::Message(format!($($args),*))
    };
}


#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("{0}")]
    Syntax(#[from] ParserError),
    #[error("{0}")]
    Message(String),
    #[error("Could not read or create migration file.")]
    IO(#[from] Arc<io::Error>),
    #[error("Dependency cycle detected invlonving the tables: {0:?}.")]
    Cycle(Vec<String>),

    #[error("The DATABASE_URL environment variable could not be parsed.")]
    InvalidDatabaseUrl,
}

impl Error {
    pub(crate) fn kind(&self) -> &'static str {
        match self {
            Self::Cycle(_) => "CycleError",
            Self::Message(_) => "error",
            Self::IO(_) => "IOError",
            Self::InvalidDatabaseUrl => "error",
            Self::Syntax(_) => "SyntaxError",
        }
    }

    pub(crate) fn as_json(&self) -> String {
        let err_msg = format!("{}", self);
        let kind = self.kind();
        format!(
            r#"{{"kind":{kind},"message":{message}}}"#,
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
