pub(crate) use crate::prelude::*;
pub use column::Column;
pub use constraint::{Constraint, *};
use Data::*;

mod column;
pub mod constraint;

pub struct Model {
    pub name: Ident,
    data: DataStruct,
    pub columns: Vec<Column>,
    pub constraints: Vec<Constraint>,
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

impl Model {
    // include
    fn init(&mut self) -> Result<()> {
        for field in &self.data.fields {
            let col_name = field.ident.clone().unwrap();
            let constrs = Constraint::from_attrs(&field.attrs, &field)?;
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
}
