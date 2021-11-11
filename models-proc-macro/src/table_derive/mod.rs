use crate::prelude::*;
use column::*;
use constraint::NamedConstraint;
use validation::*;

mod column;
mod constraint;
mod validation;

struct TableDerive {
    name_lowercase: String,
    table_name: Ident,
    columns: Vec<Column>,
    constraints: Vec<NamedConstraint>,
    validation: Vec<Validation>,
}

impl TableDerive {
    fn new(model: &crate::model2::Model) -> Self {
        let name_lowercase = model.name.to_string().to_lowercase();
        let columns = Column::new();
        let constraints = NamedConstraint::new(model);
        let validation = Validation::new(model);
        Self {
            name_lowercase,
            table_name: model.name.clone(),
            columns,
            constraints,
            validation,
        }
    }
}

impl ToTokens for TableDerive {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let name_lowercase = &self.name_lowercase;
        let table_name = &self.table_name;
        let columns = &self.columns;
        let constraints = &self.constraints;
        let validation = &self.validation;

        tokens.extend(quote! {
            impl ::models::private::Model for #table_name {
                fn target() -> ::models::private::Table {
                    let mut __models_table = ::models::private::Table::new(#name_lowercase);
                    #(#columns)*
                    #(#constraints)*
                    #(#validation)*
                    __models_table
                }
            }
        });
    }
}
