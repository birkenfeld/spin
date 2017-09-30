// Spin RPC library, copyright 2015-2017 Georg Brandl.

//! Universal server executable.

#[macro_use]
extern crate spin;
extern crate spin_devices;
#[macro_use]
extern crate log;

use spin_devices::*;


fn main() {
    spin_server_main!(devtypes = [
        echo_Echo => echo::EchoDevice::create,
        network_StringIO => network::NetworkDevice::create,
        serial_StringIO => serial::SerialDevice::create,
        cryo_Sensor => cryo::CryoDevice::create,
    ]);
}