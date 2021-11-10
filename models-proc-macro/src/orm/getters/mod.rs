use getter::Getter;


use crate::model::constraint::{Constraint};
use crate::prelude::*;
mod getter;

pub struct Getters(Vec<Getter>);

impl Getters {
    pub fn new(model: &Model) -> Self {
        let mut getters = vec![];
        for cons in &model.constraints {
            match &cons.constr {
                Constraint::ForeignKey(fk) => {
                    let getter = Getter::foreign_key(&model.name, &cons.field_name, fk);
                    getters.push(getter);
                }
                Constraint::Primary(pk) => {
                    for field in pk.columns.iter() {
                        let getter = Getter::primary_key(
                            &model.name,
                            model.field_type(field).unwrap().clone(),
                            pk,
                        );
                        getters.push(getter);
                    }
                }

                Constraint::Unique(u) => {
                    for field in u.columns.iter() {
                        let getter = Getter::primary_key(
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

impl ToTokens for Getters {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let getters = &self.0;
        tokens.extend(quote! {
            #(#getters)*
        })
    }
}
