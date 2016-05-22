// Spin RPC library, copyright 2015 Georg Brandl.
//
//! Error and Result types for Spin.

use std::error;
use std::fmt;

use zmq;
use protobuf;

use spin_proto as pr;


#[derive(Debug)]
pub enum Error {
    ZmqError(zmq::Error),
    ProtoError(protobuf::ProtobufError),
    SpinError { reason: String, desc: String, origin: String },
}

impl Error {
    pub fn into_proto(self) -> pr::Error {
        let mut err = pr::Error::new();
        match self {
            Error::SpinError { desc, reason, origin } => {
                err.set_desc(desc);
                err.set_reason(reason);
                err.set_origin(origin);
            },
            Error::ZmqError(ref e) => {
                err.set_desc(error::Error::description(e).into());
                err.set_reason("ZmqError".into());
            },
            Error::ProtoError(ref e) => {
                err.set_desc(error::Error::description(e).into());
                err.set_reason("ProtocolError".into());
            }
        }
        err
    }

    pub fn from_proto(mut err: pr::Error) -> Error {
        Error::SpinError {
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
            Error::ZmqError(ref e) => e.description(),
            Error::ProtoError(ref e) => e.description(),
            Error::SpinError { ref desc, .. } => desc,
        }
    }
}

impl From<zmq::Error> for Error {
    fn from(e: zmq::Error) -> Error {
        Error::ZmqError(e)
    }
}

impl From<protobuf::ProtobufError> for Error {
    fn from(e: protobuf::ProtobufError) -> Error {
        Error::ProtoError(e)
    }
}

pub type SpinResult<T> = Result<T, Error>;

pub fn spin_err<T>(reason: &str, msg: &str) -> SpinResult<T> {
    Err(Error::SpinError { reason: reason.into(),
                           desc: msg.into(),
                           origin: "".into() })
}
