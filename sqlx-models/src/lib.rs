mod migration;
mod model;
mod prelude;
mod sorter;
mod error;
pub use sqlx_models_proc_macro::Model;

#[doc(hidden)]
pub mod private {
    use once_cell::sync::Lazy;

    pub use super::migration::{
        table::{constraint, Column},
        Migration, Table,
    };
    pub use super::sorter::Sorter;
    pub static MIGRATIONS: Lazy<Sorter> = Lazy::new(Sorter::new);
    /// Do not use the types defined in this module.
    /// They are intended to be used only through the macro API.
    /// Changes in this module are not considered to be breaking changes.
    pub use super::model::{Dialect, Model, SqlType};
}
