use model::ForeignKey;
use crate::prelude::*;

pub struct ForeignGetter<'a> {
    table_name: &'a Ident,
    foreign_key: &'a ForeignKey,
}
