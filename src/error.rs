// Spin RPC library, copyright 2015, 2016 Georg Brandl.
//
//! Error and Result types for Spin.

use std::error;
use std::fmt;

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
pub const PROTO_ERROR:    &'static str = "ProtocolError";


#[derive(Debug)]
pub struct Error {
    reason: String,
    desc:   String,
    origin: String,
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

impl From<zmq::Error> for Error {
    fn from(e: zmq::Error) -> Error {
        Error {
            reason: ZMQ_ERROR.into(),
            desc:   error::Error::description(&e).into(),
            origin: String::new(),
        }
    }
}

impl From<protobuf::ProtobufError> for Error {
    fn from(e: protobuf::ProtobufError) -> Error {
        Error {
            reason: PROTO_ERROR.into(),
            desc:   error::Error::description(&e).into(),
            origin: String::new(),
        }
    }
}

pub type SpinResult<T> = Result<T, Error>;

pub fn spin_err<T>(reason: &str, msg: &str) -> SpinResult<T> {
    Err(Error { reason: reason.into(),
                desc: msg.into(),
                origin: String::new() })
}
