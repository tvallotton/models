



fn find_by<Model, Field>(column: Field, value: Field::Type) -> Result<Model, super::error::Error>
where
    Field: Find<Model>,
{
    Field::find_by(column, value)
}

trait Find<Model> {
    type Type;
    fn find_by(column: Self, value: Self::Type) -> Result<Model, super::error::Error>;
}
