use crate::prelude::*;
use std::sync::Mutex;
mod migration;

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
            self.commit()
        }
    }

    fn commit(&self) {
        let mut migr = self.0.lock().unwrap();
        migr.migrate();
        let error;

        if let Err(err) = &migr.result {
            let err_msg = format!("{}", err);
            let kind = err.kind();
            error = format!(
                r#"{{"kind":{kind},"message":{message}}}"#,
                kind = kind,
                message = err_msg
            );
        } else {
            error = "null".into();
        }

        let json = format!(
            r#"{{"success": {success:?},"error": {error}}}"#,
            success = &migr.success,
            error = error
        );
        println!("<SQLX-MODELS-OUTPUT>{0}</SQLX-MODELS-OUTPUT>", json);
    }
}
