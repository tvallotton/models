use crate::migration::Table;
use crate::prelude::*;

use sqlx_models_parser::ast::{ForeignKey, TableConstraint};
use std::sync::Mutex;
use std::sync::MutexGuard;
use std::thread::sleep;
use std::time::Duration;
use topological_sort::TopologicalSort;
struct Node {
    name: String,
    dep: Vec<String>,
}
impl From<Table> for Node {
    fn from(table: Table) -> Self {
        let dep = table
            .constraints
            .into_iter()
            .filter_map(|constr| match constr {
                TableConstraint::ForeignKey(ForeignKey { foreign_table, .. }) => {
                    Some(foreign_table.to_string().to_lowercase())
                }
                _ => None,
            })
            .collect();
        let name = table.name.to_string().to_lowercase();
        Self { name, dep }
    }
}

fn get_migrations_dir() -> String {
    var("MIGRATIONS_DIR").unwrap_or_else(|_| "migrations/".into())
}
struct Queue {
    sort: TopologicalSort<String>,
    tables_run: usize,
}
pub struct Scheduler {
    inner: Mutex<Queue>,
    directory: Mutex<String>,
}
impl Scheduler {
    pub fn new() -> Self {
        Self {
            inner: Mutex::new(Queue {
                sort: TopologicalSort::new(),
                tables_run: 0,
            }),
            directory: Mutex::new(get_migrations_dir()),
        }
    }
    pub fn get_directory<T: crate::model::Model>(&self) -> MutexGuard<String> {
        let dialect = crate::migration::Migration::get_dialect();
        let table = T::target(dialect);
        let name;
        {
            let node: Node = table.into();
            let mut sort = self.inner.lock().unwrap();
            name = node.name.clone();
            for dep in node.dep {
                sort.sort.add_dependency(dep, &node.name);
            }
        }
        sleep(Duration::from_millis(50));
        self.lock(&name)
    }

    fn lock(&self, name: &str) -> MutexGuard<String> {
        for _ in 0..500 {
            {
                let directory = self.directory.lock().unwrap();
                let mut sort = self.inner.lock().unwrap();
                let current_turn = sort.sort.clone().nth(sort.tables_run);
                if Some(name) == current_turn.as_deref() {
                    sort.tables_run += 1;
                    return directory;
                }
            }
            sleep(Duration::from_millis(1));
        }
        self.directory.lock().unwrap()
    }
}
