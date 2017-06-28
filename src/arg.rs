// Spin RPC library, copyright 2015-2017 Georg Brandl.

//! Argument type library, wrapper around some protobuf types.

use toml;
use spin_proto as pr;

pub use spin_proto::DataType;
pub use spin_proto::CmdDesc;
pub use spin_proto::AttrDesc;
pub use spin_proto::PropDesc;

use error::{SpinResult, ARG_ERROR};


pub fn cmd_info(name: &str, doc: &str, intype: DataType, outtype: DataType) -> CmdDesc {
    CmdDesc {
        name: name.into(),
        doc: doc.into(),
        intype: intype.into(),
        outtype: outtype.into(),
        indoc: "".into(),
        outdoc: "".into(),
    }
}

pub fn attr_info(name: &str, doc: &str, dtype: DataType) -> AttrDesc {
    AttrDesc {
        name: name.into(),
        doc: doc.into(),
        type_: dtype.into(),
        unit: "".into(),
    }
}

pub fn prop_info(name: &str, doc: &str, dtype: DataType, default: Value) -> PropDesc {
    PropDesc {
        name: name.into(),
        doc: doc.into(),
        type_: dtype.into(),
        default: Some(default.into_inner()),  // XXX
    }
}


#[derive(Debug, Clone)]
pub struct Value(pr::Value);

impl Value {
    pub fn new(val: pr::Value) -> Value {
        Value(val)
    }

    pub fn into_inner(self) -> pr::Value {
        self.0
    }

