// Spin RPC library, copyright 2015, 2016 Georg Brandl.
//
//! Main library module; just re-exports the public API.

#![feature(box_syntax, question_mark)]

#[macro_use]
extern crate log;
extern crate url;
extern crate zmq;
extern crate fern;
extern crate time;
extern crate toml;
extern crate argparse;
extern crate protobuf;
extern crate spin_proto;

pub mod arg;
pub mod error;
pub mod config;
pub mod device;
pub mod server;
pub mod client;
mod db;
mod util;

// For client use, re-export the needed API here.
pub use arg::Value;
pub use client::Client;
pub use error::{Error, SpinResult};
