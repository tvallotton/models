mod migration_generation;
// mod getters;
mod model;
mod prelude;
use migration_generation::*;
use model::*;
use prelude::*;

#[proc_macro_derive(Model, attributes(model, primary_key, foreign_key, unique, default))]
pub fn model(input: TokenStream) -> TokenStream {
<<<<<<< HEAD

    let derive = parse_macro_input!(input as Model);
    
=======
    let derive = parse_macro_input!(input as Model);

>>>>>>> down-migrations
    let migrations = generate_migration(&derive.name);
    let template = quote! {
        #derive
        #migrations
    };
    template.into()
}
