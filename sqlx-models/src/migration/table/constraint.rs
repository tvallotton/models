use super::get_changes::Name;
use crate::prelude::*;
use TableConstraint::*;

impl Name for TableConstraint {
    fn name(&self) -> &Ident {
        self.name()
    }
}

pub fn name(constr: &TableConstraint) -> &Option<Ident> {
    match constr {
        Unique { name, .. } => name,
        ForeignKey { name, .. } => name,
        Check { name, .. } => name,
    }
}

pub fn primary(name: &str, fields: &[&str]) -> TableConstraint {
    let name = Some(Ident::new(name));
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

pub fn unique(name: &str, fields: &[&str]) -> TableConstraint {
    let name = Some(Ident::new(name));
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

pub fn foreign_key(
    name: &str,
    local_col: &str,
    foreign_table: &str,
    foreign_col: &str,
) -> TableConstraint {
    ForeignKey {
        name: Some(Ident::new(name)),
        foreign_table: ObjectName(vec![Ident::new(foreign_table)]),
        referred_columns: vec![Ident::new(foreign_col)],
        columns: vec![Ident::new(local_col)],
    }
}
