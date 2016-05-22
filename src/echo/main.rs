// Spin RPC library, copyright 2015, 2016 Georg Brandl.
//
//! Test echo server executable.

#![feature(box_syntax, question_mark)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate spin;

use spin::arg::*;
use spin::config::DevProp;
use spin::device::Device;
use spin::error::SpinResult;


struct EchoDevice {
    name: String,
    value: f64,
}

impl EchoDevice {
    fn create(name: String, props: Vec<DevProp>) -> Box<Device> {
        let mut dev = box EchoDevice { name: name, value: 0. };
        for prop in props {
            // TODO: more automatic?
            if prop.name == "default_value" {
                if let Ok(v) = f64::from_value(prop.value) {
                    dev.value = v;
                }
            }
        }
        dev
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

    fn get_def_value(&mut self) -> SpinResult<Value> {
        Ok(Value::new(0.0))
    }

    fn set_def_value(&mut self, val: Value) -> SpinResult<()> {
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
        default_value => ("Default for 'value' attribute.", DataType::Double,
                          42.0_f64, get_def_value, set_def_value)
    ]
);


fn main() {
    server_main!(devtypes = [
        Echo => EchoDevice::create
    ]);
}
