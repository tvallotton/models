mod derive_model;
mod migration_generation;
mod prelude;
use derive_model::*;
use prelude::*;
use migration_generation::*; 
/*

    TODO:
        check that columns exist in foreign key constraints.

*/

#[proc_macro_derive(Model, attributes(model, primary_key, foreign_key, unique))]
pub fn model(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let derive = Model::derive(&input);
    let migrations = generate_migration(&input.ident); 
    let template = quote! {
        #derive
        #migrations
    };
    template.into()
}

// impl ::sqlx_models::Model for User {
//     fn target(__sqlx_models_dialect: ::sqlx_models::Dialect) -> ::sqlx_models::Table {
//         let mut __sqlx_models_table = ::sqlx_models::Table::new("User");
//         __sqlx_models_table.columns.push(::sqlx_models::Column::new(
//             "id",
//             <i32 as ::sqlx_models::SqlType>::as_sql(__sqlx_models_dialect),
//             ::std::vec::Vec::new(),
//         ));
//         __sqlx_models_table.columns.push(::sqlx_models::Column::new(
//             "second_id",
//             <i32 as ::sqlx_models::SqlType>::as_sql(__sqlx_models_dialect),
//             ::std::vec::Vec::new(),
//         ));
//         __sqlx_models_table
//             .constraints
//             .push(::sqlx_models::constraint::primary(&["id", "second_id"]));
//         let _ = |__sqlx_models_validation: User| {
//             __sqlx_models_validation.second_id;
//         };
//         __sqlx_models_table
//     }
// }
