mod prelude;
use prelude::*;
use proc_macro2::TokenStream as TokenStream2;

// #[proc_macro_derive(FromRow, attributes(model, foreig_key, unique))]
// pub fn from_row(input: TokenStream) -> TokenStream {
//     todo!()
// }
use Data::*;
#[proc_macro_derive(Model, attributes(model))]
pub fn model(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    assert!(input.generics.params.is_empty(), "Models cannot be generic");
    let name = input.ident;
    match input.data {
        Struct(data) => read_fields(name, data),
        _ => panic!("Enums and unions are not supported."),
    }
}

fn read_fields(name: Ident, data: DataStruct) -> TokenStream {
    // let mut fields = HashMap::new();

    for field in data.fields {
        let col = get_column(field);
    }
    todo!()
}

fn get_column(field: Field) -> TokenStream2 {
    let ty = field.ty;
    let ident = field.ident;
    quote! {
        Column {
        name: stringify!(#ident).into(),
        r#type: <#ty as ::sqlx_models::SqlType>::to_sql()
        }
    }
}
