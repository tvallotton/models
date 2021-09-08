use crate::prelude::*;
use std::{
    fs::*,
    sync::{Mutex, MutexGuard},
};
mod schema;
pub mod table;
use crate::model::Model;
use schema::*;

pub use table::Table;

pub struct Migration {
    schema: Schema,
    dialect: Dialect,
}

impl Migration {
    #[throws(Error)]
    pub(crate) fn get_dialect() -> Dialect {
        let url = &DATABASE_URL;
        match url.scheme() {
            "sqlite" => Sqlite,
            "postgres" => Postgres,
            "mysql" => Mysql,
            "mssql" => Mssql,
            "any" => Any,
            _ => error!("scheme \"{}\" is not supported", url.scheme()),
        }
    }
    #[throws(Error)]
    pub(crate) fn new(model: &dyn Model) -> Self {
        let dialect = Self::get_dialect()?;
        Self {
            schema: Schema::new(dialect)?,
            dialect,
        }
    }
    pub(crate) fn run(&self, model: &dyn Model) {
        match self.generate_migrations(model) {
            Ok(_) => println!(""),
            Err(error) => error.commit(),
        }
    }

    #[throws(Error)]
    fn generate_migrations(&self, model: &dyn Model) {
        let mut migr = Self::new(model)?;
        let target = model.target(self.dialect);
        let changes = migr.get_changes(&target);
        if !changes.is_empty() {
            migr.save_changes(target.name.clone(), changes)?;
        }
    }
    fn get_changes(&mut self, target: &Table) -> Vec<Statement> {
        let mut changes = vec![];
        loop {
            let stmts = self.schema.get_changes(target.clone());
            if stmts.is_empty() {
                break;
            }
            for stmt in stmts {
                changes.push(stmt.clone());
                self.schema.update_schema(stmt);
            }
        }
        changes
    }
    #[throws(Error)]
    fn save_changes(&self, name: ObjectName, stmts: Vec<Statement>) {
        let time = chrono::Utc::now().timestamp_nanos();
        {
            let mut file = File::create(format!("migrations/{}_{}.sql", time, name))?;
            for stmt in stmts {
                use std::io::Write;
                let stmt = Self::formatted_stmt(stmt);
                write!(file, "{};\n\n", stmt)?;
            }
        }
    }
    fn formatted_stmt(stmt: Statement) -> String {
        use sqlformat::QueryParams;
        let stmt = format!("{}", stmt);
        sqlformat::format(&stmt, &QueryParams::None, FORMAT_OPTIONS)
    }
}
