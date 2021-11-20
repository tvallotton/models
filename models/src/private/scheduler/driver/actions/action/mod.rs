use super::*;
mod temp_move;
use temp_move::Move;
#[derive(Debug)]
pub(crate) struct Action<'table> {
    pub table_name: &'table ObjectName,
    pub variant: ActionVariant<'table>,
}
#[derive(Debug)]
pub(crate) enum ActionVariant<'table> {
    CreateCol(&'table Column),

    DropCol(Ident),

    CreateConstr(&'table TableConstraint),

    DropConstr(Ident),

    TempMove(Move<'table>),

    CreateTable(&'table Table),
}

impl<'table> Action<'table> {
    pub fn is_fallible(&self) -> bool {
        if let ActionVariant::CreateCol(col) = &self.variant {
            col.has_default() || col.is_nullable()
        } else {
            false
        }
    }

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

    pub fn move_to(old: &'table Table, cols: &ColChange<'table>, cons: &ConsChange<'table>) -> Self {
        let move_ = Move::new(old, cons, cols);
        Self {
            table_name: &old.name,
            variant: ActionVariant::TempMove(move_),
        }
    }

    pub fn to_statements(self) -> Result<Vec<Statement>> {
        use ActionVariant::*;
        let mut out = vec![];
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
                        column_def: ColumnDef::from(column.clone()),
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
        TableConstraint::ForeignKey(fk) => &fk.columns,
        TableConstraint::Unique(unique) => &unique.columns,
        _ => return false,
    };
    let names = names.iter().map(ToString::to_string);

    for col in names {
        for table_name in tables.iter().map(|t| t.name().unwrap()) {
            if col.to_string() == table_name {
                return true;
            }
        }
    }
    false
}
