//! # Models
//! Models is a SQL migration management tool. It supports PostgreSQL, MySQL,
//! and SQLite.
//!
//! ## Features
//! ### Enabled by default:
//! * [`sqlformat`]: enables formatting for the generated SQL.
//! *
//!
//! ### Optional:
//! * `orm`: enables orm like functionality.
//! * [`sqlx`]: implements SQLx traits over models' types.
//! * [`serde`]: implements Serialize and Deserialize over models' types.
//! * [`json`]: enables the `Json<T>` datatype.
//! # Quick Start
//!
//! install the CLI by running the following command:
//! ```ignore
//! $ cargo install models-cli
//! ```
//!
//! Now run the following command to create an environment file with the
//! `DATABASE_URL` variable set:
//! ```ignore
//! $ echo "DATABASE_URL=sqlite://database.db" > .env
//! ```
//! Alternatively it can be set as a environment variable with the following
//! command:
//! ```ignore
//! $ export DATABASE_URL=sqlite://database.db
//! ```
//! We now can create the database running the following command:
//! ```ignore
//! $ models database create
//! ```
//! This command will have created an SQLite file called `database.db`.
//! You can now derive the `Model` trait on your structures,
//! and `models` will manage the migrations for you. For example, write at
//! `src/main.rs`:
//! ```rust
//! #![allow(dead_code)]
//! use models::Model;
//!
//! #[derive(Model)]
//! #[table_name("users")]
//! struct User {
//!     #[primary_key]
//!     id: AutoIncrement,
//!     #[unique]
//!     email: String,
//!     password: String,
//!     is_admin: bool,
//! }
//!
//! #[derive(Model)]
//! struct Post {
//!     #[primary_key]
//!     id: AutoIncrement,
//!     #[foreign_key(User.id)]
//!     author_id: i32,
//!     #[default("<Untitled Post>")]
//!     title: String,
//!     content: String,
//! }
//!
//! #[derive(Model)]
//! struct PostLike {
//!     #[foreign_key(User.id, on_delete="cascade")]
//!     #[primary_key(post_id)]
//!     profile_id: i32,
//!     #[foreign_key(Post.id, on_delete="cascade")]
//!     post_id: i32,
//! }
//!
//! #[derive(Model)]
//! struct CommentLike {
//!     #[foreign_key(User.id)]
//!     #[primary_key(comment_id)]
//!     profile_id: i32,
//!     #[foreign_key(Comment.id)]
//!     comment_id: i32,
//!     is_dislike: bool,
//! }
//!
//! #[derive(Model)]
//! struct Comment {
//!     #[primary_key]
//!     id: i32,
//!     #[foreign_key(User.id)]
//!     author: i32,
//!     #[foreign_key(Post.id)]
//!     post: i32,
//! }
//! fn main() {}
//! ```
//!
//! If you now run the following command, your migrations should be
//! automatically created.
//! ```ignore
//! $ models generate
//! ```
//! The output should look like this:
//! ```ignore
//! Generated: migrations/1632280793452 profile
//! Generated: migrations/1632280793459 post
//! Generated: migrations/1632280793465 postlike
//! Generated: migrations/1632280793471 comment
//! Generated: migrations/1632280793476 commentlike
//! ```
//! You can check out the generated migrations at the `migrations/` folder.
//! To execute these migrations you can execute the following command:
//! ```ignore
//! models migrate run
//! ```
//! The output should look like this:
//! ``` ignore
//! Applied 1631716729974/migrate profile (342.208µs)
//! Applied 1631716729980/migrate post (255.958µs)
//! Applied 1631716729986/migrate comment (287.792µs)
//! Applied 1631716729993/migrate postlike (349.834µs)
//! Applied 1631716729998/migrate commentlike (374.625µs)
//! ```
//! If we later modify those structures in our application, we can generate new
//! migrations to update the tables.
//!
//! ## Reverting migration
//! Models can generate down migrations with the `-r` flag. Note that simple and
//! reversible migrations cannot be mixed:
//! ```ignore
//! $ models generate -r
//! ```
//! In order to revert the last migration executed you can run:
//! ```ignore
//! $ models migrate revert
//! ```
//! If you later want to see which migrations are yet to be applied you can also
//! excecute:
//! ```ignore
//! $ models migrate info
//! ```
//! ## Avaibale Attributes
//! ### primary_key
//! It's used to mark the primary key fo the table.
//! ```ignore
//!     #[primary_key]
//!     id: i32,
//! ```
//! for tables with multicolumn primary keys, the following syntax is used:
//! ```ignore
//!     #[primary_key(second_id)]
//!     first_id: i32,
//!     second_id: i32,
//! ```
//! This is equivalent to:
//! ```sql
//!     PRIMARY KEY (first_id, second_id),
//! ```
//!
//! ### foreign_key
//! It is used to mark a foreign key constraint.
//! ```ignore
//!     #[foreign_key(Profile.id)]
//!     profile: i32,
//! ```
//! It can also specify `on_delete` and `on_update` constraints:
//! ```ignore
//!     #[foreign_key(Profile.id, on_delete="cascade")]
//!     profile_id: i32,
//! ```
//! This is equivalent to:
//! ```sql
//!     FOREIGN KEY (profile_id) REFERENCES profile (id) ON DELETE CASCADE,
//! ```
//! ### default
//! It can be used to set a default value for a column.
//! ```ignore
//!     #[default(false)] // when using SQLite use 0 or 1
//!     is_admin: bool,
//!     #[default("")]
//!     text: String,
//!     #[default(0)]
//!     number: i32,
//! ```
//!
//! ### unique
//! It is used to mark a unique constraint.
//! ```ignore
//!     #[unique]
//!     email: String,
//! ```
//! For multicolumn unique constraints the following syntax is used:
//! ```ignore
//!     #[unique(post_id)]
//!     profile_id: String,
//!     post_id: i32,
//! ```
//! This is equivalent to:
//! ```sql
//!     UNIQUE (profile_id, post_id),
//! ```
//!
//! # ORM
//! The `orm` feature enables the following methods on structs:
//! ### insert
//! The `insert` method can be used to insert a new row into a table.
//! it will
//! ```rust
//! let user =  User {
//!     email: "example@gmail.com",
//!     password: "password123",
//!     ..Default::default()
//! };
//! user.insert().await?;
//! ```
//! ### save
//! The `save` method can be used to update a row in a table. If no rows are updated, the
//! value will be inserted into the table.
//! ```rust
//! user.save().await?;
//! ```
//!
//! ### delete
//! The `delete` method can be used to delete a row from a table.
//! ```
//! user.delete().await?;
//! ```
//! ### find
//! the `find` method can be used to get a row from a table.
//! ```rust
//! let user: Option<User> = User::find(1).await?;
//! ```
//!
//! ### get_all
//! The `get_all` method can be used to get all rows from a table.
//! ```rust
//! let users: Vec<User> = User::get_all().await?;
//! ```
//!
//! ### find_by
//! The `find_by` function can be used to get a row from a table from a specific column.
//! If the column is unique, then the output will be an option, otherwise it will be a vector.  
//! ```rust
//! let user: Option<User> = models::find_by(User::email, "example@gmail.com").await?;
//! let admins: Vec<User> = models::find_by(User::is_admin, true).await?;
//! ```
//! ### foreign keys
//! Foreign keys automatically define getters for both structs involved in the relation:
//! ```rust
//! let user: User = post.author().await?;
//! let posts: Vec<Post> = user.posts().await?;
//! ```
//! The convention is to name foreign keys ending in `_id`.  
//! The ending is removed in the names of getters, as in `author_id => author()`.
//! Additionally, it is assumed that struct names are singular,
//! this is pluralized for the foreign struct getter, as in `Post => posts()`.
//! The default names of the getters can be overriden with the following syntax:
//! ```rust
//! #[foreign_key(User.id, self.getter = "name1", foreign.getter = "name2")]
//! author_id: i32,
//! ```
//! To opt out of getters you can use the following syntax:
//! ```rust
//! #[foreign_key(User.id, self.getter = None, foreign.getter = None)]
//! author_id: i32,
//! ```
//!
//!
//!
//!
//!
// #![allow(unused_imports)]
#[macro_use]
extern crate models_proc_macro;

#[macro_use]
extern crate fehler; 

use models_parser;
pub use models_proc_macro::Model;

#[macro_use]
mod error;
mod dialect;

mod prelude;

// #[cfg(tests)]
mod tests;
pub mod types;

use prelude::Model;
pub use types::*;

#[doc(hidden)]
pub mod private;

#[cfg_attr(doc_cfg, doc(cfg(feature = "orm")))]
#[cfg(feature = "orm")]
pub mod orm;

