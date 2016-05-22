// Spin RPC library, copyright 2015, 2016 Georg Brandl.
//
//! Server framework.

use std::env::current_dir;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::sync::{Arc, Mutex};
use std::thread;
use argparse::*;
use daemonize;
use zmq;

use arg::*;
use config::{ServerConfig, DevConfig};
use client::Client;
use device::{Device, run_device, general_error_reply};
use error::{SpinResult, spin_err, SOCKET_ERROR};
use util;
use logging;

pub type DevConstructor = fn(&str) -> Box<Device>;

pub struct Server {
    pub name: String,
    pub config: ServerConfig,
    pub address: util::ServerAddress,
    context: Arc<Mutex<zmq::Context>>,
    devcons: Vec<(DevConfig, DevConstructor)>,
}

impl Server {

    /// Construct a new "empty" server.
    pub fn new(name: &str, config: ServerConfig, addr: Option<String>,
               db_addr: Option<String>, use_db: bool) -> Server {
        Server {
            name: name.into(),
            address: util::ServerAddress::new(addr, db_addr, use_db),
            context: Arc::new(Mutex::new(zmq::Context::new())),
            devcons: Vec::with_capacity(config.devices.len()),
            config: config,
        }
    }

    /// Construct a new server from command-line args.
    pub fn from_args(use_db: bool, config: Option<ServerConfig>) -> Option<Server> {
        let mut name = String::new();
        let mut configfile = None;
        let mut address = None;
        let mut database = None;
        let mut debug = false;
        let mut arg_use_db = true;
        let mut log_path = String::from("log");
        let mut pid_path = String::from("pid");
        let mut daemonize = false;
        let mut user = None::<String>;
        let mut group = None::<String>;
        let result = {
            let mut ap = ArgumentParser::new();
            ap.refer(&mut name).add_argument("name", Store, "Server name.").required();
            ap.refer(&mut configfile).add_argument("config", StoreOption, "Config file.");
            ap.refer(&mut address).add_option(&["--bind"], StoreOption, "Bind [host]:[port].");
            ap.refer(&mut database).add_option(&["--db"], StoreOption, "DB [host]:[port].");
            ap.refer(&mut debug).add_option(&["-v"], StoreTrue, "Debug mode.");
            ap.refer(&mut arg_use_db).add_option(&["-n"], StoreFalse, "No database mode.");
            ap.refer(&mut log_path).add_option(&["--log"], Store, "Logging path.");
            ap.refer(&mut pid_path).add_option(&["--pid"], Store, "PID path for daemon.");
            ap.refer(&mut daemonize).add_option(&["-d"], StoreTrue, "Daemonize.");
            ap.refer(&mut user).add_option(&["--user"], StoreOption, "Daemon user.");
            ap.refer(&mut group).add_option(&["--group"], StoreOption, "Daemon group.");
            ap.parse_args()
        };
        let log_path = current_dir().unwrap().join(log_path);
        let pid_path = current_dir().unwrap().join(pid_path);
        let _ = util::ensure_dir(&pid_path);
        let _ = logging::init(&log_path, &name, debug, !daemonize);
        if daemonize {
            let pid_file = pid_path.join(name.replace("/", "-") + ".pid");
            let mut daemon = daemonize::Daemonize::new().pid_file(pid_file);
            if let Some(user) = user {
                daemon = daemon.user(user.as_str());
            }
            if let Some(group) = group {
                daemon = daemon.group(group.as_str());
            }
            if let Err(err) = daemon.start() {
                error!("could not daemonize process: {}", err);
            }
        }
        let server_config = config.unwrap_or_else(|| ServerConfig::from_file(configfile));
        result.ok().map(|_| Server::new(&name, server_config, address, database,
                                        use_db && arg_use_db))
    }

    /// Add a constructed device.
    pub fn add_device(&mut self, devconfig: DevConfig, dev_const: DevConstructor) {
        self.devcons.push((devconfig, dev_const));
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
                    return spin_err(SOCKET_ERROR, "cannot find free port");
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
            for &(ref devconfig, _) in &self.devcons {
                my_devs.push(devconfig.name.clone());
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
        for (devconfig, dev_const) in self.devcons.drain(..) {
            let inproc_addr = String::from("inproc://") + &devconfig.name;
            let mut dev_sock = util::create_socket(&self.context, zmq::REP)?;
            dev_sock.bind(&inproc_addr)?;  // must bind before connect

            let mut local_sock = util::create_socket(&self.context, zmq::REQ)?;
            local_sock.connect(&inproc_addr)?;
            devsockets.insert(devconfig.name.clone(), pollsockets.len());
            pollsockets.push(local_sock);

            let mut dev = dev_const(&devconfig.name);
            dev.set_name(devconfig.name);
            let prop_map = HashMap::from_iter(devconfig.props.into_iter()
                                              .map(|p| (p.name, p.value)));
            dev.init_props(prop_map);
            // ignore init failures here, we will retry on each RPC call
            let _ = dev.init_device();
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

#[macro_export]
macro_rules! server_main {
    (devtypes = $($toks:tt)+) => { server_main!(use_db = true, static_config = None,
                                                devtypes = $($toks)+); };
    (use_db = $use_db:expr,
     static_config = $staticconfig:expr,
     devtypes = [$($dtype:ident => $dconstr:expr),*]) => {
        match ::spin::server::Server::from_args($use_db, $staticconfig) {
            None => return,
            Some(mut server) => {
                for device in ::std::mem::replace(&mut server.config.devices, vec![]) {
                    $(
                        if device.devtype == stringify!($dtype) {
                            server.add_device(device, $dconstr);
                            continue;
                        }
                    )*;
                    warn!("device type {} for device {} not handled by this server",
                          device.devtype, device.name);
                }

                info!("server running...");
                if let Err(e) = server.run() {
                    error!("Error running server: {}", e);
                }
            }
        }
    };
}
