// Spin RPC library, copyright 2015-2017 Georg Brandl.

//! Device implementations submodule.

extern crate fnv;
#[macro_use]
extern crate log;
extern crate serialport;
#[macro_use]
extern crate spin;

pub mod echo;
pub mod network;
pub mod serial;
pub mod cryo;
