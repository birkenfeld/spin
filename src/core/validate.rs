// Spin RPC library, copyright 2015-2020 Georg Brandl.

//! Validators for property/attribute/argument values.

use crate::arg::{FromValue, Value};
use crate::error::{SpinResult, ARG_ERROR, CONFIG_ERROR};

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

pub trait IntoDefault {
    fn into_default(&self) -> SpinResult<Value>;
}

impl<T: Clone> IntoDefault for T
where
    Value: From<T>,
{
    fn into_default(&self) -> SpinResult<Value> {
        Ok(Value::from(self.clone()))
    }
}

pub struct Mandatory;

impl IntoDefault for Mandatory {
    fn into_default(&self) -> SpinResult<Value> {
        spin_err!(CONFIG_ERROR, "this property is mandatory")
    }
}

impl From<Mandatory> for Value {
    fn from(_: Mandatory) -> Value {
        Value::from(())
    }
}
