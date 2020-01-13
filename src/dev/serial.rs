// Spin RPC library, copyright 2015-2020 Georg Brandl.

//! Serial port device.

use std::time::Duration;
use serialport::{self, SerialPort};

use spin::{spin_device_impl, spin_err};
use spin::device::Device;
use spin::error::{Error as SpinError, SpinResult, CONFIG_ERROR, IO_ERROR};
use spin::base::StringIO;
use spin::support::comm::{CommClient, CommThread};
use spin::validate::Mandatory;

#[derive(Default)]
pub struct SerialDevice {
    props: SerialDeviceProps,
    timeout: f64,
    comm: Option<CommClient<Box<dyn SerialPort>>>,
}

spin_device_impl!(
    SerialDevice,
    SerialDeviceProps,
    bases = [
        string_io
    ],
    cmds = [ ],
    attrs = [ ],
    props = [
        devfile  => ("Device file name.", String, Mandatory),
        baudrate => ("Baud rate.", u32, 9600),
    ],
);

struct Error(serialport::Error);

impl From<Error> for SpinError {
    fn from(e: Error) -> SpinError {
        SpinError::with(
            IO_ERROR.into(),
            (e.0).to_string(),
            module_path!().into(),
        )
    }
}

impl SerialDevice {
    pub fn create(_name: &str) -> Box<dyn Device> {
        Box::new(SerialDevice::default())
    }

    fn init(&mut self) -> SpinResult<()> {
        if self.props.devfile.is_empty() {
            return spin_err!(CONFIG_ERROR, "need a devfile configured");
        }
        self.timeout = self.props.string_io.timeout;
        let timeout = Duration::from_millis((self.timeout * 1000.) as u64);
        let devfile = self.props.devfile.clone();
        let baudrate = self.props.baudrate;

        let connect = move || -> SpinResult<(Box<dyn SerialPort>, Box<dyn SerialPort>)> {
            log::info!("opening {}...", devfile);
            let mut port = serialport::open(&devfile).map_err(Error)?;
            let mut settings = port.settings();
            settings.baud_rate = baudrate;
            port.set_all(&settings).unwrap();
            let mut rport = serialport::open(&devfile).map_err(Error)?;
            rport.set_all(&settings).unwrap();
            Ok((rport, port))
        };

        self.comm = Some(CommThread::spawn(
            Box::new(connect),
            self.props.string_io.sol.as_bytes(),
            self.props.string_io.eol.as_bytes(),
            timeout,
        )?);
        Ok(())
    }

    fn delete(&mut self) {
        self.comm.take(); // close the connection and join the thread if it exists
    }

    fn convert(&self, bytes: SpinResult<Vec<u8>>) -> SpinResult<String> {
        bytes.map(|v| {
            String::from_utf8(v)
                .unwrap_or_else(|e| String::from_utf8_lossy(&e.into_bytes()).into_owned())
        })
    }

    fn get_comm(&self) -> SpinResult<&CommClient<Box<dyn SerialPort>>> {
        if let Some(ref comm) = self.comm {
            Ok(comm)
        } else {
            spin_err!(IO_ERROR, "connection not open")
        }
    }
}

impl StringIO for SerialDevice {
    fn cmd_communicate(&mut self, arg: String) -> SpinResult<String> {
        self.convert(self.get_comm()?.communicate(arg.as_bytes()))
    }

    fn cmd_flush(&mut self, _: ()) -> SpinResult<()> {
        Ok(())
    }

    fn cmd_read(&mut self, arg: u32) -> SpinResult<String> {
        self.convert(self.get_comm()?.read(arg))
    }

    fn cmd_write(&mut self, arg: String) -> SpinResult<u32> {
        self.get_comm()?.write(arg.as_bytes(), false)
    }

    fn cmd_readline(&mut self, _: ()) -> SpinResult<String> {
        self.convert(self.get_comm()?.readline())
    }

    fn cmd_writeline(&mut self, arg: String) -> SpinResult<u32> {
        self.get_comm()?.write(arg.as_bytes(), true)
    }

    fn read_timeout(&mut self) -> SpinResult<f64> {
        Ok(self.timeout)
    }

    fn write_timeout(&mut self, val: f64) -> SpinResult<()> {
        // XXX does not reset timeout on streams -> needs reopen
        self.timeout = val;
        Ok(())
    }
}
