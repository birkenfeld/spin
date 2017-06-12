// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct Value {
    // message fields
    vtype: ::std::option::Option<DataType>,
    bool: ::std::vec::Vec<bool>,
    double: ::std::vec::Vec<f64>,
    float: ::std::vec::Vec<f32>,
    int32: ::std::vec::Vec<i32>,
    int64: ::std::vec::Vec<i64>,
    uint32: ::std::vec::Vec<u32>,
    uint64: ::std::vec::Vec<u64>,
    string: ::protobuf::RepeatedField<::std::string::String>,
    bytes: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Value {}

impl Value {
    pub fn new() -> Value {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Value {
        static mut instance: ::protobuf::lazy::Lazy<Value> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Value,
        };
        unsafe {
            instance.get(Value::new)
        }
    }

    // required .DataType vtype = 10;

    pub fn clear_vtype(&mut self) {
        self.vtype = ::std::option::Option::None;
    }

    pub fn has_vtype(&self) -> bool {
        self.vtype.is_some()
    }

    // Param is passed by value, moved
    pub fn set_vtype(&mut self, v: DataType) {
        self.vtype = ::std::option::Option::Some(v);
    }

    pub fn get_vtype(&self) -> DataType {
        self.vtype.unwrap_or(DataType::Void)
    }

    fn get_vtype_for_reflect(&self) -> &::std::option::Option<DataType> {
        &self.vtype
    }

    fn mut_vtype_for_reflect(&mut self) -> &mut ::std::option::Option<DataType> {
        &mut self.vtype
    }

    // repeated bool bool = 1;

    pub fn clear_bool(&mut self) {
        self.bool.clear();
    }

    // Param is passed by value, moved
    pub fn set_bool(&mut self, v: ::std::vec::Vec<bool>) {
        self.bool = v;
    }

    // Mutable pointer to the field.
    pub fn mut_bool(&mut self) -> &mut ::std::vec::Vec<bool> {
        &mut self.bool
    }

    // Take field
    pub fn take_bool(&mut self) -> ::std::vec::Vec<bool> {
        ::std::mem::replace(&mut self.bool, ::std::vec::Vec::new())
    }

    pub fn get_bool(&self) -> &[bool] {
        &self.bool
    }

    fn get_bool_for_reflect(&self) -> &::std::vec::Vec<bool> {
        &self.bool
    }

    fn mut_bool_for_reflect(&mut self) -> &mut ::std::vec::Vec<bool> {
        &mut self.bool
    }

    // repeated double double = 2;

    pub fn clear_double(&mut self) {
        self.double.clear();
    }

    // Param is passed by value, moved
    pub fn set_double(&mut self, v: ::std::vec::Vec<f64>) {
        self.double = v;
    }

    // Mutable pointer to the field.
    pub fn mut_double(&mut self) -> &mut ::std::vec::Vec<f64> {
        &mut self.double
    }

    // Take field
    pub fn take_double(&mut self) -> ::std::vec::Vec<f64> {
        ::std::mem::replace(&mut self.double, ::std::vec::Vec::new())
    }

    pub fn get_double(&self) -> &[f64] {
        &self.double
    }

    fn get_double_for_reflect(&self) -> &::std::vec::Vec<f64> {
        &self.double
    }

    fn mut_double_for_reflect(&mut self) -> &mut ::std::vec::Vec<f64> {
        &mut self.double
    }

    // repeated float float = 3;

    pub fn clear_float(&mut self) {
        self.float.clear();
    }

    // Param is passed by value, moved
    pub fn set_float(&mut self, v: ::std::vec::Vec<f32>) {
        self.float = v;
    }

    // Mutable pointer to the field.
    pub fn mut_float(&mut self) -> &mut ::std::vec::Vec<f32> {
        &mut self.float
    }

    // Take field
    pub fn take_float(&mut self) -> ::std::vec::Vec<f32> {
        ::std::mem::replace(&mut self.float, ::std::vec::Vec::new())
    }

    pub fn get_float(&self) -> &[f32] {
        &self.float
    }

    fn get_float_for_reflect(&self) -> &::std::vec::Vec<f32> {
        &self.float
    }

    fn mut_float_for_reflect(&mut self) -> &mut ::std::vec::Vec<f32> {
        &mut self.float
    }

    // repeated sint32 int32 = 4;

    pub fn clear_int32(&mut self) {
        self.int32.clear();
    }

    // Param is passed by value, moved
    pub fn set_int32(&mut self, v: ::std::vec::Vec<i32>) {
        self.int32 = v;
    }

    // Mutable pointer to the field.
    pub fn mut_int32(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.int32
    }

