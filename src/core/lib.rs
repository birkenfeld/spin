// Spin RPC library, copyright 2015-2020 Georg Brandl.

//! Main library module; just re-exports the public API.

#[macro_use]
pub mod error;
pub mod arg;
#[macro_use]
pub mod device;
pub mod config;
pub mod server;
pub mod client;
pub mod base;
pub mod support;
pub mod validate;
mod util;

// For client use, re-export the needed API here.
pub use arg::{FromValue, Value};
pub use error::{Error, SpinResult};
pub use client::Client;
