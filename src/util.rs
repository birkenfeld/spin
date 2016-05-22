// Spin RPC library, copyright 2015, 2016 Georg Brandl.

//! Utilities.

use std::io;
use std::env;
use std::fs::{File, OpenOptions, DirBuilder};
use std::path::Path;
use std::sync::{Arc, Mutex};

use zmq;
use url;

use db;
use error::{SpinResult, ADDRESS_ERROR};

/// Make it easier to write our signatures.
pub type ZmqResult<T> = Result<T, zmq::Error>;


/// Create a new Zmq socket from a thread-shared Context
pub fn create_socket(ctx: &Arc<Mutex<zmq::Context>>, ty: zmq::SocketType) -> ZmqResult<zmq::Socket> {
    let mut ctx = ctx.lock().unwrap();
    let sock = ctx.socket(ty)?;
    sock.set_linger(0)?;
    Ok(sock)
}

/// Poll a number of sockets. Return indices that have poll events.
pub fn poll_sockets(sockets: &[zmq::Socket], timeout: i64) -> Result<Vec<usize>, zmq::Error> {
    let mut items: Vec<_> = sockets.iter().map(|s| s.as_poll_item(zmq::POLLIN)).collect();
    let num_items = zmq::poll(items.as_mut_slice(), timeout)?;
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
        let msg = sock.recv_msg(0)?;
        result.push((*msg).to_vec());
        if !sock.get_rcvmore()? {
            return Ok(result);
        }
    }
}

/// Write a multipart message to a socket.
pub fn send_message(sock: &mut zmq::Socket, parts: &[&[u8]]) -> ZmqResult<()> {
    for part in parts.iter().take(parts.len() - 1) {
        sock.send(part, zmq::SNDMORE)?;
    }
    sock.send(&parts[parts.len()-1], 0)
}

/// Write a multipart message (as vec of vecs) to a socket.
pub fn send_full_message(sock: &mut zmq::Socket, parts: Vec<Vec<u8>>) -> ZmqResult<()> {
    for part in parts.iter().take(parts.len() - 1) {
        sock.send(part, zmq::SNDMORE)?;
    }
    sock.send(&parts[parts.len()-1], 0)
}


/// Helper object for server address handling.
#[derive(Debug, Clone)]
pub struct ServerAddress {
    bind_host: String,
    ext_host: String,
    pub port: u16,
    pub use_random_port: bool,
    pub use_db: bool,
    pub db_hostport: String,
}

impl ServerAddress {
    pub fn new(addr_arg: Option<String>, db_arg: Option<String>, use_db: bool) -> ServerAddress {
        let mut addr = ServerAddress { bind_host: "*".into(),
                                       ext_host: "localhost".into(),  // XXX
                                       port: 0,
                                       use_random_port: true,
                                       use_db: use_db,
                                       db_hostport: "".into() };
        let db_spec = match db_arg {
            Some(arg) => arg,
            None => match env::var("SPINDB") {
                Ok(val) => val,
                Err(_) => "".into(),
            }
        };
        if let Some((host, port)) = ServerAddress::parse_host_port(&db_spec, "localhost",
                                                                   db::DEFAULT_DB_PORT) {
            addr.db_hostport = format!("{}:{}", host, port);
        }
        if let Some(arg) = addr_arg {
            if let Some((host, port)) = ServerAddress::parse_host_port(&arg, "*", 0) {
                if host != "*" {
                    addr.ext_host = host.clone();
                }
                addr.bind_host = host;
                addr.port = port;
                if port != 0 {
                    addr.use_random_port = false;
                }
            }
        }
        addr
    }

    fn parse_host_port(arg: &str, defaulthost: &'static str,
                       defaultport: u16) -> Option<(String, u16)> {
        let mut parts_iter = arg.splitn(2, ':');
        match parts_iter.next() {
            Some(mut host) => {
                let port = parts_iter.next()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(defaultport);
                if host == "" { host = defaulthost; }
                Some((host.into(), port))
            }
            None => None,
        }
    }

    pub fn get_endpoint(&self) -> String {
        format!("tcp://{}:{}", self.bind_host, self.port)
    }

    pub fn get_ext_endpoint(&self) -> String {
        format!("tcp://{}:{}", self.ext_host, self.port)
    }
}

/// Helper object for client -> device addresses
#[derive(Debug)]
pub struct DeviceAddress {
    pub devname: String,
    pub endpoint: String,
    pub use_db: bool,
    pub db_hostport: String,
}

impl DeviceAddress {
    /// Parse a connection URI.
    pub fn parse_uri(uri: &str) -> SpinResult<DeviceAddress> {
        //let mut parser = url::Url::parse(uri);
        //parser.scheme_type_mapper(DeviceAddress::scheme_mapper);
        match url::Url::parse(uri) {
            Err(_)  => spin_err!(ADDRESS_ERROR, "invalid device address"),
            Ok(uri) => {
                if uri.scheme() != "spin" && uri.scheme() != "spindb" {
                    return spin_err!(ADDRESS_ERROR, "invalid scheme");
                }
                let host = uri.domain().unwrap_or("localhost");
                let port = uri.port().unwrap_or(9999);
                let path = uri.path()[1..].to_owned();
                Ok(DeviceAddress {
                    devname: path,
                    endpoint: format!("tcp://{}:{}", host, port),
                    use_db: uri.scheme() == "spindb",
                    db_hostport: format!("{}:{}", host, port),
                })
            }
        }
    }
}


/// A less verbose way of opening files.
pub fn open_file<P: AsRef<Path>>(path: P, mode: &str) -> io::Result<File> {
    let mut opt = OpenOptions::new();
    for ch in mode.chars() {
        match ch {
            'r' => { opt.read(true); },
            'w' => { opt.write(true).create(true); },
            'a' => { opt.write(true).append(true); },
            _   => { },  // ignore unsupported chars
        }
    }
    opt.open(path)
}


/// mkdir -p utility.
pub fn ensure_dir<P: AsRef<Path>>(path: P) -> io::Result<()> {
    if path.as_ref().is_dir() {
        return Ok(());
    }
    DirBuilder::new().recursive(true).create(path)
}
