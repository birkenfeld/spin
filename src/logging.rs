// Spin RPC library, copyright 2015, 2016 Georg Brandl.

//! Additions to the log4rs library for logging.

use std::fmt;
use std::error::Error;
use std::fs::{File, remove_file};
use std::io::{self, Stdout, Write, BufWriter};
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use time::{Timespec, Tm, Duration, get_time, now, strftime};
use log::{LogLevel, LogRecord, LogLevelFilter};
use log4rs;
use log4rs::append::Append;
use log4rs::encode::Encode;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::encode::writer::simple::SimpleWriter;
use log4rs::config::{Config, Root, Appender};
use ansi_term::Colour::{Red, White, Purple};

use util::{ensure_dir, open_file};


struct ConsoleAppender {
    prefix: String,
    stdout: Stdout,
}

impl fmt::Debug for ConsoleAppender {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("<console>")
    }
}

impl ConsoleAppender {
    pub fn new(srvname: &str) -> ConsoleAppender {
        ConsoleAppender { prefix: srvname.into(),
                          stdout: io::stdout() }
    }
}

impl Append for ConsoleAppender {
    fn append(&self, record: &LogRecord) -> Result<(), Box<Error>> {
        let mut stdout = self.stdout.lock();
        let time_str = strftime("[%H:%M:%S]", &now()).unwrap();
        let msg = match record.level() {
            LogLevel::Error => Red.bold().paint(
                format!("[{:-10}] ERROR: {}", self.prefix, record.args())),
            LogLevel::Warn  => Purple.paint(
                format!("[{:-10}] WARNING: {}", self.prefix, record.args())),
            LogLevel::Debug => White.paint(
                format!("[{:-10}] {}", self.prefix, record.args())),
            _ => From::from(
                format!("[{:-10}] {}", self.prefix, record.args())),
        };
        writeln!(stdout, "{} {}", White.paint(time_str), msg)?;
        stdout.flush()?;
        Ok(())
    }
}

type Writer = SimpleWriter<BufWriter<File>>;

#[derive(Debug)]
struct RollingFileAppender {
    dir:     PathBuf,
    prefix:  String,
    link_fn: PathBuf,
    file:    Mutex<(Option<Writer>, Timespec)>,
    pattern: PatternEncoder,
}

impl RollingFileAppender {
    pub fn new(dir: &Path, prefix: &str) -> RollingFileAppender {
        let thisday = Tm { tm_hour: 0, tm_min: 0, tm_sec: 0, tm_nsec: 0, ..now() };
        let roll_at = (thisday + Duration::days(1)).to_timespec();
        let pattern = PatternEncoder::new("{d(%H:%M:%S,%f)} : {l:<5} : {m}{n}");
        let link_fn = dir.join("current");
        let prefix = prefix.replace("/", "-");
        RollingFileAppender { dir: dir.to_path_buf(), prefix: prefix, link_fn: link_fn,
                              file: Mutex::new((None, roll_at)), pattern: pattern }
    }

    fn rollover(&self, file_opt: &mut Option<Writer>, roll_at: &mut Timespec) -> io::Result<()> {
        file_opt.take();  // will drop the file if open
        let time = strftime("%Y-%m-%d", &now()).unwrap();
        let full = format!("{}-{}.log", self.prefix, time);
        let new_fn = self.dir.join(full);
        let fp = open_file(&new_fn, "wa")?;
        *file_opt = Some(SimpleWriter(BufWriter::new(fp)));
        let _ = remove_file(&self.link_fn);
        let _ = symlink(&new_fn.file_name().unwrap(), &self.link_fn);
        *roll_at = *roll_at + Duration::days(1);
        Ok(())
    }
}

impl Append for RollingFileAppender {
    fn append(&self, record: &LogRecord) -> Result<(), Box<Error>> {
        let (ref mut file_opt, ref mut roll_at) = *self.file.lock().unwrap();
        if file_opt.is_none() || get_time() >= *roll_at {
            self.rollover(file_opt, roll_at)?;
        }
        let fp = file_opt.as_mut().unwrap();
        self.pattern.encode(fp, record)?;
        fp.flush()?;
        Ok(())
    }
}


pub fn init<P: AsRef<Path>>(log_path: P, srvname: &str, debug: bool,
                            use_stdout: bool) -> io::Result<()> {
    ensure_dir(log_path.as_ref())?;

    let file_appender = RollingFileAppender::new(log_path.as_ref(), srvname);
    let mut root_cfg = Root::builder().appender("file");
    if use_stdout {
        root_cfg = root_cfg.appender("con");
    }
    let mut config = Config::builder()
        .appender(Appender::builder().build("file", box file_appender));
    if use_stdout {
        let con_appender = ConsoleAppender::new(srvname);
        config = config.appender(Appender::builder().build("con", box con_appender));
    }
    let config = config.build(root_cfg.build(if debug { LogLevelFilter::Debug }
                                             else { LogLevelFilter::Info }))
                       .expect("error building logging config");

    let _ = log4rs::init_config(config);
    Ok(())
}
