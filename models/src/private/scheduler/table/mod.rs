use crate::prelude::*;
mod column;
mod constraint;
pub use column::*;
pub use constraint::*;

pub struct Table {
    pub(crate) name: ObjectName,
    if_not_exists: bool,
    or_replace: bool,
    pub(crate) columns: Vec<Column>,
    pub(crate) constraints: Vec<TableConstraint>,
}

impl Table {
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
}
