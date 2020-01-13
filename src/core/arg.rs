// Spin RPC library, copyright 2015-2020 Georg Brandl.

//! Argument type library, wrapper around some protobuf types.

use std::fmt;

use spin_proto as pr;

pub use spin_proto::DataType;
pub use spin_proto::CmdDesc;
pub use spin_proto::AttrDesc;
pub use spin_proto::PropDesc;

use spin_proto::value::Val;

use crate::error::{SpinResult, ARG_ERROR};


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
        r#type: dtype.into(),
        unit: "".into(),
    }
}

pub fn prop_info(name: &str, doc: &str, dtype: DataType, default: Value) -> PropDesc {
    PropDesc {
        name: name.into(),
        doc: doc.into(),
        r#type: dtype.into(),
        default: default.0,
    }
}

#[derive(Clone)]
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
                None => Some(Value::from(())), // we don't know a type
                // All Array elements have the same TOML type. (This is not
                // encoded in the datatype.) Therefore we can just switch on
                // the last value.
                Some(last) => match last {
                    toml::Value::String(s) => arr_impl!(arr, s, String),
                    toml::Value::Integer(i) => arr_impl!(arr, i, Integer),
                    toml::Value::Float(f) => arr_impl!(arr, f, Float),
                    toml::Value::Boolean(b) => arr_impl!(arr, b, Boolean),
                    _ => None,
                },
            },
            _ => None,
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
        if self.get_dtype() == newtype {
            return Some(self);
        }
        let inner = self.0;
        match inner.val {
            Some(Val::Double(v)) => match newtype {
                DataType::Float => Some(Value::from(v as f32)),
                _ => None,
            },
            Some(Val::Float(v)) => match newtype {
                DataType::Double => Some(Value::from(v as f64)),
                _ => None,
            },
            Some(Val::Int32(v)) => match newtype {
                DataType::Int64  => Some(Value::from(v as i64)),
                DataType::Uint32 => Some(Value::from(v as u32)),
                DataType::Uint64 => Some(Value::from(v as u64)),
                DataType::Float  => Some(Value::from(v as f32)),
                DataType::Double => Some(Value::from(v as f64)),
                _ => None,
            },
            Some(Val::Int64(v)) => match newtype {
                DataType::Int32  => Some(Value::from(v as i32)),
                DataType::Uint32 => Some(Value::from(v as u32)),
                DataType::Uint64 => Some(Value::from(v as u64)),
                DataType::Float  => Some(Value::from(v as f32)),
                DataType::Double => Some(Value::from(v as f64)),
                _ => None,
            },
            Some(Val::Uint32(v)) => match newtype {
                DataType::Int32  => Some(Value::from(v as i32)),
                DataType::Int64  => Some(Value::from(v as i64)),
                DataType::Uint64 => Some(Value::from(v as u64)),
                DataType::Float  => Some(Value::from(v as f32)),
                DataType::Double => Some(Value::from(v as f64)),
                _ => None,
            },
            Some(Val::Uint64(v)) => match newtype {
                DataType::Int32  => Some(Value::from(v as i32)),
                DataType::Int64  => Some(Value::from(v as i64)),
                DataType::Uint32 => Some(Value::from(v as u32)),
                DataType::Float  => Some(Value::from(v as f32)),
                DataType::Double => Some(Value::from(v as f64)),
                _ => None,
            },
            Some(Val::DoubleArray(pr::DoubleArray { array })) => match newtype {
                DataType::FloatArray => conv_arr!(array, f32),
                _ => None,
            },
            Some(Val::FloatArray(pr::FloatArray { array })) => match newtype {
                DataType::DoubleArray => conv_arr!(array, f64),
                _ => None,
            },
            Some(Val::Int32Array(pr::Int32Array { array })) => match newtype {
                DataType::Int64Array  => conv_arr!(array, i64),
                DataType::Uint32Array => conv_arr!(array, u32),
                DataType::Uint64Array => conv_arr!(array, u64),
                DataType::FloatArray  => conv_arr!(array, f32),
                DataType::DoubleArray => conv_arr!(array, f64),
                _ => None,
            },
            Some(Val::Int64Array(pr::Int64Array { array })) => match newtype {
                DataType::Int32Array  => conv_arr!(array, i32),
                DataType::Uint32Array => conv_arr!(array, u32),
                DataType::Uint64Array => conv_arr!(array, u64),
                DataType::FloatArray  => conv_arr!(array, f32),
                DataType::DoubleArray => conv_arr!(array, f64),
                _ => None,
            },
            Some(Val::Uint32Array(pr::Uint32Array { array })) => match newtype {
                DataType::Int32Array  => conv_arr!(array, i32),
                DataType::Int64Array  => conv_arr!(array, i64),
                DataType::Uint64Array => conv_arr!(array, u64),
                DataType::FloatArray  => conv_arr!(array, f32),
                DataType::DoubleArray => conv_arr!(array, f64),
                _ => None,
            },
            Some(Val::Uint64Array(pr::Uint64Array { array })) => match newtype {
                DataType::Int32Array  => conv_arr!(array, i32),
                DataType::Int64Array  => conv_arr!(array, i64),
                DataType::Uint32Array => conv_arr!(array, u32),
                DataType::FloatArray  => conv_arr!(array, f32),
                DataType::DoubleArray => conv_arr!(array, f64),
                _ => None,
            },
            _ => None,
        }
    }

    fn get_dtype(&self) -> DataType {
        match self.0.val {
            None => DataType::Void,
            Some(Val::Bool(_)) => DataType::Bool,
            Some(Val::Double(_)) => DataType::Double,
            Some(Val::Float(_)) => DataType::Float,
            Some(Val::Int32(_)) => DataType::Int32,
            Some(Val::Int64(_)) => DataType::Int64,
            Some(Val::Uint32(_)) => DataType::Uint32,
            Some(Val::Uint64(_)) => DataType::Uint64,
            Some(Val::String(_)) => DataType::String,
            Some(Val::ByteArray(_)) => DataType::ByteArray,
            Some(Val::BoolArray(_)) => DataType::BoolArray,
            Some(Val::DoubleArray(_)) => DataType::DoubleArray,
            Some(Val::FloatArray(_)) => DataType::FloatArray,
            Some(Val::Int32Array(_)) => DataType::Int32Array,
            Some(Val::Int64Array(_)) => DataType::Int64Array,
            Some(Val::Uint32Array(_)) => DataType::Uint32Array,
            Some(Val::Uint64Array(_)) => DataType::Uint64Array,
            Some(Val::StringArray(_)) => DataType::StringArray,
            Some(Val::Int64StringArray(_)) => DataType::Int64StringArray,
            Some(Val::DoubleStringArray(_)) => DataType::DoubleStringArray,
        }
    }

    // Helper for the FromValue implementations.
    fn invalid_type<T>(self, dtype: DataType) -> SpinResult<T> {
        let msg = format!("wrong argument type: {:?}, expected {:?}",
                          self.get_dtype(), dtype);
        spin_err!(ARG_ERROR, &msg)
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0.val {
            None => write!(f, "Value(())"),
            Some(Val::Bool(v)) => write!(f, "Value({})", v),
            Some(Val::Double(v)) => write!(f, "Value({})", v),
            Some(Val::Float(v)) => write!(f, "Value({})", v),
            Some(Val::Int32(v)) => write!(f, "Value({})", v),
            Some(Val::Int64(v)) => write!(f, "Value({})", v),
            Some(Val::Uint32(v)) => write!(f, "Value({})", v),
            Some(Val::Uint64(v)) => write!(f, "Value({})", v),
            Some(Val::String(ref v)) => write!(f, "Value({:?})", v),
            Some(Val::ByteArray(ref v)) => write!(f, "Value({:?})", v),
            Some(Val::BoolArray(pr::BoolArray { ref array })) =>
                write!(f, "Value({:?})", array),
            Some(Val::DoubleArray(pr::DoubleArray { ref array })) =>
                write!(f, "Value({:?})", array),
            Some(Val::FloatArray(pr::FloatArray { ref array })) =>
                write!(f, "Value({:?})", array),
            Some(Val::Int32Array(pr::Int32Array { ref array })) =>
                write!(f, "Value({:?})", array),
            Some(Val::Int64Array(pr::Int64Array { ref array })) =>
                write!(f, "Value({:?})", array),
            Some(Val::Uint32Array(pr::Uint32Array { ref array })) =>
                write!(f, "Value({:?})", array),
            Some(Val::Uint64Array(pr::Uint64Array { ref array })) =>
                write!(f, "Value({:?})", array),
            Some(Val::StringArray(pr::StringArray { ref array })) =>
                write!(f, "Value({:?})", array),
            Some(Val::Int64StringArray(pr::Int64StringArray { ref intarray, ref strarray })) =>
                write!(f, "Value({:?}, {:?})", intarray, strarray),
            Some(Val::DoubleStringArray(pr::DoubleStringArray { ref dblarray, ref strarray })) =>
                write!(f, "Value({:?}, {:?})", dblarray, strarray),
        }
    }
}

