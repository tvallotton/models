//! This module is publicly accessible, but the interface can be subject to
//! changes. This module is intended for macros only.
//! Changes to elements in this module are not considered a breaking change. Do
//! not depend directly on this module.
#[cfg(feature="orm")]
pub mod orm;
mod scheduler;
use once_cell::sync::Lazy;
pub(crate) use scheduler::driver::migration::Migration;
pub use scheduler::{
    table::{constraint, Column, Table},
    Scheduler,
};
pub trait Model {
    fn target() -> Table;
}

pub static SCHEDULER: Lazy<Scheduler> = Lazy::new(Scheduler::new);

#[cfg(feature = "orm")]
pub use orm::DATABASE_CONNECTION;
