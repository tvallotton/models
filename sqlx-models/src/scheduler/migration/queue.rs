use super::*;
pub use std::collections::HashSet;
pub use topological_sort::TopologicalSort;
#[derive(Debug)]
pub(crate) struct Queue {
    pub tables: HashMap<String, Table>,
    pub sort: TopologicalSort<String>,
}

impl Queue {
    pub fn new() -> Self {
        Self {
            tables: HashMap::new(),
            sort: TopologicalSort::new(),
        }
    }
    pub fn len(&self) -> usize {
        self.tables.len()
    }

    pub fn insert(&mut self, table: Table) {
        let table_name = table.dep_name();
        self.tables.insert(table_name.clone(), table.clone());
        self.sort.insert(table_name.clone());
        for dep in table.dependencies() {
            self.sort.add_dependency(dep, table_name.clone())
        }
    }
    pub fn pop(&mut self) -> Option<Table> {
        self.sort.pop().and_then(|value| self.tables.remove(&value))
    }
}
