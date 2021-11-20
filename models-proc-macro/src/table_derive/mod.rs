use crate::prelude::*;
use column::*;
use constraint::NamedConstraint;
use validation::*;

mod column;
mod constraint;
mod validation;

pub struct TableDerive<'a> {
    table_name: &'a str,
    model_name: &'a Ident,
    columns: Vec<TableColumn>,
    constraints: Vec<NamedConstraint>,
    validation: Vec<Validation>,
}

impl<'a> TableDerive<'a> {
    pub fn new(model: &'a Model) -> Self {
        let table_name = &model.table_name;
        let columns = TableColumn::new(model);
        let constraints = NamedConstraint::new(model);
        let validation = Validation::new(model);
        Self {
            table_name,
            model_name: &model.model_name,
            columns,
            constraints,
            validation,
        }
    }
}

impl<'a> ToTokens for TableDerive<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let table_name = &self.table_name;
        let model_name = &self.model_name;
        let columns = &self.columns;
        let constraints = &self.constraints;
        let validation = &self.validation;

        tokens.extend(quote! {
            impl ::models::private::Model for #model_name {
                const TABLE_NAME: &'static str = #table_name;
                fn target() -> ::models::private::Table {
                    let mut __models_table = ::models::private::Table::new(#table_name);
                    #(#columns)*
                    #(#constraints)*
                    #(#validation)*
                    __models_table
                }
            }
        });
    }
}
