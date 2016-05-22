// Spin RPC library, copyright 2015 Georg Brandl.
//
//! Argument type library, wrapper around the protobuf Value.

use spin_proto as pr;

pub use spin_proto::DataType;
pub use spin_proto::AttributeInfo;
pub use spin_proto::CommandInfo;

use error::{SpinResult, spin_err};


pub fn cmd_info(name: &str, doc: &str, intype: DataType, outtype: DataType) -> CommandInfo {
    let mut c = CommandInfo::new();
    c.set_name(name.into());
    c.set_doc(doc.into());
    c.set_intype(intype);
    c.set_outtype(outtype);
    c
}

pub fn attr_info(name: &str, doc: &str, dtype: DataType) -> AttributeInfo {
    let mut a = AttributeInfo::new();
    a.set_name(name.into());
    a.set_doc(doc.into());
    a.set_field_type(dtype);
    a
}


#[derive(Debug)]
pub struct Value(pr::Value);

impl Value {
    pub fn new<T: Into<Value>>(val: T) -> Value {
        val.into()
    }

    pub fn void() -> Value {
        let mut v = pr::Value::new();
        v.set_void(true);
        Value(v)
    }

    pub fn into_inner(self) -> pr::Value {
        self.0
    }
}

pub trait FromValue {
    fn from_value(Value) -> SpinResult<Self>;
}


impl From<pr::Value> for Value {
    fn from(v: pr::Value) -> Value {
        Value(v)
    }
}

impl Into<pr::Value> for Value {
    fn into(self) -> pr::Value {
        self.0
    }
}


macro_rules! impl_traits {
    ($ty:ty, $setter:ident) => {
        impl From<$ty> for Value {
            fn from(val: $ty) -> Value {
                let mut v = pr::Value::new();
                v.$setter(val.into());
                Value(v)
            }
        }
    };
    ($ty:ty, $setter:ident, $getter:ident, $checker:ident) => {
        impl From<$ty> for Value {
            fn from(val: $ty) -> Value {
                let mut v = pr::Value::new();
                v.$setter(val);
                Value(v)
            }
        }
        impl FromValue for $ty {
            #[allow(unused_mut)]
            fn from_value(mut v: Value) -> SpinResult<$ty> {
                if (v.0).$checker() {
                    Ok((v.0).$getter())
                } else {
                    spin_err("ArgumentError", "wrong type")
                }
            }
        }
    }
}

impl_traits!(bool, set_bool, get_bool, has_bool);
impl_traits!(f64, set_double, get_double, has_double);
impl_traits!(f32, set_float, get_float, has_float);
impl_traits!(i32, set_int32, get_int32, has_int32);
impl_traits!(i64, set_int64, get_int64, has_int64);
impl_traits!(u32, set_uint32, get_uint32, has_uint32);
impl_traits!(u64, set_uint64, get_uint64, has_uint64);
impl_traits!(String, set_string, take_string, has_string);
impl_traits!(&'static str, set_string);
