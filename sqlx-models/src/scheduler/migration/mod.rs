mod queue;
mod schema;
use crate::prelude::*;
use crate::{
    error::Error, //
    // model::Dialect,
    scheduler::table::Table,
};
use queue::Queue;
pub use schema::Schema;
use std::fs::File;

pub(crate) struct Migration {
    pub result: Result<Schema, Error>,
    pub queue: Queue,

    pub success: Vec<(i64, String)>,
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

    pub fn migrate(&mut self) {
        loop {
            match self.queue.pop() {
                Some(target) => self.migrate_table(target),
                None => {
                    if self.queue.len() != 0 && self.result.is_ok() {
                        self.result =
                            Err(Error::Cycle(self.queue.tables.keys().cloned().collect()));
                    }
                    break;
                }
            }
        }
    }
    #[throws(Error)]
    fn try_migration(&mut self, target: Table) {
        if let Ok(schema) = &mut self.result {
            let table_name = target.dep_name();
            let changes = Self::get_changes(target, schema)?;
            let time = Self::save(&table_name, changes)?;
            if let Some(time) = time {
                self.success.push((time, table_name));
            }
        }
    }

    fn migrate_table(&mut self, target: Table) {
        match self.try_migration(target) {
            Err(error) => {
                self.result = Err(error);
            }
            _ => (),
        }
    }

    #[throws(Error)]
    fn get_changes(target: Table, schema: &mut Schema) -> Vec<Statement> {
        let mut changes = vec![];

        loop {
            let stmts = schema.get_changes(target.clone())?;
            if stmts.is_empty() {
                break;
            }
            for stmt in stmts {
                changes.push(stmt.clone());
                schema.update_schema(stmt)?;
            }
        }
        changes
    }

    fn save(name: &str, stmts: Vec<Statement>) -> Result<Option<i64>, Error> {
        if stmts.is_empty() {
            return Ok(None);
        }
        let time = chrono::Utc::now().timestamp_millis();

        let file_name = format!(
            "{}/{}_{}.sql", //
            *MIGRATIONS_DIR, time, name
        );
        let mut file = File::create(file_name)?;
        for stmt in stmts {
            use std::io::Write;
            let stmt = Self::formatted_stmt(stmt);
            write!(file, "{};\n\n", stmt)?;
        }
        Ok(Some(time))
    }

    fn formatted_stmt(stmt: Statement) -> String {
        use sqlformat::QueryParams;
        let stmt = format!("{}", stmt);
        sqlformat::format(&stmt, &QueryParams::None, FORMAT_OPTIONS)
    }
}
