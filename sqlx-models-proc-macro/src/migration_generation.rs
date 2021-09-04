use crate::prelude::*;
// SQLX_MODELS_GENERATE_MIGRATION=true
// SQLX_MODELS_GENERATE_MIGRATIONS
pub fn generate_migration(name: &Ident) -> TokenStream2 {
    
    if let Ok(value) = std::env::var("SQLX_MODELS_GENERATE_MIGRATIONS") {
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
        &format!("__sqlx_models_generate_migration_{}", name),
        proc_macro2::Span::call_site(),
    );
    quote! {
        #[test]
        fn #test_name() {
            let _sqlx_models_generated_migration = ::sqlx_models::private::Migration::new::<#name>();
            _sqlx_models_generated_migration.run();
        }
    }
}
