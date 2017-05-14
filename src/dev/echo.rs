// Spin RPC library, copyright 2015-2017 Georg Brandl.

//! Echo device.

use device::Device;
use error::SpinResult;


#[derive(Default)]
pub struct EchoDevice {
    props: EchoDeviceProps,
    value: f64,
}

spin_device_impl!(
    EchoDevice,
    EchoDeviceProps,
    cmds = [
        Echo  => ("Sends back the same string.", String, String, cmd_echo),
    ],
    attrs = [
        value => ("A double value.", Double, read_value, write_value),
    ],
    props = [
        default_value => ("Default for 'value' attribute.", Double, 42.0),
    ]
);

impl EchoDevice {
    pub fn create(_name: &str) -> Box<Device> {
        box EchoDevice::default()
    }

    fn init(&mut self) -> SpinResult<()> {
        self.value = self.props.default_value;
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
