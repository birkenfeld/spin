// Spin RPC library, copyright 2015, 2016 Georg Brandl.
//
//! Argument type library, wrapper around some protobuf types.

use toml;
use protobuf::RepeatedField;
use spin_proto as pr;

pub use spin_proto::DataType;
pub use spin_proto::CmdDesc;
pub use spin_proto::AttrDesc;
pub use spin_proto::PropDesc;

use error::{SpinResult, spin_err, ARG_ERROR};


pub fn cmd_info(name: &str, doc: &str, intype: DataType, outtype: DataType) -> CmdDesc {
    let mut c = CmdDesc::new();
    c.set_name(name.into());
    c.set_doc(doc.into());
    c.set_intype(intype);
    c.set_outtype(outtype);
    c
}

pub fn attr_info(name: &str, doc: &str, dtype: DataType) -> AttrDesc {
    let mut a = AttrDesc::new();
    a.set_name(name.into());
    a.set_doc(doc.into());
    a.set_field_type(dtype);
    a
}

pub fn prop_info(name: &str, doc: &str, dtype: DataType, default: Value) -> PropDesc {
    let mut a = PropDesc::new();
    a.set_name(name.into());
    a.set_doc(doc.into());
    a.set_field_type(dtype);
    a.set_default(default.into());
    a
}


#[derive(Debug, Clone)]
pub struct Value(pr::Value);

impl Value {
    pub fn new<T: Into<Value>>(val: T) -> Value {
        val.into()
    }

    pub fn void() -> Value {
        let mut v = pr::Value::new();
        v.set_vtype(DataType::Void);
        Value(v)
    }

    pub fn into_inner(self) -> pr::Value {
        self.0
    }

    pub fn from_toml(val: &toml::Value) -> Option<Value> {
        match *val {
            toml::Value::String(ref s) => Some(Value::from(s.clone())),
            toml::Value::Integer(i) => Some(Value::from(i)),
            toml::Value::Float(f) => Some(Value::from(f)),
            toml::Value::Boolean(b) => Some(Value::from(b)),
            toml::Value::Array(_) => {
                // TODO
                None
            },
            _ => None
        }
    }

    pub fn extract_clone<T: FromValue>(&self) -> SpinResult<T> {
        T::from_value(self.clone())
    }

    pub fn extract<T: FromValue>(self) -> SpinResult<T> {
        T::from_value(self)
    }

    pub fn convert(self, newtype: DataType) -> Option<Value> {
        if self.0.get_vtype() == newtype {
            return Some(self);
        }
        let inner = self.into_inner();
        match inner.get_vtype() {
            DataType::Double => match newtype {
                DataType::Float => Some(Value::from(inner.get_double()[0] as f32)),
                _ => None
            },
            DataType::Float => match newtype {
                DataType::Double => Some(Value::from(inner.get_float()[0] as f64)),
                _ => None
            },
            DataType::Int32 => match newtype {
                DataType::Int64 => Some(Value::from(inner.get_int32()[0] as i64)),
                DataType::UInt32 => Some(Value::from(inner.get_int32()[0] as u32)),
                DataType::UInt64 => Some(Value::from(inner.get_int32()[0] as u64)),
                DataType::Float => Some(Value::from(inner.get_int32()[0] as f32)),
                DataType::Double => Some(Value::from(inner.get_int32()[0] as f64)),
                _ => None
            },
            DataType::Int64 => match newtype {
                DataType::Int32 => Some(Value::from(inner.get_int64()[0] as i32)),
                DataType::UInt32 => Some(Value::from(inner.get_int64()[0] as u32)),
                DataType::UInt64 => Some(Value::from(inner.get_int64()[0] as u64)),
                DataType::Float => Some(Value::from(inner.get_int64()[0] as f32)),
                DataType::Double => Some(Value::from(inner.get_int64()[0] as f64)),
                _ => None
            },
            DataType::UInt32 => match newtype {
                DataType::Int32 => Some(Value::from(inner.get_uint32()[0] as i32)),
                DataType::Int64 => Some(Value::from(inner.get_uint32()[0] as i64)),
                DataType::UInt64 => Some(Value::from(inner.get_uint32()[0] as u64)),
                DataType::Float => Some(Value::from(inner.get_uint32()[0] as f32)),
                DataType::Double => Some(Value::from(inner.get_uint32()[0] as f64)),
                _ => None
            },
            DataType::UInt64 => match newtype {
                DataType::Int32 => Some(Value::from(inner.get_uint64()[0] as i32)),
                DataType::Int64 => Some(Value::from(inner.get_uint64()[0] as i64)),
                DataType::UInt32 => Some(Value::from(inner.get_uint64()[0] as u32)),
                DataType::Float => Some(Value::from(inner.get_uint64()[0] as f32)),
                DataType::Double => Some(Value::from(inner.get_uint64()[0] as f64)),
                _ => None
            },
            // TODO: arrays
            _ => None
        }
    }
}

pub trait FromValue: Default where Self: Sized {
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

impl From<&'static str> for Value {
    fn from(val: &'static str) -> Value {
        let mut v = pr::Value::new();
        v.set_vtype(DataType::String);
        v.mut_string().push(val.into());
        Value(v)
    }
}

impl From<String> for Value {
    fn from(val: String) -> Value {
        let mut v = pr::Value::new();
        v.set_vtype(DataType::String);
        v.mut_string().push(val);
        Value(v)
    }
}

