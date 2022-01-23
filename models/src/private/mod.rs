//! This module is publicly accessible, but the interface can be subject to
//! changes. This module is intended for macros only.
//! Changes to elements in this module are not considered a breaking change. Do
//! not depend directly on this module.
//!

pub use once_cell::sync::Lazy;
#[cfg(feature = "orm")]
pub use sqlx;

pub use super::dialect::Dialect;
pub(crate) mod scheduler;

pub(crate) use scheduler::driver::migration::Migration;
pub use scheduler::{
    table::{
        constraint,
        Column,
        Table,
    },
    Scheduler,
};
pub trait Model {
    const TABLE_NAME: &'static str;

    fn target() -> Table;
}

pub static SCHEDULER: Lazy<Scheduler> = Lazy::new(Scheduler::new);
