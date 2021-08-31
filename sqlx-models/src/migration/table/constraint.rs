use crate::prelude::*;
pub enum Constraint {
    Unique {
        name: Option<Ident>,
        columns: Vec<Ident>,
        is_primary: bool,
    },
    ForeignKey {
        name: Option<Ident>,
        foreign_table: ObjectName,
        foreign_columns: Vec<Ident>,
        local_columns: Vec<Ident>,
    },
}

impl Constraint {
    pub(crate) fn name(&self) -> &Option<Ident> {
        match self {
            Constraint::ForeignKey { name, .. } => name,
            Constraint::Unique { name, .. } => name,
        }
    }


    pub fn primary(fields: Vec<&str>) -> Self {
        let name = None; 
        let mut columns = vec![];
        for field in fields {
            columns.push(Ident::new(field)); 
        }
        Self::Unique {
            name, columns, is_primary: true
        }
    }
}

impl From<TableConstraint> for Constraint {
    fn from(constr: TableConstraint) -> Constraint {
        use TableConstraint::*;
        match constr {
            ForeignKey {
                name,
                columns,
                foreign_table,
                referred_columns,
            } => Constraint::ForeignKey {
                name,
                foreign_table,
                foreign_columns: referred_columns,
                local_columns: columns,
            },
            Unique {
                name,
                columns,
                is_primary,
            } => Constraint::Unique {
                name,
                columns,
                is_primary,
            },
            _ => panic!(""),
        }
    }
}
