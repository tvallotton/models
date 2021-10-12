use crate::prelude::*;
use getter::Getter;

mod getter;

pub struct Getters(Vec<Getter>);

impl Getters {
    pub fn new(model: &Model) -> Self {
        todo!()
    }
}

impl ToTokens for Getters {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let getters = self.0; 
        tokens.extend(quote! {
            #(#getters)*
        })
    }
}
