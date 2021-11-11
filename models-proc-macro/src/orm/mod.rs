use crate::prelude::*;
// mod constructor;
mod getters;

use getters::Getters;

use crate::model::Model;

pub struct ORM<'a> {
    getters: Getters<'a>,
}

impl<'a> ORM<'a> {
    pub fn new(model: &Model) -> Self {
        let getters = Getters::new(model);

        ORM { getters }
    }
}

impl<'a> ToTokens for ORM<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let getters = &self.getters;

        tokens.extend(quote! {
                #getters
        })
    }
}
