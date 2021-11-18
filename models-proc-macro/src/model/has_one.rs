use crate::prelude::*;
use inflector::cases::snakecase::to_snake_case;
pub struct HasOne {
    pub foreign_table: Path,
    pub foreign_column: Ident,
    getter: Option<Ident>,
}

impl Parse for HasOne {
    fn parse(input: parse::ParseStream) -> Result<Self> {
        let content;
        let _paren = parenthesized!(content in input);

        let foreign_table = content.parse()?;
        let foreign_column = content.parse()?;
        let mut getter = None;
        content.parse::<Token![.]>()?;

        if content.parse::<Token![,]>().is_ok() {
            let getter_str = content.parse::<LitStr>()?;
            let getter_ident = Ident::new(&getter_str.value(), getter_str.span());
            getter = Some(getter_ident);
        }
        Ok(Self {
            foreign_table,
            foreign_column,
            getter,
        })
    }
}

impl HasOne {
    pub fn getter(&self) -> Ident {
        if let Some(getter) = self.getter.clone() {
            getter
        } else {
            let foreign_table = &self.foreign_table.get_ident().unwrap();
            let getter = to_snake_case(&foreign_table.to_string());
            Ident::new(&getter, foreign_table.span().unwrap())
        }
    }

    pub fn try_from_attr(attr: &Attribute) -> Option<Result<Self>> {
        if attr.path.is_ident("has_many") {
            Some(parse(attr.tokens.clone().into()))
        } else {
            None
        }
    }
}
