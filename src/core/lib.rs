// Spin RPC library, copyright 2015-2017 Georg Brandl.

//! Main library module; just re-exports the public API.

#![feature(box_syntax)]

#[macro_use]
extern crate log;
extern crate url;
extern crate zmq;
extern crate fnv;
extern crate toml;
extern crate prost;
extern crate regex;
extern crate mlzlog;
extern crate argparse;
extern crate daemonize;
extern crate parking_lot;
#[macro_use]
extern crate lazy_static;

extern crate spin_proto;

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
pub use arg::{Value, FromValue};
pub use error::{Error, SpinResult};
pub use client::Client;
