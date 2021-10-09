#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::upper_case_acronyms)]

#[cfg(not(feature = "std"))]
extern crate alloc;

pub mod ast;
#[macro_use]
pub mod dialect;
pub mod parser;
pub mod tokenizer;

#[doc(hidden)]
// This is required to make utilities accessible by both the crate-internal
// unit-tests and by the integration tests <https://stackoverflow.com/a/44541071/1026>
// External users are not supposed to rely on this module.
pub mod test_utils;
