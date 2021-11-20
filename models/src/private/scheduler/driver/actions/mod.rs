pub mod action;
mod compare;
mod crud;
mod inner;

use action::{depends, Action};
pub use compare::*;
use crud::*;
use inner::*;

use super::schema::Schema;
use crate::prelude::*;
#[derive(Debug)]
pub(crate) struct Actions<'table> {
    name: &'table ObjectName,
    actions: Vec<Action<'table>>,
}
impl<'table> Actions<'table> {
    pub fn new(schema: &'table Schema, target: &'table Table) -> Result<Self> {
        let table = schema.get_table(&target.name);

        let mut out = Self {
            name: &target.name,
            actions: vec![],
        };
        out.init(Inner { table, target })?;
        Ok(out)
    }

    fn init(&mut self, inner: Inner<'table>) -> Result<()> {
        if inner.table.is_none() {
            let action = Action::create_table(inner.target);
            self.actions.push(action);
            return Ok(());
        }
        let columns = inner.columns();
        let constraints = inner.constraints();

        if move_required(&columns, &constraints) {
            self.perform_move(&inner, columns, constraints)?;
        } else {
            let table_name = &inner.target.name;
            for col in columns.delete {
                let action = Action::drop_col(table_name, col);
                self.actions.push(action);
            }
            for cons in constraints.delete {
                let action = Action::drop_cons(table_name, cons)?;
                self.actions.push(action);
            }
            for cons in &constraints.update {
                let action = Action::drop_cons(table_name, cons)?;
                self.actions.push(action);
            }

            for col in columns.create {
                let action = Action::create_column(table_name, col);
                self.actions.push(action);
            }
            for cons in &constraints.create {
                let action = Action::create_cons(table_name, cons);
                self.actions.push(action);
            }

            for cons in &constraints.update {
                let action = Action::create_cons(table_name, cons);
                self.actions.push(action);
            }
        }
        Ok(())
    }

    fn perform_move(
        &mut self,
        inner: &Inner<'table>,
        cols: ColChange<'table>,
        cons: ConsChange<'table>,
    ) -> Result<()> {
        // constraints are dropped so they do not conflict
        if matches!(*DIALECT, PostgreSQL | MySQL) {
            for con in &inner.table.unwrap().constraints {
                let drop_cons = Action::drop_cons(&inner.table.unwrap().name, con)?;
                self.actions.push(drop_cons);
            }
        }
        let move_action = Action::move_to(inner.table.unwrap(), &cols, &cons);
        self.actions.push(move_action);
        let table_name = &inner.target.name;

        // moves do not create columns as their names may conflict with constraints.
        for &col in &cols.create {
            let action = Action::create_column(table_name, col);
            self.actions.push(action);
        }
        // created constraints that could not have been created in move.
        // Not all constraints may be created in a move
        // because they depended on columns that where not yet created.
        // SQLite does not enforce constraints so these are all created
        // in the move step
        for &cons in &cons.create {
            if depends(cons, &cols.create) && !matches!(*DIALECT, SQLite) {
                let action = Action::create_cons(table_name, cons);
                self.actions.push(action);
            }
        }
        Ok(())
    }

    pub fn as_migrations(self) -> Result<Vec<Migration>> {
        let mut migrations = vec![];
        let mut migr = Migration::new(self.name.clone());
        for action in self.actions {
            if action.is_fallible() && !migr.is_empty() {
                migrations.push(migr);
                migr = Migration::new(self.name.clone())
            }
            migr.push_up(action)?;
        }
        migrations.push(migr);

        Ok(migrations)
    }
}
pub(crate) fn move_required<'table>(cols: &ColChange<'table>, cons: &ConsChange<'table>) -> bool {
    let sqlite_conditions = DIALECT.requires_move()
        && !(cols.update.is_empty()
            && cols.delete.is_empty()
            && cons.delete.is_empty()
            && cons.create.is_empty()
            && cons.update.is_empty());
    sqlite_conditions || !cols.update.is_empty()
}
