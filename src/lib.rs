// Spin RPC library, copyright 2015-2017 Georg Brandl.

//! Main library module; just re-exports the public API.

#![feature(box_syntax)]

#[macro_use]
extern crate log;
extern crate url;
extern crate zmq;
extern crate fnv;
extern crate time;
extern crate toml;
extern crate prost;
extern crate bytes;
extern crate mlzlog;
extern crate argparse;
extern crate daemonize;
extern crate spin_proto;

#[macro_use]
pub mod error;
pub mod arg;
pub mod config;
#[macro_use]
pub mod device;
pub mod server;
pub mod client;
pub mod dev;
mod db;
mod util;

// For client use, re-export the needed API here.
pub use arg::{Value, FromValue};
pub use error::{Error, SpinResult};
pub use client::Client;
