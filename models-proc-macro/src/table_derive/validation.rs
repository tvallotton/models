use crate::model2;
use crate::prelude::*;
use model2::Constraint::*;

use quote::ToTokens;

 pub struct Validation {
    field: Ident,
    table_name: Path,
}

impl Validation {
    pub fn new(model: &model2::Model) -> Vec<Self> {
        
        let mut out = vec![];
        for cons in &model.constraints {
            match cons {
                ForeignKey(fk) => out.push(Self {
                    field: fk.foreign_column,
                    table_name: fk.foreign_table,
                }),
                Primary(unique) | Unique(unique) => {
                    for field in unique.columns() {
                        out.push(Self {
                            field: field.clone(),
                            table_name: model.name.clone().into(),
                        });
                    }
                }
            }
        }
        out
    }
}

impl ToTokens for Validation {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let field = &self.field;
        let table_name = &self.table_name;
        tokens.extend(quote! {
            let _ = |__models_validation: #table_name| {
                __models_validation.#field;
            }
        });
    }
}
