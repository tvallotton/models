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

    fn query(&self, format: &str) -> String {
           format!("select {{foreign_table}}.* from {{foreign_table}} 
                inner join {table_name} 
                on {table_name}.{local_column} = {{foreign_table}}.{foreign_column} 
                where {table_name}.{local_column} = {format};",
                table_name = self.table_name, 
                foreign_column = self.variant.foreign_column, 
                local_column = self.variant.local_column, 
                format = format, 
            )
    }
    pub fn query_any(&self) -> String {
        self.query("?")
    }

    pub fn query_postgres(&self) -> String {
        self.query("$1")
    }
}


impl<'a> ToTokens for ForeignGetter<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let getter_name = self.name();
        let table_name = self.model_name;
        let postgres_query = self.query_postgres(); 
        let query_any = self.query_any(); 
        let ForeignKey {
            foreign_table,
            foreign_column,
            local_column,
            getter,
            on_delete,
            on_update,
        } = self.variant;
        tokens.extend(quote! {
            const _: () = {
                impl #table_name {
                    pub async fn #getter_name(&self) -> std::result::Result<#foreign_table, ::models::orm::Error> {
                        let __models_conn: &'static ::models::orm::Connection = ::models::orm::DATABASE_CONNECTION.as_ref().map_err(Clone::clone)?;
                         
                        static QUERY: ::models::private::Lazy<String> = ::models::private::Lazy::new(move || {
                                // this unwrap is safe beacuse the function would have returned on the previous let binding. 
                                let __models_conn: &'static ::models::orm::Connection = ::models::orm::DATABASE_CONNECTION.as_ref().map_err(Clone::clone).unwrap();
                                 if let ::models::private::Dialect::PostgreSQL = __models_conn.dialect {
                                    format!(#postgres_query, 
                                        foreign_table = <#foreign_table as ::models::private::Model>::TABLE_NAME
                                    )
                                } else {
                                    format!(#query_any, 
                                        foreign_table = <#foreign_table as ::models::private::Model>::TABLE_NAME
                                    )
                                }
                        }); 
                        
                        Ok(::models::private::sqlx::query_as(
                            &*QUERY
                        ).bind(&self.#local_column)
                        .fetch_one(&__models_conn.pool)
                        .await?)

                    }
                }
            }; 
        });
    }
}
