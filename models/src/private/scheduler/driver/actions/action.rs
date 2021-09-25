use super::constraint::ActionConstraint;
use super::*;
use crate::prelude::*;
pub struct Action<'table> {
    pub table_name: &'table ObjectName,
    pub data: ActionData<'table>,
}
pub enum ActionData<'table> {
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
        old_cols: Vec<&'table ObjectName>,
        new_cols: Vec<&'table ObjectName>,
        old_cons: Vec<&'table TableConstraint>,
        new_cons: Vec<&'table TableConstraint>,
    },

    Rename {
        new_name: ObjectName,
    },

    CreateTable(&'table Table),
}

impl<'table> Action<'table> {
    fn into_statements(self) {}
    pub(super) fn create_table(target: &'table Table) -> Self {
        Self {
            table_name: &target.name,
            data: ActionData::CreateTable(target),
        }
    }

    pub(super) fn move_to(
        old: &'table Table,
        cols: &mut ColCRUD<'table>,
        cons: &mut ConsCRUD<'table>,
    ) {
        let mut new_cols = vec![];
        let old_cols = vec![];
        let constraints = vec![];

        for col in &old.columns {
            if cols.to_delete(col) {
                continue;
            }
        }

        // Self {
        //     table_name: &old.name,
        //     data: ActionData::TempMove {
        //         old_cols:
        //     }
        // }
    }
    fn is_fallible() {}
}
