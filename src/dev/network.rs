// Spin RPC library, copyright 2015-2020 Georg Brandl.

//! Network device.

use std::net::TcpStream;
use std::time::Duration;

use spin::{spin_device_impl, spin_err};
use spin::device::Device;
use spin::error::{SpinResult, CONFIG_ERROR, IO_ERROR};
use spin::base::StringIO;
use spin::support::comm::{CommClient, CommThread};
use spin::validate::Mandatory;

#[derive(Default)]
pub struct NetworkDevice {
    props: NetworkDeviceProps,
    timeout: f64,
    comm: Option<CommClient<TcpStream>>,
}

spin_device_impl!(
    NetworkDevice,
    NetworkDeviceProps,
    bases = [
        string_io
    ],
    cmds = [ ],
    attrs = [ ],
    props = [
        host        => ("Host to connect to.", String, Mandatory),
        port        => ("Port to connect to.", u32, 0),
    ],
);

impl NetworkDevice {
    pub fn create(_name: &str) -> Box<dyn Device> {
        Box::new(NetworkDevice::default())
    }

    fn init(&mut self) -> SpinResult<()> {
        if self.props.host.is_empty() || self.props.port == 0 {
            return spin_err!(CONFIG_ERROR, "need a host and port != 0 configured");
        }
        self.timeout = self.props.string_io.timeout;
        let address = format!("{}:{}", self.props.host, self.props.port);
        let timeout = Duration::from_millis((self.timeout * 1000.) as u64);

        let connect = move || -> SpinResult<(TcpStream, TcpStream)> {
            log::info!("connecting to {}...", address);
            let wstream = TcpStream::connect(address.as_str())?;
            wstream.set_write_timeout(Some(timeout))?;
            wstream.set_nodelay(true)?;
            let rstream = wstream.try_clone()?;
            log::info!("connection established to {}", address);
            Ok((rstream, wstream))
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

    fn get_comm(&self) -> SpinResult<&CommClient<TcpStream>> {
        if let Some(ref comm) = self.comm {
            Ok(comm)
        } else {
            spin_err!(IO_ERROR, "connection not open")
        }
    }
}

impl StringIO for NetworkDevice {
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
