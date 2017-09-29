// Spin RPC library, copyright 2015-2017 Georg Brandl.

//! Validators for property/attribute/argument values.

use regex;

use arg::{Value, FromValue};
use error::{SpinResult, ARG_ERROR};

pub trait CanValidate {
    type Base;
    fn validate(base: Value) -> SpinResult<Self::Base>;
}

impl<T: FromValue> CanValidate for T {
    type Base = T;
    fn validate(base: Value) -> SpinResult<T> {
        T::from_value(base)
    }
}

pub struct Subdev;

impl CanValidate for Subdev {
    type Base = String;
    fn validate(base: Value) -> SpinResult<String> {
        let rx = regex::Regex::new("spin(db)?://.*").unwrap();
        let v = String::from_value(base)?;
        if !rx.is_match(&v) {
            spin_err!(ARG_ERROR, "value needs to be a device URI")
        } else {
            Ok(v)
        }
    }
}
