mod schema_initialization;

use crate::scheduler::Table;
use crate::prelude::*;
// use fs::*;
use std::{convert::TryInto, path::PathBuf};
use Statement::*;

#[derive(Debug, Clone)]
pub struct Schema {
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

    pub(crate) fn update_schema(&mut self, stmt: Statement) {
        match stmt {
            CreateTable(_) => self.create_table(stmt.try_into().unwrap()),
            AlterTable(ast::AlterTable {
                name,
                operation: AlterTableOperation::RenameTable { table_name },
            }) => self.rename_table(name, table_name),
            AlterTable(alter) => self.alter_table(alter.name, alter.operation),
            Drop(drop) => self.drop_tables(drop.names, drop.if_exists, drop.cascade),
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
                        ForeignKey(ast::ForeignKey { foreign_table, .. }) => foreign_table == name,
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
                    self.cascade(name);
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

    fn create_table(&mut self, table: Table) {
        let table = table;
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
