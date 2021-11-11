use crate::prelude::*;
use model::Column;

pub struct TableColumn(Column);

impl TableColumn {
    pub fn new(model: &Model) -> Vec<Self> {
        model
            .columns
            .iter() //
            .cloned()
            .map(TableColumn)
            .collect()
    }
}

impl ToTokens for TableColumn {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let column_name = &self.0.name;
        let ty = &self.0.ty;
        let default = &self.0.default;
        let temp = if let Some(default) = default {
            quote! {
                __models_table.columns.push(
                    ::models::private::Column::new_with_default(
                        stringify!(#column_name),
                        <#ty as ::models::types::IntoSQL>::into_sql(),
                        <#ty as ::models::types::IntoSQL>::IS_NULLABLE,
                        #default
                ));
            }
        } else {
            quote! {
                __models_table.columns.push(
                    ::models::private::Column::new(
                        stringify!(#column_name),
                        <#ty as ::models::types::IntoSQL>::into_sql(),
                        <#ty as ::models::types::IntoSQL>::IS_NULLABLE,
                ));
            }
        };
        tokens.extend(temp);
    }
}
