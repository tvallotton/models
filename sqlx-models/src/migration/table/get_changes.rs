use super::*;

type Stmts = Vec<Statement>;

pub(crate) trait Name {
    fn name(&self) -> &Ident;
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

    fn get_vecs<T: Name + Eq + Clone>(now: &Vec<T>, target: &Vec<T>) -> (Vec<T>, Vec<T>, Vec<T>) {
        let mut to_change = vec![];
        let mut to_delete = vec![];
        let mut to_create = vec![];
        for c1 in target {
            for c0 in now {
                if c1.name() == c0.name() && c1 != c0 {
                    to_change.push(c1.clone())
                }
            }
            if !now.iter().any(|c0| c0.name() == c1.name()) {
                to_create.push(c1.clone());
            }
        }

        for c0 in now {
            if !target.iter().any(|c1| c1.name() == c0.name()) {
                to_delete.push(c0.clone());
            }
        }
        (to_delete, to_change, to_create)
    }

    fn get_changes_cols(&self, target: &Table, schema: &Schema) -> Vec<Statement> {
        let (to_delete, to_change, to_create) = Self::get_vecs(&self.columns, &target.columns);
        let mut stmts = vec![];
        if !to_delete.is_empty() && (!to_change.is_empty() || schema.dialect.requires_move()) {
            stmts.extend(self.move_to(to_delete, to_change));
        } else {
            for col in to_delete {
                let stmt = self.delete_col(col);
                stmts.push(stmt)
            }
        }

        for col in to_create {
            let stmt = self.create_col(col);
            stmts.push(stmt)
        }
        stmts
    }

    fn get_changes_cons(&self, target: &Table, schema: &Schema) -> Vec<Statement> {
        let (to_delete, to_change, to_create) =
            Self::get_vecs(&self.constraints, &target.constraints);
        let mut stmts = vec![];

        if !to_delete.is_empty() && (!to_change.is_empty() || schema.dialect.requires_move()) {
            stmts.extend(self.move_to(to_delete, to_change));
        } else {
            for col in to_delete {
                let stmt = self.delete_col(col);
                stmts.push(stmt)
            }
        }

        for col in to_create {
            let stmt = self.create_col(col);
            stmts.push(stmt)
        }
        stmts
    }

    fn delete_col(&self, col: Column) -> Statement {
        // if schema.dialect.requires_move() {
        //     return self.change_with_move(col, None, schema);
        // }
        Statement::AlterTable {
            name: self.name.clone(),
            operation: AlterTableOperation::DropColumn {
                column_name: col.name,
                if_exists: false,
                cascade: true,
            },
        }
    }

    fn move_to(&self, to_delete: Vec<Column>, to_change: Vec<Column>) -> Vec<Statement> {
        let mut out = vec![];

        let mut old_table = self.clone();
        let mut target = self.clone();
        target.name = ObjectName(vec![Ident::new("temp")]);

        for delete in to_delete {
            let i = target
                .columns
                .iter()
                .position(|col| *col == delete)
                .unwrap();
            target.columns.remove(i);
            old_table.columns.remove(i);
        }

        for change in to_change {
            let i = target
                .columns
                .iter()
                .position(|col| col.name == change.name)
                .unwrap();
            target.columns[i] = change;
        }
        // create table
        out.push(target.clone().into());
        // move self to temporary
        out.extend(old_table.move_stmt(&target));

        // move temporary back to self
        out.push(target.rename_stmt(&old_table.name));
        out
    }

    fn move_stmt(&self, target: &Table) -> Vec<Statement> {
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
        dbg!(&insert);
        out.push(insert);
        use Statement::*;
        out.push(Drop {
            object_type: ObjectType::Table,
            if_exists: false,
            names: vec![self.name.clone()],
            cascade: true,
            purge: false,
        });
        out
    }
    pub(crate) fn get_changes(&self, target: &Table, schema: &Schema) -> Vec<Statement> {
        let changes = self.get_changes_cols(target, schema);
        changes
    }
}
