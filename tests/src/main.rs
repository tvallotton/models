#[allow(dead_code)]
#[allow(unused_imports)]
use sqlx_models::Model;

#[derive(Model)]
struct User_ {
    #[primary_key]
    id: i32,
    
    email: String, 
    password: String,

}

#[derive(Model)]
struct Post {
    #[primary_key]
    id: i32,
    #[foreign_key(User_.id)]
    author: i32,
    title: String,
}

fn main() {}
