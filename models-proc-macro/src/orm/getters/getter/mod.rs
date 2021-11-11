use crate::model::constraint;
use crate::prelude::*; 
use foreign_getter::ForeignGetter;
use unique_getter::UniqueGetter;



mod foreign_getter;
mod unique_getter;

pub enum Getter<'a> {
    ForeignKey(ForeignGetter<'a>),
    Unique (UniqueGetter<'a>),
    Primary (UniqueGetter<'a>)
}

pub struct Unique<'a> {
    table_name: &'a Ident,
    columns: HashMap<&'a Ident, &'a Type>,
}
impl<'a> Getter<'a> {
    fn getter_name(&self) -> Ident {
        match self {
            Self::ForeignKey(foreign_key) => foreign_key.getter(),
            Self::Primary (unique) => {
                if let Some(getter) = unique.getter() {
                    getter
                } else {
                    let span = unique.columns().next().unwrap().span();
                    Ident::new("find", span)
                }
            }

            Self::Unique (unique) => {
                if let Some(getter) = unique.getter() {
                    getter
                } else {
                    let columns = unique.columns().fold(String::new(), |mut acc, ident| {
                        acc.push('_');
                        acc += &ident.to_string();
                        acc
                    });
                    let span = unique.columns().next().unwrap().span();
                    Ident::new(&format!("find_by{}", columns), span)
                }
            }
        }
    }

    pub fn foreign_key(table_name: &'a Ident, fk: &'a constraint::ForeignKey) -> Self {
        Self::ForeignKey {
            table_name,
            foreign_key: fk,
        }
    }

    pub fn primary_key(table: &Ident, ty: Type, pk: &constraint::Unique) -> Self {
        todo!()
    }
    pub fn unique_key(table: &Ident, ty: Type, pk: &constraint::Unique) -> Self {
        todo!()
    }
}

impl<'a> ToTokens for Getter<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let getter_name = self.getter_name();
        match self {
            Self::Unique {
                table_name,
                columns,
                unique,
            }
            | Self::Primary {
                table_name,
                columns,
                unique,
            } => {
                let getter_name = unique.getter();
                let fields = columns.keys();
                let types = columns.values();
                tokens.extend(quote! {
                    impl #table_name {
                        pub async fn #getter_name(#(#fields: #types),*) -> std::result::Result<Self, ::models::ORMError> {
                            ::models::private::DATABASE_CONNECTION.as_ref().map_err(Clone::clone)?.query_key::<Self, Self>(
                                stringify!(#table_name),
                                &[#(stringify!(#fields)),*],
                                &[#(#fields),*],
                            ).await
                        }
                    }
                });
            }
            Self::ForeignKey {
                table_name,
                foreign_key,
            } => {

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
                })
            }
        }
    }
}