    pub fn from_toml(val: toml::Value) -> Option<Value> {
        macro_rules! arr_impl {
            ($arr:ident, $el:ident, $variant:ident) => {{
                let mut all = Vec::with_capacity($arr.len() + 1);
                for el in $arr {
                    if let toml::Value::$variant(s) = el {
                        all.push(s);
                    }
                }
                all.push($el);
                Some(Value::from(all))
            }}
        }
        match val {
            toml::Value::String(s) => Some(Value::from(s)),
            toml::Value::Integer(i) => Some(Value::from(i)),
            toml::Value::Float(f) => Some(Value::from(f)),
            toml::Value::Boolean(b) => Some(Value::from(b)),
            toml::Value::Array(mut arr) => match arr.pop() {
                None => Some(Value::from(())),  // we don't know a type
                // All Array elements have the same TOML type. (This is not
                // encoded in the datatype.) Therefore we can just switch on
                // the last value.
                Some(last) => match last {
                    toml::Value::String(s) => arr_impl!(arr, s, String),
                    toml::Value::Integer(i) => arr_impl!(arr, i, Integer),
                    toml::Value::Float(f) => arr_impl!(arr, f, Float),
                    toml::Value::Boolean(b) => arr_impl!(arr, b, Boolean),
                    _ => None
                },
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
        macro_rules! conv_arr {
            ($arr:expr, $ty:ty) => {
                Some(Value::from($arr.into_iter().map(|v| v as $ty).collect::<Vec<_>>()))
            }
        }
        if self.0.vtype() == Some(newtype) {
            return Some(self);
        }
        let inner = self.into_inner();
        match inner.vtype().unwrap() {
            DataType::Double => match newtype {
                DataType::Float => Some(Value::from(inner.double[0] as f32)),
                _ => None
            },
            DataType::Float => match newtype {
                DataType::Double => Some(Value::from(inner.float[0] as f64)),
                _ => None
            },
            DataType::Int32 => match newtype {
                DataType::Int64 => Some(Value::from(inner.int32[0] as i64)),
                DataType::Uint32 => Some(Value::from(inner.int32[0] as u32)),
                DataType::Uint64 => Some(Value::from(inner.int32[0] as u64)),
                DataType::Float => Some(Value::from(inner.int32[0] as f32)),
                DataType::Double => Some(Value::from(inner.int32[0] as f64)),
                _ => None
            },
            DataType::Int64 => match newtype {
                DataType::Int32 => Some(Value::from(inner.int64[0] as i32)),
                DataType::Uint32 => Some(Value::from(inner.int64[0] as u32)),
                DataType::Uint64 => Some(Value::from(inner.int64[0] as u64)),
                DataType::Float => Some(Value::from(inner.int64[0] as f32)),
                DataType::Double => Some(Value::from(inner.int64[0] as f64)),
                _ => None
            },
            DataType::Uint32 => match newtype {
                DataType::Int32 => Some(Value::from(inner.uint32[0] as i32)),
                DataType::Int64 => Some(Value::from(inner.uint32[0] as i64)),
                DataType::Uint64 => Some(Value::from(inner.uint32[0] as u64)),
                DataType::Float => Some(Value::from(inner.uint32[0] as f32)),
                DataType::Double => Some(Value::from(inner.uint32[0] as f64)),
                _ => None
            },
            DataType::Uint64 => match newtype {
                DataType::Int32 => Some(Value::from(inner.uint64[0] as i32)),
                DataType::Int64 => Some(Value::from(inner.uint64[0] as i64)),
                DataType::Uint32 => Some(Value::from(inner.uint64[0] as u32)),
                DataType::Float => Some(Value::from(inner.uint64[0] as f32)),
                DataType::Double => Some(Value::from(inner.uint64[0] as f64)),
                _ => None
            },
            DataType::DoubleArray => match newtype {
                DataType::FloatArray => conv_arr!(inner.double, f32),
                _ => None
            },
            DataType::FloatArray => match newtype {
                DataType::DoubleArray => conv_arr!(inner.float, f64),
                _ => None
            },
            DataType::Int32Array => match newtype {
                DataType::Int64Array => conv_arr!(inner.int32, i64),
                DataType::Uint32Array => conv_arr!(inner.int32, u32),
                DataType::Uint64Array => conv_arr!(inner.int32, u64),
                DataType::FloatArray => conv_arr!(inner.int32, f32),
                DataType::DoubleArray => conv_arr!(inner.int32, f64),
                _ => None
            },
            DataType::Int64Array => match newtype {
                DataType::Int32Array => conv_arr!(inner.int64, i32),
                DataType::Uint32Array => conv_arr!(inner.int64, u32),
                DataType::Uint64Array => conv_arr!(inner.int64, u64),
                DataType::FloatArray => conv_arr!(inner.int64, f32),
                DataType::DoubleArray => conv_arr!(inner.int64, f64),
                _ => None
            },
            DataType::Uint32Array => match newtype {
                DataType::Int32Array => conv_arr!(inner.uint32, i32),
                DataType::Int64Array => conv_arr!(inner.uint32, i64),
                DataType::Uint64Array => conv_arr!(inner.uint32, u64),
                DataType::FloatArray => conv_arr!(inner.uint32, f32),
                DataType::DoubleArray => conv_arr!(inner.uint32, f64),
                _ => None
            },
            DataType::Uint64Array => match newtype {
                DataType::Int32Array => conv_arr!(inner.uint64, i32),
                DataType::Int64Array => conv_arr!(inner.uint64, i64),
                DataType::Uint32Array => conv_arr!(inner.uint64, u32),
                DataType::FloatArray => conv_arr!(inner.uint64, f32),
                DataType::DoubleArray => conv_arr!(inner.uint64, f64),
                _ => None
            },
            _ => None
        }
    }

    // Helper for the FromValue implementations.
    fn ensure_type(self, dtype: DataType) -> SpinResult<pr::Value> {
        if self.0.vtype() == Some(dtype) {
            Ok(self.0)
        } else {
            let msg = format!("wrong argument type: {:?}, expected {:?}",
                              self.0.vtype(), dtype);
            spin_err!(ARG_ERROR, &msg)
        }
    }

    // Helper for the From implementations.
    fn construct<F: FnOnce(&mut pr::Value)>(dtype: DataType, populate: F) -> Value {
        let mut v = pr::Value::default();
        v.set_vtype(dtype);
        populate(&mut v);
        Value(v)
    }
}

pub trait FromValue: Default where Self: Sized {
    fn from_value(Value) -> SpinResult<Self>;
}


impl From<()> for Value {
    fn from(_: ()) -> Value {
        Value::construct(DataType::Void, |_| {})
    }
}

impl FromValue for () {
    fn from_value(v: Value) -> SpinResult<()> {
        v.ensure_type(DataType::Void)?;
        Ok(())
    }
}

impl<'a> From<&'a str> for Value {
    fn from(val: &'a str) -> Value {
        Value::construct(DataType::String, |v| v.string.push(val.into()))
    }
}

impl From<String> for Value {
    fn from(val: String) -> Value {
        Value::construct(DataType::String, |v| v.string.push(val))
    }
}

impl FromValue for String {
    fn from_value(v: Value) -> SpinResult<String> {
        Ok(v.ensure_type(DataType::String)?.string.pop().unwrap())
    }
}

impl From<Vec<String>> for Value {
    fn from(val: Vec<String>) -> Value {
        Value::construct(DataType::StringArray, |v| v.string = val)
    }
}

impl FromValue for Vec<String> {
    fn from_value(v: Value) -> SpinResult<Vec<String>> {
        Ok(v.ensure_type(DataType::StringArray)?.string)
    }
}

impl From<(Vec<i32>, Vec<String>)> for Value {
    fn from((ival, sval): (Vec<i32>, Vec<String>)) -> Value {
        Value::construct(DataType::Int64StringArray, |v| {
            v.int32 = ival;
            v.string = sval;
        })
    }
}

impl FromValue for (Vec<i64>, Vec<String>) {
    fn from_value(v: Value) -> SpinResult<(Vec<i64>, Vec<String>)> {
        let inner = v.ensure_type(DataType::Int64StringArray)?;
        Ok((inner.int64, inner.string))
    }
}

impl From<(Vec<f64>, Vec<String>)> for Value {
    fn from((fval, sval): (Vec<f64>, Vec<String>)) -> Value {
        Value::construct(DataType::DoubleStringArray, |v| {
            v.double = fval;
            v.string = sval;
        })
    }
}

impl FromValue for (Vec<f64>, Vec<String>) {
    fn from_value(v: Value) -> SpinResult<(Vec<f64>, Vec<String>)> {
        let inner = v.ensure_type(DataType::DoubleStringArray)?;
        Ok((inner.double, inner.string))
    }
}

impl From<Vec<u8>> for Value {
    fn from(val: Vec<u8>) -> Value {
        Value::construct(DataType::ByteArray, |v| v.bytes = Some(val))
    }
}

impl FromValue for Vec<u8> {
    fn from_value(v: Value) -> SpinResult<Vec<u8>> {
        Ok(v.ensure_type(DataType::ByteArray)?.bytes.unwrap())
    }
}

macro_rules! impl_traits {
    ($ty:ty, $dtype:ident, $vectype:ident, $member:ident) => {
        impl From<$ty> for Value {
            fn from(val: $ty) -> Value {
                Value::construct(DataType::$dtype, |v| v.$member = vec![val])
            }
        }
        impl FromValue for $ty {
            fn from_value(v: Value) -> SpinResult<$ty> {
                Ok(v.ensure_type(DataType::$dtype)?.$member[0])
            }
        }
        impl From<Vec<$ty>> for Value {
            fn from(val: Vec<$ty>) -> Value {
                Value::construct(DataType::$vectype, |v| v.$member = val)
            }
        }
        impl FromValue for Vec<$ty> {
            fn from_value(v: Value) -> SpinResult<Vec<$ty>> {
                Ok(v.ensure_type(DataType::$vectype)?.$member)
            }
        }
    }
}

impl_traits!(bool, Bool, BoolArray, bool);
impl_traits!(f64, Double, DoubleArray, double);
impl_traits!(f32, Float, FloatArray, float);
impl_traits!(i32, Int32, Int32Array, int32);
impl_traits!(i64, Int64, Int64Array, int64);
impl_traits!(u32, Uint32, Uint32Array, uint32);
impl_traits!(u64, Uint64, Uint64Array, uint64);
