use crate::prelude::*;
use crate::Model;

mod schema;
pub mod table;

use schema::*;

pub use table::Table;

pub struct Migration {
    schema: Schema,
    target: Table,
}

impl Migration {
    pub fn get_dialect() -> Dialect {
        let url = &DATABASE_URL;
        match url.scheme() {
            "sqlite" => Sqlite,
            "postgres" => Postgres,
            "mysql" => Mysql,
            "mssql" => Mssql,
            "any" => Any,
            _ => panic!("scheme \"{}\" is not supported", url.scheme()),
        }
    }

    pub fn new<T: Model>() -> Self {
        let dialect = Self::get_dialect();
        Self {
            schema: Schema::new(dialect),
            target: T::target(),
        }
    }

    pub fn run(self) {
        let changes = self.schema.get_changes(self.target.clone());
        self.save_changes(self.target.name.clone(), changes);
    }

    fn save_changes(&self, name: ObjectName, stmts: Vec<Statement>) {
        let time = chrono::Utc::now().timestamp_nanos();
        let mut file = std::fs::File::create(format!("migrations/{}_{}.sql", time, name)).unwrap();
        for stmt in stmts {
            use std::io::Write;
            write!(file, "{}", stmt).unwrap();
        }
    }
}

#[test]
fn generate_migrations() {
    struct Example {
        // id: i32,
    }

    impl Model for Example {
        fn target() -> Table {
            let dialect = Migration::get_dialect();
            let mut table = Table::new("Example".into());
            table.columns.push(Column {
                name: "id".into(),
                r#type: <i32 as SqlType>::as_sql(dialect),
                options: vec![],
            });
            table
        }
    }

    let migration = Migration::new::<Example>();
    migration.run();
}
