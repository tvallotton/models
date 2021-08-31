use crate::prelude::*;
use Data::*;

struct ColumnNames(Vec<Ident>);

impl Parse for ColumnNames {
    fn parse(input: parse::ParseStream) -> Result<Self> {
        let mut out = ColumnNames(vec![]);
        let content;
        if input.is_empty() {
            Ok(out)
        } else {
            let _paren = parenthesized!(content in input);
            while !content.is_empty() {
                out.0.push(content.parse().unwrap());
            }
            Ok(out)
        }
    }
}

struct ForeignKey {
    tables: Vec<Path>,
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
            out.tables.push(content.parse::<Path>()?);
            content.parse::<Token![.]>()?;
            out.columns.push(content.parse::<Ident>()?);
        }
        Ok(out)
    }
}

fn is_attribute(path: &Path) -> bool {
    path.is_ident("foreign_key") || path.is_ident("primary_key") || path.is_ident("unique")
}

pub struct Model<'a> {
    name: &'a Ident,
    data: &'a DataStruct,
}

impl<'a> Model<'a> {
    pub fn derive(input: &'a DeriveInput) -> TokenStream2 {
        assert!(input.generics.params.is_empty(), "Models cannot be generic");
        let name = &input.ident;
        match &input.data {
            Struct(data) => {
                let model = Self { name, data };
                model.generate_code()
            }
            _ => panic!("Sql models have to be structs, enums and unions are not supported."),
        }
    }

    fn generate_code(self) -> TokenStream2 {
        let name = self.name;
        let columns = Self::get_columns(&self.data.fields);
        let constraints = self.get_constraints(&self.data.fields);
        quote! {
          impl ::sqlx_models::Model for #name {
            fn target(__sqlx_models_dialect: ::sqlx_models::Dialect) -> ::sqlx_models::Table {
                let mut __sqlx_models_table = ::sqlx_models::Table::new(stringify!(#name));
                #columns
                #constraints
                __sqlx_models_table
            }
          }
        }
    }

    fn get_columns(fields: &Fields) -> TokenStream2 {
        let mut columns = quote!();
        for field in fields {
            let col = Self::get_column(field);
            columns.extend(quote! {
                __sqlx_models_table.columns.push(#col);
            });
        }
        columns
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

    fn get_constraints(&self, fields: &Fields) -> TokenStream2 {
        let mut constraints = quote!();
        for field in fields {
            let constr = self.get_constr(field);
            constraints.extend(constr)
        }
        constraints
    }

    fn get_constr(&self, field: &Field) -> TokenStream2 {
        let ident = field.ident.as_ref().unwrap();
        let mut constraints = vec![];
        let mut validation = vec![];

        for attr in &field.attrs {
            let path = &attr.path;
            if !is_attribute(path) {
                continue;
            }
            let tokens: TokenStream = attr.tokens.clone().into();

            if path.is_ident("foreign_key") {
                let (constr, val) = Self::foreign_key(field, tokens);
                constraints.extend(constr);
                validation.extend(val);
            } else {
                let cols = parse::<ColumnNames>(tokens).unwrap().0;
                let constrs = Self::unique_constraints(path, ident, &cols);
                let val = self.unique_constr_validation(&cols);
                constraints.push(constrs);
                validation.extend(val)
            }
        }

        quote! {
            #(__sqlx_models_table.constraints.push(#constraints);)*
            #(#validation)*
        }
    }

    fn foreign_key(field: &Field, tokens: TokenStream) -> (Vec<TokenStream2>, Vec<TokenStream2>) {
        let ForeignKey { tables, columns } = parse(tokens).unwrap();
        let col = field.ident.as_ref().unwrap();
        let mut constraints = vec![];
        let mut validation = vec![];
        for (table, referred_col) in tables.iter().zip(columns.iter()) {
            constraints.push(quote! {
                ::sqlx_models::constraint::foreign_key(
                    stringify!(#col),
                    stringify!(#table),
                    stringify!(#referred_col),
                )
            });
            let val = Self::foreign_key_validation(table, referred_col, &field.ty);
            validation.push(val)
        }
        (constraints, validation)
    }

    fn foreign_key_validation(forign_table: &Path, ref_col: &Ident, ty: &Type) -> TokenStream2 {
        quote! {
            let _ = |__sqlx_models_validation: #forign_table| {
                let _: #ty = __sqlx_models_validation.#ref_col; 
            };
        }
    }

    fn unique_constr_validation(&self, colnames: &[Ident]) -> Vec<TokenStream2> {
        let mut validations = vec![];
        let struct_ident = self.name;
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
    fn unique_constraints(path: &Path, name: &Ident, cols: &[Ident]) -> TokenStream2 {
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
}
