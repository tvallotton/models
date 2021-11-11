use crate::prelude::*;

pub struct UniqueGetter<'a> {
    table_name: &'a Ident,
    unique: &'a constraint::Unique,
    columns: HashMap<&'a Ident, &'a Type>,
}