pub trait FromValue: Default
where
    Self: Sized,
{
    const DATA_TYPE: DataType;
    fn from_value(v: Value) -> SpinResult<Self>;
}


impl From<()> for Value {
    fn from(_: ()) -> Value {
        Value(pr::Value { val: None })
    }
}

impl FromValue for () {
    const DATA_TYPE: DataType = DataType::Void;
    fn from_value(v: Value) -> SpinResult<()> {
        if v.0.val.is_none() {
            Ok(())
        } else {
            v.invalid_type(Self::DATA_TYPE)
        }
    }
}

impl<'a> From<&'a str> for Value {
    fn from(val: &'a str) -> Value {
        Value::from(val.to_owned())
    }
}

impl From<(Vec<i64>, Vec<String>)> for Value {
    fn from((ival, sval): (Vec<i64>, Vec<String>)) -> Value {
        Value(pr::Value {
            val: Some(Val::Int64StringArray(pr::Int64StringArray {
                intarray: ival,
                strarray: sval,
            })),
        })
    }
}

impl FromValue for (Vec<i64>, Vec<String>) {
    const DATA_TYPE: DataType = DataType::Int64StringArray;
    fn from_value(v: Value) -> SpinResult<(Vec<i64>, Vec<String>)> {
        if let Some(Val::Int64StringArray(pr::Int64StringArray { intarray, strarray })) = v.0.val {
            Ok((intarray, strarray))
        } else {
            v.invalid_type(Self::DATA_TYPE)
        }
    }
}

