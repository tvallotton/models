use crate::prelude::*;
mod action;
mod crud;

mod compare;
use super::schema::Schema;
use action::Action;
mod constraint;
use compare::*;
use crud::*;



pub(crate) struct Actions<'input> {
    table: Option<&'input Table>,
    target: &'input Table,
    actions: Vec<Action<'input>>,
}

impl<'input> Actions<'input> {
    pub fn new(schema: &'input Schema, target: &'input Table) -> Self {
        let table = schema.get_table(&target.name);
        let out = Self {
            table,
            target,
            actions: vec![],
        };
        out.init();
        out
    }
    fn get_crud<T: Compare>(current: &'input [T], target: &'input [T]) -> CRUD<'input, T> {
        let mut update = vec![];
        let mut delete = vec![];
        let mut create = vec![];
        for c1 in target {
            for c0 in current {
                if c1.are_modified(c0) {
                    update.push(c1.clone())
                }
            }

            if !current.iter().any(|c0| c0.are_equal(c1)) {
                create.push(c1.clone());
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

    fn columns(&'input self) -> ColCRUD<'input> {
        let current = &self.table.unwrap().columns;
        let target = &self.target.columns;
        Self::get_crud(current, target)
    }

    fn constraints(&'input self) -> ConsCRUD<'input> {
        let current = &self.table.unwrap().constraints;
        let target = &self.target.constraints;
        Self::get_crud(current, target)
    }

    fn init(&self) {
        if self.table.is_none() {
            let action = Action::create_table(self.target);
            self.actions.push(action);
            return;
        }
        let columns = self.columns();
        let constraints = self.constraints();

        if self.move_required(&columns, &constraints) {
        } else {
        }
    }

    pub fn as_migrations(self) -> Vec<Migration> {
        todo!()
    }

    pub fn move_required(self, cols: &ColCRUD<'input>, cons: &ConsCRUD<'input>) -> bool {
        let sqlite_conditions = DIALECT.requires_move()
            && !(cols.update.is_empty()
                && cols.delete.is_empty()
                && cons.delete.is_empty()
                && cons.create.is_empty()
                && cons.update.is_empty());
        sqlite_conditions || !cols.update.is_empty()
    }

    pub fn is_fallible() -> bool {
        todo!()
    }
}
