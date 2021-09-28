use crate::prelude::*;
// SQLX_MODELS_GENERATE_MIGRATION=true
<<<<<<< HEAD
// SQLX_MODELS_GENERATE_MIGRATIONS

pub fn generate_migration(name: &Ident) -> TokenStream2 {
    if let Ok(value) = std::env::var("SQLX_MODELS_GENERATE_MIGRATIONS") {
=======
// MODELS_GENERATE_MIGRATIONS

pub fn generate_migration(name: &Ident) -> TokenStream2 {
    if let Ok(value) = std::env::var("MODELS_GENERATE_MIGRATIONS") {
>>>>>>> down-migrations
        if value.to_lowercase() == "true" {
            generate_migration_unchecked(name)
        } else {
            quote!()
        }
    } else {
        quote!()
    }
}

fn generate_migration_unchecked(name: &Ident) -> TokenStream2 {
    let test_name = Ident::new(
<<<<<<< HEAD
        &format!("__sqlx_models_generate_migration_{}", name),
=======
        &format!("__models_generate_migration_{}", name),
>>>>>>> down-migrations
        proc_macro2::Span::call_site(),
    );
    quote! {
        #[test]
        fn #test_name() {
<<<<<<< HEAD
            ::sqlx_models::private::MIGRATIONS.register(
                <#name as ::sqlx_models::private::Model>::target()
=======
            ::models::private::SCHEDULER.register(
                <#name as ::models::private::Model>::target()
>>>>>>> down-migrations
            );
        }
    }
}
