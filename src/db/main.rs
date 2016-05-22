// Spin RPC library, copyright 2015 Georg Brandl.
//
//! Database server executable.

#![feature(associated_consts,box_syntax)]

#[macro_use]
extern crate log;
extern crate spin;

use std::collections::HashMap;

use spin::arg::*;
use spin::server;
use spin::device;
use spin::device::Device;
use spin::error::{SpinResult, spin_err};


struct DbDevice {
    name: String,
    devmap: HashMap<String, String>,
    srvmap: HashMap<String, String>,
}

impl DbDevice {
    fn cmd_register(&mut self, arg: Value) -> SpinResult<Value> {
        let mut s: Vec<String> = try!(FromValue::from_value(arg));
        if s.len() < 3 {
            return spin_err("DbError", "need to have at least one devname");
        }
        let address = s.swap_remove(0);
        let srvname = s.swap_remove(1);
        info!("registering server {} at {}...", srvname, address);
        self.srvmap.insert(srvname.clone(), address);
        for devname in s {
            info!("   ... device {}", devname);
            self.devmap.insert(devname, srvname.clone());
        }
        Ok(Value::void())
    }

    fn cmd_query(&self, arg: Value) -> SpinResult<Value> {
        let devname = try!(String::from_value(arg));
        info!("requested {}", devname);
        match self.devmap.get(&devname) {
            None => spin_err("DbError", "device not found"),
            Some(srvname) => match self.srvmap.get(srvname) {
                None => spin_err("DbError", "server not found"),
                Some(srvaddr) => {
                    info!("   ... is at {}", srvaddr);
                    Ok(Value::from(srvaddr.clone()))
                }
            }
        }
    }
}


impl device::Device for DbDevice {
    const CLSNAME: &'static str = "DbDevice";

    fn get_name(&self) -> &str { &self.name }

    fn get_commands(&self) -> Vec<CommandDesc> {
        vec![
            cmd_info("Register", "Register a server and its devices.",
                     DataType::StringArray, DataType::Void),
            cmd_info("Query", "Query information about a device.",
                     DataType::String, DataType::String),
            ]
    }

    fn get_attributes(&self) -> Vec<AttrDesc> {
        vec![]
    }

    fn exec_cmd(&mut self, cmd: &str, arg: Value) -> SpinResult<Value> {
        match cmd {
            "Register" => self.cmd_register(arg),
            "Query"    => self.cmd_query(arg),
            _          => spin_err("CommandError", "No such command"),
        }
    }

    fn read_attr(&mut self, _attr: &str) -> SpinResult<Value> {
        spin_err("AttributeError", "No such attribute")
    }

    fn write_attr(&mut self, _attr: &str, _val: Value) -> SpinResult<()> {
        spin_err("AttributeError", "No such attribute")
    }
}


fn main() {
    match server::Server::from_args(false) {
        None => return,
        Some(mut server) => {
            let dev = DbDevice { name: "sys/spin/db".into(),
                                 devmap: HashMap::new(),
                                 srvmap: HashMap::new() };
            server.add_device(Box::new(dev));

            info!("database server running...");
            if let Err(e) = server.run() {
                error!("Error running server: {}", e);
            }
        }
    }
}
