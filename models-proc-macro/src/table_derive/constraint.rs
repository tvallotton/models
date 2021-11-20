use crate::prelude::*;
use model::ForeignKey;
pub use model::{Constraint, Unique};
pub use proc_macro2::Span;

pub struct NamedConstraint {
    constr_name: String,
    table_name: Ident,
    constr: Constraint,
}

impl NamedConstraint {
    pub fn new(model: &Model) -> Vec<Self> {
        model
            .constraints
            .iter()
            .map(|cons| {
                let constr_name = cons.name(&model.model_name);
                let constr = cons.clone();
                let table_name = model.model_name.clone();
                Self {
                    constr_name,
                    table_name,
                    constr,
                }
            })
            .collect()
    }

    fn foreign_to_tokens(&self, foreign: &ForeignKey) -> TokenStream2 {
        let constr_name = &self.constr_name;
        let foreign_column = &foreign.foreign_column;
        let foreign_table = &foreign.foreign_table;
        let local_column = &foreign.local_column;
        let on_update = &foreign.on_update();
        let on_delete = &foreign.on_delete();
        quote! {
            __models_table.constraints.push(
                ::models::private::constraint::foreign_key(
                    #constr_name,
                    stringify!(#local_column),
                    <#foreign_table as models::private::Model>::TABLE_NAME,
                    stringify!(#foreign_column),
                    #on_delete,
                    #on_update,
                )
            );
        }
    }

    fn unique_to_tokens(&self, unique: &Unique) -> TokenStream2 {
        if unique.is_primary {
            self.key_to_tokens(unique, "primary")
        } else {
            self.key_to_tokens(unique, "unique")
        }
    }

    fn key_to_tokens(&self, unique: &Unique, method: &str) -> TokenStream2 {
        let constr_name = &self.constr_name;
        let columns = unique.columns();
        let method = Ident::new(method, Span::call_site());
        quote! {
            __models_table.constraints.push(
                ::models::private::constraint::#method(
                    #constr_name,
                    &[#(stringify!(#columns)),*]
                )
            );
        }
    }
}

impl ToTokens for NamedConstraint {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use Constraint::*;
        tokens.extend(match &self.constr {
            ForeignKey(foreign) => self.foreign_to_tokens(foreign),
            Unique(unique) => self.unique_to_tokens(unique),
        });
    }
}
