mod migration_generation;


pub(crate) mod model;
#[cfg(feature = "orm")]
mod orm;
mod prelude;
mod table_derive;
use migration_generation::*;

use table_derive::*;
// use orm::*;
use prelude::*;

#[proc_macro_derive(
    Model,
    attributes(primary_key, foreign_key, unique, default, auto_increment)
)]
pub fn model(input: TokenStream) -> TokenStream {
    let model = parse_macro_input!(input as Model);
    let derive = TableDerive::new(&model);
    let migrations = generate_migration(&model.model_name);
    let mut template = quote! {
        #derive
        #migrations
    };
    template.extend(quote!());
    #[cfg(feature = "orm")]
    {
        let orm = orm::ORM::new(&model);
        template.extend(quote! {
            #orm
        });
    }
    template.into()
}
