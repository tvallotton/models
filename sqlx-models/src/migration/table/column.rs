use crate::{migration::state::State, prelude::*};

use ast::ColumnDef;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Column {
    pub name: String,
    pub r#type: DataType,
    pub options: Vec<ColumnOptionDef>,
}

impl Column {
    pub(super) fn get_changes(&mut self, target: &Column, dialect: Dialect) -> Vec<Statement> {
        if target == self {
            vec![]
        } else {
            match dialect {
                Sqlite => {
                    todo!()
                },
                Postgres => {
                    todo!()
                }
                Mysql => {
                    todo!()
                }
                Mssql => {
                    todo!()
                }
                Any => {
                    todo!()
                }
            }
        }
    }
}

impl From<ColumnDef> for Column {
    fn from(col: ColumnDef) -> Self {
        Column {
            name: col.name.value,
            options: col.options,
            r#type: col.data_type,
        }
    }
}

impl From<Column> for ColumnDef {
    fn from(col: Column) -> Self {
        ColumnDef {
            name: ast::Ident::new(col.name),
            options: col.options,
            data_type: col.r#type,
            collation: None,
        }
    }
}
