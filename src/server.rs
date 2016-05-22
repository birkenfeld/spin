// Spin RPC library, copyright 2015 Georg Brandl.
//
//! Server framework.

use std::collections::HashMap;
use std::io;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;

use argparse::*;
use zmq;

use arg::*;
use client::Client;
use device::{Device, run_device, general_error_reply};
use error::{SpinResult, spin_err};
use util;


#[allow(dead_code)]
pub struct Server<'srv> {
    name: String,
    pub address: util::ServerAddress,
    context: Arc<Mutex<zmq::Context>>,
    clsmap: HashMap<String, &'srv str>,
    devmap: HashMap<String, Box<Device>>,
    debug: bool,
}

impl<'srv> Server<'srv> {

    /// Construct a new "empty" server.
    pub fn new(name: &str, address: Option<&str>, debug: bool) -> Server<'srv> {
        util::setup_logging(debug);
        Server {
            name: name.into(),
            address: util::ServerAddress::parse(address),
            context: Arc::new(Mutex::new(zmq::Context::new())),
            clsmap: HashMap::new(),
            devmap: HashMap::new(),
            debug: debug,
        }
    }

    /// Construct a new server from command-line args.
    pub fn from_args() -> Option<Server<'srv>> {
        let mut name = String::from("");
        let mut address = String::from("");
        let mut debug = false;
        let result = {
            let mut ap = ArgumentParser::new();
            ap.refer(&mut name).add_argument("name", Store, "Server name.").required();
            ap.refer(&mut address).add_option(&["-b"], Store, "Bind address.");
            ap.refer(&mut debug).add_option(&["-d"], StoreTrue, "Debug.");
            ap.parse_args()
        };
        result.ok().map(|_| Server::new(&name, Some(&address), debug))
    }

    /// Add a constructed device.
    pub fn add_device(&mut self, dev: Box<Device>) {
        self.devmap.insert(dev.get_name().into(), dev);
    }

    const MIN_PORT: u16 = 11000;
    const MAX_PORT: u16 = 65000;

    /// Bind the external socket.
    fn bind_external(&mut self) -> SpinResult<zmq::Socket> {
        // external socket that takes requests
        let mut sock = try!(util::create_socket(&self.context, zmq::ROUTER));
        if self.address.srv_port == 0 {
            // random port!
            let mut port = Server::MIN_PORT;
            loop {
                let addr = format!("tcp://{}:{}", self.address.srv_host, port);
                match sock.bind(&addr) {
                    Ok(_) => break,
                    Err(zmq::Error::EADDRINUSE) => port += 1,
                    Err(e) => return Err(e.into()),
                }
                if port > Server::MAX_PORT {
                    return spin_err("SocketError", "cannot find free port");
                }
            }
            self.address.srv_port = port;
        } else {
            let addr = format!("tcp://{}:{}", self.address.srv_host, self.address.srv_port);
            try!(sock.bind(&addr));
        }
        Ok(sock)
    }

    fn register_to_db(&mut self) -> SpinResult<()> {
        if let Some((ref host, ref port)) = self.address.db_addr {
            let db_addr = format!("tcp://{}:{}", host, port);
            debug!("{:?}", db_addr);
            let mut db_cl = try!(Client::new(&db_addr, "sys/spin/db"));
            let my_addr = format!("tcp://{}:{}", self.address.srv_host, self.address.srv_port);
            try!(db_cl.exec_cmd("Register", Value::from(my_addr)));
        }
        Ok(())
    }

    fn start_devices(&mut self,
                     pollsockets: &mut Vec<zmq::Socket>)
                     -> SpinResult<HashMap<String, usize>> {
        let mut devsockets = HashMap::new();
        // create a socket pair and a thread for every device
        for (name, dev) in self.devmap.drain() {
            let inproc_addr = String::from("inproc://") + &name;
            let mut dev_sock = try!(util::create_socket(&self.context, zmq::REP));
            try!(dev_sock.bind(&inproc_addr));  // must bind before connect

            let mut local_sock = try!(util::create_socket(&self.context, zmq::REQ));
            try!(local_sock.connect(&inproc_addr));
            devsockets.insert(name, pollsockets.len());
            pollsockets.push(local_sock);
            
            thread::spawn(move || {
                // moves dev_sock and dev into this thread
                run_device(dev_sock, dev);
            });
        }
        Ok(devsockets)
    }

    fn msg_from_extern(&mut self, msg: Vec<Vec<u8>>,
                       devsockets: &HashMap<String, usize>,
                       pollsockets: &mut Vec<zmq::Socket>) -> SpinResult<()> {
        // check number of message parts:
        // 0 - zmq client socket id
        // 1 - empty
        // 2 - device name
        // 3 - serialized request
        if msg.len() < 4 {
            warn!("ill formed message");
            // no need to send a serialized error response; client doesn't observe our protocol
            try!(util::send_message(&mut pollsockets[0], &[&msg[0], &msg[1], &[], &[]]));
            return Ok(());
        }
        // must decode the device name from bytes
        match String::from_utf8(msg[2].clone()) {
            Err(_) => {
                warn!("invalid utf-8 in device name");
                let rsp = try!(general_error_reply("DeviceError", "ill formed message", &msg[3]));
                try!(util::send_message(&mut pollsockets[0], &[&msg[0], &msg[1], &msg[2], &rsp]));
            },
            Ok(ref devname) => {
                match devsockets.get(devname) {
                    None => {
                        warn!("device not found: {}", devname);
                        let rsp = try!(general_error_reply("DeviceError", "no such device", &msg[3]));
                        try!(util::send_message(&mut pollsockets[0], &[&msg[0], &msg[1], &msg[2], &rsp]));
                    },
                    Some(&sindex) => {
                        let sock = &mut pollsockets[sindex];
                        try!(util::send_full_message(sock, msg));
                    },
                }
            }
        }
        Ok(())
    }

    /// Create a thread for each device and run the server main loop.
    pub fn run(mut self) -> SpinResult<()> {
        // bind external interface
        let ext_sock = try!(self.bind_external());

        try!(self.register_to_db());

        let mut pollsockets = Vec::new();
        pollsockets.push(ext_sock);

        // create a thread per device and add its socket to pollsockets
        let devsockets = try!(self.start_devices(&mut pollsockets));

        // run main loop
        loop {
            for index in try!(util::poll_sockets(&pollsockets, 1000)) {
                // receive a message
                let msg = {
                    let ref mut socket = pollsockets[index];
                    try!(util::recv_message(socket))
                };
                if index == 0 {
                    // if it came from outside, forward it
                    try!(self.msg_from_extern(msg, &devsockets, &mut pollsockets));
                } else {
                    // else it is a reply, send it back to outside
                    try!(util::send_full_message(&mut pollsockets[0], msg));
                    // TMP: give some indication of successful request
                    io::stderr().write(&['.' as u8]).unwrap();
                    io::stderr().flush().unwrap();
                }
            }
        }
    }
}
