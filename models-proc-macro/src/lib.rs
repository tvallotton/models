mod migration_generation;

pub(crate) mod model;
#[cfg(feature = "orm")]
mod orm;
mod prelude;
mod table_derive;
use migration_generation::*;

use models_parser::dialect::PostgreSqlDialect;
use table_derive::*;
// use orm::*;
use prelude::*;

#[proc_macro_derive(
    Model,
    attributes(
        primary_key,
        foreign_key,
        unique,
        default,
        auto_increment,
        has_many,
        has_one,
        table_name
    )
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

#[proc_macro_derive(WrapperStruct, attributes(wrapped))]
pub fn wrapper_struct(input: TokenStream) -> TokenStream {
    let derive: DeriveInput = parse_macro_input!(input);
    let ident = &derive.ident;

    let generics = &derive.generics;
    let quoted_generics = quoted_generics(&generics);
    let wrapped = match derive.data {
        | Data::Struct(data) => {
            data.fields
                .into_iter()
                .next()
                .expect("expected one field")
                .ty
        }
        | _ => panic!("sum types are not supported."),
    };

    // let x = if let Some(attr) = derive.attrs.pop() {
    //     quote![
    //         impl #generics IntoSQL for #ident #quoted_generics {
    //             fn into_sql() -> Result<DataType> {
    //                 Ok(DataType::#attr)
    //             }
    //         }
    //     ]
    // } else {
    //     quote![]
    // };
    quote![
        impl #generics std::ops::Deref for  #ident #quoted_generics {
            type Target =  #wrapped;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl #generics std::ops::DerefMut for  #ident #quoted_generics{
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl #generics AsRef<#wrapped> for  #ident #quoted_generics {
            fn as_ref(&self) -> &#wrapped {
                &self.0
            }
        }

        impl #generics  AsMut<#wrapped> for  #ident #quoted_generics {
            fn as_mut(&mut self) -> &mut #wrapped {
                &mut self.0
            }
        }

        impl #generics From< #wrapped> for  #ident #quoted_generics {
            fn from(obj:  #wrapped) -> Self {
                 #ident(obj)
            }
        }
    ]
    .into()
}

fn quoted_generics(generics: &Generics) -> TokenStream2 {
    match generics.params.iter().next() {
        | None => {
            quote![]
        }
        | Some(GenericParam::Const(ConstParam { ident, .. })) => quote![<#ident>],
        | _ => quote![#generics],
    }
}
