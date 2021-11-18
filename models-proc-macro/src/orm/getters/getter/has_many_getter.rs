use crate::prelude::*;

pub struct HasManyGetter<'a>(pub &'a HasMany);

impl<'a> HasManyGetter<'a> {
    pub fn name(&self) -> Ident {
        self.0.getter()
    }
}


impl<'a> ToTokens for HasManyGetter<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        todo!()
    }
}