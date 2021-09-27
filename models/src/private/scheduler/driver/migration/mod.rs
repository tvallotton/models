use super::actions::{action::Action, Actions};
use super::schema::Schema;
use super::Report;
use crate::prelude::*;
use fs::File;
#[derive(Debug)]
pub(crate) struct Migration {
    up: Vec<Statement>,
    down: Vec<Statement>,
    name: ObjectName,
}

fn timestamp() -> u128 {
    time::SystemTime::now()
        .duration_since(time::UNIX_EPOCH)
        .unwrap()
        .as_micros()
}

impl Migration {
    pub fn new(name: ObjectName) -> Self {
        Self {
            up: vec![],
            down: vec![],
            name,
        }
    }

    pub fn create_down(&mut self, old: Schema, new: &Schema, table: &ObjectName) -> Result {
        if let Some(target) = old.get_table(table) {
            let actions = Actions::new(&new, &target)?;
            
            self.down = actions
                .as_migrations()? //
                .into_iter()
                .map(|mig| mig.up)
                .fold(vec![], |mut x, mut y| {
                    x.append(&mut y);
                    x
                });
        } else {
            let drop_stmt = Statement::Drop(Drop {
                object_type: ObjectType::Table,
                if_exists: false,
                names: vec![table.clone()],
                cascade: DIALECT.supports_cascade(),
                purge: false,
            });
            self.down.push(drop_stmt);
        }
        Ok(())
    }

    pub fn up(&self) -> &[Statement] {
        &self.up[..]
    }

    pub fn is_empty(&self) -> bool {
        self.up.is_empty()
    }
    pub fn push_up(&mut self, action: Action) -> Result {
        let stmts = action.to_statements()?;
        self.up.extend(stmts);
        Ok(())
    }

    pub fn commit(self) -> Result<Option<Report>> {
        if self.is_empty() {
            return Ok(None);
        }
        let timestamp = timestamp();

        let name = format!("{}/{}_{}", *MIGRATIONS_DIR, timestamp, self.name);
        
        let up_name = format!("{}.up.sql", name);
        let down_name = format!("{}.down.sql", name);
        let mut up_file = File::create(up_name)?;
        let mut down_file = File::create(down_name)?;
        
        for stmt in self.up {
            use std::io::Write;
            #[cfg(feature = "sqlformat")]
            let stmt = Self::formatted_stmt(stmt);
            write!(up_file, "{};\n\n", stmt)?;
        }
        
        for stmt in self.down {
            use std::io::Write;
            #[cfg(feature = "sqlformat")]
            let stmt = Self::formatted_stmt(stmt);
            write!(down_file, "{};\n\n", stmt)?;
        }
        let name = self.name.to_string().to_lowercase();
        Ok(Some(Report { timestamp, name }))
    }

    #[cfg(feature = "sqlformat")]
    fn formatted_stmt(stmt: Statement) -> String {
        use sqlformat::QueryParams;
        let stmt = format!("{}", stmt);
        sqlformat::format(&stmt, &QueryParams::None, FORMAT_OPTIONS)
    }
}
