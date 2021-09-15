use super::*;

type Stmts = Vec<Statement>;
type Constraints = Vec<TableConstraint>;
type Change = (Vec<Column>, Vec<TableConstraint>);
type Columns = Vec<Column>;
pub(crate) trait Name: Eq + Clone + std::fmt::Debug {
    fn name(&self) -> Result<&Ident, Error>;
    fn are_equal(&self, other: &Self) -> bool;
}

impl Table {
    fn cols_to_string(&self) -> String {
        let mut out = String::new();
        for (i, col) in self.columns.iter().enumerate() {
            out += &ColumnDef::from(col.clone()).name.to_string();
            if self.columns.len() != i + 1 {
                out += ","
            }
        }
        out
    }

    fn get_vecs<T: Name>(now: &[T], target: &[T]) -> (Vec<T>, Vec<T>, Vec<T>) {
        let mut to_change = vec![];
        let mut to_delete = vec![];
        let mut to_create = vec![];
        for c1 in target {
            for c0 in now {
                if c1.are_equal(c0) && !c1.are_equal(c0) {
                    to_change.push(c1.clone())
                }
            }
            if !now.iter().any(|c0| c0.are_equal(c1)) {
                to_create.push(c1.clone());
            }
        }

        for c0 in now {
            if !target.iter().any(|c1| c0.are_equal(c1)) {
                to_delete.push(c0.clone());
            }
        }
        (to_delete, to_change, to_create)
    }

    fn create_cons(&self, cons: TableConstraint) -> Statement {
        Statement::AlterTable(AlterTable {
            name: self.name.clone(),
            operation: AlterTableOperation::AddConstraint(cons),
        })
    }
    #[throws(Error)]
    fn delete_cons(&self, cons: TableConstraint) -> Statement {
        Statement::AlterTable(AlterTable {
            name: self.name.clone(),
            operation: AlterTableOperation::DropConstraint {
                name: cons.name()?.clone(),
                cascade: true,
                restrict: false,
            },
        })
    }

    fn delete_col(&self, col: Column) -> Statement {
        // if schema.dialect.requires_move() {
        //     return self.change_with_move(col, None, schema);
        // }
        Statement::AlterTable(AlterTable {
            name: self.name.clone(),
            operation: AlterTableOperation::DropColumn {
                column_name: col.name,
                if_exists: false,
                cascade: true,
            },
        })
    }
    #[throws(Error)]
    fn move_to(&self, delete: Change, change: Columns, create: Constraints) -> Stmts {
        let mut out: Stmts = vec![];

        let mut old_table = self.clone();
        let mut new_table = self.clone();
        new_table.name = ObjectName(vec![Ident::new("temp")]);

        for del in delete.0 {
            let i = new_table
                .columns
                .iter()
                .position(|col| *col == del)
                .unwrap();

            new_table.columns.remove(i);
            old_table.columns.remove(i);
        }
        for del in delete.1 {
            new_table.constraints = new_table
                .constraints
                .into_iter()
                .filter(|cons| cons.are_equal(&del))
                .collect();
        }

        for ch in change {
            let i = new_table
                .columns
                .iter()
                .position(|col| col.name == ch.name)
                .unwrap();
            new_table.columns[i] = ch;
        }
        for cr in create {
            new_table.constraints.push(cr);
        }

        // create table
        out.push(new_table.clone().into());
        // move self to temporary
        out.extend(old_table.move_stmt(&new_table)?);

        // move temporary back to self
        out.push(new_table.rename_stmt(&old_table.name));
        out
    }
    #[throws(Error)]
    fn move_stmt(&self, target: &Table) -> Stmts {
        let mut out = vec![];
        let insert = format!(
            "INSERT INTO {} ({}) SELECT {} FROM {};",
            target.name,
            target.cols_to_string(),
            self.cols_to_string(),
            self.name
        );
        let insert = parse_sql(&dialect::GenericDialect {}, &insert)
            .unwrap()
            .into_iter() //
            .next()
            .unwrap();

        out.push(insert);

        out.push(Statement::Drop(Drop {
            object_type: ObjectType::Table,
            if_exists: false,
            names: vec![self.name.clone()],
            cascade: !DIALECT.clone()?.requires_move(),
            purge: false,
        }));
        out
    }

    // fn get_changes_cols(&self, target: &Table, schema: &Schema) -> Stmts {
    //     let (to_delete, to_change, to_create) = Self::get_vecs(&self.columns, &target.columns);
    //     let mut stmts = vec![];
    //     if !to_delete.is_empty() && (!to_change.is_empty() || schema.dialect.requires_move()) {
    //         stmts.extend(self.move_to(to_delete, to_change, schema));
    //     } else {
    //         for col in to_delete {
    //             let stmt = self.delete_col(col);
    //             stmts.push(stmt)
    //         }
    //     }

    //     for col in to_create {
    //         let stmt = self.create_col(col);
    //         stmts.push(stmt)
    //     }
    //     stmts
    // }
    pub fn constrs_changes(&self, target: &Table) -> (Constraints, Constraints) {
        let (to_delete, to_change, to_create) =
            Self::get_vecs(&self.constraints, &target.constraints);
        let to_delete = to_delete
            .into_iter()
            .chain(to_change.clone().into_iter())
            .collect();
        let to_create = to_create.into_iter().chain(to_change.into_iter()).collect();

        (to_delete, to_create)
    }
    pub fn col_changes(&self, target: &Table) -> (Columns, Columns, Columns) {
        Self::get_vecs(&self.columns, &target.columns)
    }
    #[throws(Error)]
    pub(crate) fn get_changes(&self, target: &Table) -> Stmts {
        let mut stmts = vec![];
        let (del_col, change, create_col) = self.col_changes(target);
        let (del_cons, create_cons) = self.constrs_changes(target);

        for col in create_col {
            let stmt = self.create_col(col);
            stmts.push(stmt)
        }

        if ((!del_col.is_empty() || !create_cons.is_empty() || !del_cons.is_empty())
            && DIALECT.clone()?.requires_move())
            || !change.is_empty()
        {
            let s = self.move_to((del_col, del_cons), change, create_cons)?;
            stmts.extend(s);
        } else {
            for cons in del_cons {
                let stmt = self.delete_cons(cons)?;
                stmts.push(stmt);
            }

            for col in del_col {
                let stmt = self.delete_col(col);
                stmts.push(stmt);
            }
            for cons in create_cons {
                let stmt = self.create_cons(cons);
                stmts.push(stmt);
            }
        }

        stmts
    }
}

#[test]
fn func() {}
