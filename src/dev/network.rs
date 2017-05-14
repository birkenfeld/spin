// Spin RPC library, copyright 2015-2017 Georg Brandl.

//! Network device.

use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::time::Duration;

use device::Device;
use error::{CONFIG_ERROR, IO_ERROR, SpinResult};


#[derive(Default)]
pub struct NetworkDevice {
    props: NetworkDeviceProps,
    timeout: f64,
    streams: Option<(BufReader<TcpStream>, TcpStream)>,
}

spin_device_impl!(
    NetworkDevice,
    NetworkDeviceProps,
    cmds = [
        Communicate => ("Send string, receive string.", String, String, cmd_communicate),
        Flush       => ("Flush all data.", Void, Void, cmd_flush),
        Read        => ("Read at most N chars.", UInt32, String, cmd_read),
        Write       => ("Write a string.", String, UInt32, cmd_write),
        ReadLine    => ("Read a line.", Void, String, cmd_readline),
        WriteLine   => ("Write a line.", String, UInt32, cmd_writeline),
    ],
    attrs = [
        timeout     => ("Timeout for communication.", Double, read_timeout, write_timeout),
    ],
    props = [
        host        => ("Host to connect to.", String, String::new()),
        port        => ("Port to connect to.", UInt32, 0),
        sol         => ("Start-of-line string.", String, String::new()),
        eol         => ("End-of-line string.", String, String::from("\n")),
        timeout     => ("Initial timeout.", Double, 2.0),
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
        self.timeout = self.props.timeout;
        let address = format!("{}:{}", self.props.host, self.props.port);
        info!("connecting to {}...", address);
        let wstream = TcpStream::connect(address.as_str())?;
        let timeout = Duration::from_millis((self.timeout * 1000.) as u64);
        wstream.set_read_timeout(Some(timeout))?;
        wstream.set_write_timeout(Some(timeout))?;
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

    fn cmd_communicate(&mut self, arg: String) -> SpinResult<String> {
        if let Some((ref mut rstream, ref mut wstream)) = self.streams {
            wstream.write_all(self.props.sol.as_bytes())?;
            wstream.write_all(arg.as_bytes())?;
            wstream.write_all(self.props.eol.as_bytes())?;
            Self::_read_line(rstream, &self.props.eol)
        } else {
            spin_err!(IO_ERROR, "connection not open")
        }
    }

    fn cmd_flush(&self, _: ()) -> SpinResult<()> {
        Ok(())
    }

    fn cmd_read(&self, _arg: u32) -> SpinResult<String> {
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
            Self::_read_line(rstream, &self.props.eol)
        } else {
            spin_err!(IO_ERROR, "connection not open")
        }
    }

    fn cmd_writeline(&mut self, arg: String) -> SpinResult<u32> {
        if let Some((_, ref mut wstream)) = self.streams {
            let mut written = wstream.write(self.props.sol.as_bytes())?;
            written += wstream.write(arg.as_bytes())?;
            written += wstream.write(self.props.eol.as_bytes())?;
            Ok(written as u32)
        } else {
            spin_err!(IO_ERROR, "connection not open")
        }
    }

    fn read_timeout(&self) -> SpinResult<f64> {
        Ok(self.timeout)
    }

    fn write_timeout(&mut self, val: f64) -> SpinResult<()> {
        self.timeout = val;
        Ok(())
    }
}