    // Take field
    pub fn take_int32(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.int32, ::std::vec::Vec::new())
    }

    pub fn get_int32(&self) -> &[i32] {
        &self.int32
    }

    fn get_int32_for_reflect(&self) -> &::std::vec::Vec<i32> {
        &self.int32
    }

    fn mut_int32_for_reflect(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.int32
    }

    // repeated sint64 int64 = 5;

    pub fn clear_int64(&mut self) {
        self.int64.clear();
    }

    // Param is passed by value, moved
    pub fn set_int64(&mut self, v: ::std::vec::Vec<i64>) {
        self.int64 = v;
    }

    // Mutable pointer to the field.
    pub fn mut_int64(&mut self) -> &mut ::std::vec::Vec<i64> {
        &mut self.int64
    }

    // Take field
    pub fn take_int64(&mut self) -> ::std::vec::Vec<i64> {
        ::std::mem::replace(&mut self.int64, ::std::vec::Vec::new())
    }

    pub fn get_int64(&self) -> &[i64] {
        &self.int64
    }

    fn get_int64_for_reflect(&self) -> &::std::vec::Vec<i64> {
        &self.int64
    }

    fn mut_int64_for_reflect(&mut self) -> &mut ::std::vec::Vec<i64> {
        &mut self.int64
    }

    // repeated uint32 uint32 = 6;

    pub fn clear_uint32(&mut self) {
        self.uint32.clear();
    }

    // Param is passed by value, moved
    pub fn set_uint32(&mut self, v: ::std::vec::Vec<u32>) {
        self.uint32 = v;
    }

    // Mutable pointer to the field.
    pub fn mut_uint32(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.uint32
    }

    // Take field
    pub fn take_uint32(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.uint32, ::std::vec::Vec::new())
    }

    pub fn get_uint32(&self) -> &[u32] {
        &self.uint32
    }

    fn get_uint32_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.uint32
    }

    fn mut_uint32_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.uint32
    }

    // repeated uint64 uint64 = 7;

    pub fn clear_uint64(&mut self) {
        self.uint64.clear();
    }

    // Param is passed by value, moved
    pub fn set_uint64(&mut self, v: ::std::vec::Vec<u64>) {
        self.uint64 = v;
    }

    // Mutable pointer to the field.
    pub fn mut_uint64(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.uint64
    }

    // Take field
    pub fn take_uint64(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.uint64, ::std::vec::Vec::new())
    }

    pub fn get_uint64(&self) -> &[u64] {
        &self.uint64
    }

    fn get_uint64_for_reflect(&self) -> &::std::vec::Vec<u64> {
        &self.uint64
    }

    fn mut_uint64_for_reflect(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.uint64
    }

    // repeated string string = 8;

    pub fn clear_string(&mut self) {
        self.string.clear();
    }

    // Param is passed by value, moved
    pub fn set_string(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.string = v;
    }

    // Mutable pointer to the field.
    pub fn mut_string(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.string
    }

    // Take field
    pub fn take_string(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.string, ::protobuf::RepeatedField::new())
    }

    pub fn get_string(&self) -> &[::std::string::String] {
        &self.string
    }

    fn get_string_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.string
    }

    fn mut_string_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.string
    }

    // optional bytes bytes = 9;

    pub fn clear_bytes(&mut self) {
        self.bytes.clear();
    }

    pub fn has_bytes(&self) -> bool {
        self.bytes.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bytes(&mut self, v: ::std::vec::Vec<u8>) {
        self.bytes = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_bytes(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.bytes.is_none() {
            self.bytes.set_default();
        }
        self.bytes.as_mut().unwrap()
    }

    // Take field
    pub fn take_bytes(&mut self) -> ::std::vec::Vec<u8> {
        self.bytes.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_bytes(&self) -> &[u8] {
        match self.bytes.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_bytes_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.bytes
    }

    fn mut_bytes_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.bytes
    }
}

impl ::protobuf::Message for Value {
    fn is_initialized(&self) -> bool {
        if self.vtype.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.vtype = ::std::option::Option::Some(tmp);
                },
                1 => {
                    ::protobuf::rt::read_repeated_bool_into(wire_type, is, &mut self.bool)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_double_into(wire_type, is, &mut self.double)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_float_into(wire_type, is, &mut self.float)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_sint32_into(wire_type, is, &mut self.int32)?;
                },
                5 => {
                    ::protobuf::rt::read_repeated_sint64_into(wire_type, is, &mut self.int64)?;
                },
                6 => {
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.uint32)?;
                },
                7 => {
                    ::protobuf::rt::read_repeated_uint64_into(wire_type, is, &mut self.uint64)?;
                },
                8 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.string)?;
                },
                9 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.bytes)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.vtype {
            my_size += ::protobuf::rt::enum_size(10, v);
        }
        if !self.bool.is_empty() {
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(self.bool.len() as u32) + (self.bool.len() * 1) as u32;
        }
        if !self.double.is_empty() {
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(self.double.len() as u32) + (self.double.len() * 8) as u32;
        }
        if !self.float.is_empty() {
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(self.float.len() as u32) + (self.float.len() * 4) as u32;
        }
        if !self.int32.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_zigzag_size(4, &self.int32);
        }
        if !self.int64.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_zigzag_size(5, &self.int64);
        }
        if !self.uint32.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(6, &self.uint32);
        }
        if !self.uint64.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(7, &self.uint64);
        }
        for value in &self.string {
            my_size += ::protobuf::rt::string_size(8, &value);
        };
        if let Some(ref v) = self.bytes.as_ref() {
            my_size += ::protobuf::rt::bytes_size(9, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.vtype {
            os.write_enum(10, v.value())?;
        }
        if !self.bool.is_empty() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32((self.bool.len() * 1) as u32)?;
            for v in &self.bool {
                os.write_bool_no_tag(*v)?;
            };
        }
        if !self.double.is_empty() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32((self.double.len() * 8) as u32)?;
            for v in &self.double {
                os.write_double_no_tag(*v)?;
            };
        }
        if !self.float.is_empty() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32((self.float.len() * 4) as u32)?;
            for v in &self.float {
                os.write_float_no_tag(*v)?;
            };
        }
        if !self.int32.is_empty() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32(::protobuf::rt::vec_packed_varint_zigzag_data_size(&self.int32))?;
            for v in &self.int32 {
                os.write_sint32_no_tag(*v)?;
            };
        }
        if !self.int64.is_empty() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32(::protobuf::rt::vec_packed_varint_zigzag_data_size(&self.int64))?;
            for v in &self.int64 {
                os.write_sint64_no_tag(*v)?;
            };
        }
        if !self.uint32.is_empty() {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(&self.uint32))?;
            for v in &self.uint32 {
                os.write_uint32_no_tag(*v)?;
            };
        }
        if !self.uint64.is_empty() {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(&self.uint64))?;
            for v in &self.uint64 {
                os.write_uint64_no_tag(*v)?;
            };
        }
        for v in &self.string {
            os.write_string(8, &v)?;
        };
        if let Some(ref v) = self.bytes.as_ref() {
            os.write_bytes(9, &v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Value {
    fn new() -> Value {
        Value::new()
    }

    fn descriptor_static(_: ::std::option::Option<Value>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<DataType>>(
                    "vtype",
                    Value::get_vtype_for_reflect,
                    Value::mut_vtype_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "bool",
                    Value::get_bool_for_reflect,
                    Value::mut_bool_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "double",
                    Value::get_double_for_reflect,
                    Value::mut_double_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "float",
                    Value::get_float_for_reflect,
                    Value::mut_float_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeSint32>(
                    "int32",
                    Value::get_int32_for_reflect,
                    Value::mut_int32_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeSint64>(
                    "int64",
                    Value::get_int64_for_reflect,
                    Value::mut_int64_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "uint32",
                    Value::get_uint32_for_reflect,
                    Value::mut_uint32_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "uint64",
                    Value::get_uint64_for_reflect,
                    Value::mut_uint64_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "string",
                    Value::get_string_for_reflect,
                    Value::mut_string_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "bytes",
                    Value::get_bytes_for_reflect,
                    Value::mut_bytes_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Value>(
                    "Value",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Value {
    fn clear(&mut self) {
        self.clear_vtype();
        self.clear_bool();
        self.clear_double();
        self.clear_float();
        self.clear_int32();
        self.clear_int64();
        self.clear_uint32();
        self.clear_uint64();
        self.clear_string();
        self.clear_bytes();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Value {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Value {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Error {
    // message fields
    reason: ::protobuf::SingularField<::std::string::String>,
    desc: ::protobuf::SingularField<::std::string::String>,
    origin: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Error {}

impl Error {
    pub fn new() -> Error {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Error {
        static mut instance: ::protobuf::lazy::Lazy<Error> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Error,
        };
        unsafe {
            instance.get(Error::new)
        }
    }

    // required string reason = 1;

    pub fn clear_reason(&mut self) {
        self.reason.clear();
    }

    pub fn has_reason(&self) -> bool {
        self.reason.is_some()
    }

    // Param is passed by value, moved
    pub fn set_reason(&mut self, v: ::std::string::String) {
        self.reason = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_reason(&mut self) -> &mut ::std::string::String {
        if self.reason.is_none() {
            self.reason.set_default();
        }
        self.reason.as_mut().unwrap()
    }

    // Take field
    pub fn take_reason(&mut self) -> ::std::string::String {
        self.reason.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_reason(&self) -> &str {
        match self.reason.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_reason_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.reason
    }

    fn mut_reason_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.reason
    }

    // required string desc = 2;

    pub fn clear_desc(&mut self) {
        self.desc.clear();
    }

    pub fn has_desc(&self) -> bool {
        self.desc.is_some()
    }

    // Param is passed by value, moved
    pub fn set_desc(&mut self, v: ::std::string::String) {
        self.desc = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_desc(&mut self) -> &mut ::std::string::String {
        if self.desc.is_none() {
            self.desc.set_default();
        }
        self.desc.as_mut().unwrap()
    }

    // Take field
    pub fn take_desc(&mut self) -> ::std::string::String {
        self.desc.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_desc(&self) -> &str {
        match self.desc.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_desc_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.desc
    }

    fn mut_desc_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.desc
    }

    // required string origin = 3;

    pub fn clear_origin(&mut self) {
        self.origin.clear();
    }

    pub fn has_origin(&self) -> bool {
        self.origin.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin(&mut self, v: ::std::string::String) {
        self.origin = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_origin(&mut self) -> &mut ::std::string::String {
        if self.origin.is_none() {
            self.origin.set_default();
        }
        self.origin.as_mut().unwrap()
    }

    // Take field
    pub fn take_origin(&mut self) -> ::std::string::String {
        self.origin.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_origin(&self) -> &str {
        match self.origin.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_origin_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.origin
    }

    fn mut_origin_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.origin
    }
}

impl ::protobuf::Message for Error {
    fn is_initialized(&self) -> bool {
        if self.reason.is_none() {
            return false;
        }
        if self.desc.is_none() {
            return false;
        }
        if self.origin.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.reason)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.desc)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.origin)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.reason.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.desc.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.origin.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.reason.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.desc.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.origin.as_ref() {
            os.write_string(3, &v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Error {
    fn new() -> Error {
        Error::new()
    }

    fn descriptor_static(_: ::std::option::Option<Error>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "reason",
                    Error::get_reason_for_reflect,
                    Error::mut_reason_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "desc",
                    Error::get_desc_for_reflect,
                    Error::mut_desc_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "origin",
                    Error::get_origin_for_reflect,
                    Error::mut_origin_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Error>(
                    "Error",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Error {
    fn clear(&mut self) {
        self.clear_reason();
        self.clear_desc();
        self.clear_origin();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Error {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CmdDesc {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    doc: ::protobuf::SingularField<::std::string::String>,
    intype: ::std::option::Option<DataType>,
    outtype: ::std::option::Option<DataType>,
    indoc: ::protobuf::SingularField<::std::string::String>,
    outdoc: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CmdDesc {}

impl CmdDesc {
    pub fn new() -> CmdDesc {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CmdDesc {
        static mut instance: ::protobuf::lazy::Lazy<CmdDesc> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CmdDesc,
        };
        unsafe {
            instance.get(CmdDesc::new)
        }
    }

    // required string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        }
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }

    // required string doc = 2;

    pub fn clear_doc(&mut self) {
        self.doc.clear();
    }

    pub fn has_doc(&self) -> bool {
        self.doc.is_some()
    }

    // Param is passed by value, moved
    pub fn set_doc(&mut self, v: ::std::string::String) {
        self.doc = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_doc(&mut self) -> &mut ::std::string::String {
        if self.doc.is_none() {
            self.doc.set_default();
        }
        self.doc.as_mut().unwrap()
    }

    // Take field
    pub fn take_doc(&mut self) -> ::std::string::String {
        self.doc.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_doc(&self) -> &str {
        match self.doc.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_doc_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.doc
    }

    fn mut_doc_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.doc
    }

    // required .DataType intype = 3;

    pub fn clear_intype(&mut self) {
        self.intype = ::std::option::Option::None;
    }

    pub fn has_intype(&self) -> bool {
        self.intype.is_some()
    }

    // Param is passed by value, moved
    pub fn set_intype(&mut self, v: DataType) {
        self.intype = ::std::option::Option::Some(v);
    }

    pub fn get_intype(&self) -> DataType {
        self.intype.unwrap_or(DataType::Void)
    }

    fn get_intype_for_reflect(&self) -> &::std::option::Option<DataType> {
        &self.intype
    }

    fn mut_intype_for_reflect(&mut self) -> &mut ::std::option::Option<DataType> {
        &mut self.intype
    }

    // required .DataType outtype = 4;

    pub fn clear_outtype(&mut self) {
        self.outtype = ::std::option::Option::None;
    }

    pub fn has_outtype(&self) -> bool {
        self.outtype.is_some()
    }

    // Param is passed by value, moved
    pub fn set_outtype(&mut self, v: DataType) {
        self.outtype = ::std::option::Option::Some(v);
    }

    pub fn get_outtype(&self) -> DataType {
        self.outtype.unwrap_or(DataType::Void)
    }

    fn get_outtype_for_reflect(&self) -> &::std::option::Option<DataType> {
        &self.outtype
    }

    fn mut_outtype_for_reflect(&mut self) -> &mut ::std::option::Option<DataType> {
        &mut self.outtype
    }

    // required string indoc = 5;

    pub fn clear_indoc(&mut self) {
        self.indoc.clear();
    }

    pub fn has_indoc(&self) -> bool {
        self.indoc.is_some()
    }

    // Param is passed by value, moved
    pub fn set_indoc(&mut self, v: ::std::string::String) {
        self.indoc = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_indoc(&mut self) -> &mut ::std::string::String {
        if self.indoc.is_none() {
            self.indoc.set_default();
        }
        self.indoc.as_mut().unwrap()
    }

    // Take field
    pub fn take_indoc(&mut self) -> ::std::string::String {
        self.indoc.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_indoc(&self) -> &str {
        match self.indoc.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_indoc_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.indoc
    }

    fn mut_indoc_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.indoc
    }

    // required string outdoc = 6;

    pub fn clear_outdoc(&mut self) {
        self.outdoc.clear();
    }

    pub fn has_outdoc(&self) -> bool {
        self.outdoc.is_some()
    }

    // Param is passed by value, moved
    pub fn set_outdoc(&mut self, v: ::std::string::String) {
        self.outdoc = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_outdoc(&mut self) -> &mut ::std::string::String {
        if self.outdoc.is_none() {
            self.outdoc.set_default();
        }
        self.outdoc.as_mut().unwrap()
    }

    // Take field
    pub fn take_outdoc(&mut self) -> ::std::string::String {
        self.outdoc.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_outdoc(&self) -> &str {
        match self.outdoc.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_outdoc_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.outdoc
    }

    fn mut_outdoc_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.outdoc
    }
}

impl ::protobuf::Message for CmdDesc {
    fn is_initialized(&self) -> bool {
        if self.name.is_none() {
            return false;
        }
        if self.doc.is_none() {
            return false;
        }
        if self.intype.is_none() {
            return false;
        }
        if self.outtype.is_none() {
            return false;
        }
        if self.indoc.is_none() {
            return false;
        }
        if self.outdoc.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.doc)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.intype = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.outtype = ::std::option::Option::Some(tmp);
                },
                5 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.indoc)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.outdoc)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.doc.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(v) = self.intype {
            my_size += ::protobuf::rt::enum_size(3, v);
        }
        if let Some(v) = self.outtype {
            my_size += ::protobuf::rt::enum_size(4, v);
        }
        if let Some(ref v) = self.indoc.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
        }
        if let Some(ref v) = self.outdoc.as_ref() {
            my_size += ::protobuf::rt::string_size(6, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.doc.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(v) = self.intype {
            os.write_enum(3, v.value())?;
        }
        if let Some(v) = self.outtype {
            os.write_enum(4, v.value())?;
        }
        if let Some(ref v) = self.indoc.as_ref() {
            os.write_string(5, &v)?;
        }
        if let Some(ref v) = self.outdoc.as_ref() {
            os.write_string(6, &v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CmdDesc {
    fn new() -> CmdDesc {
        CmdDesc::new()
    }

    fn descriptor_static(_: ::std::option::Option<CmdDesc>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    CmdDesc::get_name_for_reflect,
                    CmdDesc::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "doc",
                    CmdDesc::get_doc_for_reflect,
                    CmdDesc::mut_doc_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<DataType>>(
                    "intype",
                    CmdDesc::get_intype_for_reflect,
                    CmdDesc::mut_intype_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<DataType>>(
                    "outtype",
                    CmdDesc::get_outtype_for_reflect,
                    CmdDesc::mut_outtype_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "indoc",
                    CmdDesc::get_indoc_for_reflect,
                    CmdDesc::mut_indoc_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "outdoc",
                    CmdDesc::get_outdoc_for_reflect,
                    CmdDesc::mut_outdoc_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CmdDesc>(
                    "CmdDesc",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CmdDesc {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_doc();
        self.clear_intype();
        self.clear_outtype();
        self.clear_indoc();
        self.clear_outdoc();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CmdDesc {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CmdDesc {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AttrDesc {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    doc: ::protobuf::SingularField<::std::string::String>,
    field_type: ::std::option::Option<DataType>,
    unit: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AttrDesc {}

impl AttrDesc {
    pub fn new() -> AttrDesc {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AttrDesc {
        static mut instance: ::protobuf::lazy::Lazy<AttrDesc> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AttrDesc,
        };
        unsafe {
            instance.get(AttrDesc::new)
        }
    }

    // required string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        }
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }

    // required string doc = 2;

    pub fn clear_doc(&mut self) {
        self.doc.clear();
    }

    pub fn has_doc(&self) -> bool {
        self.doc.is_some()
    }

    // Param is passed by value, moved
    pub fn set_doc(&mut self, v: ::std::string::String) {
        self.doc = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_doc(&mut self) -> &mut ::std::string::String {
        if self.doc.is_none() {
            self.doc.set_default();
        }
        self.doc.as_mut().unwrap()
    }

    // Take field
    pub fn take_doc(&mut self) -> ::std::string::String {
        self.doc.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_doc(&self) -> &str {
        match self.doc.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_doc_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.doc
    }

    fn mut_doc_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.doc
    }

    // required .DataType type = 3;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: DataType) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> DataType {
        self.field_type.unwrap_or(DataType::Void)
    }

    fn get_field_type_for_reflect(&self) -> &::std::option::Option<DataType> {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::std::option::Option<DataType> {
        &mut self.field_type
    }

    // required string unit = 4;

    pub fn clear_unit(&mut self) {
        self.unit.clear();
    }

    pub fn has_unit(&self) -> bool {
        self.unit.is_some()
    }

    // Param is passed by value, moved
    pub fn set_unit(&mut self, v: ::std::string::String) {
        self.unit = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_unit(&mut self) -> &mut ::std::string::String {
        if self.unit.is_none() {
            self.unit.set_default();
        }
        self.unit.as_mut().unwrap()
    }

    // Take field
    pub fn take_unit(&mut self) -> ::std::string::String {
        self.unit.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_unit(&self) -> &str {
        match self.unit.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_unit_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.unit
    }

    fn mut_unit_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.unit
    }
}

impl ::protobuf::Message for AttrDesc {
    fn is_initialized(&self) -> bool {
        if self.name.is_none() {
            return false;
        }
        if self.doc.is_none() {
            return false;
        }
        if self.field_type.is_none() {
            return false;
        }
        if self.unit.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.doc)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.unit)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.doc.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(v) = self.field_type {
            my_size += ::protobuf::rt::enum_size(3, v);
        }
        if let Some(ref v) = self.unit.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.doc.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(v) = self.field_type {
            os.write_enum(3, v.value())?;
        }
        if let Some(ref v) = self.unit.as_ref() {
            os.write_string(4, &v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AttrDesc {
    fn new() -> AttrDesc {
        AttrDesc::new()
    }

    fn descriptor_static(_: ::std::option::Option<AttrDesc>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    AttrDesc::get_name_for_reflect,
                    AttrDesc::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "doc",
                    AttrDesc::get_doc_for_reflect,
                    AttrDesc::mut_doc_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<DataType>>(
                    "type",
                    AttrDesc::get_field_type_for_reflect,
                    AttrDesc::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "unit",
                    AttrDesc::get_unit_for_reflect,
                    AttrDesc::mut_unit_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AttrDesc>(
                    "AttrDesc",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AttrDesc {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_doc();
        self.clear_field_type();
        self.clear_unit();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AttrDesc {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AttrDesc {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PropDesc {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    doc: ::protobuf::SingularField<::std::string::String>,
    field_type: ::std::option::Option<DataType>,
    default: ::protobuf::SingularPtrField<Value>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PropDesc {}

impl PropDesc {
    pub fn new() -> PropDesc {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PropDesc {
        static mut instance: ::protobuf::lazy::Lazy<PropDesc> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PropDesc,
        };
        unsafe {
            instance.get(PropDesc::new)
        }
    }

    // required string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        }
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }

    // required string doc = 2;

    pub fn clear_doc(&mut self) {
        self.doc.clear();
    }

    pub fn has_doc(&self) -> bool {
        self.doc.is_some()
    }

    // Param is passed by value, moved
    pub fn set_doc(&mut self, v: ::std::string::String) {
        self.doc = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_doc(&mut self) -> &mut ::std::string::String {
        if self.doc.is_none() {
            self.doc.set_default();
        }
        self.doc.as_mut().unwrap()
    }

    // Take field
    pub fn take_doc(&mut self) -> ::std::string::String {
        self.doc.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_doc(&self) -> &str {
        match self.doc.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_doc_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.doc
    }

    fn mut_doc_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.doc
    }

    // required .DataType type = 3;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: DataType) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> DataType {
        self.field_type.unwrap_or(DataType::Void)
    }

    fn get_field_type_for_reflect(&self) -> &::std::option::Option<DataType> {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::std::option::Option<DataType> {
        &mut self.field_type
    }

    // optional .Value default = 4;

    pub fn clear_default(&mut self) {
        self.default.clear();
    }

    pub fn has_default(&self) -> bool {
        self.default.is_some()
    }

    // Param is passed by value, moved
    pub fn set_default(&mut self, v: Value) {
        self.default = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_default(&mut self) -> &mut Value {
        if self.default.is_none() {
            self.default.set_default();
        }
        self.default.as_mut().unwrap()
    }

    // Take field
    pub fn take_default(&mut self) -> Value {
        self.default.take().unwrap_or_else(|| Value::new())
    }

    pub fn get_default(&self) -> &Value {
        self.default.as_ref().unwrap_or_else(|| Value::default_instance())
    }

    fn get_default_for_reflect(&self) -> &::protobuf::SingularPtrField<Value> {
        &self.default
    }

    fn mut_default_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Value> {
        &mut self.default
    }
}

impl ::protobuf::Message for PropDesc {
    fn is_initialized(&self) -> bool {
        if self.name.is_none() {
            return false;
        }
        if self.doc.is_none() {
            return false;
        }
        if self.field_type.is_none() {
            return false;
        }
        for v in &self.default {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.doc)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.default)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.doc.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(v) = self.field_type {
            my_size += ::protobuf::rt::enum_size(3, v);
        }
        if let Some(ref v) = self.default.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.doc.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(v) = self.field_type {
            os.write_enum(3, v.value())?;
        }
        if let Some(ref v) = self.default.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PropDesc {
    fn new() -> PropDesc {
        PropDesc::new()
    }

    fn descriptor_static(_: ::std::option::Option<PropDesc>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    PropDesc::get_name_for_reflect,
                    PropDesc::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "doc",
                    PropDesc::get_doc_for_reflect,
                    PropDesc::mut_doc_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<DataType>>(
                    "type",
                    PropDesc::get_field_type_for_reflect,
                    PropDesc::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Value>>(
                    "default",
                    PropDesc::get_default_for_reflect,
                    PropDesc::mut_default_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PropDesc>(
                    "PropDesc",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PropDesc {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_doc();
        self.clear_field_type();
        self.clear_default();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PropDesc {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PropDesc {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Request {
    // message fields
    seqno: ::std::option::Option<u32>,
    rtype: ::std::option::Option<ReqType>,
    name: ::protobuf::SingularField<::std::string::String>,
    value: ::protobuf::SingularPtrField<Value>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Request {}

impl Request {
    pub fn new() -> Request {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Request {
        static mut instance: ::protobuf::lazy::Lazy<Request> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Request,
        };
        unsafe {
            instance.get(Request::new)
        }
    }

    // required uint32 seqno = 1;

    pub fn clear_seqno(&mut self) {
        self.seqno = ::std::option::Option::None;
    }

    pub fn has_seqno(&self) -> bool {
        self.seqno.is_some()
    }

    // Param is passed by value, moved
    pub fn set_seqno(&mut self, v: u32) {
        self.seqno = ::std::option::Option::Some(v);
    }

    pub fn get_seqno(&self) -> u32 {
        self.seqno.unwrap_or(0)
    }

    fn get_seqno_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.seqno
    }

    fn mut_seqno_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.seqno
    }

    // required .ReqType rtype = 2;

    pub fn clear_rtype(&mut self) {
        self.rtype = ::std::option::Option::None;
    }

    pub fn has_rtype(&self) -> bool {
        self.rtype.is_some()
    }

    // Param is passed by value, moved
    pub fn set_rtype(&mut self, v: ReqType) {
        self.rtype = ::std::option::Option::Some(v);
    }

    pub fn get_rtype(&self) -> ReqType {
        self.rtype.unwrap_or(ReqType::ReqPing)
    }

    fn get_rtype_for_reflect(&self) -> &::std::option::Option<ReqType> {
        &self.rtype
    }

    fn mut_rtype_for_reflect(&mut self) -> &mut ::std::option::Option<ReqType> {
        &mut self.rtype
    }

    // optional string name = 3;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        }
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }

    // optional .Value value = 4;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: Value) {
        self.value = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut Value {
        if self.value.is_none() {
            self.value.set_default();
        }
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> Value {
        self.value.take().unwrap_or_else(|| Value::new())
    }

    pub fn get_value(&self) -> &Value {
        self.value.as_ref().unwrap_or_else(|| Value::default_instance())
    }

    fn get_value_for_reflect(&self) -> &::protobuf::SingularPtrField<Value> {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Value> {
        &mut self.value
    }
}

impl ::protobuf::Message for Request {
    fn is_initialized(&self) -> bool {
        if self.seqno.is_none() {
            return false;
        }
        if self.rtype.is_none() {
            return false;
        }
        for v in &self.value {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.seqno = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.rtype = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.value)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.seqno {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.rtype {
            my_size += ::protobuf::rt::enum_size(2, v);
        }
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        if let Some(ref v) = self.value.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.seqno {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.rtype {
            os.write_enum(2, v.value())?;
        }
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(3, &v)?;
        }
        if let Some(ref v) = self.value.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Request {
    fn new() -> Request {
        Request::new()
    }

    fn descriptor_static(_: ::std::option::Option<Request>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "seqno",
                    Request::get_seqno_for_reflect,
                    Request::mut_seqno_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ReqType>>(
                    "rtype",
                    Request::get_rtype_for_reflect,
                    Request::mut_rtype_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    Request::get_name_for_reflect,
                    Request::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Value>>(
                    "value",
                    Request::get_value_for_reflect,
                    Request::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Request>(
                    "Request",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Request {
    fn clear(&mut self) {
        self.clear_seqno();
        self.clear_rtype();
        self.clear_name();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Request {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Request {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Response {
    // message fields
    seqno: ::std::option::Option<u32>,
    rtype: ::std::option::Option<RespType>,
    error: ::protobuf::SingularPtrField<Error>,
    value: ::protobuf::SingularPtrField<Value>,
    cmds: ::protobuf::RepeatedField<CmdDesc>,
    attrs: ::protobuf::RepeatedField<AttrDesc>,
    props: ::protobuf::RepeatedField<PropDesc>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Response {}

impl Response {
    pub fn new() -> Response {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Response {
        static mut instance: ::protobuf::lazy::Lazy<Response> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Response,
        };
        unsafe {
            instance.get(Response::new)
        }
    }

    // required uint32 seqno = 1;

    pub fn clear_seqno(&mut self) {
        self.seqno = ::std::option::Option::None;
    }

    pub fn has_seqno(&self) -> bool {
        self.seqno.is_some()
    }

    // Param is passed by value, moved
    pub fn set_seqno(&mut self, v: u32) {
        self.seqno = ::std::option::Option::Some(v);
    }

    pub fn get_seqno(&self) -> u32 {
        self.seqno.unwrap_or(0)
    }

    fn get_seqno_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.seqno
    }

    fn mut_seqno_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.seqno
    }

    // required .RespType rtype = 2;

    pub fn clear_rtype(&mut self) {
        self.rtype = ::std::option::Option::None;
    }

    pub fn has_rtype(&self) -> bool {
        self.rtype.is_some()
    }

    // Param is passed by value, moved
    pub fn set_rtype(&mut self, v: RespType) {
        self.rtype = ::std::option::Option::Some(v);
    }

    pub fn get_rtype(&self) -> RespType {
        self.rtype.unwrap_or(RespType::RespVoid)
    }

    fn get_rtype_for_reflect(&self) -> &::std::option::Option<RespType> {
        &self.rtype
    }

    fn mut_rtype_for_reflect(&mut self) -> &mut ::std::option::Option<RespType> {
        &mut self.rtype
    }

    // optional .Error error = 3;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: Error) {
        self.error = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut Error {
        if self.error.is_none() {
            self.error.set_default();
        }
        self.error.as_mut().unwrap()
    }

    // Take field
    pub fn take_error(&mut self) -> Error {
        self.error.take().unwrap_or_else(|| Error::new())
    }

    pub fn get_error(&self) -> &Error {
        self.error.as_ref().unwrap_or_else(|| Error::default_instance())
    }

    fn get_error_for_reflect(&self) -> &::protobuf::SingularPtrField<Error> {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Error> {
        &mut self.error
    }

    // optional .Value value = 5;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: Value) {
        self.value = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut Value {
        if self.value.is_none() {
            self.value.set_default();
        }
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> Value {
        self.value.take().unwrap_or_else(|| Value::new())
    }

    pub fn get_value(&self) -> &Value {
        self.value.as_ref().unwrap_or_else(|| Value::default_instance())
    }

    fn get_value_for_reflect(&self) -> &::protobuf::SingularPtrField<Value> {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Value> {
        &mut self.value
    }

    // repeated .CmdDesc cmds = 6;

    pub fn clear_cmds(&mut self) {
        self.cmds.clear();
    }

    // Param is passed by value, moved
    pub fn set_cmds(&mut self, v: ::protobuf::RepeatedField<CmdDesc>) {
        self.cmds = v;
    }

    // Mutable pointer to the field.
    pub fn mut_cmds(&mut self) -> &mut ::protobuf::RepeatedField<CmdDesc> {
        &mut self.cmds
    }

    // Take field
    pub fn take_cmds(&mut self) -> ::protobuf::RepeatedField<CmdDesc> {
        ::std::mem::replace(&mut self.cmds, ::protobuf::RepeatedField::new())
    }

    pub fn get_cmds(&self) -> &[CmdDesc] {
        &self.cmds
    }

    fn get_cmds_for_reflect(&self) -> &::protobuf::RepeatedField<CmdDesc> {
        &self.cmds
    }

    fn mut_cmds_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CmdDesc> {
        &mut self.cmds
    }

    // repeated .AttrDesc attrs = 7;

    pub fn clear_attrs(&mut self) {
        self.attrs.clear();
    }

    // Param is passed by value, moved
    pub fn set_attrs(&mut self, v: ::protobuf::RepeatedField<AttrDesc>) {
        self.attrs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_attrs(&mut self) -> &mut ::protobuf::RepeatedField<AttrDesc> {
        &mut self.attrs
    }

    // Take field
    pub fn take_attrs(&mut self) -> ::protobuf::RepeatedField<AttrDesc> {
        ::std::mem::replace(&mut self.attrs, ::protobuf::RepeatedField::new())
    }

    pub fn get_attrs(&self) -> &[AttrDesc] {
        &self.attrs
    }

    fn get_attrs_for_reflect(&self) -> &::protobuf::RepeatedField<AttrDesc> {
        &self.attrs
    }

    fn mut_attrs_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<AttrDesc> {
        &mut self.attrs
    }

    // repeated .PropDesc props = 8;

    pub fn clear_props(&mut self) {
        self.props.clear();
    }

    // Param is passed by value, moved
    pub fn set_props(&mut self, v: ::protobuf::RepeatedField<PropDesc>) {
        self.props = v;
    }

    // Mutable pointer to the field.
    pub fn mut_props(&mut self) -> &mut ::protobuf::RepeatedField<PropDesc> {
        &mut self.props
    }

    // Take field
    pub fn take_props(&mut self) -> ::protobuf::RepeatedField<PropDesc> {
        ::std::mem::replace(&mut self.props, ::protobuf::RepeatedField::new())
    }

    pub fn get_props(&self) -> &[PropDesc] {
        &self.props
    }

    fn get_props_for_reflect(&self) -> &::protobuf::RepeatedField<PropDesc> {
        &self.props
    }

    fn mut_props_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<PropDesc> {
        &mut self.props
    }
}

impl ::protobuf::Message for Response {
    fn is_initialized(&self) -> bool {
        if self.seqno.is_none() {
            return false;
        }
        if self.rtype.is_none() {
            return false;
        }
        for v in &self.error {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.value {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.cmds {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.attrs {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.props {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.seqno = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.rtype = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.error)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.value)?;
                },
                6 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.cmds)?;
                },
                7 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.attrs)?;
                },
                8 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.props)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.seqno {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.rtype {
            my_size += ::protobuf::rt::enum_size(2, v);
        }
        if let Some(ref v) = self.error.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.value.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.cmds {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.attrs {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.props {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.seqno {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.rtype {
            os.write_enum(2, v.value())?;
        }
        if let Some(ref v) = self.error.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.value.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.cmds {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.attrs {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.props {
            os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Response {
    fn new() -> Response {
        Response::new()
    }

    fn descriptor_static(_: ::std::option::Option<Response>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "seqno",
                    Response::get_seqno_for_reflect,
                    Response::mut_seqno_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<RespType>>(
                    "rtype",
                    Response::get_rtype_for_reflect,
                    Response::mut_rtype_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Error>>(
                    "error",
                    Response::get_error_for_reflect,
                    Response::mut_error_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Value>>(
                    "value",
                    Response::get_value_for_reflect,
                    Response::mut_value_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CmdDesc>>(
                    "cmds",
                    Response::get_cmds_for_reflect,
                    Response::mut_cmds_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<AttrDesc>>(
                    "attrs",
                    Response::get_attrs_for_reflect,
                    Response::mut_attrs_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PropDesc>>(
                    "props",
                    Response::get_props_for_reflect,
                    Response::mut_props_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Response>(
                    "Response",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Response {
    fn clear(&mut self) {
        self.clear_seqno();
        self.clear_rtype();
        self.clear_error();
        self.clear_value();
        self.clear_cmds();
        self.clear_attrs();
        self.clear_props();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Response {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Response {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum DataType {
    Void = 0,
    Bool = 1,
    Double = 2,
    Float = 3,
    Int32 = 4,
    Int64 = 5,
    UInt32 = 6,
    UInt64 = 7,
    String = 8,
    ByteArray = 10,
    BoolArray = 11,
    DoubleArray = 12,
    FloatArray = 13,
    Int32Array = 14,
    Int64Array = 15,
    UInt32Array = 16,
    UInt64Array = 17,
    StringArray = 18,
    Int64StringArray = 21,
    DoubleStringArray = 22,
}

impl ::protobuf::ProtobufEnum for DataType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<DataType> {
        match value {
            0 => ::std::option::Option::Some(DataType::Void),
            1 => ::std::option::Option::Some(DataType::Bool),
            2 => ::std::option::Option::Some(DataType::Double),
            3 => ::std::option::Option::Some(DataType::Float),
            4 => ::std::option::Option::Some(DataType::Int32),
            5 => ::std::option::Option::Some(DataType::Int64),
            6 => ::std::option::Option::Some(DataType::UInt32),
            7 => ::std::option::Option::Some(DataType::UInt64),
            8 => ::std::option::Option::Some(DataType::String),
            10 => ::std::option::Option::Some(DataType::ByteArray),
            11 => ::std::option::Option::Some(DataType::BoolArray),
            12 => ::std::option::Option::Some(DataType::DoubleArray),
            13 => ::std::option::Option::Some(DataType::FloatArray),
            14 => ::std::option::Option::Some(DataType::Int32Array),
            15 => ::std::option::Option::Some(DataType::Int64Array),
            16 => ::std::option::Option::Some(DataType::UInt32Array),
            17 => ::std::option::Option::Some(DataType::UInt64Array),
            18 => ::std::option::Option::Some(DataType::StringArray),
            21 => ::std::option::Option::Some(DataType::Int64StringArray),
            22 => ::std::option::Option::Some(DataType::DoubleStringArray),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [DataType] = &[
            DataType::Void,
            DataType::Bool,
            DataType::Double,
            DataType::Float,
            DataType::Int32,
            DataType::Int64,
            DataType::UInt32,
            DataType::UInt64,
            DataType::String,
            DataType::ByteArray,
            DataType::BoolArray,
            DataType::DoubleArray,
            DataType::FloatArray,
            DataType::Int32Array,
            DataType::Int64Array,
            DataType::UInt32Array,
            DataType::UInt64Array,
            DataType::StringArray,
            DataType::Int64StringArray,
            DataType::DoubleStringArray,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<DataType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("DataType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for DataType {
}

impl ::protobuf::reflect::ProtobufValue for DataType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ReqType {
    ReqPing = 0,
    ReqExecCmd = 1,
    ReqReadAttr = 2,
    ReqWriteAttr = 3,
    ReqGetProp = 4,
    ReqSetProp = 5,
    ReqQueryAPI = 10,
}

impl ::protobuf::ProtobufEnum for ReqType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ReqType> {
        match value {
            0 => ::std::option::Option::Some(ReqType::ReqPing),
            1 => ::std::option::Option::Some(ReqType::ReqExecCmd),
            2 => ::std::option::Option::Some(ReqType::ReqReadAttr),
            3 => ::std::option::Option::Some(ReqType::ReqWriteAttr),
            4 => ::std::option::Option::Some(ReqType::ReqGetProp),
            5 => ::std::option::Option::Some(ReqType::ReqSetProp),
            10 => ::std::option::Option::Some(ReqType::ReqQueryAPI),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ReqType] = &[
            ReqType::ReqPing,
            ReqType::ReqExecCmd,
            ReqType::ReqReadAttr,
            ReqType::ReqWriteAttr,
            ReqType::ReqGetProp,
            ReqType::ReqSetProp,
            ReqType::ReqQueryAPI,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<ReqType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ReqType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ReqType {
}

impl ::protobuf::reflect::ProtobufValue for ReqType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum RespType {
    RespVoid = 0,
    RespError = 1,
    RespValue = 2,
    RespAPI = 3,
}

impl ::protobuf::ProtobufEnum for RespType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<RespType> {
        match value {
            0 => ::std::option::Option::Some(RespType::RespVoid),
            1 => ::std::option::Option::Some(RespType::RespError),
            2 => ::std::option::Option::Some(RespType::RespValue),
            3 => ::std::option::Option::Some(RespType::RespAPI),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [RespType] = &[
            RespType::RespVoid,
            RespType::RespError,
            RespType::RespValue,
            RespType::RespAPI,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<RespType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("RespType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for RespType {
}

impl ::protobuf::reflect::ProtobufValue for RespType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\tmsg.proto\"\x90\x02\n\x05Value\x12\x1f\n\x05vtype\x18\n\x20\x02(\x0e\
    2\t.DataTypeR\x05vtype\x12\x16\n\x04bool\x18\x01\x20\x03(\x08R\x04boolB\
    \x02\x10\x01\x12\x1a\n\x06double\x18\x02\x20\x03(\x01R\x06doubleB\x02\
    \x10\x01\x12\x18\n\x05float\x18\x03\x20\x03(\x02R\x05floatB\x02\x10\x01\
    \x12\x18\n\x05int32\x18\x04\x20\x03(\x11R\x05int32B\x02\x10\x01\x12\x18\
    \n\x05int64\x18\x05\x20\x03(\x12R\x05int64B\x02\x10\x01\x12\x1a\n\x06uin\
    t32\x18\x06\x20\x03(\rR\x06uint32B\x02\x10\x01\x12\x1a\n\x06uint64\x18\
    \x07\x20\x03(\x04R\x06uint64B\x02\x10\x01\x12\x16\n\x06string\x18\x08\
    \x20\x03(\tR\x06string\x12\x14\n\x05bytes\x18\t\x20\x01(\x0cR\x05bytes\"\
    K\n\x05Error\x12\x16\n\x06reason\x18\x01\x20\x02(\tR\x06reason\x12\x12\n\
    \x04desc\x18\x02\x20\x02(\tR\x04desc\x12\x16\n\x06origin\x18\x03\x20\x02\
    (\tR\x06origin\"\xa5\x01\n\x07CmdDesc\x12\x12\n\x04name\x18\x01\x20\x02(\
    \tR\x04name\x12\x10\n\x03doc\x18\x02\x20\x02(\tR\x03doc\x12!\n\x06intype\
    \x18\x03\x20\x02(\x0e2\t.DataTypeR\x06intype\x12#\n\x07outtype\x18\x04\
    \x20\x02(\x0e2\t.DataTypeR\x07outtype\x12\x14\n\x05indoc\x18\x05\x20\x02\
    (\tR\x05indoc\x12\x16\n\x06outdoc\x18\x06\x20\x02(\tR\x06outdoc\"c\n\x08\
    AttrDesc\x12\x12\n\x04name\x18\x01\x20\x02(\tR\x04name\x12\x10\n\x03doc\
    \x18\x02\x20\x02(\tR\x03doc\x12\x1d\n\x04type\x18\x03\x20\x02(\x0e2\t.Da\
    taTypeR\x04type\x12\x12\n\x04unit\x18\x04\x20\x02(\tR\x04unit\"q\n\x08Pr\
    opDesc\x12\x12\n\x04name\x18\x01\x20\x02(\tR\x04name\x12\x10\n\x03doc\
    \x18\x02\x20\x02(\tR\x03doc\x12\x1d\n\x04type\x18\x03\x20\x02(\x0e2\t.Da\
    taTypeR\x04type\x12\x20\n\x07default\x18\x04\x20\x01(\x0b2\x06.ValueR\
    \x07default\"q\n\x07Request\x12\x14\n\x05seqno\x18\x01\x20\x02(\rR\x05se\
    qno\x12\x1e\n\x05rtype\x18\x02\x20\x02(\x0e2\x08.ReqTypeR\x05rtype\x12\
    \x12\n\x04name\x18\x03\x20\x01(\tR\x04name\x12\x1c\n\x05value\x18\x04\
    \x20\x01(\x0b2\x06.ValueR\x05value\"\xdd\x01\n\x08Response\x12\x14\n\x05\
    seqno\x18\x01\x20\x02(\rR\x05seqno\x12\x1f\n\x05rtype\x18\x02\x20\x02(\
    \x0e2\t.RespTypeR\x05rtype\x12\x1c\n\x05error\x18\x03\x20\x01(\x0b2\x06.\
    ErrorR\x05error\x12\x1c\n\x05value\x18\x05\x20\x01(\x0b2\x06.ValueR\x05v\
    alue\x12\x1c\n\x04cmds\x18\x06\x20\x03(\x0b2\x08.CmdDescR\x04cmds\x12\
    \x1f\n\x05attrs\x18\x07\x20\x03(\x0b2\t.AttrDescR\x05attrs\x12\x1f\n\x05\
    props\x18\x08\x20\x03(\x0b2\t.PropDescR\x05props*\xae\x02\n\x08DataType\
    \x12\x08\n\x04Void\x10\0\x12\x08\n\x04Bool\x10\x01\x12\n\n\x06Double\x10\
    \x02\x12\t\n\x05Float\x10\x03\x12\t\n\x05Int32\x10\x04\x12\t\n\x05Int64\
    \x10\x05\x12\n\n\x06UInt32\x10\x06\x12\n\n\x06UInt64\x10\x07\x12\n\n\x06\
    String\x10\x08\x12\r\n\tByteArray\x10\n\x12\r\n\tBoolArray\x10\x0b\x12\
    \x0f\n\x0bDoubleArray\x10\x0c\x12\x0e\n\nFloatArray\x10\r\x12\x0e\n\nInt\
    32Array\x10\x0e\x12\x0e\n\nInt64Array\x10\x0f\x12\x0f\n\x0bUInt32Array\
    \x10\x10\x12\x0f\n\x0bUInt64Array\x10\x11\x12\x0f\n\x0bStringArray\x10\
    \x12\x12\x14\n\x10Int64StringArray\x10\x15\x12\x15\n\x11DoubleStringArra\
    y\x10\x16*z\n\x07ReqType\x12\x0b\n\x07ReqPing\x10\0\x12\x0e\n\nReqExecCm\
    d\x10\x01\x12\x0f\n\x0bReqReadAttr\x10\x02\x12\x10\n\x0cReqWriteAttr\x10\
    \x03\x12\x0e\n\nReqGetProp\x10\x04\x12\x0e\n\nReqSetProp\x10\x05\x12\x0f\
    \n\x0bReqQueryAPI\x10\n*C\n\x08RespType\x12\x0c\n\x08RespVoid\x10\0\x12\
    \r\n\tRespError\x10\x01\x12\r\n\tRespValue\x10\x02\x12\x0b\n\x07RespAPI\
    \x10\x03J\xb9(\n\x06\x12\x04\x04\0k\x01\n\x81\x01\n\x01\x0c\x12\x03\x04\
    \0\x1225\x20Spin\x20RPC\x20library,\x20copyright\x202015-2017\x20Georg\
    \x20Brandl.\n2@!\x20Protocol\x20buffers:\x20definition\x20of\x20message\
    \x20format\x20sent\x20over\x200MQ.\n\n\n\n\x02\x05\0\x12\x04\x06\0\x1b\
    \x01\n\n\n\x03\x05\0\x01\x12\x03\x06\x05\r\n\x0b\n\x04\x05\0\x02\0\x12\
    \x03\x07\x04\x0f\n\x0c\n\x05\x05\0\x02\0\x01\x12\x03\x07\x04\x08\n\x0c\n\
    \x05\x05\0\x02\0\x02\x12\x03\x07\r\x0e\n\x0b\n\x04\x05\0\x02\x01\x12\x03\
    \x08\x04\x0f\n\x0c\n\x05\x05\0\x02\x01\x01\x12\x03\x08\x04\x08\n\x0c\n\
    \x05\x05\0\x02\x01\x02\x12\x03\x08\r\x0e\n\x0b\n\x04\x05\0\x02\x02\x12\
    \x03\t\x04\x0f\n\x0c\n\x05\x05\0\x02\x02\x01\x12\x03\t\x04\n\n\x0c\n\x05\
    \x05\0\x02\x02\x02\x12\x03\t\r\x0e\n\x0b\n\x04\x05\0\x02\x03\x12\x03\n\
    \x04\x0f\n\x0c\n\x05\x05\0\x02\x03\x01\x12\x03\n\x04\t\n\x0c\n\x05\x05\0\
    \x02\x03\x02\x12\x03\n\r\x0e\n\x0b\n\x04\x05\0\x02\x04\x12\x03\x0b\x04\
    \x0f\n\x0c\n\x05\x05\0\x02\x04\x01\x12\x03\x0b\x04\t\n\x0c\n\x05\x05\0\
    \x02\x04\x02\x12\x03\x0b\r\x0e\n\x0b\n\x04\x05\0\x02\x05\x12\x03\x0c\x04\
    \x0f\n\x0c\n\x05\x05\0\x02\x05\x01\x12\x03\x0c\x04\t\n\x0c\n\x05\x05\0\
    \x02\x05\x02\x12\x03\x0c\r\x0e\n\x0b\n\x04\x05\0\x02\x06\x12\x03\r\x04\
    \x0f\n\x0c\n\x05\x05\0\x02\x06\x01\x12\x03\r\x04\n\n\x0c\n\x05\x05\0\x02\
    \x06\x02\x12\x03\r\r\x0e\n\x0b\n\x04\x05\0\x02\x07\x12\x03\x0e\x04\x0f\n\
    \x0c\n\x05\x05\0\x02\x07\x01\x12\x03\x0e\x04\n\n\x0c\n\x05\x05\0\x02\x07\
    \x02\x12\x03\x0e\r\x0e\n\x0b\n\x04\x05\0\x02\x08\x12\x03\x0f\x04\x0f\n\
    \x0c\n\x05\x05\0\x02\x08\x01\x12\x03\x0f\x04\n\n\x0c\n\x05\x05\0\x02\x08\
    \x02\x12\x03\x0f\r\x0e\n\x0b\n\x04\x05\0\x02\t\x12\x03\x10\x04\x15\n\x0c\
    \n\x05\x05\0\x02\t\x01\x12\x03\x10\x04\r\n\x0c\n\x05\x05\0\x02\t\x02\x12\
    \x03\x10\x12\x14\n\x0b\n\x04\x05\0\x02\n\x12\x03\x11\x04\x15\n\x0c\n\x05\
    \x05\0\x02\n\x01\x12\x03\x11\x04\r\n\x0c\n\x05\x05\0\x02\n\x02\x12\x03\
    \x11\x12\x14\n\x0b\n\x04\x05\0\x02\x0b\x12\x03\x12\x04\x15\n\x0c\n\x05\
    \x05\0\x02\x0b\x01\x12\x03\x12\x04\x0f\n\x0c\n\x05\x05\0\x02\x0b\x02\x12\
    \x03\x12\x12\x14\n\x0b\n\x04\x05\0\x02\x0c\x12\x03\x13\x04\x15\n\x0c\n\
    \x05\x05\0\x02\x0c\x01\x12\x03\x13\x04\x0e\n\x0c\n\x05\x05\0\x02\x0c\x02\
    \x12\x03\x13\x12\x14\n\x0b\n\x04\x05\0\x02\r\x12\x03\x14\x04\x15\n\x0c\n\
    \x05\x05\0\x02\r\x01\x12\x03\x14\x04\x0e\n\x0c\n\x05\x05\0\x02\r\x02\x12\
    \x03\x14\x12\x14\n\x0b\n\x04\x05\0\x02\x0e\x12\x03\x15\x04\x15\n\x0c\n\
    \x05\x05\0\x02\x0e\x01\x12\x03\x15\x04\x0e\n\x0c\n\x05\x05\0\x02\x0e\x02\
    \x12\x03\x15\x12\x14\n\x0b\n\x04\x05\0\x02\x0f\x12\x03\x16\x04\x15\n\x0c\
    \n\x05\x05\0\x02\x0f\x01\x12\x03\x16\x04\x0f\n\x0c\n\x05\x05\0\x02\x0f\
    \x02\x12\x03\x16\x12\x14\n\x0b\n\x04\x05\0\x02\x10\x12\x03\x17\x04\x15\n\
    \x0c\n\x05\x05\0\x02\x10\x01\x12\x03\x17\x04\x0f\n\x0c\n\x05\x05\0\x02\
    \x10\x02\x12\x03\x17\x12\x14\n\x0b\n\x04\x05\0\x02\x11\x12\x03\x18\x04\
    \x15\n\x0c\n\x05\x05\0\x02\x11\x01\x12\x03\x18\x04\x0f\n\x0c\n\x05\x05\0\
    \x02\x11\x02\x12\x03\x18\x12\x14\n\x0b\n\x04\x05\0\x02\x12\x12\x03\x19\
    \x04\x1b\n\x0c\n\x05\x05\0\x02\x12\x01\x12\x03\x19\x04\x14\n\x0c\n\x05\
    \x05\0\x02\x12\x02\x12\x03\x19\x18\x1a\n\x0b\n\x04\x05\0\x02\x13\x12\x03\
    \x1a\x04\x1b\n\x0c\n\x05\x05\0\x02\x13\x01\x12\x03\x1a\x04\x15\n\x0c\n\
    \x05\x05\0\x02\x13\x02\x12\x03\x1a\x18\x1a\n\n\n\x02\x04\0\x12\x04\x1d\0\
    (\x01\n\n\n\x03\x04\0\x01\x12\x03\x1d\x08\r\n\x0b\n\x04\x04\0\x02\0\x12\
    \x03\x1e\x04#\n\x0c\n\x05\x04\0\x02\0\x04\x12\x03\x1e\x04\x0c\n\x0c\n\
    \x05\x04\0\x02\0\x06\x12\x03\x1e\r\x15\n\x0c\n\x05\x04\0\x02\0\x01\x12\
    \x03\x1e\x16\x1b\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x1e\x20\"\n\x0b\n\
    \x04\x04\0\x02\x01\x12\x03\x1f\x040\n\x0c\n\x05\x04\0\x02\x01\x04\x12\
    \x03\x1f\x04\x0c\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x1f\r\x11\n\x0c\n\
    \x05\x04\0\x02\x01\x01\x12\x03\x1f\x16\x1a\n\x0c\n\x05\x04\0\x02\x01\x03\
    \x12\x03\x1f\x20!\n\x0c\n\x05\x04\0\x02\x01\x08\x12\x03\x1f\"/\n\x0f\n\
    \x08\x04\0\x02\x01\x08\xe7\x07\0\x12\x03\x1f#.\n\x10\n\t\x04\0\x02\x01\
    \x08\xe7\x07\0\x02\x12\x03\x1f#)\n\x11\n\n\x04\0\x02\x01\x08\xe7\x07\0\
    \x02\0\x12\x03\x1f#)\n\x12\n\x0b\x04\0\x02\x01\x08\xe7\x07\0\x02\0\x01\
    \x12\x03\x1f#)\n\x10\n\t\x04\0\x02\x01\x08\xe7\x07\0\x03\x12\x03\x1f*.\n\
    \x0b\n\x04\x04\0\x02\x02\x12\x03\x20\x040\n\x0c\n\x05\x04\0\x02\x02\x04\
    \x12\x03\x20\x04\x0c\n\x0c\n\x05\x04\0\x02\x02\x05\x12\x03\x20\r\x13\n\
    \x0c\n\x05\x04\0\x02\x02\x01\x12\x03\x20\x16\x1c\n\x0c\n\x05\x04\0\x02\
    \x02\x03\x12\x03\x20\x20!\n\x0c\n\x05\x04\0\x02\x02\x08\x12\x03\x20\"/\n\
    \x0f\n\x08\x04\0\x02\x02\x08\xe7\x07\0\x12\x03\x20#.\n\x10\n\t\x04\0\x02\
    \x02\x08\xe7\x07\0\x02\x12\x03\x20#)\n\x11\n\n\x04\0\x02\x02\x08\xe7\x07\
    \0\x02\0\x12\x03\x20#)\n\x12\n\x0b\x04\0\x02\x02\x08\xe7\x07\0\x02\0\x01\
    \x12\x03\x20#)\n\x10\n\t\x04\0\x02\x02\x08\xe7\x07\0\x03\x12\x03\x20*.\n\
    \x0b\n\x04\x04\0\x02\x03\x12\x03!\x040\n\x0c\n\x05\x04\0\x02\x03\x04\x12\
    \x03!\x04\x0c\n\x0c\n\x05\x04\0\x02\x03\x05\x12\x03!\r\x12\n\x0c\n\x05\
    \x04\0\x02\x03\x01\x12\x03!\x16\x1b\n\x0c\n\x05\x04\0\x02\x03\x03\x12\
    \x03!\x20!\n\x0c\n\x05\x04\0\x02\x03\x08\x12\x03!\"/\n\x0f\n\x08\x04\0\
    \x02\x03\x08\xe7\x07\0\x12\x03!#.\n\x10\n\t\x04\0\x02\x03\x08\xe7\x07\0\
    \x02\x12\x03!#)\n\x11\n\n\x04\0\x02\x03\x08\xe7\x07\0\x02\0\x12\x03!#)\n\
    \x12\n\x0b\x04\0\x02\x03\x08\xe7\x07\0\x02\0\x01\x12\x03!#)\n\x10\n\t\
    \x04\0\x02\x03\x08\xe7\x07\0\x03\x12\x03!*.\n\x0b\n\x04\x04\0\x02\x04\
    \x12\x03\"\x040\n\x0c\n\x05\x04\0\x02\x04\x04\x12\x03\"\x04\x0c\n\x0c\n\
    \x05\x04\0\x02\x04\x05\x12\x03\"\r\x13\n\x0c\n\x05\x04\0\x02\x04\x01\x12\
    \x03\"\x16\x1b\n\x0c\n\x05\x04\0\x02\x04\x03\x12\x03\"\x20!\n\x0c\n\x05\
    \x04\0\x02\x04\x08\x12\x03\"\"/\n\x0f\n\x08\x04\0\x02\x04\x08\xe7\x07\0\
    \x12\x03\"#.\n\x10\n\t\x04\0\x02\x04\x08\xe7\x07\0\x02\x12\x03\"#)\n\x11\
    \n\n\x04\0\x02\x04\x08\xe7\x07\0\x02\0\x12\x03\"#)\n\x12\n\x0b\x04\0\x02\
    \x04\x08\xe7\x07\0\x02\0\x01\x12\x03\"#)\n\x10\n\t\x04\0\x02\x04\x08\xe7\
    \x07\0\x03\x12\x03\"*.\n\x0b\n\x04\x04\0\x02\x05\x12\x03#\x040\n\x0c\n\
    \x05\x04\0\x02\x05\x04\x12\x03#\x04\x0c\n\x0c\n\x05\x04\0\x02\x05\x05\
    \x12\x03#\r\x13\n\x0c\n\x05\x04\0\x02\x05\x01\x12\x03#\x16\x1b\n\x0c\n\
    \x05\x04\0\x02\x05\x03\x12\x03#\x20!\n\x0c\n\x05\x04\0\x02\x05\x08\x12\
    \x03#\"/\n\x0f\n\x08\x04\0\x02\x05\x08\xe7\x07\0\x12\x03##.\n\x10\n\t\
    \x04\0\x02\x05\x08\xe7\x07\0\x02\x12\x03##)\n\x11\n\n\x04\0\x02\x05\x08\
    \xe7\x07\0\x02\0\x12\x03##)\n\x12\n\x0b\x04\0\x02\x05\x08\xe7\x07\0\x02\
    \0\x01\x12\x03##)\n\x10\n\t\x04\0\x02\x05\x08\xe7\x07\0\x03\x12\x03#*.\n\
    \x0b\n\x04\x04\0\x02\x06\x12\x03$\x040\n\x0c\n\x05\x04\0\x02\x06\x04\x12\
    \x03$\x04\x0c\n\x0c\n\x05\x04\0\x02\x06\x05\x12\x03$\r\x13\n\x0c\n\x05\
    \x04\0\x02\x06\x01\x12\x03$\x16\x1c\n\x0c\n\x05\x04\0\x02\x06\x03\x12\
    \x03$\x20!\n\x0c\n\x05\x04\0\x02\x06\x08\x12\x03$\"/\n\x0f\n\x08\x04\0\
    \x02\x06\x08\xe7\x07\0\x12\x03$#.\n\x10\n\t\x04\0\x02\x06\x08\xe7\x07\0\
    \x02\x12\x03$#)\n\x11\n\n\x04\0\x02\x06\x08\xe7\x07\0\x02\0\x12\x03$#)\n\
    \x12\n\x0b\x04\0\x02\x06\x08\xe7\x07\0\x02\0\x01\x12\x03$#)\n\x10\n\t\
    \x04\0\x02\x06\x08\xe7\x07\0\x03\x12\x03$*.\n\x0b\n\x04\x04\0\x02\x07\
    \x12\x03%\x040\n\x0c\n\x05\x04\0\x02\x07\x04\x12\x03%\x04\x0c\n\x0c\n\
    \x05\x04\0\x02\x07\x05\x12\x03%\r\x13\n\x0c\n\x05\x04\0\x02\x07\x01\x12\
    \x03%\x16\x1c\n\x0c\n\x05\x04\0\x02\x07\x03\x12\x03%\x20!\n\x0c\n\x05\
    \x04\0\x02\x07\x08\x12\x03%\"/\n\x0f\n\x08\x04\0\x02\x07\x08\xe7\x07\0\
    \x12\x03%#.\n\x10\n\t\x04\0\x02\x07\x08\xe7\x07\0\x02\x12\x03%#)\n\x11\n\
    \n\x04\0\x02\x07\x08\xe7\x07\0\x02\0\x12\x03%#)\n\x12\n\x0b\x04\0\x02\
    \x07\x08\xe7\x07\0\x02\0\x01\x12\x03%#)\n\x10\n\t\x04\0\x02\x07\x08\xe7\
    \x07\0\x03\x12\x03%*.\n\x0b\n\x04\x04\0\x02\x08\x12\x03&\x04\"\n\x0c\n\
    \x05\x04\0\x02\x08\x04\x12\x03&\x04\x0c\n\x0c\n\x05\x04\0\x02\x08\x05\
    \x12\x03&\r\x13\n\x0c\n\x05\x04\0\x02\x08\x01\x12\x03&\x16\x1c\n\x0c\n\
    \x05\x04\0\x02\x08\x03\x12\x03&\x20!\n\x0b\n\x04\x04\0\x02\t\x12\x03'\
    \x04\"\n\x0c\n\x05\x04\0\x02\t\x04\x12\x03'\x04\x0c\n\x0c\n\x05\x04\0\
    \x02\t\x05\x12\x03'\r\x12\n\x0c\n\x05\x04\0\x02\t\x01\x12\x03'\x16\x1b\n\
    \x0c\n\x05\x04\0\x02\t\x03\x12\x03'\x20!\n\n\n\x02\x04\x01\x12\x04*\0.\
    \x01\n\n\n\x03\x04\x01\x01\x12\x03*\x08\r\n\x0b\n\x04\x04\x01\x02\0\x12\
    \x03+\x04\x1f\n\x0c\n\x05\x04\x01\x02\0\x04\x12\x03+\x04\x0c\n\x0c\n\x05\
    \x04\x01\x02\0\x05\x12\x03+\r\x13\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03+\
    \x14\x1a\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03+\x1d\x1e\n\x0b\n\x04\x04\
    \x01\x02\x01\x12\x03,\x04\x1d\n\x0c\n\x05\x04\x01\x02\x01\x04\x12\x03,\
    \x04\x0c\n\x0c\n\x05\x04\x01\x02\x01\x05\x12\x03,\r\x13\n\x0c\n\x05\x04\
    \x01\x02\x01\x01\x12\x03,\x14\x18\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\
    \x03,\x1b\x1c\n\x0b\n\x04\x04\x01\x02\x02\x12\x03-\x04\x1f\n\x0c\n\x05\
    \x04\x01\x02\x02\x04\x12\x03-\x04\x0c\n\x0c\n\x05\x04\x01\x02\x02\x05\
    \x12\x03-\r\x13\n\x0c\n\x05\x04\x01\x02\x02\x01\x12\x03-\x14\x1a\n\x0c\n\
    \x05\x04\x01\x02\x02\x03\x12\x03-\x1d\x1e\n[\n\x02\x04\x02\x12\x042\09\
    \x012O//////////////////////////////////////////////////////////////////\
    ////////////\n\n\n\n\x03\x04\x02\x01\x12\x032\x08\x0f\n\x0b\n\x04\x04\
    \x02\x02\0\x12\x033\x04\x1f\n\x0c\n\x05\x04\x02\x02\0\x04\x12\x033\x04\
    \x0c\n\x0c\n\x05\x04\x02\x02\0\x05\x12\x033\r\x13\n\x0c\n\x05\x04\x02\
    \x02\0\x01\x12\x033\x16\x1a\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x033\x1d\
    \x1e\n\x0b\n\x04\x04\x02\x02\x01\x12\x034\x04\x1e\n\x0c\n\x05\x04\x02\
    \x02\x01\x04\x12\x034\x04\x0c\n\x0c\n\x05\x04\x02\x02\x01\x05\x12\x034\r\
    \x13\n\x0c\n\x05\x04\x02\x02\x01\x01\x12\x034\x16\x19\n\x0c\n\x05\x04\
    \x02\x02\x01\x03\x12\x034\x1c\x1d\n\x0b\n\x04\x04\x02\x02\x02\x12\x035\
    \x04!\n\x0c\n\x05\x04\x02\x02\x02\x04\x12\x035\x04\x0c\n\x0c\n\x05\x04\
    \x02\x02\x02\x06\x12\x035\r\x15\n\x0c\n\x05\x04\x02\x02\x02\x01\x12\x035\
    \x16\x1c\n\x0c\n\x05\x04\x02\x02\x02\x03\x12\x035\x1f\x20\n\x0b\n\x04\
    \x04\x02\x02\x03\x12\x036\x04\"\n\x0c\n\x05\x04\x02\x02\x03\x04\x12\x036\
    \x04\x0c\n\x0c\n\x05\x04\x02\x02\x03\x06\x12\x036\r\x15\n\x0c\n\x05\x04\
    \x02\x02\x03\x01\x12\x036\x16\x1d\n\x0c\n\x05\x04\x02\x02\x03\x03\x12\
    \x036\x20!\n\x0b\n\x04\x04\x02\x02\x04\x12\x037\x04\x20\n\x0c\n\x05\x04\
    \x02\x02\x04\x04\x12\x037\x04\x0c\n\x0c\n\x05\x04\x02\x02\x04\x05\x12\
    \x037\r\x13\n\x0c\n\x05\x04\x02\x02\x04\x01\x12\x037\x16\x1b\n\x0c\n\x05\
    \x04\x02\x02\x04\x03\x12\x037\x1e\x1f\n\x0b\n\x04\x04\x02\x02\x05\x12\
    \x038\x04!\n\x0c\n\x05\x04\x02\x02\x05\x04\x12\x038\x04\x0c\n\x0c\n\x05\
    \x04\x02\x02\x05\x05\x12\x038\r\x13\n\x0c\n\x05\x04\x02\x02\x05\x01\x12\
    \x038\x16\x1c\n\x0c\n\x05\x04\x02\x02\x05\x03\x12\x038\x1f\x20\n\n\n\x02\
    \x04\x03\x12\x04;\0@\x01\n\n\n\x03\x04\x03\x01\x12\x03;\x08\x10\n\x0b\n\
    \x04\x04\x03\x02\0\x12\x03<\x04\x1f\n\x0c\n\x05\x04\x03\x02\0\x04\x12\
    \x03<\x04\x0c\n\x0c\n\x05\x04\x03\x02\0\x05\x12\x03<\r\x13\n\x0c\n\x05\
    \x04\x03\x02\0\x01\x12\x03<\x16\x1a\n\x0c\n\x05\x04\x03\x02\0\x03\x12\
    \x03<\x1d\x1e\n\x0b\n\x04\x04\x03\x02\x01\x12\x03=\x04\x1e\n\x0c\n\x05\
    \x04\x03\x02\x01\x04\x12\x03=\x04\x0c\n\x0c\n\x05\x04\x03\x02\x01\x05\
    \x12\x03=\r\x13\n\x0c\n\x05\x04\x03\x02\x01\x01\x12\x03=\x16\x19\n\x0c\n\
    \x05\x04\x03\x02\x01\x03\x12\x03=\x1c\x1d\n\x0b\n\x04\x04\x03\x02\x02\
    \x12\x03>\x04\x1f\n\x0c\n\x05\x04\x03\x02\x02\x04\x12\x03>\x04\x0c\n\x0c\
    \n\x05\x04\x03\x02\x02\x06\x12\x03>\r\x15\n\x0c\n\x05\x04\x03\x02\x02\
    \x01\x12\x03>\x16\x1a\n\x0c\n\x05\x04\x03\x02\x02\x03\x12\x03>\x1d\x1e\n\
    \x0b\n\x04\x04\x03\x02\x03\x12\x03?\x04\x1f\n\x0c\n\x05\x04\x03\x02\x03\
    \x04\x12\x03?\x04\x0c\n\x0c\n\x05\x04\x03\x02\x03\x05\x12\x03?\r\x13\n\
    \x0c\n\x05\x04\x03\x02\x03\x01\x12\x03?\x16\x1a\n\x0c\n\x05\x04\x03\x02\
    \x03\x03\x12\x03?\x1d\x1e\n\n\n\x02\x04\x04\x12\x04B\0G\x01\n\n\n\x03\
    \x04\x04\x01\x12\x03B\x08\x10\n\x0b\n\x04\x04\x04\x02\0\x12\x03C\x04\x1f\
    \n\x0c\n\x05\x04\x04\x02\0\x04\x12\x03C\x04\x0c\n\x0c\n\x05\x04\x04\x02\
    \0\x05\x12\x03C\r\x13\n\x0c\n\x05\x04\x04\x02\0\x01\x12\x03C\x16\x1a\n\
    \x0c\n\x05\x04\x04\x02\0\x03\x12\x03C\x1d\x1e\n\x0b\n\x04\x04\x04\x02\
    \x01\x12\x03D\x04\x1e\n\x0c\n\x05\x04\x04\x02\x01\x04\x12\x03D\x04\x0c\n\
    \x0c\n\x05\x04\x04\x02\x01\x05\x12\x03D\r\x13\n\x0c\n\x05\x04\x04\x02\
    \x01\x01\x12\x03D\x16\x19\n\x0c\n\x05\x04\x04\x02\x01\x03\x12\x03D\x1c\
    \x1d\n\x0b\n\x04\x04\x04\x02\x02\x12\x03E\x04\x1f\n\x0c\n\x05\x04\x04\
    \x02\x02\x04\x12\x03E\x04\x0c\n\x0c\n\x05\x04\x04\x02\x02\x06\x12\x03E\r\
    \x15\n\x0c\n\x05\x04\x04\x02\x02\x01\x12\x03E\x16\x1a\n\x0c\n\x05\x04\
    \x04\x02\x02\x03\x12\x03E\x1d\x1e\n\x0b\n\x04\x04\x04\x02\x03\x12\x03F\
    \x04\"\n\x0c\n\x05\x04\x04\x02\x03\x04\x12\x03F\x04\x0c\n\x0c\n\x05\x04\
    \x04\x02\x03\x06\x12\x03F\r\x12\n\x0c\n\x05\x04\x04\x02\x03\x01\x12\x03F\
    \x16\x1d\n\x0c\n\x05\x04\x04\x02\x03\x03\x12\x03F\x20!\n[\n\x02\x05\x01\
    \x12\x04K\0S\x012O//////////////////////////////////////////////////////\
    ////////////////////////\n\n\n\n\x03\x05\x01\x01\x12\x03K\x05\x0c\n\x0b\
    \n\x04\x05\x01\x02\0\x12\x03L\x04\x10\n\x0c\n\x05\x05\x01\x02\0\x01\x12\
    \x03L\x04\x0b\n\x0c\n\x05\x05\x01\x02\0\x02\x12\x03L\x0e\x0f\n\x0b\n\x04\
    \x05\x01\x02\x01\x12\x03M\x04\x13\n\x0c\n\x05\x05\x01\x02\x01\x01\x12\
    \x03M\x04\x0e\n\x0c\n\x05\x05\x01\x02\x01\x02\x12\x03M\x11\x12\n\x0b\n\
    \x04\x05\x01\x02\x02\x12\x03N\x04\x14\n\x0c\n\x05\x05\x01\x02\x02\x01\
    \x12\x03N\x04\x0f\n\x0c\n\x05\x05\x01\x02\x02\x02\x12\x03N\x12\x13\n\x0b\
    \n\x04\x05\x01\x02\x03\x12\x03O\x04\x15\n\x0c\n\x05\x05\x01\x02\x03\x01\
    \x12\x03O\x04\x10\n\x0c\n\x05\x05\x01\x02\x03\x02\x12\x03O\x13\x14\n\x0b\
    \n\x04\x05\x01\x02\x04\x12\x03P\x04\x13\n\x0c\n\x05\x05\x01\x02\x04\x01\
    \x12\x03P\x04\x0e\n\x0c\n\x05\x05\x01\x02\x04\x02\x12\x03P\x11\x12\n\x0b\
    \n\x04\x05\x01\x02\x05\x12\x03Q\x04\x13\n\x0c\n\x05\x05\x01\x02\x05\x01\
    \x12\x03Q\x04\x0e\n\x0c\n\x05\x05\x01\x02\x05\x02\x12\x03Q\x11\x12\n\x0b\
    \n\x04\x05\x01\x02\x06\x12\x03R\x04\x15\n\x0c\n\x05\x05\x01\x02\x06\x01\
    \x12\x03R\x04\x0f\n\x0c\n\x05\x05\x01\x02\x06\x02\x12\x03R\x12\x14\n\n\n\
    \x02\x04\x05\x12\x04U\0Z\x01\n\n\n\x03\x04\x05\x01\x12\x03U\x08\x0f\n\
    \x0b\n\x04\x04\x05\x02\0\x12\x03V\x04\x1f\n\x0c\n\x05\x04\x05\x02\0\x04\
    \x12\x03V\x04\x0c\n\x0c\n\x05\x04\x05\x02\0\x05\x12\x03V\r\x13\n\x0c\n\
    \x05\x04\x05\x02\0\x01\x12\x03V\x15\x1a\n\x0c\n\x05\x04\x05\x02\0\x03\
    \x12\x03V\x1d\x1e\n\x0b\n\x04\x04\x05\x02\x01\x12\x03W\x04\x1f\n\x0c\n\
    \x05\x04\x05\x02\x01\x04\x12\x03W\x04\x0c\n\x0c\n\x05\x04\x05\x02\x01\
    \x06\x12\x03W\r\x14\n\x0c\n\x05\x04\x05\x02\x01\x01\x12\x03W\x15\x1a\n\
    \x0c\n\x05\x04\x05\x02\x01\x03\x12\x03W\x1d\x1e\n\x0b\n\x04\x04\x05\x02\
    \x02\x12\x03X\x04\x1e\n\x0c\n\x05\x04\x05\x02\x02\x04\x12\x03X\x04\x0c\n\
    \x0c\n\x05\x04\x05\x02\x02\x05\x12\x03X\r\x13\n\x0c\n\x05\x04\x05\x02\
    \x02\x01\x12\x03X\x15\x19\n\x0c\n\x05\x04\x05\x02\x02\x03\x12\x03X\x1c\
    \x1d\n\x0b\n\x04\x04\x05\x02\x03\x12\x03Y\x04\x1f\n\x0c\n\x05\x04\x05\
    \x02\x03\x04\x12\x03Y\x04\x0c\n\x0c\n\x05\x04\x05\x02\x03\x06\x12\x03Y\r\
    \x12\n\x0c\n\x05\x04\x05\x02\x03\x01\x12\x03Y\x15\x1a\n\x0c\n\x05\x04\
    \x05\x02\x03\x03\x12\x03Y\x1d\x1e\n\n\n\x02\x05\x02\x12\x04\\\0a\x01\n\n\
    \n\x03\x05\x02\x01\x12\x03\\\x05\r\n\x0b\n\x04\x05\x02\x02\0\x12\x03]\
    \x04\x11\n\x0c\n\x05\x05\x02\x02\0\x01\x12\x03]\x04\x0c\n\x0c\n\x05\x05\
    \x02\x02\0\x02\x12\x03]\x0f\x10\n\x0b\n\x04\x05\x02\x02\x01\x12\x03^\x04\
    \x12\n\x0c\n\x05\x05\x02\x02\x01\x01\x12\x03^\x04\r\n\x0c\n\x05\x05\x02\
    \x02\x01\x02\x12\x03^\x10\x11\n\x0b\n\x04\x05\x02\x02\x02\x12\x03_\x04\
    \x12\n\x0c\n\x05\x05\x02\x02\x02\x01\x12\x03_\x04\r\n\x0c\n\x05\x05\x02\
    \x02\x02\x02\x12\x03_\x10\x11\n\x0b\n\x04\x05\x02\x02\x03\x12\x03`\x04\
    \x10\n\x0c\n\x05\x05\x02\x02\x03\x01\x12\x03`\x04\x0b\n\x0c\n\x05\x05\
    \x02\x02\x03\x02\x12\x03`\x0e\x0f\n\n\n\x02\x04\x06\x12\x04c\0k\x01\n\n\
    \n\x03\x04\x06\x01\x12\x03c\x08\x10\n\x0b\n\x04\x04\x06\x02\0\x12\x03d\
    \x04\x20\n\x0c\n\x05\x04\x06\x02\0\x04\x12\x03d\x04\x0c\n\x0c\n\x05\x04\
    \x06\x02\0\x05\x12\x03d\r\x13\n\x0c\n\x05\x04\x06\x02\0\x01\x12\x03d\x16\
    \x1b\n\x0c\n\x05\x04\x06\x02\0\x03\x12\x03d\x1e\x1f\n\x0b\n\x04\x04\x06\
    \x02\x01\x12\x03e\x04\x20\n\x0c\n\x05\x04\x06\x02\x01\x04\x12\x03e\x04\
    \x0c\n\x0c\n\x05\x04\x06\x02\x01\x06\x12\x03e\r\x15\n\x0c\n\x05\x04\x06\
    \x02\x01\x01\x12\x03e\x16\x1b\n\x0c\n\x05\x04\x06\x02\x01\x03\x12\x03e\
    \x1e\x1f\n\x0b\n\x04\x04\x06\x02\x02\x12\x03f\x04\x20\n\x0c\n\x05\x04\
    \x06\x02\x02\x04\x12\x03f\x04\x0c\n\x0c\n\x05\x04\x06\x02\x02\x06\x12\
    \x03f\r\x12\n\x0c\n\x05\x04\x06\x02\x02\x01\x12\x03f\x16\x1b\n\x0c\n\x05\
    \x04\x06\x02\x02\x03\x12\x03f\x1e\x1f\n\x0b\n\x04\x04\x06\x02\x03\x12\
    \x03g\x04\x20\n\x0c\n\x05\x04\x06\x02\x03\x04\x12\x03g\x04\x0c\n\x0c\n\
    \x05\x04\x06\x02\x03\x06\x12\x03g\r\x12\n\x0c\n\x05\x04\x06\x02\x03\x01\
    \x12\x03g\x16\x1b\n\x0c\n\x05\x04\x06\x02\x03\x03\x12\x03g\x1e\x1f\n\x0b\
    \n\x04\x04\x06\x02\x04\x12\x03h\x04\x1f\n\x0c\n\x05\x04\x06\x02\x04\x04\
    \x12\x03h\x04\x0c\n\x0c\n\x05\x04\x06\x02\x04\x06\x12\x03h\r\x14\n\x0c\n\
    \x05\x04\x06\x02\x04\x01\x12\x03h\x16\x1a\n\x0c\n\x05\x04\x06\x02\x04\
    \x03\x12\x03h\x1d\x1e\n\x0b\n\x04\x04\x06\x02\x05\x12\x03i\x04\x20\n\x0c\
    \n\x05\x04\x06\x02\x05\x04\x12\x03i\x04\x0c\n\x0c\n\x05\x04\x06\x02\x05\
    \x06\x12\x03i\r\x15\n\x0c\n\x05\x04\x06\x02\x05\x01\x12\x03i\x16\x1b\n\
    \x0c\n\x05\x04\x06\x02\x05\x03\x12\x03i\x1e\x1f\n\x0b\n\x04\x04\x06\x02\
    \x06\x12\x03j\x04\x20\n\x0c\n\x05\x04\x06\x02\x06\x04\x12\x03j\x04\x0c\n\
    \x0c\n\x05\x04\x06\x02\x06\x06\x12\x03j\r\x15\n\x0c\n\x05\x04\x06\x02\
    \x06\x01\x12\x03j\x16\x1b\n\x0c\n\x05\x04\x06\x02\x06\x03\x12\x03j\x1e\
    \x1f\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
