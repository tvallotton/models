use crate::prelude::*;
mod action;
use super::schema::Schema;
use action::Action;
mod constraint;

pub(crate) struct Actions<'input> {
    table: Option<&'input Table>,
    target: &'input Table,
    actions: Vec<Action>,
}

impl<'input> Actions<'input> {
    pub fn new(schema: &'input Schema, target: &'input Table) -> Self {
        let table = schema.get_table(&target.name);
        Self {
            table,
            target,
            actions: vec![],
        }
    }

    pub fn as_migrations(self) -> Vec<Migration> {
        todo!()
    }

    pub fn is_move_required(self) -> bool {

    }

    pub fn is_fallible() -> bool {
        
    }
}
