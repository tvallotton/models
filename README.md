# sqlx-models
sqlx-modes is a working progress implementation for a sql migration manangement tool for applications using sqlx.



# Basic Tutorial
install the CLI by running the following command: 
```
cargo install sqlx-models-cli
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
