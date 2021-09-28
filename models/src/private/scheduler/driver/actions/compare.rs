pub use crate::prelude::*;
pub use collections::HashSet;

pub(crate) trait Compare {
    fn bodies_are_equal(&self, other: &Self) -> bool;
    fn name(&self) -> Result<String>;
    fn are_modified(&self, other: &Self) -> bool {
        let names = self.names_are_equal(other);
        let bodies = self.bodies_are_equal(other);
        names && !bodies
    }
    fn names_are_equal(&self, other: &Self) -> bool {
        let first = match self.name() {
            Ok(name) => name,
            Err(_) => return false,
        };
        let second = match other.name() {
            Ok(name) => name,
            Err(_) => return false,
        };
        

        
        first == second
    }

    fn are_equal(&self, other: &Self) -> bool {
        
        self.names_are_equal(other) && self.bodies_are_equal(other)
    }

    fn ident(&self) -> Ident;
}

impl Compare for Column {
    fn ident(&self) -> Ident {
        self.name.clone()
    }
    fn name(&self) -> Result<String, Error> {
        Ok(self.name.to_string().to_lowercase())
    }

    fn bodies_are_equal(&self, other: &Self) -> bool {
        let type1 = self.r#type.to_string().to_lowercase();
        let type2 = self.r#type.to_string().to_lowercase();

        type1 == type2 && {
            let h1 = self
                .options
                .iter()
                .map(ToString::to_string)
                .map(|string| string.to_lowercase())
                .collect::<HashSet<_>>();
            let h2 = other
                .options
                .iter()
                .map(ToString::to_string)
                .map(|string| string.to_lowercase())
                .collect::<HashSet<_>>();
            h1 == h2
        }
    }
}

impl Compare for TableConstraint {
    fn ident(&self) -> Ident {
        use TableConstraint::*;
        match self {
            Unique(ast::Unique { name, .. }) => name,
            ForeignKey(ast::ForeignKey { name, .. }) => name,
            Check(ast::Check { name, .. }) => name,
        }
        .clone()
        .unwrap()
    }
    fn name(&self) -> Result<String, Error> {
        use TableConstraint::*;
        match self {
            Unique(ast::Unique { name, .. }) => name,
            ForeignKey(ast::ForeignKey { name, .. }) => name,
            Check(ast::Check { name, .. }) => name,
        }
        .as_ref()
        .ok_or_else(|| error!("anonymous constraints are not supported."))
        .map(|name| name.to_string().to_lowercase())
    }

    fn bodies_are_equal(&self, other: &Self) -> bool {
        use TableConstraint::*;
        match (self, other) {
            (Unique(u0), Unique(u1)) => {
                u0.is_primary == u1.is_primary && {
                    let cols0 = u0
                        .columns
                        .iter()
                        .map(ToString::to_string)
                        .map(|str| str.to_lowercase())
                        .collect::<HashSet<_>>();
                    let cols1 = u1
                        .columns
                        .iter()
                        .map(ToString::to_string)
                        .map(|str| str.to_lowercase())
                        .collect::<HashSet<_>>();
                    cols0 == cols1
                }
            }
            (ForeignKey(f0), ForeignKey(f1)) => {
                f1.on_delete == f0.on_delete
                    && f1.on_update == f0.on_update
                    && {
                        let cols0 = f1
                            .referred_columns
                            .iter()
                            .map(ToString::to_string)
                            .map(|str| str.to_lowercase())
                            .collect::<HashSet<_>>();
                        let cols1 = f0
                            .referred_columns
                            .iter()
                            .map(ToString::to_string)
                            .map(|str| str.to_lowercase())
                            .collect::<HashSet<_>>();
                        cols0 == cols1
                    }
                    && {
                        let name0 = f0.foreign_table.to_string().to_lowercase();
                        let name1 = f1.foreign_table.to_string().to_lowercase();
                        name0 == name1
                    }
                    && {
                        let cols0 = f0
                            .columns
                            .iter()
                            .map(ToString::to_string)
                            .map(|str| str.to_lowercase())
                            .collect::<HashSet<_>>();
                        let cols1 = f1
                            .columns
                            .iter()
                            .map(ToString::to_string)
                            .map(|str| str.to_lowercase())
                            .collect::<HashSet<_>>();
                        cols0 == cols1
                    }
            }
            _ => false,
        }
    }
}
