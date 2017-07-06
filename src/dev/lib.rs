// Spin RPC library, copyright 2015-2017 Georg Brandl.

//! Device implementations submodule.

#[macro_use]
extern crate spin;
#[macro_use]
extern crate log;
extern crate fnv;
extern crate serialport;

pub mod echo;
pub mod network;
pub mod serial;
pub mod cryo;
