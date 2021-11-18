use super::GetterVariant;
use crate::prelude::*;
use model::ForeignKey;

pub type ForeignGetter<'a> = GetterVariant<'a, ForeignKey>;

impl<'a> ForeignGetter<'a> {
    pub fn name(&self) -> Ident {
        if let Some(getter) = self.variant.getter.clone() {
            getter
        } else {
            let column = &self.variant.local_column;
            let name = column.to_string();
            if name.ends_with("_id") {
                let len = name.chars().count();
                let name: String = name.chars().take(len - 3).collect();
                Ident::new(&name, column.span())
            } else {
                column.clone()
            }
        }
    }
}

impl<'a> ToTokens for ForeignGetter<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let getter_name = self.name();
        let table_name = self.model_name;
        let ForeignKey {
            foreign_table,
            foreign_column,
            local_column,
            getter,
            on_delete,
            on_update,
        } = self.variant;
        tokens.extend(quote! {
            impl #table_name {
                pub async fn #getter_name(&self) -> std::result::Result<#foreign_table, ::models::ORMError> {
                    ::models::private::DATABASE_CONNECTION.as_ref().map_err(Clone::clone)?.query_foreign_key(
                        stringify!(#table_name),
                        stringify!(#local_column),
                        stringify!(#foreign_table),
                        stringify!(#foreign_column),
                        &self.#local_column
                    ).await
                }
            }
        });
    }
}
