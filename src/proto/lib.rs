// Spin RPC library, copyright 2015-2017 Georg Brandl.

//! Protocol buffers: definition of message format sent over 0MQ.

extern crate prost;
#[macro_use]
extern crate prost_derive;

include!(concat!(env!("OUT_DIR"), "/spin_proto.rs"));
