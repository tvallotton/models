use crate::prelude::*;

use ast::ColumnDef;

#[derive(Debug, Clone, PartialEq)]
pub struct Column {
    pub name: String,
    pub r#type: DataType,
    pub options: Vec<ColumnOptionDef>,
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
