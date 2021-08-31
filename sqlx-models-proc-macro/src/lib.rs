mod prelude;
use prelude::*;
use proc_macro2::TokenStream as TokenStream2;

// #[proc_macro_derive(FromRow, attributes(model, foreig_key, unique))]
// pub fn from_row(input: TokenStream) -> TokenStream {
//     todo!()
// }
use Data::*;
#[proc_macro_derive(Model, attributes(model, primary_key, foreign_key, unique))]
pub fn model(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    assert!(input.generics.params.is_empty(), "Models cannot be generic");
    let name = input.ident;
    match input.data {
        Struct(data) => generate_code(name, &data),
        _ => panic!("Sql models have to be structs, enums and unions are not supported."),
    }
}

fn generate_code(name: Ident, data: &DataStruct) -> TokenStream {
    // let mut fields = HashMap::new();

    let mut columns = quote!();
    let mut constraints = quote!();
    for field in &data.fields {
        let col = get_column(field);
        let constr = get_constr(field);
        columns.extend(quote! {
            table.columns.push(#col);
        });
        constraints.extend(quote! {
            table.constraints.push(#constr)
        })
    }

    quote! {
      impl ::sqlx_models::Models for #name {
        fn table() -> ::sqlx_models::Table {
            let mut __sqlx_models_table = Table::new(stringify!(#name));
            #columns
            #constraints
            __sqlx_models_table
        }
      }
    };
    todo!()
}

fn get_column(field: &Field) -> TokenStream2 {
    let ty = &field.ty;
    let ident = field.ident.as_ref().unwrap();
    quote! {
       ::sqlx_models::Columns::new(
        stringify!(#ident),
        <#ty as ::sqlx_models::SqlType>::to_sql(__sqlx_models_dialect),
        std::vec::Vec::new()
    )}
}

fn get_constr(field: &Field) -> TokenStream2 {
    todo!()
}

fn field_primary_key(ident: &Ident, others: &TokenStream2) -> TokenStream2 {
    todo!()
}
