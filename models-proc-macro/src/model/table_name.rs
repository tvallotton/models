use crate::prelude::*;

pub struct TableName {
    pub name: String,
}

impl TableName {
    pub fn try_from_attr(attr: &Attribute) -> Option<Result<TableName>> {
        if attr.path.is_ident("table_name") {
            Some(parse(attr.tokens.clone().into()))
        } else {
            None
        }
    }
}

impl Parse for TableName {
    fn parse(input: parse::ParseStream) -> Result<Self> {
        let content;
        let _paren = parenthesized!(content in input);
        let name = content.parse::<LitStr>()?.value();
        Ok(Self { name })
    }
}
