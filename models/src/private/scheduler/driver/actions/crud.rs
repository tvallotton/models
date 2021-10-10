use super::Compare;
use crate::prelude::*;
#[derive(Debug)]
pub(crate) struct CRUD<'table, T> {
    pub create: Vec<&'table T>,
    pub delete: Vec<&'table T>,
    pub update: Vec<&'table T>,
    // pub keep: Vec<&'table T>,
}

pub(crate) type ColCRUD<'table> = CRUD<'table, Column>;
pub(crate) type ConsCRUD<'table> = CRUD<'table, TableConstraint>;

impl<'table, T: Compare> CRUD<'table, T> {
    pub fn to_delete(&self, obj: &T) -> bool {
        self.delete.iter().any(|&del| del.names_are_equal(&obj))
    }
    pub fn to_update(&self, obj: &T) -> bool {
        self.update.iter().any(|&up| up.names_are_equal(&obj))
    }
    pub fn _to_create(&self, obj: &T) -> bool {
        self.create.iter().any(|&cr| cr.names_are_equal(&obj))
    }
}

impl<'table, T: Compare + PartialEq> CRUD<'table, T> {
    pub fn new(current: &'table [T], target: &'table [T]) -> Self {
        let mut update = vec![];
        let mut delete = vec![];
        let mut create = vec![];

        for c1 in target {
            for c0 in current {
                if c1.are_modified(c0) {
                    update.push(c1);
                }
            }

            if !current
                .iter()
                .any(|c0| c0.are_equal(c1) || c0.are_modified(c1))
            {
                create.push(c1);
            }
        }

        for c0 in current {
            if target
                .iter()
                .all(|t| !c0.are_equal(t) && !c0.are_modified(t))
            {
                delete.push(c0.clone());
            }
        }

        CRUD {
            create,
            update,
            delete,
        }
    }
}
