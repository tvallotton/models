// #![allow(dead_code)]
mod migration;
mod model;
mod prelude;

pub use migration::{
    table::{constraint, Column},
    Migration, Table,
};

pub use model::{Dialect, Model, SqlType};

pub use sqlx_models_proc_macro::Model;
