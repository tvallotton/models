use crate::prelude::*;
use proc_macro2::Span;

pub struct DefaultExpr {
    expr: String,
}


impl ToTokens for DefaultExpr {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        todo!()
    }
}

impl Parse for DefaultExpr {
    fn parse(input: parse::ParseStream) -> Result<Self> {
        use sqlparser::{dialect::*, parser::Parser, tokenizer::*};
        input.parse::<Token![=]>()?;

        
        let backup = input.clone();
        let mut span = Span::call_site();
        let expr = input
            .parse::<LitBool>()
            .map(|boolean| {
                span = boolean.span();
                boolean.value().to_string()
            })
            .or_else(|_| {
                backup
                    .clone()
                    .parse::<LitInt>() //
                    .map(|int| {
                        span = int.span();
                        int.to_string()
                    })
            })
            .or_else(|_| {
                backup
                    .clone()
                    .parse::<LitStr>() //
                    .map(|string| {
                        span = string.span();
                        string.value()
                    })
            })
            .map_err(|err| {
                Error::new(err.span(), "Expected string, boolean, or numeric literal")
            })?;

        let mut lexer = Tokenizer::new(&GenericDialect {}, &expr);

        let tokens = lexer.tokenize().map_err(|err| {
            syn::Error::new(
                span,
                format!("Failed to tokenize default expression: {:?}", err.message),
            )
        })?;

        let _ = Parser::new(tokens, &GenericDialect {})
            .parse_expr()
            .map_err(|err| {
                syn::Error::new(span, format!("Failed to parse default expression: {}", err))
            });
        Ok(DefaultExpr { expr })
    }
}
