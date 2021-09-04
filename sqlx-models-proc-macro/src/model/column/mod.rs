use crate::prelude::*;
mod default; 

use default::*;



struct Column {
    name: Ident,
    ty: Type,
    default: Option<DefaultExpr>,
}

impl ToTokens for Column {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let col_name = &self.name;
        let ty = &self.ty;
        let default = &self.default;

        quote! {
            ::sqlx_models::private::Column::new(
                stringify!(#col_name),
                <#ty as ::sqlx_models::SqlType>::as_sql(),
                [
                    <#ty as ::sqlx_models::SqlType>::null_option(),
                    #default
                ]
        )};
    }
}

impl Column {
    fn new(field: Field) -> Self {
        let ty = field.ty;
        let default = Self::get_default(field.attrs);

        todo!()
    }

    fn get_default(attrs: Vec<Attribute>) -> Option<TokenStream2> {
        for attr in attrs {
            if attr.path.is_ident("default") {
                // syn::parse(attr.tokens).unwrap()
            }
        }
        todo!()
    }
}
