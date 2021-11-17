pub(crate) use crate::prelude::*;
pub use column::Column;
pub use constraint::{Constraint, *};
use Data::*;

mod column;
pub mod constraint;

/// Describes the table information as parsed from
/// the input data. This structures main purpose is to
/// be consumed by other structures for the code generation.
pub struct Model {
    /// Name of the type. 
    pub name: Ident,
    /// the name of the table. It can be overriden with
    /// the marker #[model(table_name = "overriden name")].
    /// Otherwise, it defaults to the structures name in
    /// lowercase.
    pub table_name: String,

    /// This holds only the top level foreign keys attributes, that is, foreign key 
    /// constraints defined on other tables, not on this table.
    /// The foreign key constraints defined for this table are 
    /// stored on the constraints field.
    pub foreign_keys: Vec<ForeignKey>,

    /// this field contains the columns for the table. 
    pub columns: Vec<Column>,

    /// This field contains all the constraints associated for this table. 
    /// This also includes foreign_key attributes defined on top of columns. 
    /// For foreign key attributes defined on top of the whole struct see the field 
    /// [`Self::foreign_keys`].
    pub constraints: Vec<Constraint>,
}

impl Parse for Model {
    fn parse(input: parse::ParseStream) -> Result<Self> {
        let input: DeriveInput = input.parse()?;
        let name = input.ident;
        let table_name = name.to_string().to_lowercase();
        match input.data {
            Struct(data) => {
                let mut model = Self {
                    name,
                    table_name,
                    foreign_keys: Default::default(),
                    columns: Default::default(),
                    constraints: Default::default(),
                };
                model.init(data)?;
                Ok(model)
            }
            _ => panic!("Sql models have to be structs, enums and unions are not supported."),
        }
    }
}

impl Model {
    // in populates the columns and the constraints
    fn init(&mut self, data: DataStruct) -> Result<()> {
        for field in &data.fields {
            let constrs = Constraint::from_field(&field, &field.attrs)?;
            self.constraints.extend(constrs);

            let column = Column::new(field)?;
            self.columns.push(column);
        }
        Ok(())
    }
    /// Getter method to retrieve the type of a column from
    //  it's field name.
    pub fn field_type(&self, field: &Ident) -> Option<&Type> {
        self.columns
            .iter()
            .filter(|col| &col.name == field)
            .map(|col| &col.ty)
            .next()
    }
}
