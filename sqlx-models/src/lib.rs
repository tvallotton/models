#![allow(dead_code)]

mod migration;
mod prelude;
use prelude::*;

trait Model {
    fn table() -> Table;
}
