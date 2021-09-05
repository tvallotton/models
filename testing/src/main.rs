#[allow(dead_code)]
#[allow(unused_imports)]
use sqlx_models::Model;
#[derive(Model)]
struct User {
    #[primary_key]
    id: i32,
    #[unique]
    email: String,
    password: String,

}


#[derive(Model)]
struct Post {
  #[primary_key]
  id: i32,
  #[foreign_key(User.id)]
  author: i32, 
  title: String,
  content: String, 
}

fn main() {}
