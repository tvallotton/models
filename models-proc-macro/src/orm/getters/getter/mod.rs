use crate::model::constraint::{ForeignKey, Unique};
use crate::prelude::*;
use foreign_getter::ForeignGetter;
use unique_getter::UniqueGetter;

mod foreign_getter;
mod unique_getter;

pub enum Getter<'a> {
    ForeignGetter(ForeignGetter<'a>),
    Unique(UniqueGetter<'a>),
}

impl<'a> Getter<'a> {
    fn getter_name(&self) -> Ident {
        match self {
            Self::ForeignGetter(foreign_key) => foreign_key.name(),
            Self::Unique(unique) => unique.name(),
        }
    }

    pub fn foreign_key(table_name: &'a Ident, fk: &'a ForeignKey) -> Self {
        Self::ForeignGetter(ForeignGetter {
            table_name,
            foreign_key: fk,
        })
    }

    pub fn unique_key(model: &'a Model, unique: &'a Unique) -> Self {
        let mut columns = HashMap::default();
        for col in unique.columns() {
            let ty = model.field_type(col).unwrap();
            columns.insert(col, ty);
        }
        Self::Unique(UniqueGetter {
            table_name: &model.name,
            unique,
            columns,
        })
    }
}

impl<'a> ToTokens for Getter<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            Self::Unique(unique) => unique.to_tokens(tokens),
            Self::ForeignGetter(foreign) => foreign.to_tokens(tokens),
        }
    }
}
