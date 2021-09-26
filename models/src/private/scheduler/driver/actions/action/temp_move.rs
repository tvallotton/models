use itertools::Itertools;

use super::Compare;
use crate::prelude::*;
pub struct Move<'table> {
    new_cols: Vec<&'table Column>,
    old_cols: Vec<&'table Column>,
    constraints: Vec<&'table TableConstraint>,
}

impl<'table> Move<'table> {
    pub fn to_statements(self, table_name: ObjectName) -> Result<Vec<Statement>> {
        let stmt = vec![];
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
    fn insert_statement(self, table_name: ObjectName) -> Result<Statement> {
        let new = self
            .new_cols
            .iter()
            .map(|&col| col.ident()) //
            .collect_vec();
        let old = self
            .old_cols
            .iter()
            .map(|&col| col.ident()) //
            .collect_vec();

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
    for (i, &c) in collection.iter().enumerate() {
        out += &c.to_string();
        if collection.len() != i + 1 {
            out += ","
        }
    }
    out
}
