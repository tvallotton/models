pub(crate) use crate::prelude::*;
pub use column::Column;
pub use constraint::{Constraint, *};
pub use has_many::HasMany;
pub use has_one::HasOne;
use table_name::TableName;
use Data::*;
mod column;
pub mod constraint;
mod has_many;
mod has_one;
mod table_name;

/// Describes the table information as parsed from
/// the input data. This structures main purpose is to
/// be consumed by other structures for the code generation.
pub struct Model {
    /// Name of the struct.
    pub model_name: Ident,
    /// The name of the table. It can be overriden with
    /// the marker #[model(table_name = "overriden name")].
    /// Otherwise, it defaults to the structures name in
    /// lowercase.
    pub table_name: String,

    /// This field referes to foreign key constraints
    /// in other models that reffer to the primary
    /// key of the current table.
    pub has_many: Vec<HasMany>,
    pub has_one: Vec<HasOne>,

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
        let attrs = input.attrs;
        let model_name = input.ident;
        let table_name = model_name.to_string().to_lowercase();
        match input.data {
            Struct(data) => {
                let mut model = Self {
                    model_name,
                    table_name,
                    has_many: Default::default(),
                    has_one: Default::default(),
                    columns: Default::default(),
                    constraints: Default::default(),
                };
                model.init(data, attrs)?;
                Ok(model)
            }
            _ => panic!("Sql models have to be structs, enums and unions are not supported."),
        }
    }
}

impl Model {
    fn init(&mut self, data: DataStruct, attrs: Vec<Attribute>) -> Result<()> {
        self.init_columns_and_constraints(data)?;
        self.init_top_level_attrs(attrs)?;

        Ok(())
    }
    /// initialize has_many, has_one, and table_name.
    fn init_top_level_attrs(&mut self, attrs: Vec<Attribute>) -> Result<()> {
        for attr in attrs {
            if let Some(has_many) = HasMany::try_from_attr(&attr) {
                self.has_many.push(has_many?);
            } else if let Some(has_one) = HasOne::try_from_attr(&attr) {
                self.has_one.push(has_one?);
            } else if let Some(table_name) = TableName::try_from_attr(&attr) {
                self.table_name = table_name?.name;
            }
        }
        Ok(())
    }

    fn init_columns_and_constraints(&mut self, data: DataStruct) -> Result<()> {
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
