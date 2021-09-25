//! This module is publicly accessible, but the interface can be subject to changes.
//! This module is inteded for macros only.
//! Changes to elements in this module are not considered a breaking change.
pub use crate::prelude::*;
mod scheduler;
use once_cell::sync::Lazy;
pub(crate) use scheduler::driver::migration::Migration;
pub use scheduler::{
    table::{Column, Table},
    Scheduler,
};
pub static SCHEDULER: Lazy<Scheduler> = Lazy::new(Scheduler::new);
