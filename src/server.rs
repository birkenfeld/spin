// Spin RPC library, copyright 2015 Georg Brandl.
//
//! Server framework.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

use zmq;

use device::{Device, run_device};
use error::SpinResult;
use util;


#[allow(dead_code)]
pub struct Server<'srv> {
    name: String,
    context: Arc<Mutex<zmq::Context>>,
    clsmap: HashMap<String, &'srv str>,
    devmap: HashMap<String, Box<Device>>,
}

impl<'srv> Server<'srv> {

    /// Construct a new "empty" server.
    pub fn new(name: &str) -> Server<'srv> {
        Server {
            name: name.into(),
            context: Arc::new(Mutex::new(zmq::Context::new())),
            clsmap: HashMap::new(),
            devmap: HashMap::new(),
        }
    }

    /// Configure the server from command line arguments.
    pub fn config_from_args(&mut self, _args: &Vec<String>) {
        //unimplemented!();
    }

    /// Add a constructed device.
    pub fn add_device(&mut self, dev: Box<Device>) {
        self.devmap.insert(dev.get_name().into(), dev);
    }

    fn start_devices(&mut self,
                     pollsockets: &mut Vec<zmq::Socket>)
                     -> SpinResult<HashMap<String, usize>> {
        let mut sendsockets = HashMap::new();
        // create a socket pair and a thread for every device
        for (name, dev) in self.devmap.drain() {
            let inproc_addr = String::from("inproc://") + &name;
            let mut dev_sock = try!(util::create_socket(&self.context, zmq::REP));
            try!(dev_sock.bind(&inproc_addr));  // must bind before connect

            let mut local_sock = try!(util::create_socket(&self.context, zmq::REQ));
            try!(local_sock.connect(&inproc_addr));
            sendsockets.insert(name, pollsockets.len());
            pollsockets.push(local_sock);
            
            thread::spawn(move || {
                // moves dev_sock and dev into this thread
                run_device(dev_sock, dev);
            });
        }
        Ok(sendsockets)
    }

    /// Create a thread for each device and run the server main loop.
    pub fn run(mut self) -> SpinResult<()> {
        // external socket that takes requests
        let mut ext_sock = try!(util::create_socket(&self.context, zmq::ROUTER));
        try!(ext_sock.bind("tcp://*:1357"));

        let mut pollsockets = Vec::new();
        pollsockets.push(ext_sock);

        // create a thread per device and add its socket to pollsockets
        let sendsockets = try!(self.start_devices(&mut pollsockets));

        // run main loop
        loop {
            for index in try!(util::poll_sockets(&pollsockets, 1000)) {
                // receive a message
                let msg = {
                    let ref mut socket = pollsockets[index];
                    let msg = try!(util::recv_message(socket)); // XXX
                    if msg.len() < 4 {
                        // XXX send back error
                        println!("ill formed message");
                        continue;
                    }
                    msg
                };
                // forward it
                if index == 0 {  // the external receiver
                    match String::from_utf8(msg[2].clone()) {
                        Err(_) => {
                            println!("invalid utf-8 in device name");
                            continue;
                        },
                        Ok(ref devname) => {
                            match sendsockets.get(devname) {
                                Some(&sindex) => {
                                    let sock = &mut pollsockets[sindex];
                                    try!(util::send_full_message(sock, msg));
                                },
                                None => {
                                    println!("no send socket found");
                                    continue;
                                    // XXX send back error
                                },
                            }
                        }
                    }
                } else {  // the internal sockets
                    let sock = &mut pollsockets[0];
                    try!(util::send_full_message(sock, msg));
                }
            }
        }
    }
}
