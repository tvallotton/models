use super::*;
pub(super) struct Inner<'table> {
    pub table: Option<&'table Table>,
    pub target: &'table Table,
}

impl<'table> Inner<'table> {
    pub fn columns(&self) -> ColCRUD<'table> {
        let current = &self.table.unwrap().columns;
        let target = &self.target.columns;
        CRUD::new(current, target)
    }

    pub fn constraints(&self) -> ConsCRUD<'table> {
        let current = &self.table.unwrap().constraints;
        let target = &self.target.constraints;
        CRUD::new(current, target)
    }
}
