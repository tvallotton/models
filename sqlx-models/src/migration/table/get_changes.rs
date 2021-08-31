use super::*;

impl Table {
    fn cols_to_string(&self) -> String {
        let mut out = String::new();
        for (i, col) in self.columns.iter().enumerate() {
            out += &ColumnDef::from(col.clone()).to_string();
            if self.columns.len() != i + 1 {
                out += ","
            }
        }
        out
    }
    fn get_changes_cols(&self, target: &Table, schema: &Schema) -> Vec<Statement> {
        let mut to_change = vec![];
        let mut to_delete = vec![];
        let mut to_create = vec![];
        for c1 in &target.columns {
            for c0 in &self.columns {
                if c1.name == c0.name && c1 != c0 {
                    to_change.push((c0.clone(), c1.clone()))
                }
            }
            if !self.columns.iter().any(|c0| c0.name == c1.name) {
                to_create.push(c1.clone());
            }
        }

        for c0 in &self.columns {
            if !target.columns.iter().any(|c1| c1.name == c0.name) {
                to_delete.push(c0.clone());
            }
        }
        let mut stmts = vec![];
        dbg!(&to_change); 
        for (from, to) in to_change {
            let stmt = self.change_with_move(from, Some(to), schema);
            stmts.extend(stmt)
        }

        for col in to_create {
            let stmt = self.create_col(col);
            stmts.push(stmt)
        }
        for col in to_delete {
            let stmt = self.delete_col(col, schema);
            stmts.extend(stmt)
        }
        stmts
    }

    fn delete_col(&self, col: Column, schema: &Schema) -> Vec<Statement> {
        if schema.dialect.requires_move() {
            return self.change_with_move(col, None, schema);
        }
        vec![Statement::AlterTable {
            name: self.name.clone(),
            operation: AlterTableOperation::DropColumn {
                column_name: col.name,
                if_exists: false,
                cascade: true,
            },
        }]
    }

    fn change_with_move(
        &self,
        from: Column,
        to: Option<Column>,
        schema: &Schema,
    ) -> Vec<Statement> {
        let mut out = vec![];
        // if schema.dialect.requires_move() {
        let mut target = self.clone();
        target.name = ObjectName(vec![Ident::new("temprary")]);
        let i = target.columns.iter().position(|col| *col == from).unwrap();
        if let Some(to) = to {
            target.columns[i] = to;
        } else {
            target.columns.remove(i);
        }
        // move self to temporary
        out.extend(self.move_to_stmt(&target, schema));

        // move temporary back to self
        out.push(target.rename_stmt(&self.name));
        
        out
    }
    pub(crate) fn get_changes(&self, target: &Table, schema: &Schema) -> Vec<Statement> {
        let changes = self.get_changes_cols(target, schema);
        changes
    }

    fn move_to_stmt(&self, target: &Table, schema: &Schema) -> Vec<Statement> {
        let mut out: Vec<Statement> = vec![];
        //  create table
        out.push(target.clone().into());
        //  move values

        out.push(
            parse_sql(
                &schema.dialect,
                &format!(
                    "INSERT INTO {} ({})
                VALUES (
                    SELECT {}
                    FROM {}
                );",
                    target.name,
                    target.cols_to_string(),
                    self.cols_to_string(),
                    self.name
                ),
            )
            .unwrap()
            .into_iter()
            .next()
            .unwrap(),
        );
        use Statement::*;

        // drop old table
        out.push(Drop {
            object_type: ObjectType::Table,
            if_exists: false,
            names: vec![self.name.clone()],
            cascade: true,
            purge: false,
        });
        out
    }
}
