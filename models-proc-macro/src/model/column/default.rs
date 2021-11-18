use proc_macro2::Span;

use crate::prelude::*;
#[derive(Clone)]
pub struct DefaultExpr {
    is_string: bool,
    expr: String,
}
// TODO: move this implementation to a type at crate::table_derive.
impl ToTokens for DefaultExpr {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let expr = &self.expr;
        if !self.is_string {
            tokens.extend(quote!(#expr));
        } else {
            let expr = format!("{:?}", self.expr);
            let len = expr.chars().count();
            let mut out: String = "'".into();
            for char in expr.chars().skip(1).take(len - 2) {
                out.push(char);
            }
            out.push('\'');
            tokens.extend(quote!(#out))
        }
    }
}

impl Parse for DefaultExpr {
    fn parse(input: parse::ParseStream) -> Result<Self> {
        let content;
        let _paren = parenthesized!(content in input);
        let span = Span::call_site();
        let mut is_string = false;
        let expr = match content.parse::<Lit>() {
            Ok(Lit::Bool(boolean)) => boolean.value().to_string(),
            Ok(Lit::Int(int)) => int.to_string(),
            Ok(Lit::Float(float)) => float.to_string(),
            Ok(Lit::Str(string)) => {
                is_string = true;
                string.value()
            }
            Ok(lit) => Err(Error::new(
                lit.span(),
                "Expected string, boolean, or numeric literal",
            ))?,
            Err(err) => Err(Error::new(
                err.span(),
                "Expected string, boolean, or numeric literal",
            ))?,
        };

        Ok(DefaultExpr { is_string, expr })
    }
}
