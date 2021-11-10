use crate::model::constraint;
use crate::prelude::*;
pub enum Getter {
    ForeignKey(ForeignKey),
    Unique(Unique),
    Primary(Unique),
}



pub struct ForeignKey {
    foreign_key: Ident,
    foreign_table: Type,
    table_name: Ident,
    key_name: Ident,
}
pub struct Unique {
    ty: Type,
    table_name: Ident,
    key_name: Ident,
}
impl Getter {
    fn key_name(&self) -> &Ident {
        use Getter::*;
        match self {
            ForeignKey(fk) => &fk.key_name,
            Unique(k) => &k.key_name,
            Primary(k) => &k.key_name,
        }
    }

    pub fn foreign_key(table: &Ident, key: &Ident, fk: &constraint::ForeignKey) -> Self {
        Self::ForeignKey(ForeignKey {
            foreign_key: fk.column.clone(),
            foreign_table: fk.foreign_table.clone(),
            table_name: table.clone(),
            key_name: key.clone(),
        })
    }

    pub fn primary_key(table: &Ident, ty: Type, pk: &constraint::Unique) -> Self {
        Self::Primary(Unique {
            table_name: table.clone(),
            ty,
            key_name: pk.columns[0].clone(),
        })
    }
}

impl Getter {
    fn getter_name(&self) -> Ident {
        let name = self.key_name().to_string();
        let span = self.key_name().span();
        match &self {
            Self::ForeignKey(_) => {
                if name.ends_with("_id") {
                    let len = name.chars().count();
                    let name: String = name.chars().take(len - 3).collect();
                    Ident::new(&name, span)
                } else {
                    self.key_name().clone()
                }
            }
            Self::Primary(_) => Ident::new("find", span),

            Self::Unique(_) => Ident::new(&format!("find_by{}", name), span),
        }
    }
}

impl ToTokens for Getter {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let getter_name = self.getter_name();
        match self {
            Self::Unique(Unique {
                table_name,
                key_name,
                ty,
                ..
            })
            | Self::Primary(Unique {
                table_name,
                key_name,
                ty,
                ..
            }) => {
                tokens.extend(quote! {
                    impl #table_name {
                        pub async fn #getter_name(#key_name: #ty) -> std::result::Result<Self, ::models::ORMError> {
                            ::models::private::DATABASE_CONNECTION.as_ref().map_err(Clone::clone)?.query_key::<#ty, Self>(
                                stringify!(#table_name),
                                stringify!(#key_name),
                                #key_name
                            ).await
                        }
                    }
                });
            }
            Self::ForeignKey(ForeignKey {
                table_name,
                key_name,
                foreign_key,
                foreign_table,
            }) => {
                // TODO: try to remove this unwrap if possible.
                let foreign_table = foreign_table; 
                tokens.extend(quote! {
                    impl #table_name {
                        pub async fn #getter_name(&self) -> std::result::Result<#foreign_table, ::models::ORMError> {
                            ::models::private::DATABASE_CONNECTION.as_ref().map_err(Clone::clone)?.query_foreign_key(
                                stringify!(#table_name),
                                stringify!(#key_name),
                                stringify!(#foreign_table), 
                                stringify!(#foreign_key),
                                &self.#key_name
                            ).await
                        }
                    }
                })
            }
        }
    }
}
