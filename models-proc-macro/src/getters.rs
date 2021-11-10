use crate::prelude::*; // TODO:
                       //     getters for foreign keys

use super::Model;

struct Getters<'a> {
    model: &'a Model,
    getters: Vec<Getter>,
}
enum Getter {
    Unique {
        table_name: Ident,
        column_name: Ident,
        column_type: Type,
    },

    Foreign {
        table_name: Ident,
        referred: Ident,
    },
}

impl<'a> ToTokens for Getters<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let ident = &self.model.name;
        let getters = &self.getters;
        tokens.extend(quote! {
            impl #ident {
                #(#getters)*
            }
        })
    }
}

impl<'a> ToTokens for Getter {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match &self {
            Self::Unique {
                table_name,
                column_name,
                column_type,
            } => {
                
                let query = format!("select * from {} where co;", table_name, column_name);
                tokens.extend(quote! {
                    fn #name(&self, val: #dtype) -> ::std::result::Result<Vec<_>, ::models::sqlx::Error>{
                        ::sqlx::query(#query).
                        .fetch_all(&conn)
                        .await
                    }
                });
            }
        }
    }
}

impl Getter {}
