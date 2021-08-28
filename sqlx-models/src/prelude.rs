pub use crate::migration::{
    table::{Column, Constraint},
    Table,
};

pub use lazy_static::lazy_static; 
pub use crate::model::{Dialect, Dialect::*, Model, SqlType};
pub use collections::HashMap;
pub use dotenv::*;

pub use sqlparser::*;
pub use std::{collections::*, *};

pub use ast::*;
// pub use dialect::*;
pub use parser::Parser;
pub use result::Result;
pub use url::*;

pub const parse_sql: fn(
    &dyn dialect::Dialect,
    &str,
) -> Result<Vec<Statement>, parser::ParserError> = Parser::parse_sql;