impl FromValue for String {
    fn from_value(mut v: Value) -> SpinResult<String> {
        if v.0.get_vtype() == DataType::String {
            Ok(v.0.take_string().pop().unwrap())
        } else {
            let msg = format!("wrong type: {:?}, expected String", v.0.get_vtype());
            spin_err(ARG_ERROR, &msg)
        }
    }
}

impl From<Vec<String>> for Value {
    fn from(val: Vec<String>) -> Value {
        let mut v = pr::Value::new();
        v.set_vtype(DataType::StringArray);
        v.set_string(RepeatedField::from_vec(val));
        Value(v)
    }
}

impl FromValue for Vec<String> {
    fn from_value(mut v: Value) -> SpinResult<Vec<String>> {
        if v.0.get_vtype() == DataType::StringArray {
            Ok(v.0.take_string().to_vec())
        } else {
            let msg = format!("wrong type: {:?}, expected StringArray", v.0.get_vtype());
            spin_err(ARG_ERROR, &msg)
        }
    }
}

impl From<(Vec<i32>, Vec<String>)> for Value {
    fn from((ival, sval): (Vec<i32>, Vec<String>)) -> Value {
        let mut v = pr::Value::new();
        v.set_vtype(DataType::Int32StringArray);
        v.set_int32(ival);
        v.set_string(RepeatedField::from_vec(sval));
        Value(v)
    }
}

impl FromValue for (Vec<i32>, Vec<String>) {
    fn from_value(mut v: Value) -> SpinResult<(Vec<i32>, Vec<String>)> {
        if v.0.get_vtype() == DataType::Int32StringArray {
            Ok((v.0.take_int32(), v.0.take_string().to_vec()))
        } else {
            let msg = format!("wrong type: {:?}, expected Int32StringArray", v.0.get_vtype());
            spin_err(ARG_ERROR, &msg)
        }
    }
}

impl From<(Vec<f64>, Vec<String>)> for Value {
    fn from((fval, sval): (Vec<f64>, Vec<String>)) -> Value {
        let mut v = pr::Value::new();
        v.set_vtype(DataType::DoubleStringArray);
        v.set_double(fval);
        v.set_string(RepeatedField::from_vec(sval));
        Value(v)
    }
}

impl FromValue for (Vec<f64>, Vec<String>) {
    fn from_value(mut v: Value) -> SpinResult<(Vec<f64>, Vec<String>)> {
        if v.0.get_vtype() == DataType::DoubleStringArray {
            Ok((v.0.take_double(), v.0.take_string().to_vec()))
        } else {
            let msg = format!("wrong type: {:?}, expected DoubleStringArray", v.0.get_vtype());
            spin_err(ARG_ERROR, &msg)
        }
    }
}

impl From<Vec<u8>> for Value {
    fn from(val: Vec<u8>) -> Value {
        let mut v = pr::Value::new();
        v.set_vtype(DataType::String);
        v.set_bytes(val);
        Value(v)
    }
}

impl FromValue for Vec<u8> {
    fn from_value(mut v: Value) -> SpinResult<Vec<u8>> {
        if v.0.get_vtype() == DataType::ByteArray {
            Ok(v.0.take_bytes())
        } else {
            let msg = format!("wrong type: {:?}, expected ByteArray", v.0.get_vtype());
            spin_err(ARG_ERROR, &msg)
        }
    }
}

macro_rules! impl_traits {
    ($ty:ty, $dtype:ident, $vectype:ident, $setter:ident, $getter:ident) => {
        impl From<$ty> for Value {
            fn from(val: $ty) -> Value {
                let mut v = pr::Value::new();
                v.set_vtype(DataType::$dtype);
                v.$setter(vec![val]);
                Value(v)
            }
        }
        impl FromValue for $ty {
            #[allow(unused_mut)]
            fn from_value(mut v: Value) -> SpinResult<$ty> {
                if v.0.get_vtype() == DataType::$dtype {
                    Ok((v.0).$getter()[0])
                } else {
                    let msg = format!("wrong type: {:?}, expected {:?}",
                                      v.0.get_vtype(), DataType::$dtype);
                    spin_err(ARG_ERROR, &msg)
                }
            }
        }
        impl From<Vec<$ty>> for Value {
            fn from(val: Vec<$ty>) -> Value {
                let mut v = pr::Value::new();
                v.set_vtype(DataType::$vectype);
                v.$setter(val);
                Value(v)
            }
        }
        impl FromValue for Vec<$ty> {
            #[allow(unused_mut)]
            fn from_value(mut v: Value) -> SpinResult<Vec<$ty>> {
                if v.0.get_vtype() == DataType::$vectype {
                    Ok((v.0).$getter())
                } else {
                    let msg = format!("wrong type: {:?}, expected {:?}",
                                      v.0.get_vtype(), DataType::$vectype);
                    spin_err(ARG_ERROR, &msg)
                }
            }
        }
    }
}

impl_traits!(bool, Bool, BoolArray, set_bool, take_bool);
impl_traits!(f64, Double, DoubleArray, set_double, take_double);
impl_traits!(f32, Float, FloatArray, set_float, take_float);
impl_traits!(i32, Int32, Int32Array, set_int32, take_int32);
impl_traits!(i64, Int64, Int64Array, set_int64, take_int64);
impl_traits!(u32, UInt32, UInt32Array, set_uint32, take_uint32);
impl_traits!(u64, UInt64, UInt64Array, set_uint64, take_uint64);
