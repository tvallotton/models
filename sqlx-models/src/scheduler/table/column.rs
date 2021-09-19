use crate::prelude::*;
use ast::ColumnDef;
use std::collections::HashSet;

use sqlx_models_parser::dialect::GenericDialect;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Column {
    pub name: Ident,
    pub r#type: DataType,
    pub options: Vec<ColumnOptionDef>,
}

impl super::get_changes::Compare for Column {
    fn name(&self) -> Result<String, Error> {
        Ok(self.name.to_string().to_lowercase())
    }

    fn bodies_are_equal(&self, other: &Self) -> bool {
        let type1 = self.r#type.to_string().to_lowercase();
        let type2 = self.r#type.to_string().to_lowercase();

        type1 == type2 && {
            let h1 = self
                .options
                .iter()
                .map(ToString::to_string)
                .map(|string| string.to_lowercase())
                .collect::<HashSet<_>>();
            let h2 = other
                .options
                .iter()
                .map(ToString::to_string)
                .map(|string| string.to_lowercase())
                .collect::<HashSet<_>>();
            h1 == h2
        }
    }
}

impl Column {
    pub fn new(name: &str, r#type: DataType, options: ColumnOptionDef) -> Self {
        Column {
            name: Ident::new(name.to_lowercase()),
            r#type,
            options: vec![options],
        }
    }

    pub fn new_with_default(name: &str, r#type: DataType, op: ColumnOptionDef, def: &str) -> Self {
        let dialect = GenericDialect {};
        let mut tokens = tokenizer::Tokenizer::new(&dialect, def);
        let mut parser = Parser::new(tokens.tokenize().unwrap(), &dialect);
        let expr = parser.parse_expr().unwrap();

        Column {
            name: Ident::new(name.to_lowercase()),
            r#type,
            options: vec![
                op,
                ast::ColumnOptionDef {
                    name: None,
                    option: ast::ColumnOption::Default(expr),
                },
            ],
        }
    }
}

impl From<ColumnDef> for Column {
    fn from(col: ColumnDef) -> Self {
        Column {
            name: col.name,
            options: col.options,
            r#type: col.data_type,
        }
    }
}

impl From<Column> for ColumnDef {
    fn from(col: Column) -> Self {
        ColumnDef {
            name: col.name,
            options: col.options,
            data_type: col.r#type,
            collation: None,
        }
    }
}
