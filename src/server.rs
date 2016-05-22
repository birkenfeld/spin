// Spin RPC library, copyright 2015, 2016 Georg Brandl.
//
//! Server framework.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

use argparse::*;
use zmq;

use arg::*;
use client::Client;
use device::{Device, run_device, general_error_reply};
use error::{SpinResult, spin_err};
use util;

pub type DevConstructor = fn(String) -> Box<Device>;

#[allow(dead_code)]
pub struct Server<'srv> {
    name: String,
    pub address: util::ServerAddress,
    context: Arc<Mutex<zmq::Context>>,
    clsmap: HashMap<String, &'srv str>,
    devmap: HashMap<String, DevConstructor>,
    debug: bool,
}

impl<'srv> Server<'srv> {

    /// Construct a new "empty" server.
    pub fn new(name: &str, addr: Option<String>, db_addr: Option<String>,
               use_db: bool, debug: bool) -> Server<'srv> {
        util::setup_logging(debug);
        Server {
            name: name.into(),
            address: util::ServerAddress::new(addr, db_addr, use_db),
            context: Arc::new(Mutex::new(zmq::Context::new())),
            clsmap: HashMap::new(),
            devmap: HashMap::new(),
            debug: debug,
        }
    }

    /// Construct a new server from command-line args.
    pub fn from_args(use_db: bool) -> Option<Server<'srv>> {
        let mut name = String::from("");
        let mut address = None;
        let mut database = None;
        let mut debug = false;
        let mut arg_use_db = true;
        let result = {
            let mut ap = ArgumentParser::new();
            ap.refer(&mut name).add_argument("name", Store, "Server name.").required();
            ap.refer(&mut address).add_option(&["-b"], StoreOption, "Bind [host]:[port].");
            ap.refer(&mut database).add_option(&["-d"], StoreOption, "DB [host]:[port].");
            ap.refer(&mut debug).add_option(&["-v"], StoreTrue, "Debug.");
            ap.refer(&mut arg_use_db).add_option(&["-n"], StoreFalse, "No database mode.");
            ap.parse_args()
        };
        result.ok().map(|_| Server::new(&name, address, database, use_db && arg_use_db, debug))
    }

    /// Add a constructed device.
    pub fn add_device(&mut self, name: String, dev_const: DevConstructor) {
        self.devmap.insert(name, dev_const);
    }

    /// Bind the external socket.
    fn bind_external(&mut self) -> SpinResult<zmq::Socket> {
        const MIN_PORT: u16 = 11000;
        const MAX_PORT: u16 = 65000;

        // external socket that takes requests
        let mut sock = util::create_socket(&self.context, zmq::ROUTER)?;
        if self.address.use_random_port {
            // random port!
            let mut port = MIN_PORT;
            loop {
                self.address.port = port;
                match sock.bind(&self.address.get_endpoint()) {
                    Ok(_) => break,
                    Err(zmq::Error::EADDRINUSE) => port += 1,
                    Err(e) => return Err(e.into()),
                }
                if port > MAX_PORT {
                    return spin_err("SocketError", "cannot find free port");
                }
            }
        } else {
            sock.bind(&self.address.get_endpoint())?;
        }
        info!("bound to {}", self.address.get_endpoint());
        Ok(sock)
    }

    fn register_to_db(&mut self) -> SpinResult<()> {
        if self.address.use_db {
            let db_uri = format!("spin://{}/sys/spin/db", self.address.db_hostport);
            info!("registering to database: {:?}", db_uri);
            let mut db_cl = Client::new(&db_uri)?;
            let mut my_devs = vec![self.address.get_ext_endpoint(),
                                   self.name.clone()];
            for name in self.devmap.keys() {
                my_devs.push(name.clone());
            }
            debug!("db register: {:?}", my_devs);
            db_cl.exec_cmd("Register", Value::from(my_devs))?;
            info!("   ... done");
        }
        Ok(())
    }

    fn start_devices(&mut self, pollsockets: &mut Vec<zmq::Socket>)
                     -> SpinResult<HashMap<String, usize>> {
        let mut devsockets = HashMap::new();
        // create a socket pair and a thread for every device
        for (name, dev_const) in self.devmap.drain() {
            let inproc_addr = String::from("inproc://") + &name;
            let mut dev_sock = util::create_socket(&self.context, zmq::REP)?;
            dev_sock.bind(&inproc_addr)?;  // must bind before connect

            let mut local_sock = util::create_socket(&self.context, zmq::REQ)?;
            local_sock.connect(&inproc_addr)?;
            devsockets.insert(name.clone(), pollsockets.len());
            pollsockets.push(local_sock);

            thread::spawn(move || {
                let dev = dev_const(name);
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
            util::send_message(&mut pollsockets[0], &[&msg[0], &msg[1], &[], &[]])?;
            return Ok(());
        }
        // must decode the device name from bytes
        match String::from_utf8(msg[2].clone()) {
            Err(_) => {
                warn!("invalid utf-8 in device name");
                let rsp = general_error_reply("DeviceError", "ill formed message", &msg[3])?;
                util::send_message(&mut pollsockets[0], &[&msg[0], &msg[1], &msg[2], &rsp])?;
            },
            Ok(ref devname) => {
                match devsockets.get(devname) {
                    None => {
                        warn!("device not found: {}", devname);
                        let rsp = general_error_reply("DeviceError", "no such device", &msg[3])?;
                        util::send_message(&mut pollsockets[0], &[&msg[0], &msg[1], &msg[2], &rsp])?;
                    },
                    Some(&sindex) => {
                        let sock = &mut pollsockets[sindex];
                        util::send_full_message(sock, msg)?;
                    },
                }
            }
        }
        Ok(())
    }

    /// Create a thread for each device and run the server main loop.
    pub fn run(mut self) -> SpinResult<()> {
        // bind external interface
        let ext_sock = self.bind_external()?;

        self.register_to_db()?;

        let mut pollsockets = Vec::new();
        pollsockets.push(ext_sock);

        // create a thread per device and add its socket to pollsockets
        let devsockets = self.start_devices(&mut pollsockets)?;

        // run main loop
        info!("waiting for requests...");
        loop {
            for index in util::poll_sockets(&pollsockets, 1000)? {
                // receive a message
                let msg = {
                    let socket = &mut pollsockets[index];
                    util::recv_message(socket)?
                };
                if index == 0 {
                    // if it came from outside, forward it
                    self.msg_from_extern(msg, &devsockets, &mut pollsockets)?;
                } else {
                    // else it is a reply, send it back to outside
                    util::send_full_message(&mut pollsockets[0], msg)?;
                }
            }
        }
    }
}
