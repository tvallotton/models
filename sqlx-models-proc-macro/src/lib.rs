mod migration_generation;
mod model;
mod prelude;
use migration_generation::*;
use model::*;
use prelude::*;

#[proc_macro_derive(Model, attributes(model, primary_key, foreign_key, unique))]
pub fn model(input: TokenStream) -> TokenStream {

    let derive = parse_macro_input!(input as Model);
    
    let migrations = generate_migration(&derive.name);
    let template = quote! {
        #derive
        #migrations
    };
    template.into()
}
