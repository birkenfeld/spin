// Spin RPC library, copyright 2015, 2016 Georg Brandl.
//
//! Main library module; just re-exports the public API.

#![feature(associated_consts, box_patterns, box_syntax)]

#[macro_use]
extern crate log;
extern crate url;
extern crate zmq;
extern crate fern;
extern crate time;
extern crate argparse;
extern crate protobuf;
extern crate spin_proto;

pub mod arg;
pub mod client;
pub mod server;
pub mod device;
pub mod error;
mod db;
mod util;

// pub use server::*;
// pub use device::*;
