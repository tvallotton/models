use super::*;
pub(super) struct Inner<'table> {
    pub table: Option<&'table Table>,
    pub target: &'table Table,
}

impl<'table> Inner<'table> {
    pub fn columns(&self) -> ColCRUD<'table> {
        let current = &self.table.unwrap().columns;
        let target = &self.target.columns;
        Self::get_crud(current, target)
    }

    pub fn constraints(&self) -> ConsCRUD<'table> {
        let current = &self.table.unwrap().constraints;
        let target = &self.target.constraints;
        Self::get_crud(current, target)
    }
    fn get_crud<T: Compare + PartialEq>(
        current: &'table [T],
        target: &'table [T],
    ) -> CRUD<'table, T> {
        let mut update = vec![];
        let mut delete = vec![];
        let mut create = vec![];
        let mut keep = vec![];
        for c1 in target {
            for c0 in current {
                if c1.are_modified(c0) {
                    update.push(c1)
                }
            }

            if !current.iter().any(|c0| c0.are_equal(c1)) {
                create.push(c1);
            }
        }

        for c0 in current {
            if target
                .iter()
                .all(|t| !c0.are_equal(t) && !c0.are_modified(t))
            {
                delete.push(c0.clone());
            }
        }
        for ref c0 in current {
            let to_keep = create.iter().any(|c1| c0.names_are_equal(c1))
                || delete.iter().any(|c1| c0.names_are_equal(c1))
                || update.iter().any(|c1| c0.names_are_equal(c1));
            if to_keep {
                keep.push(*c0);
            }
        }

        CRUD {
            create,
            update,
            delete,
            keep,
        }
    }
}
