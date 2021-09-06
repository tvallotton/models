use sqlx_models_parser::parser::ParserError;
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error("{0}")]
    SyntaxError(#[from] ParserError),
    
}





