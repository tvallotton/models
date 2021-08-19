mod prelude;

use prelude::*;

#[proc_macro_derive(Model, attributes(model, foreig_key, unique))]
pub fn model(input: TokenStream) -> TokenStream {
    todo!()
}

#[proc_macro_derive(FromRow, attributes(model, foreig_key, unique))]
pub fn from_row(input: TokenStream) -> TokenStream {
    todo!()
}

