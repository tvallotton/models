use crate::prelude::*;
use queries::Queries;
use sqlx::{Database, Encode, Type};
use std::sync::RwLock;
mod queries;

static DATABASE_URL: Lazy<String> = Lazy::new(|| todo!());

pub static DATABASE_CONNECTION: Lazy<DatabaseConnection> = Lazy::new(|| todo!());
trait DatabasePool: Sync + Send {
    fn execute(self, )
}

struct DatabaseConnection {
    dialect: Dialect,
    queries: Queries,
    database: Result<Box<dyn DatabasePool>, Error>,
}

impl DatabaseConnection {

    fn new() -> Self {
        todo!()
    }

    async fn query_key<'q, DB, T>(&'q self, table: &'static str, key: T)
    where
        T: 'q + Send + Encode<'q, DB> + Type<DB>,
        DB: Database,
    {
        let query = self.queries.query_key(table);

        database.execute(query, key); 
    }
}
