mod foreign_key;
mod unique;
use crate::prelude::*;
pub use foreign_key::*;
pub use unique::*;

#[derive(Clone)]
pub enum Constraint {
    ForeignKey(ForeignKey),
    Unique(Unique),
}

impl Constraint {
    /// Constructs a list of constraints from
    /// a field and it's associated attributes.

    pub fn from_field(field: &Field, attrs: &[Attribute]) -> Result<Vec<Self>> {
        let mut out = vec![];

        for attr in attrs {
            let path = &attr.path;
            let field_name = field.ident.clone().unwrap();
            let tokens = attr.tokens.clone().into();
            if attr.path.is_ident("foreign_key") {
                let fk = ForeignKey::new(tokens, field_name)?;
                out.push(Constraint::ForeignKey(fk));
            } else if path.is_ident("unique") || path.is_ident("primary_key") {
                let mut constr: Unique = parse(tokens)?;
                constr.columns.push_front(field_name);
                constr.is_primary = path.is_ident("primary_key");
                out.push(Constraint::Unique(constr));
            }
        }
        Ok(out)
    }
    pub fn name(&self, table_name: &Ident) -> String {
        let key;
        let cols: Vec<_>;
        match self {
            Self::ForeignKey(fk) => {
                key = "fkey";
                cols = vec![&fk.local_column, &fk.foreign_column];
            }

            Self::Unique(unique) => {
                key = if unique.is_primary { "pkey" } else { "key" };
                cols = unique.columns().collect();
            }
        };
        let column_names = cols.iter().fold(String::new(), |mut acc, new| {
            acc.push('_');
            acc += &new.to_string();
            acc
        });
        format!(
            "{table_name}{column_names}_{key}",
            table_name = &table_name.to_string().to_lowercase(),
            column_names = column_names,
            key = key,
        )
    }
}
