use crate::prelude::*;
mod default;

use default::*;

pub struct Column {
    pub name: Ident,
    pub ty: Type,
    pub default: Option<DefaultExpr>,
}

impl Column {
    pub fn new(field: &Field) -> Result<Self> {
        let ty = field.ty.clone();
        let default = Self::get_default(field.attrs.clone())?;
        let name = field.ident.clone().unwrap();
        Ok(Self { ty, default, name })
    }

    fn get_default(attrs: Vec<Attribute>) -> Result<Option<DefaultExpr>> {
        for attr in attrs {
            if attr.path.is_ident("default") {
                return Ok(Some(syn::parse(attr.tokens.into())?));
            }
        }
        Ok(None)
    }
}
