use crate::prelude::*;
mod constructor;
mod getters;
use crate::model::Model;
use constructor::Constructor;
use getters::Getters;


pub struct ORM {
    getters: Getters,
    constructor: Constructor,
}

impl ORM {
    fn new(model: Model) {

        let getters = Getters::new(&model);
        

    }
}

impl ToTokens for ORM {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        todo!()
    }
}
