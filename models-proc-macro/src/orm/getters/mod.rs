use std::slice::Concat;
use std::sync::
use getter::Getter;

use crate::model::constraint::{Constraint, ForeignKey, Unique};
use crate::prelude::*;
mod getter;

pub struct Getters(Vec<Getter>);

impl Getters {
    pub fn new(model: &Model) -> Self {
        let mut getters = vec![];
        for cons in &model.constraints {
            match &cons.constr {
                Constraint::ForeignKey(fk) => {
                    let getter = Getter::foreign_key(&model.name_lowercase, &cons.field_name, fk);
                    getters.push(getter);
                }
                Constraint::Primary(pk) => Getter::primary_key(table, ty, pk),

                Constraint::Unique(u) => {}
            }
        }
        Self(getters)
    }
}

impl ToTokens for Getters {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let getters = &self.0;
        tokens.extend(quote! {
            #(#getters)*
        })
    }
}
