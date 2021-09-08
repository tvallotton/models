use crate::prelude::*;
use sqlx_models_parser::parser::ParserError;
use thiserror::Error;
#[macro_use]
macro_rules! error {
    ($($args:expr),+) => {
        Err(Error::Message(format!($($args),*)))?
    };
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    SyntaxError(#[from] ParserError),
    #[error("{0}")]
    Message(String),
    #[error("Could not read or create migration file: {0}")]
    IOError(#[from] io::Error),
}

impl Error {
    pub(crate) fn commit(self) {
        println!("{}", self);
    }
}



