// Spin RPC library, copyright 2015, 2016 Georg Brandl.
//
//! Database server executable.

#![feature(box_syntax, question_mark)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate spin;

use std::collections::HashMap;

use spin::arg::*;
use spin::config::{ServerConfig, DevConfig};
use spin::device::{Device, PropMap};
use spin::error::{SpinResult, spin_err};


struct DbDevice {
    name: String,
    propmap: PropMap,
    devmap: HashMap<String, String>,
    srvmap: HashMap<String, String>,
}

impl DbDevice {
    fn create(name: String) -> Box<Device> {
        box DbDevice { name: name,
                       propmap: PropMap::new(),
                       devmap: HashMap::new(),
                       srvmap: HashMap::new() }
    }

    fn init(&mut self) { }

    fn delete(&mut self) { }

    fn cmd_register(&mut self, arg: Value) -> SpinResult<Value> {
        let mut s: Vec<String> = FromValue::from_value(arg)?;
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
        let devname = String::from_value(arg)?;
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

device_impl!(
    DbDevice,
    cmds  [
        Register => ("Register a server and its devices.",
                     DataType::StringArray, DataType::Void, cmd_register),
        Query    => ("Query information about a device.",
                     DataType::String, DataType::String, cmd_query)
    ],
    attrs [
    ],
    props [
    ]
);


fn main() {
    let static_config = Some(ServerConfig {
        devices: vec![DevConfig {
            name: "sys/spin/db".into(),
            devtype: "Db".into(),
            props: vec![],
        }]
    });
    server_main!(
        use_db = false,
        static_config = static_config,
        devtypes = [
            Db => DbDevice::create
        ]
    );
}
