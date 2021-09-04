# sqlx-models
sqlx-models provides support for automatic migration generation. To use it run
```
cargo install sqlx-models
```

then you can just derive the Model trait
```rust

#[derive(Model)]
struct User {
    #[primary_key]
    id: i32,
    #[unique]
    email: String,
    password: String
}

#[derive(Model
struct Post {
  #[primary_key]
  id: i32,
  #[foreign_key(User.id, on_delete="cascade")]
  author: i32, 
  title: String,
  content: String, 
}

```
Then you can just run 
```
sqlx migrations generate
```
And it will generate the following sql in your migrations directoy: 
```
TODO:
```
