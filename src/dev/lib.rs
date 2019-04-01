// Spin RPC library, copyright 2015-2018 Georg Brandl.

//! Device implementations submodule.

#[macro_use]
extern crate log;
#[macro_use]
extern crate spin;

pub mod echo;
pub mod network;
pub mod serial;
pub mod cryo;
