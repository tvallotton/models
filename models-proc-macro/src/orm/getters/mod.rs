use getter::Getter;

use crate::model::constraint::Constraint;
use crate::prelude::*;
mod getter;

pub struct Getters<'a>(Vec<Getter<'a>>);

impl<'a> Getters<'a> {
    pub fn new(model: &Model) -> Self {
        let mut getters = vec![];
        for cons in &model.constraints {
            match &cons {
                Constraint::ForeignKey(fk) => {
                    let getter = Getter::foreign_key(&model.name, fk);
                    getters.push(getter);
                }
                Constraint::Primary(pk) => {
                    for field in pk.columns() {
                        let getter = Getter::primary_key(
                            &model.name,
                            model.field_type(field).unwrap().clone(),
                            pk,
                        );
                        getters.push(getter);
                    }
                }

                Constraint::Unique(u) => {
                    for field in u.columns() {
                        let getter = Getter::unique_key(
                            &model.name,
                            model.field_type(field).unwrap().clone(),
                            u,
                        );
                        getters.push(getter);
                    }
                }
            }
        }
        Self(getters)
    }
}

impl<'a> ToTokens for Getters<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let getters = &self.0;
        tokens.extend(quote! {
            #(#getters)*
        })
    }
}
