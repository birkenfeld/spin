// Spin RPC library, copyright 2015-2017 Georg Brandl.

//! Test "cryo" device.

use spin::client::Client;
use spin::device::Device;
use spin::error::{SpinResult, API_ERROR, COMM_ERROR, CONFIG_ERROR, IO_ERROR};
use spin::base::AnalogInput;
use spin::validate::{Mandatory, Subdev};

#[derive(Default)]
pub struct CryoDevice {
    props: CryoDeviceProps,
    iodev: Option<Client>,
}

spin_device_impl!(
    CryoDevice,
    CryoDeviceProps,
    bases = [
        analog_input
    ],
    cmds = [ ],
    attrs = [ ],
    props = [
        iodev       => ("Network device.", Subdev, Mandatory)
    ]
);

impl CryoDevice {
    pub fn create(_name: &str) -> Box<Device> {
        Box::new(CryoDevice::default())
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

    fn get_iodev(&mut self) -> SpinResult<&mut Client> {
        self.iodev.as_mut().map_or_else(|| spin_err!(IO_ERROR, "no iodev initialized"), Ok)
    }
}

impl AnalogInput for CryoDevice {
    fn read_value(&mut self) -> SpinResult<f64> {
        let iodev = self.get_iodev()?;
        let reply: String = iodev.exec_cmd_as("Communicate", "MEAS?")?;
        let num = reply.split_whitespace().next().unwrap_or("");
        match num.parse() {
            Ok(val) => Ok(val),
            Err(_) => spin_err!(COMM_ERROR, format!("invalid response: {:?}", reply)),
        }
    }

    fn write_value(&mut self, _: f64) -> SpinResult<()> {
        spin_err!(API_ERROR, "cannot write the value attribute")
    }
}
