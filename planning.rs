//! This is a schetch of the intended API.
//!
//!

use models::{find_by, Default, Model, types::AutoIncrement};

#[derive(Model, Default)]
pub struct User {
    #[primary_key]
    pub id: AutoIncrement,
    pub email: String,
    pub password: String,
    pub is_admin: bool,
}


#[derive(Model, Default)] 
pub struct Post {
    #[primary_key]
    pub id: AutoIncrement,
    pub title: String,
    pub body: String,
    #[foreign_key(User.id, on_delete = "cascade")]
    pub author_id: i32, 
}


let user = User::find(0).await?;
let users = User::get_all().await?;
let user = User::find_by(User::email, "example@uc.cl").await?;
let posts = Post::find_by(Post::title, "Example").await?;
let posts = user.posts().await?; 
let post = posts[0].author().await?; 

if user.changed().await? {
    user.save().await?;
}


User::select().filter(Post)

Post::filter_by(User::id << 20).

filter_by(Post::title == "Example").await?;