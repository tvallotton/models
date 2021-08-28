use crate::prelude::*;

pub enum Constraint {
    Unique {
        name: Option<String>,
        columns: Vec<String>,
        is_primary: bool,
    },
    ForeignKey {
        name: Option<String>,
        foreign_table: String,
        foreign_columns: Vec<String>,
        local_columns: Vec<String>,
    },
}

impl Constraint {
    fn name(&self) -> &Option<String> {
        match self {
            Constraint::ForeignKey { name, .. } => name,
            Constraint::Unique { name, .. } => name,
        }
    }
}

impl From<TableConstraint> for Constraint {
    fn from(constr: TableConstraint) -> Constraint {
        todo!()
    }
}
