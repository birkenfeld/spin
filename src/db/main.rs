// Spin RPC library, copyright 2015-2020 Georg Brandl.

//! Database server executable.

use log::info;
use fxhash::FxHashMap as HashMap;

use spin::{spin_device_impl, spin_server_main, spin_err};
use spin::config::{DevConfig, ServerConfig};
use spin::device::Device;
use spin::error::{SpinResult, DB_ERROR};

pub const DEFAULT_DB_PORT: u16 = 9999;


#[derive(Default)]
struct DbDevice {
    props: DbDeviceProps,
    devmap: HashMap<String, String>, // device -> server
    srvmap: HashMap<String, String>, // server -> address
}

spin_device_impl!(
    DbDevice,
    DbDeviceProps,
    bases = [ ],
    cmds = [
        Register   => ("Register a server and its devices.",
                       Vec<String>, (), cmd_register),
        Unregister => ("Unregister a server and its devices.",
                       Vec<String>, (), cmd_unregister),
        Query      => ("Query information about a device.",
                       String, String, cmd_query),
        List       => ("List all devices, their server names and addresses.",
                       (), Vec<String>, cmd_list),
    ],
    attrs = [ ],
    props = [ ],
);

impl DbDevice {
    fn create(_name: &str) -> Box<dyn Device> {
        Box::new(DbDevice::default())
    }

    fn init(&mut self) -> SpinResult<()> {
        Ok(())
    }

    fn delete(&mut self) {}

    fn cmd_register(&mut self, mut info: Vec<String>) -> SpinResult<()> {
        if info.len() < 3 {
            return spin_err!(DB_ERROR, "need to have at least one devname");
        }
        let address = info.swap_remove(0);
        let srvname = info.swap_remove(1);
        info!("registering server {} at {}...", srvname, address);
        for devname in info {
            info!("   ... device {}", devname);
            self.devmap.insert(devname, srvname.clone());
        }
        self.srvmap.insert(srvname, address);
        Ok(())
    }

    fn cmd_unregister(&mut self, info: Vec<String>) -> SpinResult<()> {
        if info.len() != 2 {
            return spin_err!(DB_ERROR, "arg needs to be [address, name]");
        }
        let address = &info[0];
        let srvname = &info[1];
        info!("unregistering server {} at {}...", srvname, address);
        let mut remove = Vec::new();
        for (dev, srv) in &self.devmap {
            if srvname == srv {
                info!("   ... device {}", dev);
                remove.push(dev.clone());
            }
        }
        for dev in remove {
            self.devmap.remove(&dev);
        }
        self.srvmap.remove(srvname);
        Ok(())
    }

    fn cmd_query(&self, devname: String) -> SpinResult<String> {
        info!("requested {}", devname);
        match self.devmap.get(&devname) {
            None => spin_err!(DB_ERROR, "device not found"),
            Some(srvname) => match self.srvmap.get(srvname) {
                None => spin_err!(DB_ERROR, "server not found"),
                Some(srvaddr) => {
                    info!("   ... is at {}", srvaddr);
                    Ok(srvaddr.clone())
                }
            },
        }
    }

    fn cmd_list(&self, _: ()) -> SpinResult<Vec<String>> {
        info!("list requested");
        let mut result = Vec::with_capacity(self.devmap.len() * 3);
        for (dev, srv) in &self.devmap {
            result.push(dev.clone());
            result.push(srv.clone());
            result.push(self.srvmap[srv].clone());
        }
        Ok(result)
    }
}


fn main() {
    let static_config = Some(ServerConfig {
        devices: vec![
            DevConfig {
                name: "sys/spin/db".into(),
                devtype: "Db".into(),
                props: vec![],
            },
        ],
    });
    spin_server_main!(
        use_db = false,
        static_config = static_config,
        devtypes = [
            Db => DbDevice::create,
        ]
    );
}
