mod schema_initialization;

use crate::prelude::*;
use crate::scheduler::Table;
// use fs::*;
use std::{convert::TryInto, path::PathBuf};
use Statement::*;

#[derive(Debug, Clone)]
pub struct Schema {
    pub tables: HashMap<ObjectName, Table>,
}

impl Schema {
    #[throws(Error)]
    pub fn get_changes(&self, target: Table) -> Vec<Statement> {
        if let Some(table) = self.tables.get(&target.name) {
            table.get_changes(&target)?
        } else {
            vec![target.clone().into()]
        }
    }
    #[throws(Error)]
    pub(crate) fn update_schema(&mut self, stmt: Statement) {
        match stmt {
            CreateTable(_) => self.create_table(stmt.try_into().unwrap())?,
            AlterTable(ast::AlterTable {
                name,
                operation: AlterTableOperation::RenameTable { table_name },
            }) => self.rename_table(name, table_name)?,
            AlterTable(alter) => self.alter_table(alter.name, alter.operation)?,
            Drop(drop) => self.drop_tables(drop.names, drop.if_exists, drop.cascade)?,
            _ => (),
        }
    }
    #[throws(Error)]
    fn rename_table(&mut self, old_name: ObjectName, new_name: ObjectName) {
        let mut table = self.tables.remove(&old_name).ok_or_else(|| {
            error!(
                "Attempt to rename table {:?} to {:?}, but it does not exist",
                &old_name, &new_name
            )
        })?;
        self.cascade(&old_name);
        table.name = new_name.clone();
        self.tables.insert(new_name, table);
    }
    /// Deletes all constraints containing the table name from
    /// the remaining tables.

    fn cascade(&mut self, name: &ObjectName) {
        use TableConstraint::*;
        self.tables //
            .values_mut()
            .for_each(|table| {
                table.constraints = table
                    .constraints
                    .drain(..)
                    .filter(|constr| match constr {
                        ForeignKey(ast::ForeignKey { foreign_table, .. }) => foreign_table == name,
                        _ => true,
                    })
                    .collect()
            });
    }

    /// dropts a list of tables
    #[throws(Error)]
    fn drop_tables(&mut self, names: Vec<ObjectName>, if_exists: bool, cascade: bool) {
        for name in names.iter() {
            if !if_exists && !self.tables.contains_key(name) {
                throw!(error!(
                    "Table \"{}\" cannot be dropped as it does not exist.",
                    name
                ))
            }
            if cascade {
                self.cascade(name);
            }
            self.tables.remove(name);
        }
    }

    #[throws(Error)]
    fn alter_table(&mut self, name: ObjectName, op: AlterTableOperation) {
        self.tables
            .get_mut(&name) //
            .map(|table| table.alter_table(op))
            .ok_or_else(|| {
                error!(
                    "Failed to load migrations. Could not find the table \"{}\"",
                    name
                )
            })??;
    }

    fn create_table(&mut self, table: Table) -> Result<(), Error> {
        let table = table;
        let tables = &mut self.tables;
        if !table.if_not_exists && tables.contains_key(&table.name) && !table.or_replace {
            return Err(error!(
                "Attempting to create table \"{}\", but it already exists.",
                table.name
            ));
        }
        tables.insert(table.name.clone(), table);
        Ok(())
    }
}
