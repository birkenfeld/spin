// Spin RPC library, copyright 2015 Georg Brandl.
//
//! Test echo server executable.

#![feature(associated_consts,box_syntax)]

extern crate spin;

use std::env::args;

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


impl device::Device for EchoDevice {
    const CLSNAME: &'static str = "EchoDevice";

    fn get_name(&self) -> &str { &self.name }

    fn get_commands(&self) -> Vec<CommandInfo> {
        vec![
            cmd_info("Echo", "Sends back the same string.", DataType::String, DataType::String),
            ]
    }

    fn get_attributes(&self) -> Vec<AttributeInfo> {
        vec![
            attr_info("value", "A double value.", DataType::Double),
            ]
    }

    fn exec_cmd(&mut self, cmd: &str, arg: Value) -> SpinResult<Value> {
        match cmd {
            "Echo" => self.cmd_echo(arg),
            _      => spin_err("CommandError", "No such command"),
        }
    }

    fn read_attr(&mut self, attr: &str) -> SpinResult<Value> {
        match attr {
            "value" => self.read_value(),
            _       => spin_err("AttributeError", "No such attribute"),
        }
    }

    fn write_attr(&mut self, attr: &str, val: Value) -> SpinResult<()> {
        match attr {
            "value" => self.write_value(val),
            _       => spin_err("AttributeError", "No such command"),
        }
    }
}


fn main() {
    println!("Echo server running...");
    let mut server = server::Server::new("echo/test");
    server.config_from_args(&args().collect());

    let echodev = EchoDevice { name: "test/dev/echo".into(), value: 0. };
    server.add_device(Box::new(echodev));

    if let Err(e) = server.run() {
        println!("Error running server: {}", e);
    }
}
