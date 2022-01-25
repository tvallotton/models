//! # Models
//! Models is a SQL migration management tool. It supports PostgreSQL, MySQL,
//! and SQLite.
//!
//! ## Features
//! ### Enabled by default:
//! * [`sqlformat`]: enables formatting for the generated SQL.
//!
//! ### Optional:
//! * `orm`: enables ORM functionality.
//! * [`sqlx`]: implements SQLx traits over models' types.
//! * [`serde`]: implements Serialize and Deserialize over models' types.
//! * `json`: enables the `Json<T>` datatype.
//! 
//! # Quick Start
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
//!     id: Serial,
//!     #[unique]
//!     email: String,
//!     password: String,
//!     #[default(false)]
//!     is_admin: bool,
//! }
//!
//! #[derive(Model)]
//! struct Post {
//!     #[primary_key]
//!     id: Serial,
//!     #[foreign_key(User.id)]
//!     author_id: i32,
//!     #[default("<Untitled Post>")]
//!     title: String,
//!     content: String,
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
//! Generated: migrations/1632280793452 users
//! Generated: migrations/1632280793459 post
//! ```
//! You can check out the generated migrations at the `migrations/` folder.
//! To execute these migrations you can execute the following command:
//! ```ignore
//! models migrate run
//! ```
//! The output should look like this:
//! ``` ignore
//! Applied 1631716729974/migrate users (342.208µs)
//! Applied 1631716729980/migrate post (255.958µs)
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
//! Before deleting an applied migration it must be reverted. 
//! In order to revert the last migration executed you can run:
//! ```ignore
//! $ models migrate revert
//! ```
//! If you later want to see which migrations are yet to be applied you can also
//! excecute:
//! ```ignore
//! $ models migrate info
//! ```
//! ## Avaibale macro attributes
//! ### primary_key
//! It's used to mark the primary key fo the table.
//! ```ignore
//!     #[primary_key]
//!     id: i32,
//! ```
//! For tables with multicolumn primary keys, the following syntax is used:
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
//!     #[default(false)] 
//!     is_admin: bool,
//!     #[default("")]
//!     text: String,
//!     #[default(0)]
//!     number: i32,
//! ```
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
//! ## struct derived methods
//! The `orm` feature enables the following methods and associated functions on structs:
//! ### insert
//! The `insert` method can be used to insert a new row into a table.
//! 
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
//! the `find` method can be used to get a row from a table from the primary key.
//! If the table has multiple primary keys, then the argument should be a tuple. 
//! ```rust
//! let user: Option<User> = User::find(1).await?;
//! ```
//!
//! ### all
//! The `all` method can be used to get all rows from a table.
//! ```rust
//! let users: Vec<User> = User::all().await?;
//! ```
//!
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
//! #[foreign_key(User.id, self.getter = "user", foreign.getter = "publications")]
//! author_id: i32,
//! ```
//! To opt out of getters you can use the following syntax:
//! ```rust
//! #[foreign_key(User.id, self.getter = None, foreign.getter = None)]
//! author_id: i32,
//! ```
//! ## complex queries
//! ### query composition
//! Queries are executed when they are awaited. If they are not awaited, 
//! they can be composed to create more complex queries.
//! ```
//! // UPDATE user SET is_admin = false WHERE user.email LIKE "%@gmail%"; 
//! let authors = select(Post::author_id);
//! 
//!  let emails: Vec<String> = User::all()
//!     .filter(User::id.is_in, authors)
//!     .order_by(User::id.desc)
//!     .limit(10)
//!     .await?; 
//! ```
//! 
//! ### select
//! You may select all elements in a table with the select function. 
//! You can also select a single column by using the path operator (`Table::column`). It may also be used as a postfix method from an existing query.
//! Multiple columns can be selected with a tuple:  
//! ```
//! // SELECT * FROM user; 
//! let users: Vec<User> = select(User).await?; 
//! // SELECT user.email FROM user; 
//! let emails: Vec<User> = select(User::email).await?; 
//! // used as a method: 
//! let emails: Vec<User> = query.select(User::email).await?; 
//! ```
//! 
//! ### filter
//! the filter method can be used to introduce a where clause in the SQL query: 
//! ```
//! let admins: Vec<User> =  User::all()
//!     .filter(User::is_admin, true)
//!     .await?; 
//! ```
//! ### join
//! ```
//! 
//! join_on()
//! 
//! 
//! // select post.*, user.* from post inner join on
//! 
//! let author: Vec<User> = join(Post::author_id, User::id)
//!     .limit(1)
//!     .await?; 
//! ```
//! 
//! ### fields and operators
//! A field of a table can be referenced with the path operator (`Table::column`). 
//! Operators over fields can be expressed with the dot notation. 
//! When performing a field comparison, equality is the default operation,
//! but another operation can be used using the following syntax: 
//! ```rust
//! let _ = filter(User::email, "example@gmail.com").await?; // equality
//! let _ = filter(User::email.like, "%@gmail%").await; // LIKE operator
//! let _ = filter(User::id.lt, 100).await?;  // less than (<)
//! let _ = filter(User::id.gt, 100).await?;  // greater than (>)
//! ```
//! 
//! ### find_by
//! The `find_by` function can be used to get a row from a table from a specific column.
//! If the column is unique, then the output will be an option, otherwise it will be a vector. 
//! ```rust
//! use models::find_by; 
//! let user: Option<User> = find_by(User::email, "example@gmail.com").await?;
//! let admins: Vec<User> = find_by(User::is_admin, true).await?;
//! let user: Option<User> = find_by(Post::id.lt, )
//! ```
//! 
//! 
//! ### update
//! You may update the field of an entire column
//! using the update function
//! ```
//! models::update(User::is_admin, false); 
//! update().set(User::is_admin, false).
//! models::update(User::is_admin, false).filter(User::)
//! ```
//! 
//! 
//! ### query composition
//! Queries are executed when they are awaited. If they are not awaited, 
//! they can be composed to create more complex queries.
//! ```
//! // UPDATE user SET is_admin = false WHERE user.email LIKE "%@gmail%"; 
//! models::update(User::is_admin, false)
//!     .filter(User::email.like, "%@gmail%")
//!     .await?; 
//! 
//! // SELECT user.email FROM User where user.is_admin = true ORDER BY user.id ASC; 
//! let admins: Vec<String> = models::select(User::email)
//!     .filter(User::is_admin, true)
//!     .order_by(User::id.asc)
//!     .await?; 
//! 
//! 
//! ```
//! ### select
//! to select a column you can use 
//! the `select` function. There is also a select method used to select a column from a query. 
//! ```
//! let emails: Vec<String> = select(User::email).await?; 
//! let password: Option<String> = User::find(10).select(User::password).await?; 
//! ```
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

