// Spin RPC library, copyright 2015-2017 Georg Brandl.

//! Server framework.

use std::env::current_dir;
use std::iter::FromIterator;
use std::thread;
use fnv::FnvHashMap as HashMap;
use fnv::FnvHashSet as HashSet;
use argparse::*;
use daemonize;
use mlzlog;
use zmq;

use arg::Value;
use config::{ServerConfig, DevConfig};
use device::{Device, run_device, general_error_reply};
use error::{SpinResult, SOCKET_ERROR};
use client::Client;
use util;

pub type DevConstructor = fn(&str) -> Box<Device>;

pub struct Server {
    pub name: String,
    pub config: ServerConfig,
    pub address: util::ServerAddress,
    devcons: Vec<(DevConfig, DevConstructor)>,
}

impl Server {

    /// Construct a new "empty" server.
    pub fn new(name: &str, config: ServerConfig, addr: Option<String>,
               db_addr: Option<String>, use_db: bool) -> Server {
        Server {
            name: name.into(),
            address: util::ServerAddress::new(addr, db_addr, use_db),
            devcons: Vec::with_capacity(config.devices.len()),
            config,
        }
    }

    /// Construct a new server from command-line args.
    pub fn from_args(use_db: bool, config: Option<ServerConfig>) -> Option<Server> {
        let mut name = String::new();
        let mut configfile = None;
        let mut address = None;
        let mut database = String::new();
        let mut debug = false;
        let mut arg_use_db = true;
        let mut log_path = String::from("log");
        let mut pid_path = String::from("pid");
        let mut daemonize = false;
        let mut user = None::<String>;
        let mut group = None::<String>;
        let result = {
            let mut ap = ArgumentParser::new();
            ap.set_description("Starts a spin server.");
            ap.refer(&mut name)
              .add_argument("name", Store, "server name (name/instance)").required();
            ap.refer(&mut configfile)
              .add_argument("config", StoreOption, "config file path");
            ap.refer(&mut debug)
              .add_option(&["-v"], StoreTrue, "if given, log debug messages");
            ap.refer(&mut address)
              .add_option(&["--bind"], StoreOption, "bind address (default is a random port)")
              .metavar("[HOST]:[PORT]");
            ap.refer(&mut database)
              .envvar("SPIN_DB")
              .add_option(&["--db"], Store, "database address (default is $SPIN_DB)")
              .metavar("[HOST]:[PORT]");
            ap.refer(&mut arg_use_db)
              .add_option(&["-n"], StoreFalse, "if given, don't register with database");
            ap.refer(&mut log_path)
              .envvar("SPIN_LOGPATH")
              .add_option(&["--log"], Store, "path for logfiles (default is ./log)");
            ap.refer(&mut pid_path)
              .envvar("SPIN_PIDPATH")
              .add_option(&["--pid"], Store, "path for PID files when damonized \
                                              (default is ./pid)");
            ap.refer(&mut daemonize)
              .add_option(&["-d"], StoreTrue, "if given, daemonize at startup");
            ap.refer(&mut user)
              .add_option(&["--user"], StoreOption, "name of user to become as daemon");
            ap.refer(&mut group)
              .add_option(&["--group"], StoreOption, "name of group to become as daemon");
            ap.parse_args()
        };
        let log_path = current_dir().unwrap().join(log_path).join(name.replace("/", "-"));
        let pid_path = current_dir().unwrap().join(pid_path);
        let _ = util::ensure_dir(&pid_path);
        let _ = mlzlog::init(&log_path, &name, true, debug, !daemonize);
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
        let database = if database.is_empty() { None } else { Some(database) };
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
        let sock = util::create_socket(zmq::ROUTER)?;
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
                    return spin_err!(SOCKET_ERROR, "cannot find free port");
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

    fn start_device_responder(&mut self) -> SpinResult<()> {
        let devnames = HashSet::from_iter(self.devcons.iter().map(|v| v.0.name.clone().into_bytes()));
        let sock = util::create_socket(zmq::REP)?;
        sock.bind("inproc://device_responder")?;
        thread::spawn(move || {
            loop {
                if let Ok(devname) = sock.recv_bytes(0) {
                    let rep = if devnames.contains(&devname) { b"1" } else { b"0" };
                    sock.send(rep, 0).unwrap();
                }
            }
        });
        Ok(())
    }

    fn start_devices(&mut self, pollsockets: &mut Vec<zmq::Socket>)
                     -> SpinResult<HashMap<Vec<u8>, usize>> {
        let mut devsockets = HashMap::default();
        // for every device...
        for (devconfig, dev_constructor) in self.devcons.drain(..) {
            // create an in-process socket pair
            let inproc_addr = format!("inproc://{}", devconfig.name);
            let dev_sock = util::create_socket(zmq::REP)?;
            dev_sock.bind(&inproc_addr)?;  // must bind before connect

            let local_sock = util::create_socket(zmq::REQ)?;
            local_sock.connect(&inproc_addr)?;

            // store index of our local_sock in the "devsockets" as a Vec<u8>
            devsockets.insert(devconfig.name.clone().into(), pollsockets.len());
            pollsockets.push(local_sock);

            // create the device
            let mut dev = dev_constructor(&devconfig.name);
            dev.set_name(devconfig.name);
            let prop_map = HashMap::from_iter(devconfig.props.into_iter()
                                              .map(|p| (p.name, p.value)));
            // try to init properties -- if this fails cancel everything
            // XXX we don't have the device name in the error message
            dev.init_props(prop_map)?;
            // init the device proper -- ignore init failures here, we have a
            // chance to retry on each RPC call
            let _ = dev.init_device();
            thread::spawn(move || {
                // moves dev_sock and dev into this thread
                run_device(dev_sock, dev);
            });
        }
        Ok(devsockets)
    }

    fn msg_from_extern(&mut self, msg: Vec<Vec<u8>>,
                       devsockets: &HashMap<Vec<u8>, usize>,
                       pollsockets: &mut Vec<zmq::Socket>) -> SpinResult<()> {
        // check number of message parts:
        // 0 - zmq client socket id
        // 1 - empty
        // 2 - device name
        // 3 - serialized request
        if msg.len() != 4 {
            warn!("ill formed message");
            // no need to send a serialized error response; client doesn't observe our protocol
            pollsockets[0].send_multipart(&[&msg[0], &[], &[], &[]], 0)?;
            return Ok(());
        }
        let devname = &msg[2];
        match devsockets.get(devname) {
            None => {
                warn!("device not found: {}", String::from_utf8_lossy(devname));
                let rsp = general_error_reply("DeviceError", "no such device", &msg[3])?;
                pollsockets[0].send_multipart(&[&msg[0], &[], devname, &rsp], 0)?;
            },
            Some(&sindex) => {
                // send request on to the device thread
                util::send_full_message(&mut pollsockets[sindex], &msg)?;
            },
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

        // create a thread that replies to "is this a local device" queries
        self.start_device_responder()?;

        // create a thread per device and add its socket to pollsockets
        let devsockets = self.start_devices(&mut pollsockets)?;

        // run main loop
        info!("waiting for requests...");
        loop {
            for index in util::poll_sockets(&pollsockets, 1000)? {
                // receive a message
                let msg = pollsockets[index].recv_multipart(0)?;
                if index == 0 {
                    // if it came from outside, forward it
                    self.msg_from_extern(msg, &devsockets, &mut pollsockets)?;
                } else {
                    // else it is a reply, send it back to outside
                    util::send_full_message(&mut pollsockets[0], &msg)?;
                }
            }
        }
    }
}

#[macro_export]
macro_rules! spin_server_main {
    (devtypes = $($toks:tt)+) => { spin_server_main!(use_db = true, static_config = None,
                                                     devtypes = $($toks)+); };
    (use_db = $use_db:expr,
     static_config = $staticconfig:expr,
     devtypes = [$($dtype:ident => $dconstr:expr),* $(,)*]) => {
        match $crate::server::Server::from_args($use_db, $staticconfig) {
            None => return,
            Some(mut server) => {
                info!("creating devices...");
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
                    error!("server stopped: {}", e);
                }
            }
        }
    };
}
