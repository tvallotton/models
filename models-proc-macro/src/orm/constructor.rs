use crate::prelude::*;
pub struct Constructor {
    table_name: Ident,
}

impl Constructor {
    pub fn new(model: &Model) -> Self {
     Self {
         table_name: model.name.clone(),
     }
    }
}

impl ToTokens for Constructor {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let table_name = &self.table_name;

        tokens.extend(quote! {
            impl #table_name {
                async fn create<T: Serialize>(value: T) -> Result<Self, ::models::Error> {
                    let value =  serde_json::to_value(value);



                }
            }
        });
    }
}
