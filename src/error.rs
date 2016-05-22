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


#[derive(Debug)]
pub enum Error {
    Zmq(zmq::Error),
    Proto(protobuf::ProtobufError),
    Spin { reason: String, desc: String, origin: String },
}

impl Error {
    pub fn into_proto(self) -> pr::Error {
        let mut err = pr::Error::new();
        match self {
            Error::Spin { desc, reason, origin } => {
                err.set_desc(desc);
                err.set_reason(reason);
                err.set_origin(origin);
            },
            Error::Zmq(ref e) => {
                err.set_desc(error::Error::description(e).into());
                err.set_reason("ZmqError".into());
            },
            Error::Proto(ref e) => {
                err.set_desc(error::Error::description(e).into());
                err.set_reason("ProtocolError".into());
            }
        }
        err
    }

    pub fn from_proto(mut err: pr::Error) -> Error {
        Error::Spin {
            reason: err.take_reason(),
            origin: err.take_origin(),
            desc: err.take_desc()
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(fmt, "{:?}", self)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Zmq(ref e) => e.description(),
            Error::Proto(ref e) => e.description(),
            Error::Spin { ref desc, .. } => desc,
        }
    }
}

impl From<zmq::Error> for Error {
    fn from(e: zmq::Error) -> Error {
        Error::Zmq(e)
    }
}

impl From<protobuf::ProtobufError> for Error {
    fn from(e: protobuf::ProtobufError) -> Error {
        Error::Proto(e)
    }
}

pub type SpinResult<T> = Result<T, Error>;

pub fn spin_err<T>(reason: &str, msg: &str) -> SpinResult<T> {
    Err(Error::Spin { reason: reason.into(),
                      desc: msg.into(),
                      origin: "".into() })
}
