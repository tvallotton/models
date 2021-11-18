use crate::prelude::*;
use model::Unique;
use std::{collections::VecDeque, fmt::Write};
pub struct UniqueGetter<'a> {
    pub(super) table_name: &'a str,
    pub(super) model_name: &'a Ident,
    pub(super) unique: &'a Unique,
    pub(super) columns: VecDeque<(&'a Ident, &'a Type)>,
}

impl<'a> UniqueGetter<'a> {
    fn column_names(&self) -> impl Iterator<Item = &Ident> {
        self.columns.iter().map(|x| x.0)
    }

    fn column_types(&self) -> impl Iterator<Item = &Type> {
        self.columns.iter().map(|x| x.1)
    }
    fn query_postgres(&self) -> String {
        let mut string = format!(
            "select * from {table_name} where",
            table_name = self.table_name,
        );
        for (i, col) in self.column_names().enumerate() {
            write!(&mut string, " {} = ${}", col, i + 1).unwrap();
            if i != self.columns.len() - 1 {
                string.push(',');
            }
        }
        string.push(';');
        string
    }

    fn query_any(&self) -> String {
        let mut string = format!(
            "select * from {table_name} where",
            table_name = self.table_name,
        );

        for (i, col) in self.column_names().enumerate() {
            write!(&mut string, " {} = ?", col).unwrap();
            if i != self.columns.len() - 1 {
                string.push(',');
            }
        }
        string.push(';');
        string
    }
}

impl<'a> ToTokens for UniqueGetter<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let getter_name = self.name();
        let model_name = self.model_name;
        let fields1 = self.column_names();
        let fields2 = self.column_names();
        let types = self.column_types();
        let any_query = self.query_any();
        let postgres_query = self.query_postgres();
        tokens.extend(quote! {
        const _: () = {
            mod __ {
                use ::std::ops::Deref;
                use ::models::{
                    private,
                    private::sqlx,
                    orm,
                };
                impl super::#model_name {
                    pub async fn #getter_name(#(#fields1: #types),*) -> Result<Self, orm::Error>
                        {
                        let conn = orm::DATABASE_CONNECTION.as_ref().map_err(Clone::clone)?;
                        let query = if let private::Dialect::PostgreSQL = conn.dialect {
                            sqlx::query_as(#postgres_query)
                        } else {
                            sqlx::query_as(#any_query)
                        };
                        Ok(query
                            #(.bind(#fields2))*
                            .fetch_one(&conn.pool)
                            .await?)

                        }
                    }
                }
            };
        });
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
