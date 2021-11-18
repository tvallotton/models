use crate::model::constraint::{ForeignKey, Unique};
use crate::prelude::*;
use foreign_getter::ForeignGetter;
use getter_variant::GetterVariant;
use has_many_getter::HasManyGetter;
use has_one_getter::HasOneGetter;
use unique_getter::UniqueGetter;

mod foreign_getter;
mod getter_variant;
mod has_many_getter;
mod has_one_getter;
mod unique_getter;

pub enum Getter<'a> {
    Foreign(ForeignGetter<'a>),
    Unique(UniqueGetter<'a>),
    HasMany(HasManyGetter<'a>),
    HasOne(HasOneGetter<'a>),
}

impl<'a> Getter<'a> {
    fn getter_name(&self) -> Ident {
        match self {
            Self::Foreign(foreign_key) => foreign_key.name(),
            Self::Unique(unique) => unique.name(),
            Self::HasMany(has_many) => has_many.name(),
            Self::HasOne(has_one) => has_one.name(),
        }
    }

    pub fn foreign_key(model_name: &'a Ident, variant: &'a ForeignKey) -> Self {
        Self::Foreign(ForeignGetter {
            model_name,
            variant,
        })
    }

    pub fn unique_key(model: &'a Model, unique: &'a Unique) -> Self {
        let mut columns = HashMap::default();
        for col in unique.columns() {
            let ty = model.field_type(col).unwrap();
            columns.insert(col, ty);
        }
        Self::Unique(UniqueGetter {
            table_name: &model.table_name,
            model_name: &model.model_name,
            unique,
            columns,
        })
    }
    pub fn from_has_many(has_many: &'a HasMany) -> Self {
        Self::HasMany(HasManyGetter(has_many))
    }
    pub fn from_has_one(has_one: &'a HasOne) -> Self {
        Self::HasOne(HasOneGetter(has_one))
    }
}

impl<'a> ToTokens for Getter<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            Self::Unique(unique) => unique.to_tokens(tokens),
            Self::Foreign(foreign) => foreign.to_tokens(tokens),
            Self::HasMany(has_many) => has_many.to_tokens(tokens),
            Self::HasOne(has_one) => has_one.to_tokens(tokens),
        }
    }
}
