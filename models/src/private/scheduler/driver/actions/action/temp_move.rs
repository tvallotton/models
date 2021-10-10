use super::{Compare, *};
use crate::prelude::*;
#[derive(Debug)]
pub(crate) struct Move<'table> {
    pub(super) new_cols: Vec<&'table Column>,
    pub(super) old_cols: Vec<&'table Column>,
    pub(super) constraints: Vec<&'table TableConstraint>,
}

impl<'table> Move<'table> {
    pub fn new(old: &'table Table, cons: &ConsCRUD<'table>, cols: &ColCRUD<'table>) -> Self {
        let mut new_cols = vec![];
        let mut old_cols = vec![];
        let mut constraints = vec![];
        for col in &old.columns {
            if !cols.to_delete(col) && !cols.to_update(col) {
                new_cols.push(col);
                old_cols.push(col);
            }
        }
        for &col in &cols.update {
            new_cols.push(col);
            old_cols.push(col);
        }
        for con in &old.constraints {
            let to_delete = cons.to_delete(con);
            let to_update = cons.to_update(con);
            if !to_delete && !to_update {
                constraints.push(con);
            }
        }
        for con in &cons.update {
            if !depends(con, &cols.create) || matches!(*DIALECT, SQLite) {
                constraints.push(con);
            }
        }
        for con in &cons.create {
            if !depends(con, &cols.create) || matches!(*DIALECT, SQLite) {
                constraints.push(con);
            }
        }
        Self {
            new_cols,
            old_cols,
            constraints,
        }
    }

    pub fn to_statements(self, table_name: ObjectName) -> Result<Vec<Statement>> {
        let mut stmt = vec![];
        let create_table = self.create_table();
        let insert = self.insert_statement(table_name.clone())?;
        let drop = self.drop_statement(table_name.clone());
        let rename = self.rename(table_name);
        stmt.push(create_table);
        stmt.push(insert);
        stmt.push(drop);
        stmt.push(rename);
        Ok(stmt)
    }

    fn create_table(&self) -> Statement {
        Table {
            name: ObjectName(vec![Ident::new("temp")]),
            columns: self.new_cols.iter().map(|&c| c.clone()).collect(),
            constraints: self.constraints.iter().map(|&c| c.clone()).collect(),
            if_not_exists: false,
            or_replace: false,
        }
        .into()
    }
    fn insert_statement(&self, table_name: ObjectName) -> Result<Statement> {
        let new = self
            .new_cols
            .iter()
            .map(|&col| col.ident()) //
            .collect();
        let old = self
            .old_cols
            .iter()
            .map(|&col| col.ident()) //
            .collect();

        let insert = format!(
            "INSERT INTO temp ({}) SELECT {} FROM {};",
            to_string(new),
            to_string(old),
            table_name
        );
        let insert = parse_sql(&insert)?
            .into_iter() //
            .next()
            .unwrap();

        Ok(insert)
    }

    fn drop_statement(&self, table_name: ObjectName) -> Statement {
        Statement::Drop(Drop {
            object_type: ObjectType::Table,
            if_exists: false,
            names: vec![table_name],
            cascade: !DIALECT.requires_move(),
            purge: false,
        })
    }

    fn rename(self, table_name: ObjectName) -> Statement {
        Statement::AlterTable(AlterTable {
            name: ObjectName(vec![Ident::new("temp")]),
            operation: AlterTableOperation::RenameTable {
                table_name: table_name,
            },
        })
    }
}

fn to_string<T: ToString>(collection: Vec<T>) -> String {
    let mut out = String::new();
    for (i, c) in collection.iter().enumerate() {
        out += &c.to_string();
        if collection.len() != i + 1 {
            out += ","
        }
    }
    out
}

pub fn depends(cons: &TableConstraint, tables: &[&Column]) -> bool {
    let names = match cons {
        TableConstraint::ForeignKey(fk) => &fk.columns,
        TableConstraint::Unique(unique) => &unique.columns,
        _ => return false,
    };
    let names = names.iter().map(ToString::to_string);

    for col in names {
        for table_name in tables.iter().map(|t| t.name().unwrap()) {
            if col.to_string() == table_name {
                return true;
            }
        }
    }
    false
}
