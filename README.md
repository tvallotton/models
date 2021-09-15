# sqlx-models
sqlx-modes is a work in progress implementation for a sql migration manangement tool for applications using sqlx.



# Basic Tutorial

install the CLI by running the following command: 
```
cargo install sqlx-models-cli
```

Now create a file called `.env` with the following content: 
```
DATABASE_URL=sqlite:/database.db
```
We now can create the database running the command: 
```
sqlx database create
```
now write in `src/main.rs`: 
```rust
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
fn main() {}
```

If you now run the following command, your migrations should be automatically created (make sure your code compiles).
``` 
sqlx generate
```


the output generated should look something like this 
```sql
-- at <TIMESTAMP>_user.sql. 
CREATE TABLE user (
    id INTEGER NOT NULL,
    email TEXT NOT NULL,
    PASSWORD TEXT NOT NULL,
    is_admin BOOLEAN NOT NULL DEFAULT 0,
    CONSTRAINT user_primary_id PRIMARY KEY (id),
    CONSTRAINT user_unique_email UNIQUE (email)
);
-- at <TIMESTAMP>_post.sql. 
CREATE TABLE post (
    id INTEGER NOT NULL,
    author_ TEXT NOT NULL,
    title TEXT NOT NULL DEFAULT '<UNTITLED POST>',
    content TEXT NOT NULL,
    CONSTRAINT post_primary_id PRIMARY KEY (id),
    CONSTRAINT post_foreign_author__id FOREIGN KEY (author_) REFERENCES User(id)
);

-- at <TIMESTAMP>_comment.sql. 
CREATE TABLE COMMENT (
    id INTEGER NOT NULL,
    author INTEGER NOT NULL,
    post INTEGER NOT NULL,
    CONSTRAINT comment_primary_id PRIMARY KEY (id),
    CONSTRAINT comment_foreign_author_id FOREIGN KEY (author) REFERENCES User(id),
    CONSTRAINT comment_foreign_post_id FOREIGN KEY (post) REFERENCES Post(id)
);
-- at <TIMESTAMP>_commentlike.sql. 

CREATE TABLE commentlike (
    user INTEGER NOT NULL,
    COMMENT INTEGER NOT NULL,
    is_dislike BOOLEAN NOT NULL DEFAULT false,
    CONSTRAINT commentlike_foreign_user_id FOREIGN KEY (user) REFERENCES User(id),
    CONSTRAINT commentlike_primary_user_comment PRIMARY KEY (user, COMMENT),
    CONSTRAINT commentlike_foreign_comment_id FOREIGN KEY (COMMENT) REFERENCES COMMENT(id)
);

-- at <TIMESTAMP>_postlike.sql. 
CREATE TABLE postlike (
    user_id INTEGER NOT NULL,
    post_id INTEGER NOT NULL,
    CONSTRAINT postlike_foreign_user_id_id FOREIGN KEY (user_id) REFERENCES User(id),
    CONSTRAINT postlike_primary_user_id_post_id PRIMARY KEY (user_id, post_id),
    CONSTRAINT postlike_foreign_post_id_id FOREIGN KEY (post_id) REFERENCES Post(id)
);
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
It can also specify on_delete and on_update constraints: 
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
