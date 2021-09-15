use sqlx_models::Model;



#[derive(Model)]
struct User {
    #[primary_key]
    id: i32,
    #[unique]
    email: String,
    password: String,
    #[default = 0]
    is_admin: bool,
}

#[derive(Model)]
struct PostLike {
    #[foreign_key(User.id)]
    #[primary_key(post_id)]
    user_id: i32,
    #[foreign_key(Post.id)]
    post_id: i32,
}

#[derive(Model)]
struct CommentLike {
    #[foreign_key(User.id)]
    #[primary_key(comment)]
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
fn main() {

}