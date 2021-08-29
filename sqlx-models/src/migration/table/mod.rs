use super::Schema;
use crate::prelude::*;
use std::convert::TryFrom;
mod column;

mod constraint;
pub use column::Column;
pub use constraint::Constraint;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Table {
    pub name: ObjectName,
    pub(crate) if_not_exists: bool,
    pub(crate) or_replace: bool,
    pub columns: Vec<Column>,
    pub constraints: Vec<TableConstraint>,
}

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

    pub(super) fn get_changes(&self, target: &Table, schema: &Schema) -> Vec<Statement> {
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
        for (from, to) in to_change {
            let stmt = self.change_col(from, to, schema);
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
    fn create_col(&self, col: Column) -> Statement {
        Statement::AlterTable {
            name: self.name.clone(),
            operation: AlterTableOperation::AddColumn {
                column_def: col.clone().into(),
            },
        }
    }

    fn change_col(&self, from: Column, to: Column, schema: &Schema) -> Vec<Statement> {
        let mut out = vec![];
        // if schema.dialect.requires_move() {
        let mut target = self.clone();
        target.name = ObjectName(vec![Ident::new("temprary")]);
        let i = target.columns.iter().position(|col| *col == from).unwrap();
        target.columns[i] = to;
        // move self to temporary
        out.extend(self.move_to_stmt(&target, schema));

        // move temporary back to self
        out.push(target.rename_stmt(&self.name));
        out
    }

    fn rename_stmt(&self, name: &ObjectName) -> Statement {
        Statement::AlterTable {
            name: self.name.clone(),
            operation: AlterTableOperation::RenameTable {
                table_name: name.clone(),
            },
        }
    }
    fn delete_col(&self, col: Column, schema: &Schema) -> Vec<Statement> {
        todo!()
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


    pub(super) fn alter_table(&mut self, op: AlterTableOperation) {
        use AlterTableOperation::*;
        match op {
            AddColumn { column_def } => self.columns.push(column_def.into()),
            AddConstraint(constr) => self.constraints.push(constr),
            DropColumn {
                column_name,
                if_exists,
                cascade,
            } => self.drop_col(column_name, if_exists, cascade),
            RenameColumn {
                old_column_name,
                new_column_name,
            } => self.rename_col(old_column_name, new_column_name),
            _ => panic!(""),
        }
    }

    pub(super) fn new(name: String) -> Self {
        Table {
            name: ObjectName(vec![Ident::new(name)]),
            columns: vec![],
            constraints: vec![],
            or_replace: false,
            if_not_exists: false,
        }
    }

    pub(super) fn drop_col(&mut self, name: Ident, if_exists: bool, cascade: bool) {
        let len = self.columns.len();
        self.columns = self
            .columns
            .drain(..)
            .filter(|col| col.name != name.value)
            .collect();
        assert!(
            len != self.columns.len() || if_exists,
            "Column \"{}\" does not exists",
            name
        );
        assert!(
            !cascade,
            "Cascade while dropping columns is not supported yet."
        );
    }

    pub(super) fn rename_col(&mut self, old: Ident, new: Ident) {
        self.columns = self
            .columns
            .iter()
            .map(Clone::clone)
            .map(|mut col| {
                if col.name == old.value {
                    col.name = new.value.clone();
                }
                col
            })
            .collect()
    }
}

impl TryFrom<Statement> for Table {
    type Error = String;
    fn try_from(value: Statement) -> Result<Self, Self::Error> {
        match value {
            Statement::CreateTable {
                or_replace,
                if_not_exists,
                name,
                columns,
                constraints,
                ..
            } => Ok(Table {
                name: name,
                if_not_exists,
                or_replace,
                columns: columns.into_iter().map(Into::into).collect(),
                constraints: constraints,
            }),
            value => Err(format!(
                "Expected a \"CREATE TABLE\" statement, found {}",
                value
            )),
        }
    }
}
impl From<Table> for Statement {
    fn from(table: Table) -> Self {
        Statement::CreateTable {
            or_replace: false,
            temporary: false,
            external: false,
            if_not_exists: false,
            name: table.name,
            columns: table.columns.into_iter().map(Into::into).collect(),
            constraints: table.constraints,
            hive_distribution: HiveDistributionStyle::NONE,
            hive_formats: None,
            table_properties: vec![],
            with_options: vec![],
            file_format: None,
            location: None,
            query: None,
            without_rowid: true,
            like: None,
        }
    }
}
