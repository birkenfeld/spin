// Spin RPC library, copyright 2015-2018 Georg Brandl.

//! Server framework.

use std::iter::FromIterator;
use std::path::PathBuf;
use std::thread;
use fxhash::FxHashMap as HashMap;
use fxhash::FxHashSet as HashSet;
use structopt::{StructOpt, clap};
use daemonize;
use mlzlog;
use zmq;

use arg::Value;
use config::{DevConfig, ServerConfig};
use device::{general_error_reply, run_device, Device};
use error::{SpinResult, SOCKET_ERROR};
use client::Client;
use util;

pub type DevConstructor = fn(&str) -> Box<Device>;

#[derive(StructOpt)]
#[structopt(author="")]
#[structopt(about="Starts a spin server.")]
#[structopt(raw(setting="clap::AppSettings::UnifiedHelpMessage"))]
struct ServerArgs {
    #[structopt(help="server name (name/instance)")]
    name: String,
    #[structopt(help="config file path")]
    configfile: Option<String>,
    #[structopt(short="v", help="if given, log debug messages")]
    verbose: bool,
    #[structopt(long="bind", value_name="[HOST]:[PORT]",
                help="bind address (default is a random port)")]
    bind: Option<String>,
    #[structopt(long="db", env="SPIN_DB", value_name="[HOST]:[PORT]",
                help="database address")]
    database: Option<String>,
    #[structopt(short="n", help="if given, don't register with database")]
    no_db: bool,
    #[structopt(long="log", env="SPIN_LOGPATH", default_value="./log",
                help="path for logfiles")]
    log_path: PathBuf,
    #[structopt(long="pid", env="SPIN_PIDPATH", default_value="./pid",
                help="path for PID files when daemonized")]
    pid_path: PathBuf,
    #[structopt(short="d", help="if given, daemonize at startup")]
    daemonize: bool,
    #[structopt(long="user", help="user name for daemon process")]
    user: Option<String>,
    #[structopt(long="group", help="group name for daemon process")]
    group: Option<String>,
}

pub struct Server {
    pub name: String,
    pub config: ServerConfig,
    pub address: util::ServerAddress,
    devcons: Vec<(DevConfig, DevConstructor)>,
}

impl Server {
    /// Construct a new "empty" server.
    pub fn new(
        name: &str,
        config: ServerConfig,
        addr: Option<String>,
        db_addr: Option<String>,
        use_db: bool,
    ) -> Server {
        Server {
            name: name.into(),
            address: util::ServerAddress::new(addr, db_addr, use_db),
            devcons: Vec::with_capacity(config.devices.len()),
            config,
        }
    }

    /// Construct a new server from command-line args.
    pub fn from_args(use_db: bool, config: Option<ServerConfig>) -> Server {
        let args = ServerArgs::from_args();
        let log_path = args.log_path.join(args.name.replace("/", "-"));
        let _ = util::ensure_dir(&args.pid_path);
        let _ = mlzlog::init(Some(&log_path), &args.name, true, args.verbose, !args.daemonize);
        if args.daemonize {
            let pid_file = args.pid_path.join(args.name.replace("/", "-") + ".pid");
            let mut daemon = daemonize::Daemonize::new().pid_file(pid_file);
            if let Some(ref user) = args.user {
                daemon = daemon.user(user.as_str());
            }
            if let Some(ref group) = args.group {
                daemon = daemon.group(group.as_str());
            }
            if let Err(err) = daemon.start() {
                error!("could not daemonize process: {}", err);
            }
        }
        let config = config.unwrap_or_else(|| ServerConfig::from_file(&args.configfile));
        Server::new(&args.name, config, args.bind, args.database, use_db && !args.no_db)
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
            let mut my_devs = vec![self.address.get_ext_endpoint(), self.name.clone()];
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
        thread::spawn(move || loop {
            if let Ok(devname) = sock.recv_bytes(0) {
                let rep = if devnames.contains(&devname) {
                    b"1"
                } else {
                    b"0"
                };
                sock.send(rep, 0).unwrap();
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
            dev_sock.bind(&inproc_addr)?; // must bind before connect

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
            }
            Some(&sindex) => {
                // send request on to the device thread
                util::send_full_message(&mut pollsockets[sindex], &msg)?;
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
        let mut server = $crate::server::Server::from_args($use_db, $staticconfig);
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
    };
}
