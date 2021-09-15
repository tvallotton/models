# sqlx-models
sqlx-modes is a work in progress implementation for a sql migration management tool for applications using sqlx.
Beware, this is still under development, and some API's may be broken in the future. 


# Basic Tutorial

install the CLI by running the following command: 
```
cargo install sqlx-models-cli
```

Now run the following command to create an environment file with the `DATABASE_URL` variable set: 
```
echo "DATABASE_URL=sqlite:/database.db" > .env
```
We now can create the database running the command: 
```
sqlx database create
```
This command will have created an sqlite file called `database.db`. 
You can now derive the `Models` trait on your structures, 
and `sqlx-models` will manage the migrations for you. For example, write at `src/main.rs`: 
```rust
#![allow(dead_code)]
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
struct Comment {
    #[primary_key]
    id: i32,
    #[foreign_key(User.id)]
    author: i32,
    #[foreign_key(Post.id)]
    post: i32,
}
fn main() {}
```

If you now run the following command, your migrations should be automatically created.
``` 
sqlx generate
```
The output should look like this: 
```
Generated 1631716729974/migrate user
Generated 1631716729980/migrate post
Generated 1631716729986/migrate comment
Generated 1631716729993/migrate postlike
Generated 1631716729998/migrate commentlike
```
You can check out the generated migrations at the `migrations/` folder. To commit this migrations you can execute the following command: 
```
sqlx migrate run
```
The output should look like this: 
```
Applied 1631716729974/migrate user (342.208µs)
Applied 1631716729980/migrate post (255.958µs)
Applied 1631716729986/migrate comment (287.792µs)
Applied 1631716729993/migrate postlike (349.834µs)
Applied 1631716729998/migrate commentlike (374.625µs)
```
If we later modify those structures in our application, we can generate new migrations to update the tables. 

## Avaibale Attributes
### primary_key
It's used to mark the primary key fo the table. 
```rust
    #[primary_key]
    id: i32, 
```
for tables with multicolumn primary keys, the following syntax is used: 
```rust
    #[primary_key(second_id)]
    first_id: i32, 
    second_id: i32, 
```

### foreign_key
It is used to mark a foreign key constraint. 
```rust
    #[foreign_key(User.id)]
    user: i32, 
```
It can also specify `on_delete` and `on_update` constraints: 
```rust
    #[foreign_key(User.id, on_delete="cascade"]
    user: i32, 
```

### default
It can be used to set a default for a table. 
```rust
    #[default = 0]
    is_admin: bool, 
```

### unique
It is used to mark a unique constraint. 
```rust
    #[unique]
    email: String, 
```
For multicolumn unique constraints the following syntax is used: 
```rust
    #[unique(hash)]
    username: String,
    hash: i32,
```
