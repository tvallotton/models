mod prelude;

use prelude::*;
use proc_macro2::TokenStream as TokenStream2;

/*

    TODO:
        check that columns exist. (with an uncalled closure)

*/

use syn::parse::Parse;
use Data::*;
#[proc_macro_derive(Model, attributes(model, primary_key, foreign_key, unique))]
pub fn model(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    assert!(input.generics.params.is_empty(), "Models cannot be generic");
    let name = input.ident;
    match input.data {
        Struct(data) => generate_code(name, &data),
        _ => panic!("Sql models have to be structs, enums and unions are not supported."),
    }
}

fn generate_code(name: Ident, data: &DataStruct) -> TokenStream {
    let columns = get_columns(&data.fields);
    
    let constraints = get_constraints(&name, &data.fields);

    let template = quote! {
      impl ::sqlx_models::Model for #name {
        fn target(__sqlx_models_dialect: ::sqlx_models::Dialect) -> ::sqlx_models::Table {
            let mut __sqlx_models_table = ::sqlx_models::Table::new(stringify!(#name));
            #columns
            #constraints
            __sqlx_models_table
        }
      }
    };
    template.into()
}

fn get_columns(fields: &Fields) -> TokenStream2 {
    let mut columns = quote!();
    for field in fields {
        let col = get_column(field);
        columns.extend(quote! {
            __sqlx_models_table.columns.push(#col);
        });
    }
    columns
}

fn get_constraints(struct_ident: &Ident, fields: &Fields) -> TokenStream2 {
    let mut constraints = quote!();
    for field in fields {
        let constr = get_constr(struct_ident, field);
        constraints.extend(constr)
    }
    constraints
}

fn get_column(field: &Field) -> TokenStream2 {
    let ty = &field.ty;
    let ident = field.ident.as_ref().unwrap();
    quote! {
       ::sqlx_models::Column::new(
        stringify!(#ident),
        <#ty as ::sqlx_models::SqlType>::as_sql(__sqlx_models_dialect),
        ::std::vec::Vec::new()
    )}
}

fn is_attribute(path: &Path) -> bool {
    path.is_ident("foreign_key") || path.is_ident("primary_key") || path.is_ident("unique")
}
fn get_constr(struct_ident: &Ident, field: &Field) -> TokenStream2 {
    let ident = field.ident.as_ref().unwrap();
    let mut constraints = vec![];
    let mut validation = vec![];

    dbg!(&struct_ident);
    for attr in &field.attrs {
        let path = &attr.path;
        if !is_attribute(path) {
            continue;
        }
        let tokens: TokenStream = attr.tokens.clone().into();

        if path.is_ident("foreign_key") {
            constraints.extend(foreign_key(struct_ident, tokens));
        } else {
            let cols = parse::<ColumnNames>(tokens).unwrap().0;
            let constrs = unique_and_primary_constraints(path, ident, &cols);
            let val = unique_constr_validation(struct_ident, &cols);
            constraints.push(constrs);
            validation.extend(val)
        }
    }

    quote! {
        #(__sqlx_models_table.constraints.push(#constraints);)*
        #(#validation)*
    }
}

fn unique_constr_validation(struct_ident: &Ident, colnames: &[Ident]) -> Vec<TokenStream2> {
    let mut validations = vec![];

    dbg!(&struct_ident);
    for col in colnames {
        let val = quote! {
            let _ = |__sqlx_models_validation: #struct_ident | {
                __sqlx_models_validation.#col;
            };
        };
        validations.push(val);
    }

    validations
}

fn unique_and_primary_constraints(path: &Path, name: &Ident, cols: &[Ident]) -> TokenStream2 {
    let method = if path.is_ident("primary_key") {
        quote!(primary)
    } else if path.is_ident("unique") {
        quote!(unique)
    } else {
        return quote!();
    };

    quote! {
        ::sqlx_models::constraint::#method(
            &[stringify!(#name), #(stringify!(#cols)),*]
        )
    }
}

struct ColumnNames(Vec<Ident>);

impl Parse for ColumnNames {
    fn parse(input: parse::ParseStream) -> Result<Self> {
        let mut out = ColumnNames(vec![]);
        let content;
        let _paren = parenthesized!(content in input);
        while !content.is_empty() {
            out.0.push(content.parse().unwrap());
        }
        Ok(out)
    }
}

fn foreign_key(local_col: &Ident, tokens: TokenStream) -> Vec<TokenStream2> {
    let ForeignKey { tables, columns } = parse(tokens).unwrap();
    let mut constraints = vec![];
    for (table, col) in tables.iter().zip(columns.iter()) {
        constraints.push(quote! {
            ::sqlx_models::constraint::foreign_key(
                #local_col,
                #table,
                #col
            )
        })
    }
    constraints
}

struct ForeignKey {
    tables: Vec<Ident>,
    columns: Vec<Ident>,
}

impl Parse for ForeignKey {
    fn parse(input: parse::ParseStream) -> Result<Self> {
        let mut out = ForeignKey {
            tables: vec![],
            columns: vec![],
        };
        let content;
        let _paren = parenthesized!(content in input);
        while !content.is_empty() {
            out.tables.push(content.parse::<Ident>()?);
            content.parse::<Token![:]>()?;
            out.columns.push(content.parse::<Ident>()?);
        }
        Ok(out)
    }
}
