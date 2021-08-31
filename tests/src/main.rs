#[allow(dead_code)]
#[allow(unused_imports)]
use sqlx_models::Model;


struct User {
    id: i32
}


#[derive(Model)]
struct Post {
    #[primary_key]
    id: i32,
    #[foreign_key(User.id)]
    author: i32,
}



fn main() {}
