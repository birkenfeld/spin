// Spin RPC library, copyright 2015-2018 Georg Brandl.

//! Main library module; just re-exports the public API.

#![feature(box_syntax)]

#[macro_use]
extern crate structopt;
extern crate daemonize;
extern crate fxhash;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate mlzlog;
extern crate parking_lot;
extern crate prost;
extern crate regex;
extern crate toml;
extern crate url;
extern crate zmq;

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
pub use arg::{FromValue, Value};
pub use error::{Error, SpinResult};
pub use client::Client;
