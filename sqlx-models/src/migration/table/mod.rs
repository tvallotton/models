use std::convert::{TryFrom, TryInto};

use crate::prelude::*;
mod column;

// C++ es el mejor lenguaje de pr... 5884 Segmentation fault: 11
use column::*;

#[derive(Debug, Clone)]
pub struct Table {
    pub name: String,
    pub if_not_exists: bool,
    pub or_replace: bool,
    pub columns: Vec<Column>,
    pub constraints: Vec<TableConstraint>,
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CREATE TABLE {} (", self.name.to_string())?;
        // for column in &self.columns {
        //     write!(f, "\n{},", column.to_string())?;
        // }
        // for constraint in &self.constraints {
        //     write!(f, "\n{},", constraint.to_string())?;
        // }
        write!(f, "\n);\n")
    }
}

impl Table {
    pub fn create_table() {
        todo!()
    }

    pub fn alter_table(&mut self, op: AlterTableOperation) {
        use AlterTableOperation::*;
        match op {
            AddColumn { column_def } => self.columns.push(column_def.into()),
            AddConstraint(constr) => self.constraints.push(constr),
            DropColumn {
                column_name,
                if_exists,
                cascade,
            } => self.drop_col(column_name, if_exists, cascade),
            RenameColumn {
                old_column_name,
                new_column_name,
            } => self.rename_col(old_column_name, new_column_name),
            _ => panic!(""),
        }
        todo!()
    }

    pub fn new(name: String) -> Self {
        Table {
            name,
            columns: vec![],
            constraints: vec![],
            or_replace: false,
            if_not_exists: false,
        }
    }

    pub fn drop_col(&mut self, name: Ident, if_exists: bool, cascade: bool) {
        self.columns = self
            .columns
            .iter()
            .filter(|col| col.name == name.value)
            .map(Clone::clone)
            .collect();
    }

    pub fn rename_col(&mut self, old: Ident, new: Ident) {
        self.columns = self
            .columns
            .iter()
            .map(Clone::clone)
            .map(|mut col| {
                if col.name == old.value {
                    col.name = new.value.clone();
                }
                col
            })
            .collect()
    }
}

impl TryFrom<Statement> for Table {
    type Error = String;
    fn try_from(value: Statement) -> Result<Self, Self::Error> {
        match value {
            Statement::CreateTable {
                or_replace,
                temporary,
                external,
                if_not_exists,
                name,
                columns,
                constraints,
                hive_distribution,
                hive_formats,
                table_properties,
                with_options,
                file_format,
                location,
                query,
                without_rowid,
                like,
            } => Ok(Table {
                name: name.0.into_iter().take(1).next().unwrap().value,
                if_not_exists,
                or_replace,
                columns: columns.into_iter().map(Into::into).collect(),
                constraints: constraints,
            }),
            value => Err(format!(
                "Expected a \"CREATE TABLE\" statement, found {}",
                value
            )),
        }
    }
}
