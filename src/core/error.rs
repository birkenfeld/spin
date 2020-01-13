// Spin RPC library, copyright 2015-2020 Georg Brandl.

//! Error and Result types for Spin.

use std::error;
use std::fmt;
use std::io;

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
pub const MISSING_ERROR:  &str = "NotImplementedError";


#[derive(Debug)]
pub struct Error(pr::Error);

impl Error {
    pub fn new(err: pr::Error) -> Error {
        Error(err)
    }

    pub fn with(reason: String, desc: String, origin: String) -> Error {
        Error(pr::Error { reason, desc, origin })
    }

    pub fn into_inner(self) -> pr::Error {
        self.0
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(fmt, "{}: {}", self.0.reason, self.0.desc)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        &self.0.desc
    }
}

macro_rules! error_impl {
    ($errcls:ty => $errconst:ident) => {
        impl From<$errcls> for Error {
            fn from(e: $errcls) -> Error {
                Error(pr::Error {
                    reason: $errconst.into(),
                    desc:   e.to_string(),
                    origin: String::new(),
                })
            }
        }
    }
}

error_impl!(io::Error => IO_ERROR);
error_impl!(zmq::Error => ZMQ_ERROR);
error_impl!(prost::DecodeError => PROTO_ERROR);
error_impl!(prost::EncodeError => PROTO_ERROR);

pub type SpinResult<T> = Result<T, Error>;

#[macro_export]
macro_rules! spin_err {
    ($reason:expr, $msg:expr, $($args:tt),+) => {
        Err($crate::error::Error::with($reason.to_string(),
                                       format!($msg, $($args),*),
                                       module_path!().into()))
    };
    ($reason:expr, $msg:expr) => {
        Err($crate::error::Error::with($reason.to_string(),
                                       $msg.to_string(),
                                       module_path!().into()))
    }
}
