mod derive_model;
mod migration_generation;
mod prelude;
use derive_model::*;
use migration_generation::*;
use prelude::*;

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
