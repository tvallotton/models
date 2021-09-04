// #![allow(dead_code)]
mod migration;
mod model;
mod prelude;

pub use sqlx_models_proc_macro::Model;

#[doc(hidden)]
pub mod private {
    pub use super::migration::{
        table::{constraint, Column},
        Migration, Table,
    };
    /// Do not use the types defined in this module.
    /// They are intended to be used only through the macro API.
    /// Changes in this module are not considered to be breaking changes.
    pub use super::model::{Dialect, Model, SqlType};
}
