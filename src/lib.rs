#![warn(clippy::all, clippy::pedantic)]
#![deny(unsafe_op_in_unsafe_fn)]
// Pre-existing documentation debt; fix incrementally as code is touched .
#![allow(clippy::missing_errors_doc, clippy::must_use_candidate)]

pub mod client;
pub mod desktop;
pub mod error;
pub mod marketplace;
pub mod shim;

pub fn hello_world() -> &'static str {
    "hello from zlicenser"
}
