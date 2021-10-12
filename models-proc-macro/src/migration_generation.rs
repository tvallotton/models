use crate::prelude::*;
// SQLX_MODELS_GENERATE_MIGRATION=true
// MODELS_GENERATE_MIGRATIONS

pub fn generate_migration(name: &Ident) -> TokenStream2 {
    if let Ok(value) = std::env::var("MODELS_GENERATE_MIGRATIONS") {
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
        &format!("__models_generate_migration_{}", name),
        proc_macro2::Span::call_site(),
    );
    quote! {
        #[test]
        fn #test_name() {
            if let Ok(value) = std::env::var("MODELS_GENERATE_MIGRATIONS") {
                ::models::private::SCHEDULER.register(
                    <#name as ::models::private::Model>::target()
                );
            }
        }
    }
}
