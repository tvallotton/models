use crate::private::scheduler::{
    driver::Driver,
    table::Table,
};

fn new_driver<const N: usize>(files: [(&'static str, &'static str); N]) -> Driver {
    todo!()
}

#[test]
fn test() {
    let driver = new_driver([(
        "user",
        "create table User {

        };",
    )]);
}
