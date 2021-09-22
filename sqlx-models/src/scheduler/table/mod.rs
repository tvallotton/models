mod column;
pub mod constraint;
mod get_changes;

use crate::prelude::*;
pub use column::Column;

use std::convert::TryFrom;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Table {
    pub(crate) if_not_exists: bool,
    pub(crate) or_replace: bool,
    pub name: ObjectName,
    pub columns: Vec<Column>,
    pub constraints: Vec<TableConstraint>,
}

impl Table {
    fn create_col(&self, col: Column) -> Statement {
        Statement::AlterTable(AlterTable {
            name: self.name.clone(),
            operation: AlterTableOperation::AddColumn {
                column_def: col.into(),
            },
        })
    }

    fn rename_stmt(&self, name: &ObjectName) -> Statement {
        Statement::AlterTable(AlterTable {
            name: self.name.clone(),
            operation: AlterTableOperation::RenameTable {
                table_name: name.clone(),
            },
        })
    }
    #[throws(Error)]
    pub(super) fn alter_table(&mut self, op: AlterTableOperation) {
        use AlterTableOperation::*;
        match op {
            AddColumn { column_def } => self.columns.push(column_def.into()),
            AddConstraint(constr) => self.constraints.push(constr),
            DropConstraint { name, .. } => self.drop_constraint(name),

            DropColumn {
                column_name,
                if_exists,
                ..
            } => self.drop_col(column_name, if_exists),
            RenameColumn {
                old_column_name,
                new_column_name,
            } => self.rename_col(old_column_name, new_column_name),
            op => throw!(error!("Unsupported operation {}", op)),
        }
    }
    pub fn drop_constraint(&mut self, rm_name: Ident) {
        
        self.constraints = self
            .constraints
            .drain(..)
            .filter(|constr| constraint::name(constr).as_ref() != Some(&rm_name))
            .collect();
            
        
    }

    pub fn new(name: &str) -> Self {
        Table {
            name: ObjectName(vec![Ident::new(name)]),
            columns: vec![],
            constraints: vec![],
            if_not_exists: false,
            or_replace: false,
        }
    }

    pub(crate) fn dep_name(&self) -> String {
        self.name.to_string().to_lowercase()
    }

    pub(crate) fn dependencies(&self) -> Vec<String> {
        self.constraints
            .iter()
            .filter_map(|constr| match constr {
                TableConstraint::ForeignKey(ForeignKey { foreign_table, .. }) => {
                    Some(foreign_table.to_string().to_lowercase())
                }
                _ => None,
            })
            .collect()
    }

    pub(super) fn drop_col(&mut self, name: Ident, if_exists: bool) {
        let len = self.columns.len();
        self.columns = self
            .columns
            .drain(..)
            .filter(|col| col.name != name)
            .collect();
        assert!(
            len != self.columns.len() || if_exists,
            "Column \"{}\" does not exists",
            name
        );
    }

    pub(super) fn rename_col(&mut self, old: Ident, new: Ident) {
        self.columns = self
            .columns
            .iter()
            .map(Clone::clone)
            .map(|mut col| {
                if col.name == old {
                    col.name = new.clone();
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
            Statement::CreateTable(table) => Ok(Table {
                name: table.name,
                if_not_exists: false,
                or_replace: false,
                columns: table.columns.into_iter().map(Into::into).collect(),
                constraints: table.constraints,
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
        Statement::CreateTable(Box::new(ast::CreateTable {
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
            without_rowid: false,
            like: None,
        }))
    }
}
