mod schema_initialization;

use super::table::Table;
use crate::prelude::*;
use fs::*;
use std::{convert::TryInto, path::PathBuf};
use Statement::*;

#[derive(Debug, Clone)]
pub(crate) struct Schema {
    pub dialect: Dialect,
    pub tables: HashMap<ObjectName, Table>,
}

impl Schema {
    pub fn get_changes(&self, target: Table) -> Vec<Statement> {
        if let Some(table) = self.tables.get(&target.name) {
            table.get_changes(&target, self)
        } else {
            vec![target.clone().into()]
        }
    }

    pub(super) fn update_schema(&mut self, stmt: Statement) {
        use ObjectType::*;
        match stmt {
            CreateTable { .. } => self.create_table(stmt.try_into()),
            AlterTable {
                name,
                operation: AlterTableOperation::RenameTable { table_name },
            } => self.rename_table(name, table_name),
            AlterTable { name, operation } => self.alter_table(name, operation),
            Drop {
                object_type: Table,
                if_exists,
                names,
                cascade,
                ..
            } => self.drop_tables(names, if_exists, cascade),
            _ => (),
        }
    }

    fn rename_table(&mut self, old_name: ObjectName, new_name: ObjectName) {
        let mut table = self.tables.remove(&old_name).unwrap();
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
                        ForeignKey { foreign_table, .. } => foreign_table == name,
                        _ => true,
                    })
                    .collect()
            });
    }

    /// dropts a list of tables
    fn drop_tables(&mut self, names: Vec<ObjectName>, if_exists: bool, cascade: bool) {
        names
            .iter() //
            .for_each(|name| {
                if !if_exists && !self.tables.contains_key(name) {
                    panic!("Table \"{}\" cannot be dropped as it does not exist.", name)
                }
                if cascade {
                    self.cascade(&name);
                }
                self.tables.remove(name);
            })
    }

    fn alter_table(&mut self, name: ObjectName, op: AlterTableOperation) {
        self.tables
            .get_mut(&name) //
            .map(|table| table.alter_table(op))
            .expect(&format!(
                "Failed to load migrations. Could not find the table \"{}\"",
                name
            ));
    }
    fn create_table(&mut self, table: Result<Table, String>) {
        let table = table.unwrap();
        let tables = &mut self.tables;
        if !table.if_not_exists && tables.contains_key(&table.name) && !table.or_replace {
            panic!(
                "Incompatible migrations. Table \"{}\" already exists.",
                table.name
            );
        }
        tables.insert(table.name.clone(), table);
    }
}
