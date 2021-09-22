#![allow(dead_code)]

use sqlx_models::Model;

use sqlx::types::Json;

#[derive(Model, sqlx::FromRow, Debug)]
struct User {
    #[primary_key]
    id: i32,
    #[unique]
    email: String,
    password: String,
    #[default(0)]
    is_admin: bool, 
    friends: Json<Vec<i32>>
}

#[derive(Model)]
struct PostLike {
    #[foreign_key(User.id)]
    #[primary_key(post_id)]
    user_id: i32,
    #[foreign_key(Post.id)]
    post_id: i32,
    #[unique]
    friends: Json<Vec<i32>>
}

#[derive(Model)]
struct CommentLike {
    #[foreign_key(User.id)]
    #[primary_key(comment)]
    user: i32,
    #[foreign_key(Comment.id)]
    comment: i32,
    #[default(false)]
    is_dislike: bool,
}

#[derive(Model)]
struct Post {
    #[primary_key]
    id: i32,
    #[foreign_key(User.id)]
    author_: String,
    #[default("<UNTITLED POST>")]
    title: String,
    content: String,
    tags: Json<Vec<String>>
}

#[derive(Model)]
struct Comment {
    #[primary_key]
    id: i32,
    #[foreign_key(User.id)]
    author: i32,
    #[foreign_key(Post.id)]
    post: i32,
}
#[tokio::main]
async fn main() {
    use sqlx::Connection;
    let conn = sqlx::SqlitePool::connect("database.db").await.unwrap();

    let users: Vec<User> = sqlx::query_as("select * from user;")
        .fetch_all(&conn)
        .await
        .unwrap();
    println!("{:?}", users);
    let user = User {
        id: 0,
        is_admin: true,
        email: "tvallotton@uc.cl".into(),
        password: "password".into(),
        friends: Json(vec![1, 2, 3]),
    };


    
}
