use crate::prelude::*;
use crate::Model;
use std::*;
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
            target: T::target(dialect),
        }
    }

    pub fn run(mut self) {
        let _lock = MIGRATIONS_LOCK.lock();
        let changes = self.get_changes();

        if !changes.is_empty() {
            self.save_changes(self.target.name.clone(), changes);
        }
    }
    pub fn get_changes(&mut self) -> Vec<Statement> {
        let mut changes = vec![];
        loop {
            let stmts = self.schema.get_changes(self.target.clone());
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

    fn save_changes(&self, name: ObjectName, stmts: Vec<Statement>) {
        let time = chrono::Utc::now().timestamp_nanos();
        {
            let mut file =
                std::fs::File::create(format!("migrations/{}_{}.sql", time, name)).unwrap();
            for stmt in stmts {
                use std::io::Write;
                let stmt = Self::formatted_stmt(stmt);
                write!(file, "{};\n\n", stmt).unwrap();
            }
        }
    }
    fn formatted_stmt(stmt: Statement) -> String {
        use sqlformat::QueryParams;
        let stmt = format!("{}", stmt);
        sqlformat::format(&stmt, &QueryParams::None, FORMAT_OPTIONS)
    }
}
#[test]
fn generate_migrations() {
    struct Example {
        // id: i32,
    }

    impl Model for Example {
        fn target(dialect: Dialect) -> Table {
            let dialect = Migration::get_dialect();
            let mut table = Table::new("Example".into());
            table.columns.push(Column {
                name: "id".into(),
                r#type: <i32 as crate::SqlType>::as_sql(),
                options: vec![],
            });
            table
        }
    }

    let migration = Migration::new::<Example>();
    migration.run();
}

#[test]
fn func() {
    println!("{}", sqlformat::format(
                &"create table user (id integer primary key,  id integer primary key , id integer primary key ); select * from user, post where user.id = post.id; create table lol (email TEXT not null);",
                &sqlformat::QueryParams::None,
                FORMAT_OPTIONS,
            ));
}
