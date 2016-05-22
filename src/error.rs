// Spin RPC library, copyright 2015, 2016 Georg Brandl.

//! Error and Result types for Spin.

use std::error;
use std::fmt;
use std::io;

use zmq;
use protobuf;

use spin_proto as pr;

pub const DB_ERROR:       &'static str = "DbError";
pub const ARG_ERROR:      &'static str = "ArgError";
pub const TIMEOUT_ERROR:  &'static str = "TimeoutError";
pub const API_ERROR:      &'static str = "APIError";
pub const SOCKET_ERROR:   &'static str = "SocketError";
pub const ADDRESS_ERROR:  &'static str = "AddressError";
pub const ZMQ_ERROR:      &'static str = "ZmqError";
pub const IO_ERROR:       &'static str = "InputOutputError";
pub const PROTO_ERROR:    &'static str = "ProtocolError";
pub const CONFIG_ERROR:   &'static str = "ConfigurationError";
pub const COMM_ERROR:     &'static str = "CommunicationError";


#[derive(Debug)]
pub struct Error {
    pub reason: String,
    pub desc:   String,
    pub origin: String,
}

impl Error {
    pub fn into_proto(self) -> pr::Error {
        let mut err = pr::Error::new();
        err.set_desc(self.desc);
        err.set_reason(self.reason);
        err.set_origin(self.origin);
        err
    }

    pub fn from_proto(mut err: pr::Error) -> Error {
        Error {
            reason: err.take_reason(),
            origin: err.take_origin(),
            desc: err.take_desc()
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
error_impl!(protobuf::ProtobufError => PROTO_ERROR);

pub type SpinResult<T> = Result<T, Error>;

#[macro_export]
macro_rules! spin_err {
    ($reason:expr, $msg:expr) => {
        Err($crate::error::Error { reason: $reason.to_string(),
                                   desc: $msg.to_string(),
                                   origin: module_path!().into() })
    }
}
