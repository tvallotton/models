use super::constraint::ActionConstraint;
use super::*;
use crate::prelude::*;
pub struct Action<'table> {
    pub table_name: &'table ObjectName,
    pub variant: ActionVariant<'table>,
}
pub enum ActionVariant<'table> {
    CreateCol(&'table Column),

    DropCol {
        name: &'table Ident,
    },

    CreateConstr(&'table TableConstraint),

    DropConstr {
        name: Ident,
    },

    TempMove {
        old_cols: Vec<&'table Column>,
        new_cols: Vec<&'table Column>,

        constraints: Vec<&'table TableConstraint>,
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
            variant: ActionVariant::CreateTable(target),
        }
    }
    pub(super) fn drop_cons(
        name: &'table ObjectName,
        cons: &'table TableConstraint,
    ) -> Result<Self> {
        Ok(Self {
            table_name: name,
            variant: ActionVariant::DropConstr {
                name: Ident::new(cons.name()?),
            },
        })
    }

    pub(super) fn drop_col(name: &'table ObjectName, col: &'table Column) -> Self {
        Self {
            table_name: name,
            variant: ActionVariant::DropCol { name: &col.name },
        }
    }
    pub(super) fn create_column(table_name: &'table ObjectName, col: &'table Column) -> Self {
        Self {
            table_name,
            variant: ActionVariant::CreateCol(col),
        }
    }
    pub(super) fn create_cons(name: &'table ObjectName, cons: &'table TableConstraint) -> Self {
        Self {
            table_name: name,
            variant: ActionVariant::CreateConstr(cons),
        }
    }
    pub fn move_to(
        old: &'table Table,
        cols: &ColCRUD<'table>,
        cons: &mut ConsCRUD<'table>,
    ) -> Self {
        let mut new_cols = vec![];
        let mut old_cols = vec![];
        let mut constraints = vec![];

        for col in &old.columns {
            if cols.to_delete(col) {
                continue;
            } else {
                new_cols.push(col);
                old_cols.push(col);
            }
        }

        for cons in cons.create {
            if !depends(cons, &cols.create) || matches!(*DIALECT, SQLite) {
                constraints.push(cons);
            }
        }
        for cons in cons.update {
            if !depends(cons, &cols.create) || matches!(*DIALECT, SQLite) {
                constraints.push(cons);
            }
        }

        Self {
            table_name: &old.name,
            variant: ActionVariant::TempMove {
                old_cols,
                new_cols,
                constraints,
            },
        }
    }
}

pub fn depends(cons: &TableConstraint, tables: &[&Column]) -> bool {
    let names = match cons {
        TableConstraint::ForeignKey(fk) => fk.columns,
        TableConstraint::Unique(unique) => unique.columns,
        _ => return false,
    }
    .iter()
    .map(ToString::to_string);

    for col in names {
        for table_name in tables.iter().map(|t| t.name().unwrap()) {
            if col == table_name {
                return true;
            }
        }
    }
    false
}
