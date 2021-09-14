mod queue;
mod schema;
use crate::prelude::*;
use crate::{
    error::Error, //
    model::Dialect,
    scheduler::table::Table,
};
use queue::Queue;
pub use schema::Schema;
use std::fs::File;

pub(crate) struct Migration {
    pub result: Result<Schema, Error>,
    pub queue: Queue,

    pub success: Vec<String>,
}

impl Migration {
    pub fn new() -> Self {
        let result = Schema::new();
        Self {
            result,
            queue: Queue::new(),
            success: vec![],
        }
    }

    fn migrate(&mut self) {
        loop {
            match self.queue.pop() {
                Some(target) => self.migrate_table(target),
                None => {
                    if self.queue.len() != 0 && self.result.is_ok() {
                        self.result = Err(Error::CycleError(
                            self.queue.tables.keys().cloned().collect(),
                        ));
                    }
                    break;
                }
            }
        }
    }

    fn migrate_table(&mut self, target: Table) {
        if let Ok(schema) = &mut self.result {
            let table_name = target.dep_name();
            let changes = Self::get_changes(target, schema);
            
            match Self::save(&table_name, changes) {
                Ok(()) => {
                    self.success.push(table_name.clone());
                }
                Err(error) => {
                    self.result = Err(error);
                }
            }
        }
    }

    fn get_changes(target: Table, schema: &mut Schema) -> Vec<Statement> {
        let mut changes = vec![];

        loop {
            let stmts = schema.get_changes(target.clone());
            if stmts.is_empty() {
                break;
            }
            for stmt in stmts {
                changes.push(stmt.clone());
                schema.update_schema(stmt);
            }
        }
        changes
    }

    #[throws(Error)]
    fn save(name: &String, stmts: Vec<Statement>) {
        let time = chrono::Utc::now().timestamp_millis();
        {
            let file_name = format!(
                "migrations/{}_{}.sql", //
                time, name
            );
            let mut file = File::create(file_name) //
                .map_err(|_| Error::IOError)?;
            for stmt in stmts {
                use std::io::Write;
                let stmt = Self::formatted_stmt(stmt);
                write!(file, "{};\n\n", stmt).map_err(|_| Error::IOError)?;
            }
        }
    }

    fn formatted_stmt(stmt: Statement) -> String {
        use sqlformat::QueryParams;
        let stmt = format!("{}", stmt);
        sqlformat::format(&stmt, &QueryParams::None, FORMAT_OPTIONS)
    }
}
