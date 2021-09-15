mod column;
mod constraint;
use crate::prelude::*;
use constraint::*;
use Data::*;

use self::column::Column;
pub struct Model {
    pub name: Ident,
    name_lowercase: Ident,
    data: DataStruct,
    columns: Vec<Column>,
    constraints: Vec<NamedConstraint>,
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
            fn target() -> ::sqlx_models::private::Table {
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

            let column = Column::new(field)?;
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
