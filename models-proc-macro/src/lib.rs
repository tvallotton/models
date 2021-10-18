mod migration_generation;
// mod getters;
mod model;
#[cfg(feature = "orm")]
mod orm;
mod prelude;
use migration_generation::*;
use model::*;
// use orm::*;
use prelude::*;

#[proc_macro_derive(
    Model,
    attributes(primary_key, foreign_key, unique, default, auto_increment)
)]
pub fn model(input: TokenStream) -> TokenStream {
    let derive = parse_macro_input!(input as Model);

    let migrations = generate_migration(&derive.name);
    let template = quote! {
        #derive
        #migrations
    };
    template.into()
}
