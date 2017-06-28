// Spin RPC library, copyright 2015-2017 Georg Brandl.

//! Error and Result types for Spin.

use std::error;
use std::fmt;
use std::io;

use zmq;

use spin_proto as pr;

pub const DB_ERROR:       &str = "DbError";
pub const ARG_ERROR:      &str = "ArgError";
pub const TIMEOUT_ERROR:  &str = "TimeoutError";
pub const API_ERROR:      &str = "APIError";
pub const SOCKET_ERROR:   &str = "SocketError";
pub const ADDRESS_ERROR:  &str = "AddressError";
pub const ZMQ_ERROR:      &str = "ZmqError";
pub const IO_ERROR:       &str = "InputOutputError";
pub const PROTO_ERROR:    &str = "ProtocolError";
pub const CONFIG_ERROR:   &str = "ConfigurationError";
pub const COMM_ERROR:     &str = "CommunicationError";


#[derive(Debug)]
pub struct Error {
    pub reason: String,
    pub desc:   String,
    pub origin: String,
}

impl Error {
    pub fn into_proto(self) -> pr::Error {
        pr::Error {
            desc: self.desc,
            reason: self.reason,
            origin: self.origin,
        }
    }

    pub fn from_proto(err: pr::Error) -> Error {
        Error {
            reason: err.reason,
            origin: err.origin,
            desc: err.desc,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(fmt, "{}: {}", self.reason, self.desc)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        &self.desc
    }
}

macro_rules! error_impl {
    ($errcls:ty => $errconst:ident) => {
        impl From<$errcls> for Error {
            fn from(e: $errcls) -> Error {
                Error {
                    reason: $errconst.into(),
                    desc:   error::Error::description(&e).into(),
                    origin: String::new(),
                }
            }
        }
    }
}

error_impl!(io::Error => IO_ERROR);
error_impl!(zmq::Error => ZMQ_ERROR);

pub type SpinResult<T> = Result<T, Error>;

#[macro_export]
macro_rules! spin_err {
    ($reason:expr, $msg:expr, $($args:tt),+) => {
        Err($crate::error::Error { reason: $reason.to_string(),
                                   desc: format!($msg, $($args),*),
                                   origin: module_path!().into() })
    };
    ($reason:expr, $msg:expr) => {
        Err($crate::error::Error { reason: $reason.to_string(),
                                   desc: $msg.to_string(),
                                   origin: module_path!().into() })
    }
}
