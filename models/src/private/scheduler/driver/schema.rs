use fs::*;
use path::PathBuf;

use crate::prelude::*;
#[derive(Clone)]
pub struct Schema {
    tables: HashMap<ObjectName, Table>,
}

impl Schema {
    pub fn new() -> Result<Self> {
        let mut out = Self {
            tables: HashMap::new(),
        };
        out.init()?;
        Ok(out)
    }

    #[cfg(test)]
    fn _from_sql(sql: &str) -> Result<Self> {
        let stmts = parse_sql(sql)?;
        let mut out = Self {
            tables: HashMap::new(),
        };
        for stmt in stmts {
            out.update(&stmt)?;
        }
        Ok(out)
    }

    pub fn get_table(&self, name: &ObjectName) -> Option<&Table> {
        self.tables.get(&name)
    }

    pub fn init(&mut self) -> Result {
        let stmts = self.get_statements()?;
        for stmt in stmts {
            self.update(&stmt)?;
        }
        Ok(())
    }

    fn get_statements(&mut self) -> Result<Vec<Statement>> {
        let mut out = vec![];
        for path in self.read_dir()? {
            if !is_up_file(&path) {
                continue;
            }
            let sql = read_to_string(&path)?;
            let stmts = match parse_sql(&sql) {
                | Ok(stmts) => stmts,
                | Err(err) => return Err(Error::SyntaxAtFile(err, path)),
            };
            out.extend(stmts);
        }
        Ok(out)
    }

    fn read_dir(&self) -> Result<Vec<PathBuf>> {
        let directory = &*MIGRATIONS_DIR;
        let mut dir: Vec<_> = read_dir(directory)
            .map_err(|_| error!("could not read the \"{}\" directiory.", directory))?
            .map(|x| x.unwrap().path())
            .collect();
        dir.sort();
        Ok(dir)
    }

    pub fn update(&mut self, stmt: &Statement) -> Result {
        use Statement::*;
        match stmt {
            | CreateTable(_) => self.create_table(stmt.clone().try_into().unwrap()),
            | AlterTable(ast::AlterTable {
                name,
                operation: AlterTableOperation::RenameTable { table_name },
            }) => self.rename_table(name, table_name),
            | AlterTable(alter) => self.alter_table(&alter.name, &alter.operation),
            | Drop(drop) => self.drop_tables(drop),
            | _ => Ok(()),
        }
    }

    fn rename_table(&mut self, old_name: &ObjectName, new_name: &ObjectName) -> Result {
        let mut table = self.tables.remove(&old_name).ok_or_else(|| {
            error!(
                "attempt to rename table {:?} to {:?}, but it does not exist",
                &old_name, &new_name
            )
        })?;
        if !DIALECT.requires_move() {
            self.cascade(&old_name);
        }
        table.name = new_name.clone();
        self.tables.insert(new_name.clone(), table);
        Ok(())
    }

    fn cascade(&mut self, name: &ObjectName) {
        use TableConstraint::*;
        self.tables //
            .values_mut()
            .for_each(|table| {
                table.constraints = table
                    .constraints
                    .drain(..)
                    .filter(|constr| match constr {
                        | ForeignKey(ast::ForeignKey { foreign_table, .. }) => {
                            foreign_table == name
                        }
                        | _ => true,
                    })
                    .collect()
            });
    }

    // pub(crate) fn get_changes(&self, target: Table) -> Result<Migration> {
    //     if let Some(table) = self.tables.get(&target.name) {
    //         table.get_changes(&target)?
    //     } else {
    //         vec![target.clone().into()]
    //     }
    // }

    fn drop_tables(&mut self, drop: &ast::Drop) -> Result {
        for name in drop.names.iter() {
            if !drop.if_exists && !self.tables.contains_key(name) {
                return Err(error!(
                    "failed to load migrations. Table \"{}\" cannot be dropped as it does not exist.",
                    name
                ));
            }
            if drop.cascade {
                self.cascade(name);
            }
            self.tables.remove(name);
        }
        Ok(())
    }

    fn alter_table(&mut self, name: &ObjectName, op: &AlterTableOperation) -> Result {
        self.tables
            .get_mut(&name) //
            .map(|table| table.alter_table(op))
            .ok_or_else(|| {
                error!(
                    "failed to load migrations. Could not find the table \"{}\"",
                    name
                )
            })??;
        Ok(())
    }

    fn create_table(&mut self, table: Table) -> Result {
        let table = table;
        let tables = &mut self.tables;
        if !table.if_not_exists && tables.contains_key(&table.name) && !table.or_replace {
            return Err(error!(
                "attempting to create table \"{}\", but it already exists.",
                table.name
            ));
        }
        tables.insert(table.name.clone(), table);
        Ok(())
    }
}

fn is_up_file(file_name: &PathBuf) -> bool {
    file_name.is_file() && !file_name.to_str().unwrap().contains(".down.sql")
}
