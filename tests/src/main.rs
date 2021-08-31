#[allow(dead_code)]
#[allow(unused_imports)]
use sqlx_models::Model;

#[derive(Model)]
struct User {
    #[primary_key(second_id)]
    id: i32,
    second_id: i32,
}

#[test]
fn __generate_migrations_User() {
    let migrations = ::sqlx_models::Migration::new::<User>();
    migrations.run();
}

fn main() {}
