mod sorter;
pub use std::collections::HashSet;

pub use sorter::Sorter;

use super::*;
/// this structure is used to remove 
/// tables from the queue in topological order. 
/// It basically pops a table if it doesn't have
/// any dependencies. It then removes this table from 
/// the dependencies of the remaining tables. This way
/// if table A depends on table B, table B will aways be
/// popped before table A.
pub(crate) struct Queue {
    tables: HashMap<String, Table>,
    sorter: Sorter,
}

impl Queue {
    pub fn new() -> Self {
        Self {
            tables: HashMap::new(),
            sorter: Sorter::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.tables.len()
    }

    pub fn insert(&mut self, table: Table) {
        let table_name = &table.name();
        self.tables.insert(table_name.clone(), table.clone());
        self.sorter.insert(table_name.clone());
        for dep in table.deps() {
            self.sorter.add_dependency(dep, table_name.clone())
        }
    }

    pub fn pop(&mut self) -> Option<Table> {
        self.sorter
            .pop()
            .and_then(|value| self.tables.remove(&value))
    }

    pub fn remove_unregistered(&mut self) {
        self.sorter.remove_unregistered_depedencies()
    }

    pub fn remaining_tables(&self) -> Vec<String> {
        self.tables.clone().into_iter().map(|(k, _)| k).collect()
    }
}
