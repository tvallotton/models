use crate::prelude::*;
use model::Unique;

pub struct UniqueGetter<'a> {
    pub(super) table_name: &'a Ident,
    pub(super) unique: &'a Unique,
    pub(super) columns: HashMap<&'a Ident, &'a Type>,
}

impl<'a> ToTokens for UniqueGetter<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let table_name = self.table_name;
        let getter_name = self.name();
        let fields = self.columns.keys();
        let fields1 = self.columns.keys();
        let fields2 = self.columns.keys();
        let types = self.columns.values();
        let N = self.columns.len(); 
        tokens.extend(quote! {
            impl #table_name {
                pub async fn #getter_name(#(#fields: #types),*) -> std::result::Result<Self, ::models::ORMError> {
                    ::models::private::DATABASE_CONNECTION.as_ref().map_err(Clone::clone)?.query_key::<Self, Self, #N>(
                        stringify!(#table_name),
                        &[#(stringify!(#fields1)),*],
                        [#(#fields2),*],
                    ).await
                }
            }
        })
        ;
    }
}

impl<'a> UniqueGetter<'a> {
    pub fn name(&self) -> Ident {
        if let Some(getter) = self.unique.getter() {
            getter
        } else if self.unique.is_primary {
            self.primary_key_name()
        } else {
            self.unique_key_name()
        }
    }
    fn unique_key_name(&self) -> Ident {
        let columns = self.unique.columns().fold(String::new(), |mut acc, ident| {
            acc.push('_');
            acc += &ident.to_string();
            acc
        });
        let span = self.unique.columns().next().unwrap().span();
        Ident::new(&format!("find_by{}", columns), span)
    }

    fn primary_key_name(&self) -> Ident {
        let span = self.unique.columns().next().unwrap().span();
        Ident::new("find", span)
    }
}
