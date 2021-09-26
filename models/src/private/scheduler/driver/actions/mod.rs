use crate::prelude::*;
mod action;
mod crud;

mod compare;
use super::schema::Schema;
use action::{depends, Action};
pub use compare::*;
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

    fn init(&'input mut self) -> Result<()> {
        if self.table.is_none() {
            let action = Action::create_table(self.target);
            self.actions.push(action);
            return Ok(());
        }
        let columns = self.columns();
        let constraints = self.constraints();

        if move_required(&columns, &constraints) {
            self.perform_move(&columns, &constraints);
        } else {
            let table_name = &self.target.name;
            for col in columns.delete {
                let action = Action::drop_col(table_name, col);
                self.actions.push(action);
            }
            for cons in constraints.delete {
                let action = Action::drop_cons(table_name, cons)?;
                self.actions.push(action);
            }
            for cons in constraints.update {
                let action = Action::drop_cons(table_name, cons)?;
                self.actions.push(action);
            }

            for col in columns.create {
                let action = Action::create_column(table_name, col);
                self.actions.push(action);
            }
            for cons in constraints.create {
                let action = Action::create_cons(table_name, cons);
                self.actions.push(action);
            }

            for cons in constraints.update {
                let action = Action::create_cons(table_name, cons);
                self.actions.push(action);
            }
        }
        Ok(())
    }

    pub fn perform_move(&'input mut self, cols: &ColCRUD<'input>, cons: &ConsCRUD<'input>) {
        let move_action = Action::move_to(self.table.unwrap(), &cols, &mut cons);
        self.actions.push(move_action);
        let table_name = &self.target.name;

        // moves do not create columns as their names may conflict with constraints.
        for col in cols.create {
            let action = Action::create_column(table_name, col);
            self.actions.push(action);
        }
        // created constraints that could not have been created in move.
        // Not all constraints may be created in a move
        // because they depended on columns that where not yet created.
        // SQLite does not enforce constraints so these are all created
        // in the move step
        for cons in cons.create {
            if depends(cons, &cols.create) && !matches!(*DIALECT, SQLite) {
                let action = Action::create_cons(table_name, cons);
                self.actions.push(action);
            }
        }
    }

    pub fn as_migrations(self) -> Vec<Migration> {
        todo!()
    }


}
pub fn move_required<'table>(cols: &ColCRUD<'table>, cons: &ConsCRUD<'table>) -> bool {
    let sqlite_conditions = DIALECT.requires_move()
        && !(cols.update.is_empty()
            && cols.delete.is_empty()
            && cons.delete.is_empty()
            && cons.create.is_empty()
            && cons.update.is_empty());
    sqlite_conditions || !cols.update.is_empty()
}
