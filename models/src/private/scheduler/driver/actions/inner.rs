use super::*;
pub(super) struct Inner<'table> {
    pub table: Option<&'table Table>,
    pub target: &'table Table,
}

impl<'table> Inner<'table> {
    pub fn columns(&self) -> ColChange<'table> {
        let current = &self.table.unwrap().columns;
        let target = &self.target.columns;

        Change::new(current, target)
    }

    pub fn constraints(&self) -> ConsChange<'table> {
        let current = &self.table.unwrap().constraints;
        let target = &self.target.constraints;
        Change::new(current, target)
    }
}
