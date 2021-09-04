pub struct Constraints(pub Vec<Constraint>);

pub enum Constraint {
    ForeignKey(ForeignKey),
    Unique(Unique),
    Primary(Unique),
}

pub struct NamedConstraint {
    pub name: String,

    pub constr: Constraint,
}
use crate::prelude::*;

#[derive(Default)]
struct Unique {
    columns: Vec<Ident>,
}
impl ForeignKey {
    fn into_tokens(&self, constr_name: &str, ty: Ident, local_col: Ident) -> TokenStream2 {
        let foreign_col = &self.column;
        let foreign_table = &self.foreign_table;
        let on_update = self
            .on_update
            .clone()
            .map(|x| x.value())
            .unwrap_or(String::new());
        let on_delete = self
            .on_delete
            .clone()
            .map(|x| x.value())
            .unwrap_or(String::new());
        quote! {
            __sqlx_models_table.constraints.push(
                ::sqlx_models::private::constraint::foreign_key(
                    stringify!(#constr_name),
                    stringify!(#local_col),
                    stringigy!(#foreign_table),
                    stringify!(#foreign_col),
                    #on_delete,
                    #on_update,
                )
            );
            // Validation
            let _ = |__sqlx_models_validation: #foreign_table| {
                __sqlx_models_validation.#foreign_col;
            };
        }
    }
}

impl Unique {
    fn into_tokens(
        &self,
        constr_name: &str,
        ty: Ident,
        field_name: Ident,
        method: Ident,
    ) -> TokenStream2 {
        let columns = self.columns.iter();
        let columns1 = self.columns.iter();

        quote! {
            __sqlx_models_table.constraints.push(
                ::sqlx_models::private::constraint::#method(
                    #constr_name,
                    &[stringify!(#field_name), #(stringify!(#columns)),*]
                )
            );
            let _ = |__sqlx_models_validation: #ty| {
                #(__sqlx_models_validation.#columns1;)*
            };
        }
    }
}

impl NamedConstraint {
    fn into_tokens(&self, constr_name: &str, ty: Ident, field_name: Ident) -> TokenStream2 {
        let constr_name = &self.name;

        match &self.constr {
            Constraint::ForeignKey(fk) => fk.into_tokens(&constr_name, ty, field_name),
            // Constraint::Primary(pk) => pk.into_tokens(&constr_name, ty, field_name,  ),
            // Constraint::Unique(u) => u.into_tokens(struct_name, field_name),
            _ => todo!(),
        }
    }
}

impl Parse for Unique {
    fn parse(input: parse::ParseStream) -> Result<Self> {
        let mut out = Unique::default();
        let content;
        if input.is_empty() {
            Ok(out)
        } else {
            let _paren = parenthesized!(content in input);
            while !content.is_empty() {
                out.columns.push(content.parse().unwrap());
                if !content.is_empty() {
                    content.parse::<Token![,]>().unwrap();
                }
            }
            Ok(out)
        }
    }
}

struct ForeignKey {
    foreign_table: Path,
    column: Ident,
    on_delete: Option<LitStr>,
    on_update: Option<LitStr>,
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
        while let Ok(_) = content.parse::<Token![,]>() {
            let ident: Ident = content.parse()?;
            if ident == "on_delete" {
                content.parse::<Token![=]>()?;
                if on_delete.is_some() {
                    Err(Error::new(ident.span(), "Expected a single `on_delete`."))?
                }
                on_delete = Some(content.parse()?);
            } else if ident == "on_update" {
                content.parse::<Token![=]>()?;
                if on_update.is_some() {
                    Err(Error::new(ident.span(), "Expected a single `on_update`."))?
                }
                on_update = Some(content.parse()?);
            } else {
                Err(Error::new(
                    ident.span(),
                    "Expected `on_delete` or `on_update`.",
                ))?
            }
        }
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
            } else if attr.path.is_ident("unique") {
                out.push(Constraint::Primary(parse(tokens)?));
            }
        }
        Ok(Constraints(out))
    }
}

impl Constraint {
    pub fn column_names(&self) -> Vec<Ident> {
        match &self {
            &Constraint::Primary(primary) => primary.columns.iter().cloned().collect(),
            &Constraint::ForeignKey(foreign) => vec![foreign.column.clone()],
            &Constraint::Unique(unique) => unique.columns.iter().cloned().collect(),
        }
    }
    pub fn method(&self) -> TokenStream2 {
        match self {
            &Constraint::Primary(_) => {
                quote!(primary)
            }
            &Constraint::ForeignKey(_) => {
                quote!(foreign_key)
            }
            &Constraint::Unique(_) => {
                quote!(unique)
            }
        }
    }
}

#[test]
fn func() {
    let x = "asd";
    println!("{}", quote!(#x));
}
