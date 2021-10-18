use crate::prelude::*;

pub struct Getter {
    getter_kind: GetterKind, 
    table_name: Ident,
    getter_name: Ident,
}

enum GetterKind {
    ForeignKey, 
    Unique, 
}

impl ToTokens for Getter {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let Getter {
            table_name,
            getter_name,
        } = self;

        tokens.extend(quote! {
            impl #table_name {
                async fn #getter_name() -> std::result::Result<, ::models::sqlx::Error> {
                }
            }
        })
    }
}
