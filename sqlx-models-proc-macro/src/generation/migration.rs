use crate::prelude::*; 


struct Migration<'a> {
    name: Ident, 
}



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
            ::sqlx_models::private::MIGRATIONS.register(
                <#name as ::sqlx_models::private::Model>::target()
            );
        }
    }
}
