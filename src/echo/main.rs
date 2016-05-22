// Spin RPC library, copyright 2015 Georg Brandl.
//
//! Test echo server executable.

#![feature(associated_consts,box_syntax)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate spin;

use spin::arg::*;
use spin::server;
use spin::device;
use spin::device::Device;
use spin::error::{SpinResult, spin_err};


struct EchoDevice {
    name: String,
    value: f64,
}

impl EchoDevice {
    fn cmd_echo(&self, arg: Value) -> SpinResult<Value> {
        let s = try!(String::from_value(arg));
        Ok(Value::new(s))
    }

    fn read_value(&self) -> SpinResult<Value> {
        Ok(Value::new(self.value))
    }

    fn write_value(&mut self, val: Value) -> SpinResult<()> {
        self.value = try!(f64::from_value(val));
        Ok(())
    }
}


device_impl!(
    EchoDevice,
    cmds  [
        Echo  => ("Sends back the same string.",
                  DataType::String, DataType::String, cmd_echo)
    ],
    attrs [
        value => ("A double value.", DataType::Double,
                  read_value, write_value)
    ]
);

fn create_echo_device(name: String) -> Box<Device> {
    box EchoDevice { name: name, value: 0. }
}


fn main() {
    match server::Server::from_args(true) {
        None => return,
        Some(mut server) => {
            server.add_device("test/dev/echo".into(), create_echo_device);

            info!("echo server running...");
            if let Err(e) = server.run() {
                error!("Error running server: {}", e);
            }
        }
    }
}
