// Spin RPC library, copyright 2015, 2016 Georg Brandl.
//
//! Universal server executable.

#[macro_use] extern crate log;
#[macro_use] extern crate spin;

use spin::dev::*;


fn main() {
    spin_server_main!(devtypes = [
        echo_Echo => echo::EchoDevice::create,
        network_StringIO => network::NetworkDevice::create
    ]);
}
