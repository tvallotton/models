use crate::prelude::*;
use TableConstraint::*;

pub fn name(constr: &TableConstraint) -> &Option<Ident> {
    match constr {
        Unique { name, .. } => name,
        ForeignKey { name, .. } => name,
        Check { name, .. } => name,
    }
}

pub fn primary(fields: &[&str]) -> TableConstraint {
    let name = None;
    let mut columns = vec![];
    for field in fields {
        columns.push(Ident::new(*field));
    }
    Unique {
        name,
        columns,
        is_primary: true,
    }
}

pub fn unique(fields: &[&str]) -> TableConstraint {
    let name = None;
    let mut columns = vec![];
    for field in fields {
        columns.push(Ident::new(*field));
    }
    Unique {
        name,
        columns,
        is_primary: false,
    }
}

pub fn foreign_key(local_col: &str, foreign_table: &str, foreign_col: &str) -> TableConstraint {
    ForeignKey {
        name: None,
        foreign_table: ObjectName(vec![Ident::new(foreign_table)]),
        referred_columns: vec![Ident::new(foreign_col)],
        columns: vec![Ident::new(local_col)],
    }
}
