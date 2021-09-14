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

pub(crate) struct Migration {
    pub schema: Result<Schema, Error>,
    pub queue: Queue,

    pub success: Vec<String>,
}

impl Migration {
    pub fn new() -> Self {
        let schema = Schema::new();
        Self {
            schema,
            queue: Queue::new(),
            success: vec![],
        }
    }

    fn migrate(&self) {
        loop {
            
        }
    }

    fn migrate_table(&mut self, target: Table) {
        if self.schema.is_ok() {
            let changes = self.get_changes(target);
        }
    }

    fn get_changes(&mut self, target: Table) -> Vec<Statement> {
        let mut changes = vec![];
        let schema = self.schema.as_mut().unwrap();
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
}
