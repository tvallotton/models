use crate::prelude::*;
use std::{convert::TryFrom, iter::Copied};
mod column;
mod constraint;
pub use column::Column;
pub use constraint::Constraint;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Table {
    pub name: String,
    pub if_not_exists: bool,
    pub or_replace: bool,
    pub columns: Vec<Column>,
    pub constraints: Vec<TableConstraint>,
}

impl Table {
    pub(super) fn get_changes(&mut self, target: &Table, dialect: Dialect) -> Vec<Statement> {
        let col_move = self.is_move_required(&target);
        if dialect.requires_move() && col_move.is_some() {
            self.make_move(&target, col_move.unwrap())
        } else {
            todo!()
        }

        // let mut to_add = vec![];
        // let mut to_remove = vec![];
        // let mut to_change = vec![];

        // for col in &mut self.columns {
        //     if let Some(col_target) = target.columns.iter().filter(|c| c.name != col.name).next() {
        //         to_change.append(&mut col.get_changes(col_target, dialect));
        //     }
        // }
        // let mut out = vec![];
        // out.append(&mut to_remove);
        // out.append(&mut to_change);
        // out.append(&mut to_add);
        // out
    }

    fn make_move(&self, target: &Table, to_move: (Column, Option<Column>)) -> Vec<Statement> {
        if to_move.1.is_none() {
            self.remove_col_with_move(to_move.0)
        } else {
            self.replace_col_with_move(to_move.0, to_move.1)
        }
    }

    fn remove_col_with_move(&self, col: Column) -> Vec<Statement> {

        let tmp = self.clone(); 
        todo!()
        

    }

    fn is_move_required(&self, target: &Table) -> Option<(Column, Option<Column>)> {
        for col0 in &self.columns {
            for col1 in &target.columns {
                if col0.name == col1.name && col0 != col1 {
                    return Some((col0.clone(), Some(col1.clone())));
                }
            }
            if target.columns.iter().any(|col1| col1.name == col0.name) {
                return Some((col0.clone(), None));
            }
        }
        None
    }

    pub(super) fn alter_table(&mut self, op: AlterTableOperation) {
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
    }

    pub(super) fn new(name: String) -> Self {
        Table {
            name,
            columns: vec![],
            constraints: vec![],
            or_replace: false,
            if_not_exists: false,
        }
    }

    pub(super) fn drop_col(&mut self, name: Ident, if_exists: bool, cascade: bool) {
        let len = self.columns.len();
        self.columns = self
            .columns
            .drain(..)
            .filter(|col| col.name != name.value)
            .collect();
        assert!(
            len != self.columns.len() || if_exists,
            "Column \"{}\" does not exists",
            name
        );
        assert!(
            !cascade,
            "Cascade while dropping columns is not supported yet."
        );
    }

    pub(super) fn rename_col(&mut self, old: Ident, new: Ident) {
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
                if_not_exists,
                name,
                columns,
                constraints,
                ..
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
impl From<Table> for Statement {
    fn from(table: Table) -> Self {
        Statement::CreateTable {
            or_replace: false,
            temporary: false,
            external: false,
            if_not_exists: false,
            name: ObjectName(vec![Ident::new(table.name)]),
            columns: table.columns.into_iter().map(Into::into).collect(),
            constraints: table.constraints,
            hive_distribution: HiveDistributionStyle::NONE,
            hive_formats: None,
            table_properties: vec![],
            with_options: vec![],
            file_format: None,
            location: None,
            query: None,
            without_rowid: true,
            like: None,
        }
    }
}
