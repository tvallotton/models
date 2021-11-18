use getter::Getter;

use crate::model::constraint::Constraint;
use crate::prelude::*;
mod getter;

pub struct Getters<'a>(Vec<Getter<'a>>);

impl<'a> Getters<'a> {
    pub fn new(model: &'a Model) -> Self {
        let mut getters = Getters(vec![]);
        getters.init_columns_and_constraints(model);
        getters.init_top_level_attrs(model);
        getters
    }

    fn init_columns_and_constraints(&mut self, model: &'a Model) {
        for cons in &model.constraints {
            match &cons {
                Constraint::ForeignKey(fk) => {
                    let getter = Getter::foreign_key(&model.model_name, fk);
                    self.0.push(getter);
                }

                Constraint::Unique(u) => {
                    let getter = Getter::unique_key(model, u);
                    self.0.push(getter);
                }
            }
        }
    }
    
    fn init_top_level_attrs(&mut self, model: &'a Model) {
        for has_many in &model.has_many {
            self.0.push(Getter::from_has_many(has_many))
        }
        for has_one in &model.has_one {
            self.0.push(Getter::from_has_one(has_one))
        }
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
