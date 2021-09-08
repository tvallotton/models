use crate::migration::Migration;
use crate::migration::Table;
use crate::model::Model;
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

struct InnerScheduler {
    migration: Result<Migration, Error>,
    queue: TopologicalSort<String>,
    finished: Vec<String>,
}
pub struct Scheduler(Mutex<InnerScheduler>);

impl Scheduler {
    pub fn new(model: &dyn Model) -> Self {
        let migration = Migration::new(model);
        Self(Mutex::new(InnerScheduler {
            migration,
            queue: TopologicalSort::new(),
            finished: vec![],
        }))
    }
    #[throws(Error)]
    pub fn do_migration(&self, model: &dyn Model) -> MutexGuard<()> {

        let name;
        {   
            let table = model.target(); 
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

    fn lock(&self, name: &str) -> MutexGuard<()> {
        for _ in 0..500 {
            {
                let handle = self.handle.lock().unwrap();
                let mut sort = self.inner.lock().unwrap();
                let current_turn = sort.sort.clone().nth(sort.tables_run);
                if Some(name) == current_turn.as_deref() {
                    sort.tables_run += 1;
                    return handle;
                }
            }
            sleep(Duration::from_millis(1));
        }
        self.handle.lock().unwrap()
    }
}
