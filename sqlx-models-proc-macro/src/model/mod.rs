mod column;
mod constraint;
use crate::prelude::*;
use constraint::*;
use Data::*;

use self::column::Column;

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
                if !content.is_empty() {
                    content.parse::<Token![,]>().unwrap();
                }
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

pub struct Model {
    pub name: Ident,
    name_lowercase: Ident,
    data: DataStruct,
    columns: Vec<Column>,
    constraints: Vec<NamedConstraint>,
}
impl Parse for Model {
    fn parse(input: parse::ParseStream) -> Result<Self> {
        let input: DeriveInput = input.parse()?;
        let name = input.ident;
        let name_lowercase = Ident::new(&name.to_string().to_lowercase(), name.span());
        match input.data {
            Struct(data) => {
                let mut model = Self {
                    name,
                    data,
                    name_lowercase,
                    columns: Default::default(),
                    constraints: Default::default(),
                };
                model.init()?;
                Ok(model)
            }
            _ => panic!("Sql models have to be structs, enums and unions are not supported."),
        }
    }
}

impl ToTokens for Model {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let name = &self.name;
        let name_lowercase = &self.name_lowercase;
        let columns = &self.get_columns();
        let constraints = &self.get_constraints();
        let template = quote! {
          impl ::sqlx_models::private::Model for #name {
            fn target(__sqlx_models_dialect: ::sqlx_models::private::Dialect) -> ::sqlx_models::private::Table {
                let mut __sqlx_models_table = ::sqlx_models::private::Table::new(stringify!(#name_lowercase));
                #columns
                #constraints
                __sqlx_models_table
            }
          }
        };
        tokens.extend(template);
    }
}

impl Model {
    // include
    fn init(&mut self) -> Result<()> {
        for field in &self.data.fields {
            let col_name = field.ident.clone().unwrap();
            let constrs: Vec<_> = Constraints::from_attrs(&field.attrs)?
                .0
                .into_iter()
                .map(|constr| NamedConstraint {
                    name: self.constr_name(&constr.method(), &col_name, &constr.column_names()),
                    field_name: col_name.clone(),
                    constr,
                })
                .collect();
            self.constraints.extend(constrs);

            let column = Column::new(&field)?;
            self.columns.push(column);
        }
        Ok(())
    }

    fn get_columns(&self) -> TokenStream2 {
        let columns = self.columns.iter();
        quote! {
            #(#columns;)*
        }
    }

    fn get_constraints(&self) -> TokenStream2 {
        let columns = self
            .constraints
            .iter()
            .map(|constr| constr.into_tokens(&self.name));

        quote! {#(#columns;)*}
    }

    fn get_column(field: &Field) -> TokenStream2 {
        let ty = &field.ty;
        let ident = field.ident.as_ref().unwrap();
        quote! {
            __sqlx_models_table.columns.push(
           ::sqlx_models::private::Column::new(
            stringify!(#ident),
            <#ty as ::sqlx_models::private::SqlType>::as_sql(),
            <#ty as ::sqlx_models::private::SqlType>::null_option()
            ));
        }
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
                let (constr, val) = self.foreign_key(field, tokens);
                constraints.extend(constr);
                validation.extend(val);
            } else {
                let cols = parse::<ColumnNames>(tokens).unwrap().0;
                let constrs = self.unique_constraints(path, ident, &cols);
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

    fn foreign_key(
        &self,
        field: &Field,
        tokens: TokenStream,
    ) -> (Vec<TokenStream2>, Vec<TokenStream2>) {
        let ForeignKey { tables, columns } = parse(tokens).unwrap();
        let col = field.ident.as_ref().unwrap();
        let mut constraints = vec![];
        let mut validation = vec![];
        for (table, referred_col) in tables.iter().zip(columns.iter()) {
            let table_name = table.get_ident().unwrap();
            let table_name = Ident::new(&table_name.to_string().to_lowercase(), table_name.span());
            let constr_name = self.constr_name(&"foreign", &col, &[&table_name, referred_col]);
            constraints.push(quote! {
                ::sqlx_models::private::constraint::foreign_key(
                    #constr_name,
                    stringify!(#col),
                    stringify!(#table_name),
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
        let struct_ident = &self.name;
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
    fn unique_constraints(&self, path: &Path, name: &Ident, cols: &[Ident]) -> TokenStream2 {
        let method = if path.is_ident("primary_key") {
            quote!(primary)
        } else if path.is_ident("unique") {
            quote!(unique)
        } else {
            return quote!();
        };
        let constr_name = self.constr_name(&method, name, cols);

        quote! {
            ::sqlx_models::private::constraint::#method(
                #constr_name,
                &[stringify!(#name), #(stringify!(#cols)),*]
            )
        }
    }

    pub fn constr_name(
        &self,
        method: &impl ToString,
        name: &impl ToString,
        cols: &[impl ToString],
    ) -> String {
        let mut constr_name = String::new();
        constr_name += &self.name_lowercase.to_string();
        constr_name += "_";
        constr_name += &method.to_string();
        constr_name += "_";
        constr_name += &name.to_string();

        for col in cols.iter() {
            constr_name += "_";

            constr_name += &col.to_string();
        }
        constr_name
    }
}
