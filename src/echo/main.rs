// Spin RPC library, copyright 2015, 2016 Georg Brandl.
//
//! Test echo server executable.

#![feature(box_syntax, question_mark)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate spin;

use spin::arg::*;
use spin::device::{Device, DeviceInner};
use spin::error::SpinResult;


struct EchoDevice {
    name: String,
    inner: DeviceInner,
    value: f64,
}

impl EchoDevice {
    fn create(name: String) -> Box<Device> {
        box EchoDevice { name: name, value: 0.,
                         inner: Default::default() }
    }

    fn init(&mut self) -> SpinResult<()> {
        self.value = self.get_props()["default_value"].extract_clone()?;
        Ok(())
    }

    fn delete(&mut self) {
    }

    fn cmd_echo(&self, arg: String) -> SpinResult<String> {
        Ok(arg)
    }

    fn read_value(&self) -> SpinResult<f64> {
        Ok(self.value)
    }

    fn write_value(&mut self, val: f64) -> SpinResult<()> {
        self.value = val;
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
        value => ("A double value.", DataType::Double, read_value, write_value)
    ],
    props [
        default_value => ("Default for 'value' attribute.", DataType::Double, 42.0_f64)
    ]
);


fn main() {
    server_main!(devtypes = [
        Echo => EchoDevice::create
    ]);
}
