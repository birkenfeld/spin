// Spin RPC library, copyright 2015 Georg Brandl.
//
//! Minimal server example.

#![feature(associated_consts,box_syntax)]

extern crate spin;

use std::env::args;
use std::rc::Rc;

use spin::server;
use spin::device;
use spin::device::Device;


// this is actually empty...
struct EchoDevice {
    dummy: i64,
}

impl device::Device for EchoDevice {
    const CLSNAME: &'static str = "EchoDevice";

    fn create(name: &String) -> Box<EchoDevice> {
        box EchoDevice { dummy: 0 }
    }
}


fn main() {
    println!("starting echo server...");

    let mut server = server::Server::new();
    server.config_from_args(&args().collect());
    server.register_device_impl(EchoDevice::CLSNAME, &EchoDevice::create);
    server.run();
}
