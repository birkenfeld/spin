// Spin RPC library, copyright 2015-2017 Georg Brandl.

//! Test "cryo" device.

use client::Client;
use device::Device;
use error::{CONFIG_ERROR, IO_ERROR, API_ERROR, COMM_ERROR, SpinResult};


#[derive(Default)]
pub struct CryoDevice {
    props: CryoDeviceProps,
    iodev: Option<Client>,
}

spin_device_impl!(
    CryoDevice,
    CryoDeviceProps,
    cmds = [
    ],
    attrs = [
        value       => ("Read value.", Double, read_value, write_value)
    ],
    props = [
        iodev       => ("Network device.", String, String::new())
    ]
);

impl CryoDevice {
    pub fn create(_name: &str) -> Box<Device> {
        box CryoDevice::default()
    }

    fn init(&mut self) -> SpinResult<()> {
        if self.props.iodev.is_empty() {
            return spin_err!(CONFIG_ERROR, "need an iodev configured");
        }
        self.iodev = Some(Client::new(&self.props.iodev)?);
        Ok(())
    }

    fn delete(&mut self) {
        self.iodev.take();
    }

    fn read_value(&mut self) -> SpinResult<f64> {
        if let Some(ref mut iodev) = self.iodev {
            let reply: String = iodev.exec_cmd_as("Communicate", "MEAS?")?;
            let num = reply.split_whitespace().next().unwrap_or("");
            match num.parse() {
                Ok(val) => Ok(val),
                Err(_) => spin_err!(COMM_ERROR, format!("invalid response: {:?}", reply))
            }
        } else {
            spin_err!(IO_ERROR, "no iodev initialized")
        }
    }

    fn write_value(&mut self, _: f64) -> SpinResult<()> {
        spin_err!(API_ERROR, "cannot write the value attribute")
    }
}
