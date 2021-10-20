use crate::prelude::*;
mod column;
pub mod constraint;
pub use column::*;

use crate::private::scheduler::driver::actions::Compare;

#[derive(Clone, Debug)]
pub struct Table {
    pub(crate) name: ObjectName,
    pub if_not_exists: bool,
    pub or_replace: bool,
    pub columns: Vec<Column>,
    pub constraints: Vec<TableConstraint>,
}

impl Table {
    pub fn new(name: &str) -> Self {
        Table {
            name: ObjectName(vec![Ident::new(name)]),
            columns: vec![],
            constraints: vec![],
            if_not_exists: false,
            or_replace: false,
        }
    }

    pub(crate) fn name(&self) -> String {
        self.name.to_string().to_lowercase()
    }

    /// returns depenedencies of the table
    pub(crate) fn deps(&self) -> Vec<String> {
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

    pub(super) fn alter_table(&mut self, op: &AlterTableOperation) -> Result {
        use AlterTableOperation::*;
        match op {
            AddColumn { column_def } => self.columns.push(column_def.clone().into()),
            AddConstraint(constr) => self.constraints.push(constr.clone()),
            DropConstraint { name, .. } => self.drop_constraint(name.to_string()),

            DropColumn {
                column_name,
                if_exists,
                ..
            } => self.drop_col(column_name, *if_exists),
            RenameColumn {
                old_column_name,
                new_column_name,
            } => self.rename_col(old_column_name, new_column_name),
            op => return Err(error!("unsupported operation: \"{}\"", op)),
        }
        Ok(())
    }

    pub(super) fn drop_col(&mut self, name: &Ident, if_exists: bool) {
        let len = self.columns.len();
        self.columns = self
            .columns
            .drain(..)
            .filter(|col| &col.name != name)
            .collect();
        assert!(
            len != self.columns.len() || if_exists,
            "Column \"{}\" does not exists",
            name
        );
    }

    pub fn drop_constraint(&mut self, rm_name: String) {
        self.constraints = self
            .constraints
            .drain(..)
            .filter(|constr| constr.name().ok().as_ref() != Some(&rm_name))
            .collect();
    }

    pub(super) fn rename_col(&mut self, old: &Ident, new: &Ident) {
        self.columns = self
            .columns
            .iter()
            .map(Clone::clone)
            .map(|mut col| {
                if &col.name == old {
                    col.name = new.clone();
                }
                col
            })
            .collect()
    }
}

impl TryFrom<Statement> for Table {
    type Error = Error;

    fn try_from(value: Statement) -> Result<Self, Self::Error> {
        if let Statement::CreateTable(table) = value {
            let name = table
                .name
                .0
                .into_iter()
                .map(|ident| ident.value.to_lowercase())
                .map(Ident::new)
                .collect();
            Ok(Table {
                name: ObjectName(name),
                if_not_exists: false,
                or_replace: false,
                columns: table.columns.into_iter().map(Into::into).collect(),
                constraints: table.constraints,
            })
        } else {
            Err(error!(
                "Expected a \"CREATE TABLE\" statement, found {}",
                value
            ))
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
