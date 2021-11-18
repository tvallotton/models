use crate::prelude::*;

pub struct HasOneGetter<'a>(pub &'a HasOne);

impl<'a> HasOneGetter<'a> {
    pub fn name(&self) -> Ident {
        self.0.getter()
    }
}

impl<'a> ToTokens for HasOneGetter<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        todo!()
    }
}
