use super::constraint::ActionConstraint;
use crate::prelude::*;

pub struct Action {
    table_name: ObjectName,
    data: ActionData
}
enum ActionData {
    CreateCol {
        column_name: Ident,
        dtype: DataType,
        nullable: bool,
        
    },

    DeleteCol {
        column_name: Ident,
        dtype: DataType,
        nullable: bool,
    },

    CreateConstr {
        constr: ActionConstraint,
    },
    DeleteConstr {
        name: Ident,
    },

    TempMove {
        old_cols: Vec<ObjectName>,
        new_cols: Vec<ObjectName>,
    },

    Rename {
        new_name: ObjectName,
    },

    CreateTable(Table),
}

impl Action {
    fn into_migration(self) {
        self.sort();
    }

    fn is_fallible() {

    }
}
