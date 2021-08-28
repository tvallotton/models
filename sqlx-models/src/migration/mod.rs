use crate::Model;
use crate::{prelude::*, SqlType};

mod state;
pub mod table;

use state::*;

pub use table::Table;
struct Migration {
    dialect: Dialect,
    current_state: State,
    target: Table,
    changes: Vec<Statement>,
}

impl Migration {
    fn get_uri() -> Url {
        dotenv().ok();
        let DATABASE_URL = if let Ok(url) = var("DATABASE_URL") {
            url
        } else {
            env::var("DATABASE_URL").expect("The DATABASE_URL environment variable must be set")
        };
        Url::parse(&DATABASE_URL)
            .expect("The DATABASE_URL environment variable could not be parsed.")
    }

    pub fn get_dialect() -> Dialect {
        let url = Self::get_uri();
        match url.scheme() {
            "sqlite" => Sqlite,
            "postgres" => Postgres,
            "mysql" => Mysql,
            "mssql" => Mssql,
            "any" => Any,
            _ => panic!("scheme \"{}\" is not supported", url.scheme()),
        }
    }

    fn new<T: Model>() -> Self {
        let dialect = Self::get_dialect();
        let mut output = Migration {
            dialect: Self::get_dialect(),
            current_state: State::new(dialect),
            target: T::target(),
            changes: vec![],
        };
        output
    }

    fn run(mut self) {
        let mut failure = 0;
        let mut stmts = vec![];
        while !self.current_state.matches(&self.target) {
            let changes = self.current_state.get_changes(&self.target);
            for stmt in changes {
                self.current_state.update_state(stmt.clone());
                stmts.push(stmt);
            }
            failure += 1;
            if failure == 200 {
                panic!("Failed");
            }
        }
        Self::save_changes(stmts);
    }

    fn save_changes(stmts: Vec<Statement>) {}
}

#[test]
fn generate_migrations() {
    struct Example {
        id: i32,
    }

    impl Model for Example {
        fn target() -> Table {
            let dialect = Migration::get_dialect();
            Table {
                name: "Example".into(),
                if_not_exists: false,
                or_replace: false,
                columns: vec![Column {
                    name: "id".into(),
                    r#type: <i32 as SqlType>::as_sql(dialect),
                    options: vec![],
                }],
                constraints: vec![],
            }
        }
    }

    let migration = Migration::new::<Example>();
    migration.run();
}