impl From<(Vec<f64>, Vec<String>)> for Value {
    fn from((fval, sval): (Vec<f64>, Vec<String>)) -> Value {
        Value(pr::Value {
            val: Some(Val::DoubleStringArray(pr::DoubleStringArray {
                dblarray: fval,
                strarray: sval,
            })),
        })
    }
}

impl FromValue for (Vec<f64>, Vec<String>) {
    const DATA_TYPE: DataType = DataType::DoubleStringArray;
    fn from_value(v: Value) -> SpinResult<(Vec<f64>, Vec<String>)> {
        if let Some(Val::DoubleStringArray(pr::DoubleStringArray { dblarray, strarray })) = v.0.val {
            Ok((dblarray, strarray))
        } else {
            v.invalid_type(Self::DATA_TYPE)
        }
    }
}

impl From<Vec<u8>> for Value {
    fn from(val: Vec<u8>) -> Value {
        Value(pr::Value {
            val: Some(Val::ByteArray(val)),
        })
    }
}

impl FromValue for Vec<u8> {
    const DATA_TYPE: DataType = DataType::ByteArray;
    fn from_value(v: Value) -> SpinResult<Vec<u8>> {
        if let Some(Val::ByteArray(bytes)) = v.0.val {
            Ok(bytes)
        } else {
            v.invalid_type(Self::DATA_TYPE)
        }
    }
}

macro_rules! impl_traits {
    ($ty:ty, $dtype:ident, $vectype:ident, $member:ident) => {
        impl From<$ty> for Value {
            fn from(val: $ty) -> Value {
                Value(pr::Value { val: Some(Val::$dtype(val)) })
            }
        }
        impl FromValue for $ty {
            const DATA_TYPE: DataType = DataType::$dtype;
            fn from_value(v: Value) -> SpinResult<$ty> {
                if let Some(Val::$dtype(content)) = v.0.val {
                    Ok(content)
                } else {
                    v.invalid_type(Self::DATA_TYPE)
                }
            }
        }
        impl From<Vec<$ty>> for Value {
            fn from(val: Vec<$ty>) -> Value {
                Value(pr::Value { val: Some(Val::$vectype(
                    pr::$vectype { array: val }
                ))})
            }
        }
        impl<'a> From<&'a [$ty]> for Value {
            fn from(val: &'a [$ty]) -> Value {
                Value::from(val.to_vec())
            }
        }
        impl FromValue for Vec<$ty> {
            const DATA_TYPE: DataType = DataType::$vectype;
            fn from_value(v: Value) -> SpinResult<Vec<$ty>> {
                if let Some(Val::$vectype(pr::$vectype { array })) = v.0.val {
                    Ok(array)
                } else {
                    v.invalid_type(Self::DATA_TYPE)
                }
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
impl_traits!(String, String, StringArray, string);
