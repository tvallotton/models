use super::Compare;
use crate::prelude::*;
pub struct CRUD<'table, T> {
    pub create: Vec<&'table T>,
    pub delete: Vec<&'table T>,
    pub update: Vec<&'table T>,
}

pub type ColCRUD<'table> = CRUD<'table, Column>;
pub type ConsCRUD<'table> = CRUD<'table, TableConstraint>;

impl<'table, T: Compare> CRUD<'table, T> {
    pub fn to_delete(&self, obj: &T) -> bool {
        self.delete.iter().any(|&del| del.names_are_equal(&obj))
    }
    pub fn to_update(&self, obj: &T) -> bool {
        self.update.iter().any(|&up| up.names_are_equal(&obj))
    }
    pub fn to_create(&self, obj: &T) -> bool {
        self.create.iter().any(|&cr| cr.names_are_equal(&obj))
    }
}
