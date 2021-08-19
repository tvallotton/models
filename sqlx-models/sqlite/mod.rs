use crate::migrations::*;
use crate::prelude::*;

use quote::quote;

pub fn derive(input: DeriveInput) -> TokenStream {
    let dialect = &dialect::SQLiteDialect {};
    let migrations = parse_migrations(dialect);
    let target = get_target(input);

    todo!()
}


use syn::Fields;
fn get_target(input: DeriveInput) -> Table {
    let ident = input.ident;
    use Data::*;
    use Fields::*;
    assert!(input.generics.params.is_empty(), "Models cannot be generic");
    match input.data {
        Enum(_) | Union(_) => panic!("Enums and unions are not supported"),
        Struct(DataStruct {
            struct_token,
            fields,
            semi_token,
        }) => derive_struct(ident, fields),
    }
}

fn derive_struct(ident: syn::Ident, fields: Fields) -> Table {
    let name = ident.to_string();
    let mut table: Table = name.into();
    for field in fields.into_iter() {
        let name = field
            .ident
            .expect("Struct tuples are not supported.")
            .to_string();
        let name = ast::Ident::new(name);
        let data_type = get_sqlite_type(field.ty);
        let collation = None;
        let mut options = vec![];

        let col = ColumnDef {
            name,
            collation,
            data_type,
            options,
        };

        for attr in field.attrs {
            if attr.path.is_ident("foreign_key") {
                col.options.push(ColumnOptionDef { name: None ,
                    
                    option: ForeignKey {
        foreign_table: ObjectName,
        referred_columns: Vec<Ident>,
        on_delete: Option<ReferentialAction>,
        on_update: Option<ReferentialAction>,
    },})
            }
        }
    }

    todo!()
}

fn foreign_key(attr: Attribute, field: &Field) -> ColumnOptionDef {
    ColumnOptionDef {

    }
    
    

}


fn get_sqlite_type(ty: Type) -> DataType {
    use Type::*;
    let string = ty.to_token_stream().to_string();
    
    if "bool" == string {
        DataType::Boolean
    } else if "i8" == string
        || "i16" == string
        || "u8" == string
        || "u16" == string
        || "u32" == string
    {
        DataType::Int
    } else if "i64" == string || "u64" == string {
        DataType::BigInt
    } else if "f32" == string || string == "f64" {
        DataType::Double
    } else if "String" == string {
        DataType::Text
        // else if {}
    } else {
        panic!("the type `{}` is not supported.", path.to_token_stream())
    }
}

#[test]
fn maiN() {
    let t1 = quote![Vec<i32>];
    let t2 = quote![i32];

    println!("{}", t1.to)
}
