use crate::prelude::*;
mod default;

use default::*;

pub struct Column {
    name: Ident,
    ty: Type,
    default: Option<DefaultExpr>,
}

impl ToTokens for Column {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let col_name = &self.name;
        let ty = &self.ty;
        let default = &self.default;
        let temp = if let Some(default) = default {
            quote! {
<<<<<<< HEAD
                __sqlx_models_table.columns.push(
                    ::sqlx_models::private::Column::new_with_default(
                        stringify!(#col_name),
                        <#ty as ::sqlx_models::private::IntoSQL>::into_sql(),
                        <#ty as ::sqlx_models::private::IntoSQL>::null_option(),
=======
                __models_table.columns.push(
                    ::models::private::Column::new_with_default(
                        stringify!(#col_name),
                        <#ty as ::models::types::IntoSQL>::into_sql(),
                        <#ty as ::models::types::IntoSQL>::IS_NULLABLE,
>>>>>>> down-migrations
                        #default
                ));
            }
        } else {
            quote! {
<<<<<<< HEAD
                __sqlx_models_table.columns.push(
                    ::sqlx_models::private::Column::new(
                        stringify!(#col_name),
                        <#ty as ::sqlx_models::private::IntoSQL>::into_sql(),
                        <#ty as ::sqlx_models::private::IntoSQL>::null_option(),
=======
                __models_table.columns.push(
                    ::models::private::Column::new(
                        stringify!(#col_name),
                        <#ty as ::models::types::IntoSQL>::into_sql(),
                        <#ty as ::models::types::IntoSQL>::IS_NULLABLE,
>>>>>>> down-migrations
                ));
            }
        };
        tokens.extend(temp);
    }
}

impl Column {
    pub fn new(field: &Field) -> Result<Self> {
        let ty = field.ty.clone();
        let default = Self::get_default(field.attrs.clone())?;
        let name = field.ident.clone().unwrap();
        Ok(Self { ty, default, name })
    }

    fn get_default(attrs: Vec<Attribute>) -> Result<Option<DefaultExpr>> {
        for attr in attrs {
            if attr.path.is_ident("default") {
                return Ok(Some(syn::parse(attr.tokens.into())?));
            }
        }
        Ok(None)
    }
}
<<<<<<< HEAD

=======
>>>>>>> down-migrations
