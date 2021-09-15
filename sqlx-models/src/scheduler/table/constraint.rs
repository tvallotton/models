use super::get_changes::Name;
use crate::prelude::*;
use TableConstraint::*;

impl Name for TableConstraint {
    fn name(&self) -> Result<&Ident, Error> {
        Ok(name(self)
            .as_ref()
            .ok_or_else(|| error!("Anonymous constraints are not supported."))?)
    }

    fn are_equal(&self, other: &Self) -> bool {
        match (self.name(), other.name()) {
            (Ok(n1), Ok(n2)) => n1 == n2,
            _ => false,
        }
    }
}

pub fn name(constr: &TableConstraint) -> &Option<Ident> {
    match constr {
        Unique(ast::Unique { name, .. }) => name,
        ForeignKey(ast::ForeignKey { name, .. }) => name,
        Check(ast::Check { name, .. }) => name,
    }
}

pub fn primary(name: &str, fields: &[&str]) -> TableConstraint {
    let name = Some(Ident::new(name));
    let mut columns = vec![];
    for field in fields {
        columns.push(Ident::new(*field));
    }
    Unique(ast::Unique {
        name,
        columns,
        is_primary: true,
    })
}

pub fn unique(name: &str, fields: &[&str]) -> TableConstraint {
    let name = Some(Ident::new(name));
    let mut columns = vec![];
    for field in fields {
        columns.push(Ident::new(*field));
    }
    Unique(ast::Unique {
        name,
        columns,
        is_primary: false,
    })
}

pub fn foreign_key(
    name: &str,
    local_col: &str,
    foreign_table: &str,
    foreign_col: &str,
    on_delete: &str,
    on_update: &str,
) -> TableConstraint {
    ForeignKey(ast::ForeignKey {
        name: Some(Ident::new(name)),
        foreign_table: ObjectName(vec![Ident::new(foreign_table)]),
        referred_columns: vec![Ident::new(foreign_col)],
        columns: vec![Ident::new(local_col)],
        on_delete: match &*on_delete.to_lowercase() {
            "cascade" => Some(ast::ReferentialAction::Cascade),
            "no action" => Some(ast::ReferentialAction::NoAction),
            "restrict" => Some(ast::ReferentialAction::Restrict),
            "set default" => Some(ast::ReferentialAction::SetDefault),
            "set null" => Some(ast::ReferentialAction::SetNull),
            _ => None,
        },
        on_update: match &*on_update.to_lowercase() {
            "cascade" => Some(ast::ReferentialAction::Cascade),
            "no action" => Some(ast::ReferentialAction::NoAction),
            "restrict" => Some(ast::ReferentialAction::Restrict),
            "set default" => Some(ast::ReferentialAction::SetDefault),
            "set null" => Some(ast::ReferentialAction::SetNull),
            _ => None,
        },
    })
}
