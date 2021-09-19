use std::collections::HashSet;

use super::get_changes::Compare;
use crate::prelude::*;
use TableConstraint::*;

impl Compare for TableConstraint {
    fn name(&self) -> Result<String, Error> {
        name(self)
            .as_ref()
            .ok_or_else(|| error!("Anonymous constraints are not supported."))
            .map(|name| name.to_string().to_lowercase())
    }

    fn bodies_are_equal(&self, other: &Self) -> bool {
        use TableConstraint::*;
        match (self, other) {
            (Unique(u0), Unique(u1)) => {
                u0.is_primary == u0.is_primary && {
                    let cols0 = u0
                        .columns
                        .iter()
                        .map(ToString::to_string)
                        .map(|str| str.to_lowercase())
                        .collect::<HashSet<_>>();
                    let cols1 = u1
                        .columns
                        .iter()
                        .map(ToString::to_string)
                        .map(|str| str.to_lowercase())
                        .collect::<HashSet<_>>();
                    cols0 == cols1
                }
            }
            (ForeignKey(f0), ForeignKey(f1)) => {
                f1.on_delete == f0.on_update
                    && {
                        let cols0 = f1
                            .referred_columns
                            .iter()
                            .map(ToString::to_string)
                            .map(|str| str.to_lowercase())
                            .collect::<HashSet<_>>();
                        let cols1 = f0
                            .referred_columns
                            .iter()
                            .map(ToString::to_string)
                            .map(|str| str.to_lowercase())
                            .collect::<HashSet<_>>();
                        cols0 == cols1
                    }
                    && {
                        f0.foreign_table.to_string().to_lowercase()
                            == f1.foreign_table.to_string().to_lowercase()
                    }
                    && {
                        let cols0 = f0
                            .columns
                            .iter()
                            .map(ToString::to_string)
                            .map(|str| str.to_lowercase())
                            .collect::<HashSet<_>>();
                        let cols1 = f1
                            .columns
                            .iter()
                            .map(ToString::to_string)
                            .map(|str| str.to_lowercase())
                            .collect::<HashSet<_>>();
                        cols0 == cols1
                    }
            }
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
