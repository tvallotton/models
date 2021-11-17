use std::{
    cmp::{Eq, PartialEq},
    hash::{Hash, Hasher},
};

#[derive(Hash, PartialEq, Eq)]
pub struct Key {
    table_name: StaticPtr<str>,
    columns: StaticPtr<[&'static str]>,
}

impl Key {
    pub fn new(table: &'static str, columns: &'static [&'static str]) -> Self {
        Key {
            table_name: StaticPtr(table),
            columns: StaticPtr(columns),
        }
    }

    pub fn table_name(&self) -> &'static str {
        self.table_name.0
    }

    pub fn columns(&self) -> &'static [&'static str] {
        self.columns.0
    }
}

pub struct StaticPtr<T: 'static + ?Sized>(pub &'static T);

impl<T: ?Sized> Eq for StaticPtr<T> {}

impl<T: ?Sized> Hash for StaticPtr<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::ptr::hash(self, state);
    }
}
impl<T: ?Sized> PartialEq for StaticPtr<T> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}
