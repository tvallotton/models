use crate::prelude::*;
use crate::Model;
mod change;
mod state;
mod table;
use change::Change;
use state::*;
pub use table::Table;
struct Migration {
    dialect: &'static dyn dialect::Dialect,
    current_state: State,
    target: Table,
    changes: Vec<Change>,
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

    fn get_dialect() -> &'static dyn Dialect {
        let url = Self::get_uri();
        match url.scheme() {
            "sqlite" => &dialect::SQLiteDialect {},
            "postgres" => &dialect::PostgreSqlDialect {},
            "mysql" => &dialect::MySqlDialect {},
            "mssql" => &dialect::MsSqlDialect {},
            _ => panic!("scheme \"{}\" is not supported", url.scheme()),
        }
    }

    fn get_current_state(&mut self) {
        todo!()
    }

    fn new<T: Model>() -> Self {
        let dialect = Self::get_dialect();
        let mut output = Migration {
            dialect: Self::get_dialect(),
            current_state: State::new(dialect),
            target: T::table(),
            changes: vec![],
        };
        output.get_current_state();
        output
    }

    fn run() {}

    fn read_from_environment() -> Self {
        todo!()
    }
}

#[test]
fn generate_migrations() {}
#[test]
fn generate_migration() {}
