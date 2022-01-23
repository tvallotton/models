use crate::{
    prelude::*,
    private::scheduler::{
        driver::Driver,
        table::Table,
    },
};

// macro_rules! create_table {
//     (name:ident, {(name:ident ty:ident),*,?}) => {
//         Table {
//             name: ObjectName(vec![stringify!(name)]),
//             if_not_exists: false,
//             or_replace: false,
//             columns:

//     };
// }

fn table(sql: &str) -> Table {
    parse_sql(sql).unwrap().pop().unwrap().try_into().unwrap()
}

#[cfg(test)]
fn init() {
    match rand::random::<u32>() % 3 {
        | 0 => std::env::set_var("DATABASE_URL", "sqlite://db"),
        | 1 => std::env::set_var("DATABASE_URL", "postgres://db"),
        | 2 => std::env::set_var("DATABASE_URL", "mysql://db"),
        | _ => unreachable!(),
    }
}

#[test]
fn test() {
    init();
    let driver = Driver::_from_sql(
        "CREATE TABLE foo (
       id int primary key
    );",
    );
    // driver.migrate2();
}
