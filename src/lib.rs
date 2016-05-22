// Spin RPC library, copyright 2015 Georg Brandl.
//
//! Main library module; just re-exports the public API.

#![feature(associated_consts,box_patterns,std_misc,convert,box_syntax)]

extern crate protobuf;
extern crate spin_proto;
extern crate zmq;
#[macro_use]
extern crate log;
extern crate fern;

pub mod arg;
pub mod client;
pub mod server;
pub mod device;
pub mod error;
mod db;
mod util;

// pub use server::*;
// pub use device::*;
