mod foreign_key;
mod unique;
use crate::prelude::*;
pub use foreign_key::*;
pub use unique::*;

#[derive(Debug, Clone)]
pub enum Constraint {
    ForeignKey(ForeignKey),
    Unique(Unique),
    Primary(Unique),
}

impl Constraint {
    pub fn from_attrs(attrs: &[Attribute], field: &Field) -> Result<Vec<Self>> {
        let mut out = vec![];
        for attr in attrs {
            let field_name = field.ident.clone().unwrap();
            let tokens = attr.tokens.clone().into();
            if attr.path.is_ident("foreign_key") {
                let fk = ForeignKey::new(tokens, field_name)?;
                out.push(Constraint::ForeignKey(fk));
            } else if attr.path.is_ident("unique") {
                let mut constr: Unique = parse(tokens)?;
                constr.columns.push(field_name);
                out.push(Constraint::Unique(constr));
            } else if attr.path.is_ident("primary_key") {
                let mut constr: Unique = parse(tokens)?;
                constr.columns.push(field_name);
                out.push(Constraint::Primary(constr));
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
            Self::Primary(unique) => {
                key = "pkey";
                cols = unique.columns().collect();
            }
            Self::Unique(unique) => {
                key = "pkey";
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
