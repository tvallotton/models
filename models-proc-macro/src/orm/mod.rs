use crate::prelude::*;
mod constructor;
mod getters;
use constructor::Constructor;
use getters::Getters;

use crate::model::Model;

pub struct ORM {
    getters: Getters,
    constructor: Constructor,
}
    

impl ORM {
    fn new(model: Model) -> Self {
        let getters = Getters::new(&model);
        let constructor = Constructor::new(&model);
        ORM {
            getters,
            constructor,
        }
    }
}

impl ToTokens for ORM {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let getters = &self.getters;
        let constructor = &self.constructor;

        tokens.extend(quote! {
            #getters
            #constructor
        })
    }
}
