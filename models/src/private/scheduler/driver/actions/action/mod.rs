use super::*;
mod temp_move;
use temp_move::Move;

pub struct Action<'table> {
    pub table_name: &'table ObjectName,
    pub variant: ActionVariant<'table>,
}
pub enum ActionVariant<'table> {
    CreateCol(&'table Column),

    DropCol(Ident),

    CreateConstr(&'table TableConstraint),

    DropConstr(Ident),

    TempMove(Move<'table>),

    Rename { new_name: ObjectName },

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
            variant: ActionVariant::DropConstr(Ident::new(cons.name()?)),
        })
    }

    pub(super) fn drop_col(name: &'table ObjectName, col: &'table Column) -> Self {
        Self {
            table_name: name,
            variant: ActionVariant::DropCol(col.name.clone()),
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
    pub(super) fn move_to(
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
            variant: ActionVariant::TempMove(Move {
                old_cols,
                new_cols,
                constraints,
            }),
        }
    }

    pub(super) fn to_statements(self) -> Result<Vec<Statement>> {
        use ActionVariant::*;
        let out = vec![];
        let table_name = self.table_name.clone();
        match self.variant {
            TempMove(r#move) => {
                return r#move.to_statements(table_name);
            }
            CreateTable(table) => {
                let statement = Statement::from(table.clone());
                out.push(statement);
            }
            other => {
                let operation = match other {
                    CreateCol(column) => AlterTableOperation::AddColumn {
                        column_def: ColumnDef::from(*column),
                    },

                    DropCol(column_name) => AlterTableOperation::DropColumn {
                        column_name,
                        if_exists: false,
                        cascade: DIALECT.supports_cascade(),
                    },
                    DropConstr(name) => AlterTableOperation::DropConstraint {
                        name,
                        cascade: DIALECT.supports_cascade(),
                        restrict: false,
                    },
                    CreateConstr(constr) => AlterTableOperation::DropConstraint {
                        name: Ident::new(constr.name().unwrap()),
                        cascade: DIALECT.supports_cascade(),
                        restrict: false,
                    },
                    Rename { new_name } => AlterTableOperation::RenameTable {
                        table_name: new_name,
                    },

                    _ => todo!(),
                };

                let statement = Statement::AlterTable(AlterTable {
                    name: table_name,
                    operation,
                });
                out.push(statement);
            }
        }
        Ok(out)
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
