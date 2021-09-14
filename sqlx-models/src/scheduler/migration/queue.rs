use super::*;
pub use std::collections::HashSet;
pub use topological_sort::TopologicalSort;

pub(crate) struct Queue {
    pub total: HashSet<String>,
    pub sort: TopologicalSort<String>,
}

impl Queue {
    pub fn new() -> Self {
        Self {
            total: HashSet::new(),
            sort: TopologicalSort::new(),
        }
    }
    pub fn len(&self) -> usize {
        self.total.len()
    }
    pub fn insert(&mut self, first: String, second: String) {
        self.total.insert(second.clone());
        self.sort.add_dependency(first, second);
    }
    pub fn pop(&mut self) -> Option<String> {
        self.sort.pop().map(|value| {
            assert!(self.total.remove(&value));
            value
        })
    }
}
