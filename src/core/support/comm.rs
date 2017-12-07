// Spin RPC library, copyright 2015-2017 Georg Brandl.

//! A generic "communicator" thread.

use std::cmp;
use std::io::{Read, Write};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

use parking_lot::{Condvar, Mutex, MutexGuard};

use error::{SpinResult, COMM_ERROR};

pub type Connector<R, W> = Box<FnMut() -> SpinResult<(R, W)> + Send + 'static>;

struct CommShared<W> {
    writer: Mutex<W>,
    buffer: Mutex<Vec<u8>>,
    seen_eol: Condvar,
    connected: AtomicBool,
    stop: AtomicBool,
}

pub struct CommThread<R, W> {
    connect: Connector<R, W>,
    reader: R,
    eol: Vec<u8>,
    shared: Arc<CommShared<W>>,
}

pub struct CommClient<W> {
    sol: Vec<u8>,
    eol: Vec<u8>,
    shared: Arc<CommShared<W>>,
    timeout: Duration,
}

impl<W: Write> CommClient<W> {
    fn get_line(&self, mut buffer: MutexGuard<Vec<u8>>) -> SpinResult<Vec<u8>> {
        let neol = self.eol.len();
        let end = buffer
            .windows(neol)
            .position(|ch| ch == &self.eol[..])
            .unwrap() + neol;
        let res = buffer.drain(0..end).collect();
        Ok(res)
    }

    pub fn read(&self, max: u32) -> SpinResult<Vec<u8>> {
        let mut buffer = self.shared.buffer.lock();
        let n = buffer.len();
        let res = buffer.drain(0..cmp::min(max as usize, n)).collect();
        Ok(res)
    }

    pub fn write(&self, input: &[u8], line: bool) -> SpinResult<u32> {
        let mut writer = self.shared.writer.lock();
        if line {
            writer.write_all(&self.sol)?;
            writer.write_all(input)?;
            writer.write_all(&self.eol)?;
            Ok((input.len() + self.sol.len() + self.eol.len()) as u32)
        } else {
            writer.write_all(input)?;
            Ok(input.len() as u32)
        }
    }

    pub fn readline(&self) -> SpinResult<Vec<u8>> {
        let mut buffer = self.shared.buffer.lock();
        if self.shared.seen_eol.wait_for(&mut buffer, self.timeout).timed_out() {
            return spin_err!(COMM_ERROR, "no response");
        }
        self.get_line(buffer)
    }

    pub fn communicate(&self, input: &[u8]) -> SpinResult<Vec<u8>> {
        let mut buffer = self.shared.buffer.lock();
        buffer.clear();
        {
            let mut writer = self.shared.writer.lock();
            writer.write_all(&self.sol)?;
            writer.write_all(input)?;
            writer.write_all(&self.eol)?;
        }
        if self.shared.seen_eol.wait_for(&mut buffer, self.timeout).timed_out() {
            return spin_err!(COMM_ERROR, "no response");
        }
        self.get_line(buffer)
    }
}

impl<W> Drop for CommClient<W> {
    fn drop(&mut self) {
        self.shared.stop.store(true, Ordering::SeqCst);
    }
}

impl<R: Read + Send + 'static, W: Write + Send + 'static> CommThread<R, W> {
    pub fn spawn(mut connect: Connector<R, W>, sol: &[u8], eol: &[u8], timeout: Duration)
                 -> SpinResult<CommClient<W>> {
        let (reader, writer) = connect()?;
        let shared = Arc::new(CommShared {
            writer: Mutex::new(writer),
            buffer: Mutex::new(Vec::with_capacity(2048)),
            seen_eol: Condvar::new(),
            connected: AtomicBool::new(true),
            stop: AtomicBool::new(false),
        });
        let mut comm = CommThread {
            connect: connect,
            reader: reader,
            eol: eol.into(),
            shared: Arc::clone(&shared),
        };
        thread::spawn(move || comm.thread());
        Ok(CommClient {
            shared,
            timeout,
            sol: sol.into(),
            eol: eol.into(),
        })
    }

    fn thread(&mut self) {
        debug!("reader thread started");
        let mut buf = [0; 1024];
        while !self.shared.stop.load(Ordering::Relaxed) {
            while !self.shared.stop.load(Ordering::Relaxed) {
                match self.reader.read(&mut buf) {
                    Ok(n) if n == 0 => {
                        warn!("reader connection closed");
                        break;
                    }
                    Ok(n) => {
                        let mut buffer = self.shared.buffer.lock();
                        debug!("reader got: {:?}", &buf[..n]);
                        buffer.extend(&buf[..n]);
                        if buffer.windows(self.eol.len()).any(|ch| ch == &self.eol[..]) {
                            self.shared.seen_eol.notify_one();
                        }
                    }
                    Err(e) => {
                        println!("reader error: {}", e);
                        break;
                    }
                }
            }
            self.shared.connected.store(false, Ordering::SeqCst);
            while !self.shared.stop.load(Ordering::Relaxed) {
                match (self.connect)() {
                    Ok((reader, writer)) => {
                        self.reader = reader;
                        let mut writer_lock = self.shared.writer.lock();
                        *writer_lock = writer;
                        self.shared.connected.store(true, Ordering::SeqCst);
                        break;
                    }
                    Err(e) => {
                        warn!("error during reconnect: {}", e);
                        thread::sleep(Duration::from_millis(100));
                    }
                }
            }
        }
        debug!("reader thread exited normally");
    }
}
