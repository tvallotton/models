#[allow(dead_code)]
#[allow(unused_imports)]
use sqlx_models::Model;

#[derive(Model)]
struct User_ {
    #[primary_key]
    id: i32,
    #[unique]
    email: String,
    password: String,
    #[default = false]
    is_admin: String,
}

#[derive(Model)]
struct Post_ {
    #[primary_key]
    id: i32,
    #[foreign_key(User_.id)]
    author: i32,
    #[default = "<UNTITLED POST>"]
    title: String,
    content: String,
}

fn main() {}
