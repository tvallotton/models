use crate::prelude::*;
use std::collections::VecDeque;

#[derive(Default, Clone)]
pub struct Unique {
    pub(super) columns: VecDeque<Ident>,
    pub(super) getter: Option<LitStr>,
}

impl Unique {
    pub fn columns(&self) -> impl Iterator<Item = &Ident> {
        self.columns.iter()
    }

    pub fn getter(&self) -> Option<Ident> {
        let getter = self.getter
            .as_ref()
            .map(|lit_str| Ident::new(&lit_str.value(), lit_str.span()))

}

impl Parse for Unique {
    fn parse(input: parse::ParseStream) -> Result<Self> {
        let mut out = Unique::default();
        let content;
        if input.is_empty() {
        } else {
            let _paren = parenthesized!(content in input);
            while !content.is_empty() {
                out.columns.push_back(content.parse()?);
                if !content.is_empty() {
                    content.parse::<Token![,]>()?;
                }
                if content.peek(Token![=]) {
                    content.parse::<Token![=]>()?;
                    out.getter = Some(content.parse()?);
                }
            }
        }
        Ok(out)
    }
}
