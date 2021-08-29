use super::table::Table;
use crate::prelude::*;
use fs::*;
use std::{convert::TryInto, path::PathBuf};
use Statement::*;

#[derive(Debug, Clone)]
pub(super) struct Schema {
    pub(crate) dialect: Dialect,
    pub tables: HashMap<ObjectName, Table>,
}

impl Schema {
    /// constructs a new State from the "migrations/" directory.
    pub fn new(dialect: Dialect) -> Self {
        let mut out = Self {
            dialect,
            tables: HashMap::new(),
        };
        out.init();
        out
    }

    pub fn get_changes(&self, target: Table) -> Vec<Statement> {
        if let Some(table) = self.tables.get(&target.name) {
            table.get_changes(&target, self)
        } else {
            vec![target.clone().into()]
        }
    }

    /// Computes the current state of the schema
    /// from the "migrations/" directory.
    fn init(&mut self) {
        let stmts = self.get_statements();
        for stmt in stmts {
            self.update_schema(stmt);
        }
    }

    pub(super) fn update_schema(&mut self, stmt: Statement) {
        use ObjectType::*;
        match stmt {
            CreateTable { .. } => self.create_table(stmt.try_into()),
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
                if !(if_exists && self.tables.contains_key(name)) {
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
    /// It retrieves a vec of all statements in the "migrations/" directory
    /// In the order they were written.
    fn get_statements(&mut self) -> Vec<Statement> {
        self.read_dir()
            .into_iter()
            .filter(|file| file.is_file())
            .map(read_to_string)
            .map(Result::unwrap)
            .map(|sql| parse_sql(&self.dialect, &sql))
            .map(Result::unwrap)
            .fold(vec![], |mut a, mut b| {
                a.append(&mut b);
                a
            })
    }
    /// returns a list of all the files in the migrations directory.carg
    fn read_dir(&self) -> Vec<PathBuf> {
        let mut dir: Vec<_> = read_dir("migrations/")
            .or_else(|_| {
                create_dir("migrations/") //
                    .and_then(|_| read_dir("migrations/"))
            })
            .expect("Could not read the \"migrations/\" directiory.")
            .map(|x| x.unwrap().path())
            .collect();
        dir.sort();
        dir
    }
}

// #[test]
// fn state_new() {
//     State::new()
// }
