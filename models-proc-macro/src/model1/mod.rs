mod column;
pub mod constraint;
use constraint::*;
use Data::*;

use self::column::Column;
use crate::prelude::*;
pub struct Model {
    pub name: Ident,
    pub name_lowercase: Ident,
    data: DataStruct,
    pub columns: Vec<Column>,
    pub constraints: Vec<NamedConstraint>,
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
          impl ::models::private::Model for #name {
            fn target() -> ::models::private::Table {
                let mut __models_table = ::models::private::Table::new(stringify!(#name_lowercase));
                #columns
                #constraints
                __models_table
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
            let constrs: Vec<_> = Constraints::from_attrs(&field.attrs, &field)?
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
   pub fn field_type(&self, field: &Ident) -> Option<&Type> {
        self.columns
            .iter()
            .filter(|col| &col.name == field)
            .map(|col| &col.ty)
            .next()
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