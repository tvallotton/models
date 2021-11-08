pub struct Constraints(pub Vec<Constraint>);
#[derive(Debug)]
pub enum Constraint {
    ForeignKey(ForeignKey),
    Unique(Unique),
    Primary(Unique),
}
#[derive(Debug)]
pub struct NamedConstraint {
    pub name: String,
    pub field_name: Ident,
    pub constr: Constraint,
}
use std::fmt::Debug;

use crate::prelude::*;

#[derive(Default, Debug)]
pub struct Unique {
   pub columns: Vec<Ident>,
}
impl ForeignKey {
    fn into_tokens(&self, constr_name: &str, local_col: &Ident) -> TokenStream2 {
        let foreign_col = &self.column;
        let foreign_table = &self.foreign_table.get_ident();

        let on_update = self
            .on_update
            .clone()
            .map(|x| x.value())
            .unwrap_or_default();
        let on_delete = self
            .on_delete
            .clone()
            .map(|x| x.value())
            .unwrap_or_default();
        quote! {
            __models_table.constraints.push(
                ::models::private::constraint::foreign_key(
                    #constr_name,
                    stringify!(#local_col),
                    stringify!(#foreign_table),
                    stringify!(#foreign_col),
                    #on_delete,
                    #on_update,
                )
            );
            // Validation
            let _ = |__models_validation: #foreign_table| {
                __models_validation.#foreign_col;
            };
        }
    }
}

impl Unique {
    fn into_tokens(
        &self,
        constr_name: &str,
        ty: &Ident,
        field_name: &Ident,
        method: TokenStream2,
    ) -> TokenStream2 {
        let columns = self.columns.iter();
        let columns1 = self.columns.iter();

        quote! {
            __models_table.constraints.push(
                ::models::private::constraint::#method(
                    #constr_name,
                    &[stringify!(#field_name), #(stringify!(#columns)),*]
                )
            );
            let _ = |__models_validation: #ty| {
                #(__models_validation.#columns1;)*
            };
        }
    }
}

impl NamedConstraint {
    pub fn into_tokens(&self, ty: &Ident) -> TokenStream2 {
        match &self.constr {
            Constraint::ForeignKey(fk) => {
                let constr_name = self.constr_name(ty, &[fk.column.clone()], "foreign");
                fk.into_tokens(&constr_name, &self.field_name)
            }
            Constraint::Primary(pk) => {
                let constr_name = self.constr_name(ty, &pk.columns, "primary");
                pk.into_tokens(&constr_name, ty, &self.field_name, quote!(primary))
            }
            Constraint::Unique(u) => {
                let constr_name = self.constr_name(ty, &u.columns, "unique");
                u.into_tokens(&constr_name, ty, &self.field_name, quote!(unique))
            }
        }
    }

    pub fn constr_name(&self, ty: &Ident, cols: &[impl ToString], method: &str) -> String {
        let mut constr_name = String::new();
        constr_name += &ty.to_string().to_lowercase();
        constr_name += "_";
        constr_name += method;
        constr_name += "_";
        constr_name += &self.field_name.to_string();

        for col in cols.iter() {
            constr_name += "_";

            constr_name += &col.to_string();
        }
        constr_name
    }
}

impl Parse for Unique {
    fn parse(input: parse::ParseStream) -> Result<Self> {
        let mut out = Unique::default();
        let content;
        if input.is_empty() {
        } else {
            let _paren = parenthesized!(content in input);
            while !content.is_empty() {
                out.columns.push(content.parse().unwrap());
                if !content.is_empty() {
                    content.parse::<Token![,]>().unwrap();
                }
            }
        }
        Ok(out)
    }
}

pub struct ForeignKey {
    pub foreign_table: Path,
    pub column: Ident,
    on_delete: Option<LitStr>,
    on_update: Option<LitStr>,
}

impl std::fmt::Debug for ForeignKey {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}

impl Parse for ForeignKey {
    fn parse(input: parse::ParseStream) -> Result<Self> {
        let content;
        let _paren = parenthesized!(content in input);

        let foreign_table = content.parse::<Path>()?;
        content.parse::<Token![.]>()?;
        let mut on_delete = None;
        let mut on_update = None;
        let column = content.parse::<Ident>()?;
        while content.parse::<Token![,]>().is_ok() {
            let ident: Ident = content.parse()?;
            if ident == "on_delete" {
                content.parse::<Token![=]>()?;
                if on_delete.is_some() {
                    return Err(Error::new(ident.span(), "Expected a single `on_delete`."));
                }
                on_delete = Some(content.parse()?);
            } else if ident == "on_update" {
                content.parse::<Token![=]>()?;
                if on_update.is_some() {
                    return Err(Error::new(ident.span(), "Expected a single `on_update`."));
                }
                on_update = Some(content.parse()?);
            } else {
                return Err(Error::new(
                    ident.span(),
                    "Expected `on_delete` or `on_update`.",
                ));
            }
        }
        is_valid(&on_delete)?;
        is_valid(&on_update)?;
        Ok(ForeignKey {
            foreign_table,
            column,
            on_delete,
            on_update,
        })
    }
}

impl Constraints {
    pub fn from_attrs(attrs: &[Attribute]) -> Result<Self> {
        let mut out = vec![];
        for attr in attrs {
            let tokens = attr.tokens.clone().into();
            if attr.path.is_ident("foreign_key") {
                out.push(Constraint::ForeignKey(parse(tokens)?));
            } else if attr.path.is_ident("unique") {
                out.push(Constraint::Unique(parse(tokens)?));
            } else if attr.path.is_ident("primary_key") {
                out.push(Constraint::Primary(parse(tokens)?));
            }
        }
        Ok(Constraints(out))
    }
}

impl Constraint {
    pub fn column_names(&self) -> Vec<Ident> {
        match &self {
            Constraint::Primary(primary) => primary.columns.to_vec(),
            Constraint::ForeignKey(foreign) => vec![foreign.column.clone()],
            Constraint::Unique(unique) => unique.columns.to_vec(),
        }
    }

    pub fn method(&self) -> TokenStream2 {
        match self {
            Constraint::Primary(_) => {
                quote!(primary)
            }
            Constraint::ForeignKey(_) => {
                quote!(foreign_key)
            }
            Constraint::Unique(_) => {
                quote!(unique)
            }
        }
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
