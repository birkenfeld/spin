// Spin RPC library, copyright 2015-2017 Georg Brandl.

//! Network device.

use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::time::Duration;

use device::Device;
use error::{CONFIG_ERROR, IO_ERROR, SpinResult};
use base::StringIO;


#[derive(Default)]
pub struct NetworkDevice {
    props: NetworkDeviceProps,
    timeout: f64,
    streams: Option<(BufReader<TcpStream>, TcpStream)>,
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
        host        => ("Host to connect to.", String, String::new()),
        port        => ("Port to connect to.", Uint32, 0),
    ],
);

impl NetworkDevice {
    pub fn create(_name: &str) -> Box<Device> {
        box NetworkDevice::default()
    }

    fn init(&mut self) -> SpinResult<()> {
        if self.props.host.is_empty() || self.props.port == 0 {
            return spin_err!(CONFIG_ERROR, "need a host and port != 0 configured");
        }
        self.timeout = self.props.string_io.timeout;
        let address = format!("{}:{}", self.props.host, self.props.port);
        info!("connecting to {}...", address);
        let wstream = TcpStream::connect(address.as_str())?;
        let timeout = Duration::from_millis((self.timeout * 1000.) as u64);
        wstream.set_read_timeout(Some(timeout))?;
        wstream.set_write_timeout(Some(timeout))?;
        wstream.set_nodelay(true)?;
        let rstream = BufReader::new(wstream.try_clone()?);
        self.streams = Some((rstream, wstream));
        info!("connection established to {}", address);
        Ok(())
    }

    fn delete(&mut self) {
        self.streams.take();  // consume and close the connection if it exists
    }

    fn _read_line(rstream: &mut BufReader<TcpStream>, eolstr: &str) -> SpinResult<String> {
        let eol = eolstr.as_bytes();
        let end_byte = eol[eol.len() - 1];
        let mut buffer = Vec::with_capacity(64);
        rstream.read_until(end_byte, &mut buffer)?;
        Ok(String::from_utf8_lossy(&buffer).into_owned())
    }
}

impl StringIO for NetworkDevice {
    fn cmd_communicate(&mut self, arg: String) -> SpinResult<String> {
        if let Some((ref mut rstream, ref mut wstream)) = self.streams {
            wstream.write_all(self.props.string_io.sol.as_bytes())?;
            wstream.write_all(arg.as_bytes())?;
            wstream.write_all(self.props.string_io.eol.as_bytes())?;
            Self::_read_line(rstream, &self.props.string_io.eol)
        } else {
            spin_err!(IO_ERROR, "connection not open")
        }
    }

    fn cmd_flush(&mut self, _: ()) -> SpinResult<()> {
        Ok(())
    }

    fn cmd_read(&mut self, _arg: u32) -> SpinResult<String> {
        Ok("".into())
    }

    fn cmd_write(&mut self, arg: String) -> SpinResult<u32> {
        if let Some((_, ref mut wstream)) = self.streams {
            let written = wstream.write(arg.as_bytes())?;
            Ok(written as u32)
        } else {
            spin_err!(IO_ERROR, "connection not open")
        }
    }

    fn cmd_readline(&mut self, _: ()) -> SpinResult<String> {
        if let Some((ref mut rstream, _)) = self.streams {
            Self::_read_line(rstream, &self.props.string_io.eol)
        } else {
            spin_err!(IO_ERROR, "connection not open")
        }
    }

    fn cmd_writeline(&mut self, arg: String) -> SpinResult<u32> {
        if let Some((_, ref mut wstream)) = self.streams {
            let mut written = wstream.write(self.props.string_io.sol.as_bytes())?;
            written += wstream.write(arg.as_bytes())?;
            written += wstream.write(self.props.string_io.eol.as_bytes())?;
            Ok(written as u32)
        } else {
            spin_err!(IO_ERROR, "connection not open")
        }
    }

    fn read_timeout(&mut self) -> SpinResult<f64> {
        Ok(self.timeout)
    }

    fn write_timeout(&mut self, val: f64) -> SpinResult<()> {
        self.timeout = val;
        Ok(())
    }
}
