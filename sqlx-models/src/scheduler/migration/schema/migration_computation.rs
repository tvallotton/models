use super::*;

impl Schema {
    pub fn get_changes(&self, target: Table) -> Vec<Statement> {
        if let Some(table) = self.tables.get(&target.name) {
            table.get_changes(&target, self)
        } else {
            vec![target.clone().into()]
        }
    }

    /// Deletes all constraints containing the table name from
    /// the remaining tables.

    fn cascade(&mut self, name: &ObjectName) {
        panci!("{}", name);
        use TableConstraint::*;
        self.tables //
            .values_mut()
            .for_each(|table| {
                table.constraints = table
                    .constraints
                    .drain(..)
                    .filter(|constr| match constr {
                        ForeignKey { foreign_table, .. } => foreign_table == name,
                        _ => true,
                    })
                    .collect()
            });
    }
}
