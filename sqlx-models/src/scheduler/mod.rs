use crate::prelude::*;
use std::{fs::*, sync::Mutex};
mod migration;
pub(crate) use migration::Schema;

pub mod table;

use migration::Migration;

pub use table::Table;

pub struct Scheduler(Mutex<Migration>);

impl Scheduler {
    pub(crate) fn new() -> Self {
        Self(Mutex::new(Migration::new()))
    }
    /// Allows tables to register themselves into the migration.
    /// The first table to register will wait for 250 milliseconds before
    /// generating the migration files.
    pub fn register(&self, table: Table) {
        let len;
        {
            let mut migration = self.0.lock().unwrap();
            len = migration.queue.len();

            migration.queue.insert(table)
        }

        if len == 0 {
            std::thread::sleep(time::Duration::from_millis(250));
            self.0.lock().unwrap();
        }
    }

    pub fn commit(&self) {}

    // fn run<T>(&self, table: Table) {
    //     match self.generate_migrations(table) {
    //         Ok(_) => println!(""),
    //         Err(error) => error.commit(),
    //     }
    // }

    // #[throws(Error)]
    // fn generate_migrations(&self, target: Table) {
    //     let changes = self.get_changes(&target);
    //     if !changes.is_empty() {
    //         self.save_changes(target.name.clone(), changes)?;
    //     }
    // }

    // #[throws(Error)]
    // fn save_changes(&self, name: ObjectName, stmts: Vec<Statement>) {
    //     let time = chrono::Utc::now().timestamp_nanos();
    //     {
    //         let mut file = File::create(format!("migrations/{}_{}.sql", time, name))?;
    //         for stmt in stmts {
    //             use std::io::Write;
    //             let stmt = Self::formatted_stmt(stmt);
    //             write!(file, "{};\n\n", stmt)?;
    //         }
    //     }
    // }
    fn formatted_stmt(stmt: Statement) -> String {
        use sqlformat::QueryParams;
        let stmt = format!("{}", stmt);
        sqlformat::format(&stmt, &QueryParams::None, FORMAT_OPTIONS)
    }
}
