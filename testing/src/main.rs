#[allow(dead_code)]
#[allow(unused_imports)]
use sqlx_models::Model;

use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::thread::sleep;
use std::time::Duration;
static X: Lazy<Mutex<i32>> = Lazy::new(|| Mutex::new(0));

#[derive(Model)]
struct User {
    #[primary_key]
    id: i32,
    #[unique]
    email: String,
    password: String,
    #[default = false]
    is_admin: String,
}

#[derive(Model)]
struct PostLike {
    #[foreign_key(User.id)]
    #[primary_key(post)] // both user_id and post_id are primary keys.
    user_id: i32,
    #[foreign_key(Post.id)]
    post_id: i32,
}

#[derive(Model)]
struct CommentLike {
    #[primary_key(comment)]
    #[foreign_key(User.id)]
    user: i32,
    #[foreign_key(Comment.id)]
    comment: i32,
    #[default = false]
    is_dislike: bool,
}

#[derive(Model)]
struct Post {
    #[primary_key]
    id: i32,
    #[foreign_key(User.id)]
    author_: String,
    #[default = "<UNTITLED POST>"]
    title: String,
    content: String,
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

fn main() {}
