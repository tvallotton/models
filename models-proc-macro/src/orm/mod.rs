use crate::prelude::*;
// mod constructor;
mod getters;

use getters::Getters;

use crate::model::Model;

pub struct ORM {
    getters: Getters,
}

impl ORM {
    fn new(model: Model) -> Self {
        let getters = Getters::new(&model);

        ORM { getters }
    }
}

impl ToTokens for ORM {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let getters = &self.getters;
        tokens.extend(quote! {
            #getters

        })
    }
}
