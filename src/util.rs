// Spin RPC library, copyright 2015 Georg Brandl.
//
//! Utilities.

use std::env;
use std::sync::{Arc, Mutex};

use log;
use log::LogLevelFilter::*;
use fern;
use time;
use zmq;
use url;

use db;
use error::{SpinResult, spin_err};

/// Make it easier to write our signatures.
pub type ZmqResult<T> = Result<T, zmq::Error>;


/// Create a new Zmq socket from a thread-shared Context
pub fn create_socket(ctx: &Arc<Mutex<zmq::Context>>, ty: zmq::SocketType) -> ZmqResult<zmq::Socket> {
    let mut ctx = ctx.lock().unwrap();
    ctx.socket(ty)
}

/// Poll a number of sockets. Return indices that have poll events.
pub fn poll_sockets<'a>(sockets: &Vec<zmq::Socket>,
                        timeout: i64) -> Result<Vec<usize>, zmq::Error> {
    let mut items: Vec<_> = sockets.iter().map(|s| s.as_poll_item(zmq::POLLIN)).collect();
    let num_items = try!(zmq::poll(items.as_mut_slice(), timeout));
    let mut rv = Vec::new();
    if num_items > 0 {
        for (i, item) in items.iter().enumerate() {
            if item.get_revents() & zmq::POLLIN > 0 {
                rv.push(i);
            }
        }
    }
    Ok(rv)
}

/// Get a multipart message from a socket.
pub fn recv_message(sock: &mut zmq::Socket) -> ZmqResult<Vec<Vec<u8>>> {
    let mut result = Vec::new();
    loop  {
        let msg = try!(sock.recv_msg(0));
        result.push((*msg).to_vec());
        if !try!(sock.get_rcvmore()) {
            return Ok(result);
        }
    }
}

/// Write a multipart message to a socket.
pub fn send_message(sock: &mut zmq::Socket, parts: &[&[u8]]) -> ZmqResult<()> {
    for i in 0..parts.len()-1 {
        try!(sock.send(&parts[i], zmq::SNDMORE));
    }
    sock.send(&parts[parts.len()-1], zmq::DONTWAIT)
}

/// Write a multipart message (as vec of vecs) to a socket.
pub fn send_full_message(sock: &mut zmq::Socket, parts: Vec<Vec<u8>>) -> ZmqResult<()> {
    for i in 0..parts.len()-1 {
        try!(sock.send(&parts[i], zmq::SNDMORE));
    }
    sock.send(&parts[parts.len()-1], zmq::DONTWAIT)
}


/// Helper object for server address handling.
#[derive(Debug)]
pub struct ServerAddress {
    // we don't use SocketAddr because zmq doesn't either
    pub srv_host: String,
    pub srv_port: u16,  // 0 means: random
    pub db_addr: Option<(String, u16)>,
}

impl ServerAddress {
    pub fn parse(arg: Option<&str>) -> ServerAddress {
        let mut addr = ServerAddress { srv_host: "*".into(), srv_port: 0, db_addr: None };
        if let Ok(ref val) = env::var("SPINDB") {
            addr.db_addr = ServerAddress::parse_host_port(val, db::DEFAULT_DB_PORT);
        }
        if let Some(arg) = arg {
            if let Some((host, port)) = ServerAddress::parse_host_port(arg, 0) {
                addr.srv_host = host;
                addr.srv_port = port;
            }
        }
        addr
    }

    fn parse_host_port(arg: &str, defaultport: u16) -> Option<(String, u16)> {
        let mut parts_iter = arg.splitn(2, ':');
        match parts_iter.next() {
            Some(mut host) => {
                let port = parts_iter.next()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(defaultport);
                if host == "" { host = "*"; }
                Some((host.into(), port))
            }
            None => None,
        }
    }
}

/// Helper object for client -> device addresses
#[derive(Debug)]
pub struct DeviceAddress {
    pub devname: String,
    pub endpoint: String,
    pub use_db: bool,
}

impl DeviceAddress {
    fn scheme_mapper(scheme: &str) -> url::SchemeType {
        match scheme {
            "spin" => url::SchemeType::Relative(0),
            "spindb" => url::SchemeType::Relative(9999),
            _ => url::SchemeType::NonRelative,
        }
    }
    
    /// Parse a connection URI.
    pub fn parse_uri(uri: &str) -> SpinResult<DeviceAddress> {
        let mut parser = url::UrlParser::new();
        parser.scheme_type_mapper(DeviceAddress::scheme_mapper);
        match parser.parse(uri) {
            Err(_)  => spin_err("AddressError", "invalid device address"),
            Ok(uri) => {
                if uri.scheme != "spin" && uri.scheme != "spindb" {
                    return spin_err("AddressError", "invalid scheme");
                }
                let host = uri.domain().unwrap_or("localhost");
                let port = uri.port().unwrap_or(9999);
                let path = match uri.serialize_path() {
                    None => return spin_err("AddressError", "no devname found"),
                    Some(ser) => String::from(&ser[1..])
                };
                Ok(DeviceAddress {
                    devname: path,
                    endpoint: format!("tcp://{}:{}", host, port),
                    use_db: uri.scheme == "spindb",
                })
            }
        }
    }
}


/// Function for log formatting.
fn log_format(msg: &str, level: &log::LogLevel,
              _location: &log::LogLocation) -> String {
    format!("[{}][{}] {}", time::now().strftime("%H:%M:%S").unwrap(),
            level, msg)
}

/// Configure logging.
pub fn setup_logging(debug: bool) {
    let loglevel = if debug { Debug } else { Info };
    let log_config = fern::DispatchConfig {
        format: box log_format,
        output: vec![fern::OutputConfig::stdout()],
        level:  loglevel,
    };
    fern::init_global_logger(log_config, loglevel).unwrap();
}
