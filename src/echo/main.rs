// Spin RPC library, copyright 2015, 2016 Georg Brandl.
//
//! Test echo server executable.

#![feature(box_syntax, question_mark)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate spin;

use spin::arg::*;
use spin::device::{Device, PropMap};
use spin::error::SpinResult;


struct EchoDevice {
    name: String,
    value: f64,
    propmap: PropMap,
}

impl EchoDevice {
    fn create(name: String) -> Box<Device> {
        box EchoDevice { name: name, value: 0., propmap: PropMap::new() }
    }

    fn init(&mut self) {
        self.value = self.propmap["default_value"].clone().into_inner().get_double()[0];
    }

    fn delete(&mut self) {
    }

    fn cmd_echo(&self, arg: Value) -> SpinResult<Value> {
        let s = String::from_value(arg)?;
        Ok(Value::new(s))
    }

    fn read_value(&self) -> SpinResult<Value> {
        Ok(Value::new(self.value))
    }

    fn write_value(&mut self, val: Value) -> SpinResult<()> {
        self.value = f64::from_value(val)?;
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
