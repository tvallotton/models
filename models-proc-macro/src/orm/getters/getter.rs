use crate::prelude::*;

pub enum Getter {
    ForeignKey(ForeignKey),
    Unique(Unique),
    Primary(Unique),
}

pub struct ForeignKey {
    foreign_key: Ident,
    foreign_table: Ident,
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
             Self::Unique (Unique { table_name, key_name, ty,..})
              |  Self::Primary ( Unique{table_name, key_name, ty, ..}) => {
                    tokens.extend(quote! {
                    impl #table_name {
                        pub async fn #getter_name(#key_name: #ty) -> std::result::Result<Self, ::models::sqlx::Error> {
                            ::models::private::DATABASE_CONNECTION.query_key(
                                stringify!(#table_name),
                                stringify!(#key_name),
                                #key_name
                            )
                        }
                    }
                });
            }
           |  Self::ForeignKey (ForeignKey {
                table_name,
                key_name,
                foreign_key,foreign_table
            }) => {
                tokens.extend(quote! {
                    pub async fn #getter_name(&self) -> std::result::Result<Self, ::models::sqlx::Error> {
                        ::models::private::DATABASE_CONNECTION.query_foreign_key(
                            stringify!(#table_name),
                            stringify(#key_name),
                            stringify(#foreign_table)
                            stringify(#foreign_key)
                            &self.#key_name
                        )
                    }
                })
            }
        }
    }
}
