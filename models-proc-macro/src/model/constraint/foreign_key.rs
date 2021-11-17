use crate::prelude::*;

#[derive(Clone)]
pub struct ForeignKey {
    pub foreign_table: Path,
    pub foreign_column: Ident,
    pub local_column: Ident,
    pub getter: Option<Ident>,
    pub on_delete: Option<LitStr>,
    pub on_update: Option<LitStr>,
}

impl ForeignKey {
    pub(super) fn new(tokens: TokenStream, local_column: Ident) -> Result<Self> {
        let fk: ParseForeignKey = parse(tokens)?;
        Ok(Self {
            foreign_column: fk.foreign_column,
            foreign_table: fk.foreign_table,
            local_column,
            getter: fk.getter,
            on_delete: fk.on_delete,
            on_update: fk.on_update,
        })
    }

    pub fn on_delete(&self) -> String {
        self.on_delete
            .iter()
            .map(|x| x.value())
            .next()
            .unwrap_or_default()
    }
    pub fn on_update(&self) -> String {
        self.on_update
            .iter()
            .map(|x| x.value())
            .next()
            .unwrap_or_default()
    }

    pub fn getter(&self) -> Ident {
        if let Some(getter) = self.getter.clone() {
            getter
        } else {
            let name = self.local_column.to_string();
            if name.ends_with("_id") {
                let len = name.chars().count();
                let name: String = name.chars().take(len - 3).collect();
                Ident::new(&name, self.local_column.span())
            } else {
                self.local_column.clone()
            }
        }
    }
}

pub(super) struct ParseForeignKey {
    pub foreign_table: Path,
    pub foreign_column: Ident,
    pub getter: Option<Ident>,
    pub on_delete: Option<LitStr>,
    pub on_update: Option<LitStr>,
}

impl Parse for ParseForeignKey {
    fn parse(input: parse::ParseStream) -> Result<Self> {
        let content;
        let mut on_delete = None;
        let mut on_update = None;
        let mut getter = None;
        let _paren = parenthesized!(content in input);

        let foreign_table = content.parse()?;
        content.parse::<Token![.]>()?;
        let foreign_column = content.parse::<Ident>()?;
        while content.parse::<Token![,]>().is_ok() {
            let ident: Ident = content.parse()?;
            if ident == "on_delete" {
                content.parse::<Token![=]>()?;
                if on_delete.is_some() {
                    return Err(Error::new(
                        ident.span(),
                        "Expected a single value for `on_delete`.",
                    ));
                }
                on_delete = Some(content.parse()?);
            } else if ident == "on_update" {
                content.parse::<Token![=]>()?;
                if on_update.is_some() {
                    return Err(Error::new(
                        ident.span(),
                        "Expected a single value for `on_update`.",
                    ));
                }
                on_update = Some(content.parse()?);
            } else if ident == "getter" {
                content.parse::<Token![=]>()?;
                if getter.is_some() {
                    return Err(Error::new(
                        ident.span(),
                        "Expected a single value for `getter`.",
                    ));
                }
                let getter_lit: LitStr = content.parse()?;
                let span = getter_lit.span();
                let ident = getter_lit.value();
                getter = Some(Ident::new(&ident, span));
            } else {
                return Err(Error::new(
                    ident.span(),
                    "Expected `getter`, `on_delete` or `on_update`.",
                ));
            }
        }
        is_valid(&on_delete)?;
        is_valid(&on_update)?;
        Ok(ParseForeignKey {
            foreign_table,
            foreign_column,
            on_delete,
            on_update,
            getter,
        })
    }
}

fn is_valid(on_delete: &Option<LitStr>) -> Result<()> {
    if let Some(string) = on_delete {
        if matches!(
            &*string.value(),
            "cascade" | "set null" | "restrict" | "no action"
        ) {
            return Ok(());
        } else {
            return Err(Error::new(
                string.span(),
                format!(
                    "invalid referential integrity constraint. Found {:?}, expected one of: {:?}",
                    string.value(),
                    ["restrict", "cascade", "set null", "no action"],
                ),
            ));
        }
    }
    Ok(())
}
