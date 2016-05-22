// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default)]
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
    cached_size: ::std::cell::Cell<u32>,
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
            instance.get(|| {
                Value {
                    vtype: ::std::option::Option::None,
                    bool: ::std::vec::Vec::new(),
                    double: ::std::vec::Vec::new(),
                    float: ::std::vec::Vec::new(),
                    int32: ::std::vec::Vec::new(),
                    int64: ::std::vec::Vec::new(),
                    uint32: ::std::vec::Vec::new(),
                    uint64: ::std::vec::Vec::new(),
                    string: ::protobuf::RepeatedField::new(),
                    bytes: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
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
        };
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
}

impl ::protobuf::Message for Value {
    fn is_initialized(&self) -> bool {
        if self.vtype.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.vtype = ::std::option::Option::Some(tmp);
                },
                1 => {
                    try!(::protobuf::rt::read_repeated_bool_into(wire_type, is, &mut self.bool));
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_double_into(wire_type, is, &mut self.double));
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_float_into(wire_type, is, &mut self.float));
                },
                4 => {
                    try!(::protobuf::rt::read_repeated_sint32_into(wire_type, is, &mut self.int32));
                },
                5 => {
                    try!(::protobuf::rt::read_repeated_sint64_into(wire_type, is, &mut self.int64));
                },
                6 => {
                    try!(::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.uint32));
                },
                7 => {
                    try!(::protobuf::rt::read_repeated_uint64_into(wire_type, is, &mut self.uint64));
                },
                8 => {
                    try!(::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.string));
                },
                9 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.bytes));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.vtype.iter() {
            my_size += ::protobuf::rt::enum_size(10, *value);
        };
        if !self.bool.is_empty() {
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(self.bool.len() as u32) + (self.bool.len() * 1) as u32;
        };
        if !self.double.is_empty() {
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(self.double.len() as u32) + (self.double.len() * 8) as u32;
        };
        if !self.float.is_empty() {
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(self.float.len() as u32) + (self.float.len() * 4) as u32;
        };
        if !self.int32.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_zigzag_size(4, &self.int32);
        };
        if !self.int64.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_zigzag_size(5, &self.int64);
        };
        if !self.uint32.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(6, &self.uint32);
        };
        if !self.uint64.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(7, &self.uint64);
        };
        for value in self.string.iter() {
            my_size += ::protobuf::rt::string_size(8, &value);
        };
        for value in self.bytes.iter() {
            my_size += ::protobuf::rt::bytes_size(9, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.vtype {
            try!(os.write_enum(10, v.value()));
        };
        if !self.bool.is_empty() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            // TODO: Data size is computed again, it should be cached
            try!(os.write_raw_varint32((self.bool.len() * 1) as u32));
            for v in self.bool.iter() {
                try!(os.write_bool_no_tag(*v));
            };
        };
        if !self.double.is_empty() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            // TODO: Data size is computed again, it should be cached
            try!(os.write_raw_varint32((self.double.len() * 8) as u32));
            for v in self.double.iter() {
                try!(os.write_double_no_tag(*v));
            };
        };
        if !self.float.is_empty() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            // TODO: Data size is computed again, it should be cached
            try!(os.write_raw_varint32((self.float.len() * 4) as u32));
            for v in self.float.iter() {
                try!(os.write_float_no_tag(*v));
            };
        };
        if !self.int32.is_empty() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            // TODO: Data size is computed again, it should be cached
            try!(os.write_raw_varint32(::protobuf::rt::vec_packed_varint_zigzag_data_size(&self.int32)));
            for v in self.int32.iter() {
                try!(os.write_sint32_no_tag(*v));
            };
        };
        if !self.int64.is_empty() {
            try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
            // TODO: Data size is computed again, it should be cached
            try!(os.write_raw_varint32(::protobuf::rt::vec_packed_varint_zigzag_data_size(&self.int64)));
            for v in self.int64.iter() {
                try!(os.write_sint64_no_tag(*v));
            };
        };
        if !self.uint32.is_empty() {
            try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
            // TODO: Data size is computed again, it should be cached
            try!(os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(&self.uint32)));
            for v in self.uint32.iter() {
                try!(os.write_uint32_no_tag(*v));
            };
        };
        if !self.uint64.is_empty() {
            try!(os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited));
            // TODO: Data size is computed again, it should be cached
            try!(os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(&self.uint64)));
            for v in self.uint64.iter() {
                try!(os.write_uint64_no_tag(*v));
            };
        };
        for v in self.string.iter() {
            try!(os.write_string(8, &v));
        };
        if let Some(v) = self.bytes.as_ref() {
            try!(os.write_bytes(9, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Value>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
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
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "vtype",
                    Value::has_vtype,
                    Value::get_vtype,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_bool_accessor(
                    "bool",
                    Value::get_bool,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_f64_accessor(
                    "double",
                    Value::get_double,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_f32_accessor(
                    "float",
                    Value::get_float,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_i32_accessor(
                    "int32",
                    Value::get_int32,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_i64_accessor(
                    "int64",
                    Value::get_int64,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_u32_accessor(
                    "uint32",
                    Value::get_uint32,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_u64_accessor(
                    "uint64",
                    Value::get_uint64,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_string_accessor(
                    "string",
                    Value::get_string,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "bytes",
                    Value::has_bytes,
                    Value::get_bytes,
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

impl ::std::cmp::PartialEq for Value {
    fn eq(&self, other: &Value) -> bool {
        self.vtype == other.vtype &&
        self.bool == other.bool &&
        self.double == other.double &&
        self.float == other.float &&
        self.int32 == other.int32 &&
        self.int64 == other.int64 &&
        self.uint32 == other.uint32 &&
        self.uint64 == other.uint64 &&
        self.string == other.string &&
        self.bytes == other.bytes &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Value {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Error {
    // message fields
    reason: ::protobuf::SingularField<::std::string::String>,
    desc: ::protobuf::SingularField<::std::string::String>,
    origin: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
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
            instance.get(|| {
                Error {
                    reason: ::protobuf::SingularField::none(),
                    desc: ::protobuf::SingularField::none(),
                    origin: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
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
        };
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
        };
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
        };
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
}

impl ::protobuf::Message for Error {
    fn is_initialized(&self) -> bool {
        if self.reason.is_none() {
            return false;
        };
        if self.desc.is_none() {
            return false;
        };
        if self.origin.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.reason));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.desc));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.origin));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.reason.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in self.desc.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in self.origin.iter() {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.reason.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.desc.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.origin.as_ref() {
            try!(os.write_string(3, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Error>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
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
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "reason",
                    Error::has_reason,
                    Error::get_reason,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "desc",
                    Error::has_desc,
                    Error::get_desc,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "origin",
                    Error::has_origin,
                    Error::get_origin,
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

impl ::std::cmp::PartialEq for Error {
    fn eq(&self, other: &Error) -> bool {
        self.reason == other.reason &&
        self.desc == other.desc &&
        self.origin == other.origin &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
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
    cached_size: ::std::cell::Cell<u32>,
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
            instance.get(|| {
                CmdDesc {
                    name: ::protobuf::SingularField::none(),
                    doc: ::protobuf::SingularField::none(),
                    intype: ::std::option::Option::None,
                    outtype: ::std::option::Option::None,
                    indoc: ::protobuf::SingularField::none(),
                    outdoc: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
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
        };
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
        };
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
        };
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
        };
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
}

impl ::protobuf::Message for CmdDesc {
    fn is_initialized(&self) -> bool {
        if self.name.is_none() {
            return false;
        };
        if self.doc.is_none() {
            return false;
        };
        if self.intype.is_none() {
            return false;
        };
        if self.outtype.is_none() {
            return false;
        };
        if self.indoc.is_none() {
            return false;
        };
        if self.outdoc.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.doc));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.intype = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.outtype = ::std::option::Option::Some(tmp);
                },
                5 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.indoc));
                },
                6 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.outdoc));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.name.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in self.doc.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in self.intype.iter() {
            my_size += ::protobuf::rt::enum_size(3, *value);
        };
        for value in self.outtype.iter() {
            my_size += ::protobuf::rt::enum_size(4, *value);
        };
        for value in self.indoc.iter() {
            my_size += ::protobuf::rt::string_size(5, &value);
        };
        for value in self.outdoc.iter() {
            my_size += ::protobuf::rt::string_size(6, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.name.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.doc.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.intype {
            try!(os.write_enum(3, v.value()));
        };
        if let Some(v) = self.outtype {
            try!(os.write_enum(4, v.value()));
        };
        if let Some(v) = self.indoc.as_ref() {
            try!(os.write_string(5, &v));
        };
        if let Some(v) = self.outdoc.as_ref() {
            try!(os.write_string(6, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<CmdDesc>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
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
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "name",
                    CmdDesc::has_name,
                    CmdDesc::get_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "doc",
                    CmdDesc::has_doc,
                    CmdDesc::get_doc,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "intype",
                    CmdDesc::has_intype,
                    CmdDesc::get_intype,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "outtype",
                    CmdDesc::has_outtype,
                    CmdDesc::get_outtype,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "indoc",
                    CmdDesc::has_indoc,
                    CmdDesc::get_indoc,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "outdoc",
                    CmdDesc::has_outdoc,
                    CmdDesc::get_outdoc,
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

impl ::std::cmp::PartialEq for CmdDesc {
    fn eq(&self, other: &CmdDesc) -> bool {
        self.name == other.name &&
        self.doc == other.doc &&
        self.intype == other.intype &&
        self.outtype == other.outtype &&
        self.indoc == other.indoc &&
        self.outdoc == other.outdoc &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CmdDesc {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct AttrDesc {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    doc: ::protobuf::SingularField<::std::string::String>,
    field_type: ::std::option::Option<DataType>,
    unit: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
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
            instance.get(|| {
                AttrDesc {
                    name: ::protobuf::SingularField::none(),
                    doc: ::protobuf::SingularField::none(),
                    field_type: ::std::option::Option::None,
                    unit: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
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
        };
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
        };
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
        };
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
}

impl ::protobuf::Message for AttrDesc {
    fn is_initialized(&self) -> bool {
        if self.name.is_none() {
            return false;
        };
        if self.doc.is_none() {
            return false;
        };
        if self.field_type.is_none() {
            return false;
        };
        if self.unit.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.doc));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                4 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.unit));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.name.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in self.doc.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in self.field_type.iter() {
            my_size += ::protobuf::rt::enum_size(3, *value);
        };
        for value in self.unit.iter() {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.name.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.doc.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.field_type {
            try!(os.write_enum(3, v.value()));
        };
        if let Some(v) = self.unit.as_ref() {
            try!(os.write_string(4, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<AttrDesc>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
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
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "name",
                    AttrDesc::has_name,
                    AttrDesc::get_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "doc",
                    AttrDesc::has_doc,
                    AttrDesc::get_doc,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "type",
                    AttrDesc::has_field_type,
                    AttrDesc::get_field_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "unit",
                    AttrDesc::has_unit,
                    AttrDesc::get_unit,
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

impl ::std::cmp::PartialEq for AttrDesc {
    fn eq(&self, other: &AttrDesc) -> bool {
        self.name == other.name &&
        self.doc == other.doc &&
        self.field_type == other.field_type &&
        self.unit == other.unit &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for AttrDesc {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct PropDesc {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    doc: ::protobuf::SingularField<::std::string::String>,
    field_type: ::std::option::Option<DataType>,
    default: ::protobuf::SingularPtrField<Value>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
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
            instance.get(|| {
                PropDesc {
                    name: ::protobuf::SingularField::none(),
                    doc: ::protobuf::SingularField::none(),
                    field_type: ::std::option::Option::None,
                    default: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
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
        };
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
        };
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
        };
        self.default.as_mut().unwrap()
    }

    // Take field
    pub fn take_default(&mut self) -> Value {
        self.default.take().unwrap_or_else(|| Value::new())
    }

    pub fn get_default(&self) -> &Value {
        self.default.as_ref().unwrap_or_else(|| Value::default_instance())
    }
}

impl ::protobuf::Message for PropDesc {
    fn is_initialized(&self) -> bool {
        if self.name.is_none() {
            return false;
        };
        if self.doc.is_none() {
            return false;
        };
        if self.field_type.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.doc));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                4 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.default));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.name.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in self.doc.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in self.field_type.iter() {
            my_size += ::protobuf::rt::enum_size(3, *value);
        };
        for value in self.default.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.name.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.doc.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.field_type {
            try!(os.write_enum(3, v.value()));
        };
        if let Some(v) = self.default.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<PropDesc>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
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
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "name",
                    PropDesc::has_name,
                    PropDesc::get_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "doc",
                    PropDesc::has_doc,
                    PropDesc::get_doc,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "type",
                    PropDesc::has_field_type,
                    PropDesc::get_field_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "default",
                    PropDesc::has_default,
                    PropDesc::get_default,
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

impl ::std::cmp::PartialEq for PropDesc {
    fn eq(&self, other: &PropDesc) -> bool {
        self.name == other.name &&
        self.doc == other.doc &&
        self.field_type == other.field_type &&
        self.default == other.default &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for PropDesc {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Request {
    // message fields
    seqno: ::std::option::Option<u32>,
    rtype: ::std::option::Option<ReqType>,
    name: ::protobuf::SingularField<::std::string::String>,
    value: ::protobuf::SingularPtrField<Value>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
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
            instance.get(|| {
                Request {
                    seqno: ::std::option::Option::None,
                    rtype: ::std::option::Option::None,
                    name: ::protobuf::SingularField::none(),
                    value: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
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
        };
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
        };
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> Value {
        self.value.take().unwrap_or_else(|| Value::new())
    }

    pub fn get_value(&self) -> &Value {
        self.value.as_ref().unwrap_or_else(|| Value::default_instance())
    }
}

impl ::protobuf::Message for Request {
    fn is_initialized(&self) -> bool {
        if self.seqno.is_none() {
            return false;
        };
        if self.rtype.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.seqno = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.rtype = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name));
                },
                4 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.value));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.seqno.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.rtype.iter() {
            my_size += ::protobuf::rt::enum_size(2, *value);
        };
        for value in self.name.iter() {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        for value in self.value.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.seqno {
            try!(os.write_uint32(1, v));
        };
        if let Some(v) = self.rtype {
            try!(os.write_enum(2, v.value()));
        };
        if let Some(v) = self.name.as_ref() {
            try!(os.write_string(3, &v));
        };
        if let Some(v) = self.value.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Request>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
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
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "seqno",
                    Request::has_seqno,
                    Request::get_seqno,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "rtype",
                    Request::has_rtype,
                    Request::get_rtype,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "name",
                    Request::has_name,
                    Request::get_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "value",
                    Request::has_value,
                    Request::get_value,
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

impl ::std::cmp::PartialEq for Request {
    fn eq(&self, other: &Request) -> bool {
        self.seqno == other.seqno &&
        self.rtype == other.rtype &&
        self.name == other.name &&
        self.value == other.value &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Request {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
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
    cached_size: ::std::cell::Cell<u32>,
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
            instance.get(|| {
                Response {
                    seqno: ::std::option::Option::None,
                    rtype: ::std::option::Option::None,
                    error: ::protobuf::SingularPtrField::none(),
                    value: ::protobuf::SingularPtrField::none(),
                    cmds: ::protobuf::RepeatedField::new(),
                    attrs: ::protobuf::RepeatedField::new(),
                    props: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
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
        };
        self.error.as_mut().unwrap()
    }

    // Take field
    pub fn take_error(&mut self) -> Error {
        self.error.take().unwrap_or_else(|| Error::new())
    }

    pub fn get_error(&self) -> &Error {
        self.error.as_ref().unwrap_or_else(|| Error::default_instance())
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
        };
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> Value {
        self.value.take().unwrap_or_else(|| Value::new())
    }

    pub fn get_value(&self) -> &Value {
        self.value.as_ref().unwrap_or_else(|| Value::default_instance())
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
}

impl ::protobuf::Message for Response {
    fn is_initialized(&self) -> bool {
        if self.seqno.is_none() {
            return false;
        };
        if self.rtype.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.seqno = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.rtype = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.error));
                },
                5 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.value));
                },
                6 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.cmds));
                },
                7 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.attrs));
                },
                8 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.props));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.seqno.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.rtype.iter() {
            my_size += ::protobuf::rt::enum_size(2, *value);
        };
        for value in self.error.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.value.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.cmds.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.attrs.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.props.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.seqno {
            try!(os.write_uint32(1, v));
        };
        if let Some(v) = self.rtype {
            try!(os.write_enum(2, v.value()));
        };
        if let Some(v) = self.error.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.value.as_ref() {
            try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in self.cmds.iter() {
            try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in self.attrs.iter() {
            try!(os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in self.props.iter() {
            try!(os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Response>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
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
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "seqno",
                    Response::has_seqno,
                    Response::get_seqno,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "rtype",
                    Response::has_rtype,
                    Response::get_rtype,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "error",
                    Response::has_error,
                    Response::get_error,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "value",
                    Response::has_value,
                    Response::get_value,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "cmds",
                    Response::get_cmds,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "attrs",
                    Response::get_attrs,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "props",
                    Response::get_props,
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

impl ::std::cmp::PartialEq for Response {
    fn eq(&self, other: &Response) -> bool {
        self.seqno == other.seqno &&
        self.rtype == other.rtype &&
        self.error == other.error &&
        self.value == other.value &&
        self.cmds == other.cmds &&
        self.attrs == other.attrs &&
        self.props == other.props &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Response {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
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

    fn enum_descriptor_static(_: Option<DataType>) -> &'static ::protobuf::reflect::EnumDescriptor {
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

    fn enum_descriptor_static(_: Option<ReqType>) -> &'static ::protobuf::reflect::EnumDescriptor {
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

    fn enum_descriptor_static(_: Option<RespType>) -> &'static ::protobuf::reflect::EnumDescriptor {
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

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x09, 0x6d, 0x73, 0x67, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xc7, 0x01, 0x0a, 0x05,
    0x56, 0x61, 0x6c, 0x75, 0x65, 0x12, 0x18, 0x0a, 0x05, 0x76, 0x74, 0x79, 0x70, 0x65, 0x18, 0x0a,
    0x20, 0x02, 0x28, 0x0e, 0x32, 0x09, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x54, 0x79, 0x70, 0x65, 0x12,
    0x10, 0x0a, 0x04, 0x62, 0x6f, 0x6f, 0x6c, 0x18, 0x01, 0x20, 0x03, 0x28, 0x08, 0x42, 0x02, 0x10,
    0x01, 0x12, 0x12, 0x0a, 0x06, 0x64, 0x6f, 0x75, 0x62, 0x6c, 0x65, 0x18, 0x02, 0x20, 0x03, 0x28,
    0x01, 0x42, 0x02, 0x10, 0x01, 0x12, 0x11, 0x0a, 0x05, 0x66, 0x6c, 0x6f, 0x61, 0x74, 0x18, 0x03,
    0x20, 0x03, 0x28, 0x02, 0x42, 0x02, 0x10, 0x01, 0x12, 0x11, 0x0a, 0x05, 0x69, 0x6e, 0x74, 0x33,
    0x32, 0x18, 0x04, 0x20, 0x03, 0x28, 0x11, 0x42, 0x02, 0x10, 0x01, 0x12, 0x11, 0x0a, 0x05, 0x69,
    0x6e, 0x74, 0x36, 0x34, 0x18, 0x05, 0x20, 0x03, 0x28, 0x12, 0x42, 0x02, 0x10, 0x01, 0x12, 0x12,
    0x0a, 0x06, 0x75, 0x69, 0x6e, 0x74, 0x33, 0x32, 0x18, 0x06, 0x20, 0x03, 0x28, 0x0d, 0x42, 0x02,
    0x10, 0x01, 0x12, 0x12, 0x0a, 0x06, 0x75, 0x69, 0x6e, 0x74, 0x36, 0x34, 0x18, 0x07, 0x20, 0x03,
    0x28, 0x04, 0x42, 0x02, 0x10, 0x01, 0x12, 0x0e, 0x0a, 0x06, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67,
    0x18, 0x08, 0x20, 0x03, 0x28, 0x09, 0x12, 0x0d, 0x0a, 0x05, 0x62, 0x79, 0x74, 0x65, 0x73, 0x18,
    0x09, 0x20, 0x01, 0x28, 0x0c, 0x22, 0x35, 0x0a, 0x05, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x12, 0x0e,
    0x0a, 0x06, 0x72, 0x65, 0x61, 0x73, 0x6f, 0x6e, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x12, 0x0c,
    0x0a, 0x04, 0x64, 0x65, 0x73, 0x63, 0x18, 0x02, 0x20, 0x02, 0x28, 0x09, 0x12, 0x0e, 0x0a, 0x06,
    0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x18, 0x03, 0x20, 0x02, 0x28, 0x09, 0x22, 0x7a, 0x0a, 0x07,
    0x43, 0x6d, 0x64, 0x44, 0x65, 0x73, 0x63, 0x12, 0x0c, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18,
    0x01, 0x20, 0x02, 0x28, 0x09, 0x12, 0x0b, 0x0a, 0x03, 0x64, 0x6f, 0x63, 0x18, 0x02, 0x20, 0x02,
    0x28, 0x09, 0x12, 0x19, 0x0a, 0x06, 0x69, 0x6e, 0x74, 0x79, 0x70, 0x65, 0x18, 0x03, 0x20, 0x02,
    0x28, 0x0e, 0x32, 0x09, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x54, 0x79, 0x70, 0x65, 0x12, 0x1a, 0x0a,
    0x07, 0x6f, 0x75, 0x74, 0x74, 0x79, 0x70, 0x65, 0x18, 0x04, 0x20, 0x02, 0x28, 0x0e, 0x32, 0x09,
    0x2e, 0x44, 0x61, 0x74, 0x61, 0x54, 0x79, 0x70, 0x65, 0x12, 0x0d, 0x0a, 0x05, 0x69, 0x6e, 0x64,
    0x6f, 0x63, 0x18, 0x05, 0x20, 0x02, 0x28, 0x09, 0x12, 0x0e, 0x0a, 0x06, 0x6f, 0x75, 0x74, 0x64,
    0x6f, 0x63, 0x18, 0x06, 0x20, 0x02, 0x28, 0x09, 0x22, 0x4c, 0x0a, 0x08, 0x41, 0x74, 0x74, 0x72,
    0x44, 0x65, 0x73, 0x63, 0x12, 0x0c, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x02,
    0x28, 0x09, 0x12, 0x0b, 0x0a, 0x03, 0x64, 0x6f, 0x63, 0x18, 0x02, 0x20, 0x02, 0x28, 0x09, 0x12,
    0x17, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x03, 0x20, 0x02, 0x28, 0x0e, 0x32, 0x09, 0x2e,
    0x44, 0x61, 0x74, 0x61, 0x54, 0x79, 0x70, 0x65, 0x12, 0x0c, 0x0a, 0x04, 0x75, 0x6e, 0x69, 0x74,
    0x18, 0x04, 0x20, 0x02, 0x28, 0x09, 0x22, 0x57, 0x0a, 0x08, 0x50, 0x72, 0x6f, 0x70, 0x44, 0x65,
    0x73, 0x63, 0x12, 0x0c, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09,
    0x12, 0x0b, 0x0a, 0x03, 0x64, 0x6f, 0x63, 0x18, 0x02, 0x20, 0x02, 0x28, 0x09, 0x12, 0x17, 0x0a,
    0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x03, 0x20, 0x02, 0x28, 0x0e, 0x32, 0x09, 0x2e, 0x44, 0x61,
    0x74, 0x61, 0x54, 0x79, 0x70, 0x65, 0x12, 0x17, 0x0a, 0x07, 0x64, 0x65, 0x66, 0x61, 0x75, 0x6c,
    0x74, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x06, 0x2e, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x22,
    0x56, 0x0a, 0x07, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x0d, 0x0a, 0x05, 0x73, 0x65,
    0x71, 0x6e, 0x6f, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0d, 0x12, 0x17, 0x0a, 0x05, 0x72, 0x74, 0x79,
    0x70, 0x65, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0e, 0x32, 0x08, 0x2e, 0x52, 0x65, 0x71, 0x54, 0x79,
    0x70, 0x65, 0x12, 0x0c, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09,
    0x12, 0x15, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x06, 0x2e, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x22, 0xad, 0x01, 0x0a, 0x08, 0x52, 0x65, 0x73, 0x70,
    0x6f, 0x6e, 0x73, 0x65, 0x12, 0x0d, 0x0a, 0x05, 0x73, 0x65, 0x71, 0x6e, 0x6f, 0x18, 0x01, 0x20,
    0x02, 0x28, 0x0d, 0x12, 0x18, 0x0a, 0x05, 0x72, 0x74, 0x79, 0x70, 0x65, 0x18, 0x02, 0x20, 0x02,
    0x28, 0x0e, 0x32, 0x09, 0x2e, 0x52, 0x65, 0x73, 0x70, 0x54, 0x79, 0x70, 0x65, 0x12, 0x15, 0x0a,
    0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x06, 0x2e, 0x45,
    0x72, 0x72, 0x6f, 0x72, 0x12, 0x15, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x05, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x06, 0x2e, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x12, 0x16, 0x0a, 0x04, 0x63,
    0x6d, 0x64, 0x73, 0x18, 0x06, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x08, 0x2e, 0x43, 0x6d, 0x64, 0x44,
    0x65, 0x73, 0x63, 0x12, 0x18, 0x0a, 0x05, 0x61, 0x74, 0x74, 0x72, 0x73, 0x18, 0x07, 0x20, 0x03,
    0x28, 0x0b, 0x32, 0x09, 0x2e, 0x41, 0x74, 0x74, 0x72, 0x44, 0x65, 0x73, 0x63, 0x12, 0x18, 0x0a,
    0x05, 0x70, 0x72, 0x6f, 0x70, 0x73, 0x18, 0x08, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x09, 0x2e, 0x50,
    0x72, 0x6f, 0x70, 0x44, 0x65, 0x73, 0x63, 0x2a, 0xae, 0x02, 0x0a, 0x08, 0x44, 0x61, 0x74, 0x61,
    0x54, 0x79, 0x70, 0x65, 0x12, 0x08, 0x0a, 0x04, 0x56, 0x6f, 0x69, 0x64, 0x10, 0x00, 0x12, 0x08,
    0x0a, 0x04, 0x42, 0x6f, 0x6f, 0x6c, 0x10, 0x01, 0x12, 0x0a, 0x0a, 0x06, 0x44, 0x6f, 0x75, 0x62,
    0x6c, 0x65, 0x10, 0x02, 0x12, 0x09, 0x0a, 0x05, 0x46, 0x6c, 0x6f, 0x61, 0x74, 0x10, 0x03, 0x12,
    0x09, 0x0a, 0x05, 0x49, 0x6e, 0x74, 0x33, 0x32, 0x10, 0x04, 0x12, 0x09, 0x0a, 0x05, 0x49, 0x6e,
    0x74, 0x36, 0x34, 0x10, 0x05, 0x12, 0x0a, 0x0a, 0x06, 0x55, 0x49, 0x6e, 0x74, 0x33, 0x32, 0x10,
    0x06, 0x12, 0x0a, 0x0a, 0x06, 0x55, 0x49, 0x6e, 0x74, 0x36, 0x34, 0x10, 0x07, 0x12, 0x0a, 0x0a,
    0x06, 0x53, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x10, 0x08, 0x12, 0x0d, 0x0a, 0x09, 0x42, 0x79, 0x74,
    0x65, 0x41, 0x72, 0x72, 0x61, 0x79, 0x10, 0x0a, 0x12, 0x0d, 0x0a, 0x09, 0x42, 0x6f, 0x6f, 0x6c,
    0x41, 0x72, 0x72, 0x61, 0x79, 0x10, 0x0b, 0x12, 0x0f, 0x0a, 0x0b, 0x44, 0x6f, 0x75, 0x62, 0x6c,
    0x65, 0x41, 0x72, 0x72, 0x61, 0x79, 0x10, 0x0c, 0x12, 0x0e, 0x0a, 0x0a, 0x46, 0x6c, 0x6f, 0x61,
    0x74, 0x41, 0x72, 0x72, 0x61, 0x79, 0x10, 0x0d, 0x12, 0x0e, 0x0a, 0x0a, 0x49, 0x6e, 0x74, 0x33,
    0x32, 0x41, 0x72, 0x72, 0x61, 0x79, 0x10, 0x0e, 0x12, 0x0e, 0x0a, 0x0a, 0x49, 0x6e, 0x74, 0x36,
    0x34, 0x41, 0x72, 0x72, 0x61, 0x79, 0x10, 0x0f, 0x12, 0x0f, 0x0a, 0x0b, 0x55, 0x49, 0x6e, 0x74,
    0x33, 0x32, 0x41, 0x72, 0x72, 0x61, 0x79, 0x10, 0x10, 0x12, 0x0f, 0x0a, 0x0b, 0x55, 0x49, 0x6e,
    0x74, 0x36, 0x34, 0x41, 0x72, 0x72, 0x61, 0x79, 0x10, 0x11, 0x12, 0x0f, 0x0a, 0x0b, 0x53, 0x74,
    0x72, 0x69, 0x6e, 0x67, 0x41, 0x72, 0x72, 0x61, 0x79, 0x10, 0x12, 0x12, 0x14, 0x0a, 0x10, 0x49,
    0x6e, 0x74, 0x36, 0x34, 0x53, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x41, 0x72, 0x72, 0x61, 0x79, 0x10,
    0x15, 0x12, 0x15, 0x0a, 0x11, 0x44, 0x6f, 0x75, 0x62, 0x6c, 0x65, 0x53, 0x74, 0x72, 0x69, 0x6e,
    0x67, 0x41, 0x72, 0x72, 0x61, 0x79, 0x10, 0x16, 0x2a, 0x7a, 0x0a, 0x07, 0x52, 0x65, 0x71, 0x54,
    0x79, 0x70, 0x65, 0x12, 0x0b, 0x0a, 0x07, 0x52, 0x65, 0x71, 0x50, 0x69, 0x6e, 0x67, 0x10, 0x00,
    0x12, 0x0e, 0x0a, 0x0a, 0x52, 0x65, 0x71, 0x45, 0x78, 0x65, 0x63, 0x43, 0x6d, 0x64, 0x10, 0x01,
    0x12, 0x0f, 0x0a, 0x0b, 0x52, 0x65, 0x71, 0x52, 0x65, 0x61, 0x64, 0x41, 0x74, 0x74, 0x72, 0x10,
    0x02, 0x12, 0x10, 0x0a, 0x0c, 0x52, 0x65, 0x71, 0x57, 0x72, 0x69, 0x74, 0x65, 0x41, 0x74, 0x74,
    0x72, 0x10, 0x03, 0x12, 0x0e, 0x0a, 0x0a, 0x52, 0x65, 0x71, 0x47, 0x65, 0x74, 0x50, 0x72, 0x6f,
    0x70, 0x10, 0x04, 0x12, 0x0e, 0x0a, 0x0a, 0x52, 0x65, 0x71, 0x53, 0x65, 0x74, 0x50, 0x72, 0x6f,
    0x70, 0x10, 0x05, 0x12, 0x0f, 0x0a, 0x0b, 0x52, 0x65, 0x71, 0x51, 0x75, 0x65, 0x72, 0x79, 0x41,
    0x50, 0x49, 0x10, 0x0a, 0x2a, 0x43, 0x0a, 0x08, 0x52, 0x65, 0x73, 0x70, 0x54, 0x79, 0x70, 0x65,
    0x12, 0x0c, 0x0a, 0x08, 0x52, 0x65, 0x73, 0x70, 0x56, 0x6f, 0x69, 0x64, 0x10, 0x00, 0x12, 0x0d,
    0x0a, 0x09, 0x52, 0x65, 0x73, 0x70, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x10, 0x01, 0x12, 0x0d, 0x0a,
    0x09, 0x52, 0x65, 0x73, 0x70, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x10, 0x02, 0x12, 0x0b, 0x0a, 0x07,
    0x52, 0x65, 0x73, 0x70, 0x41, 0x50, 0x49, 0x10, 0x03, 0x4a, 0x93, 0x26, 0x0a, 0x06, 0x12, 0x04,
    0x04, 0x00, 0x69, 0x01, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x00, 0x12, 0x04, 0x04, 0x00, 0x19, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x00, 0x01, 0x12, 0x03, 0x04, 0x05, 0x0d, 0x0a, 0x0b, 0x0a, 0x04,
    0x05, 0x00, 0x02, 0x00, 0x12, 0x03, 0x05, 0x04, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x05, 0x04, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x02,
    0x12, 0x03, 0x05, 0x0d, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x01, 0x12, 0x03, 0x06,
    0x04, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x06, 0x04, 0x08,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x06, 0x0d, 0x0e, 0x0a, 0x0b,
    0x0a, 0x04, 0x05, 0x00, 0x02, 0x02, 0x12, 0x03, 0x07, 0x04, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x07, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x02, 0x02, 0x12, 0x03, 0x07, 0x0d, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x03, 0x12,
    0x03, 0x08, 0x04, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x08,
    0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03, 0x02, 0x12, 0x03, 0x08, 0x0d, 0x0e,
    0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x04, 0x12, 0x03, 0x09, 0x04, 0x0f, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x09, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x04, 0x02, 0x12, 0x03, 0x09, 0x0d, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02,
    0x05, 0x12, 0x03, 0x0a, 0x04, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x05, 0x01, 0x12,
    0x03, 0x0a, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x05, 0x02, 0x12, 0x03, 0x0a,
    0x0d, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x06, 0x12, 0x03, 0x0b, 0x04, 0x0f, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x0b, 0x04, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x06, 0x02, 0x12, 0x03, 0x0b, 0x0d, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x05,
    0x00, 0x02, 0x07, 0x12, 0x03, 0x0c, 0x04, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x07,
    0x01, 0x12, 0x03, 0x0c, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x07, 0x02, 0x12,
    0x03, 0x0c, 0x0d, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x08, 0x12, 0x03, 0x0d, 0x04,
    0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x08, 0x01, 0x12, 0x03, 0x0d, 0x04, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x08, 0x02, 0x12, 0x03, 0x0d, 0x0d, 0x0e, 0x0a, 0x0b, 0x0a,
    0x04, 0x05, 0x00, 0x02, 0x09, 0x12, 0x03, 0x0e, 0x04, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x09, 0x01, 0x12, 0x03, 0x0e, 0x04, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x09,
    0x02, 0x12, 0x03, 0x0e, 0x12, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x0a, 0x12, 0x03,
    0x0f, 0x04, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x0f, 0x04,
    0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0a, 0x02, 0x12, 0x03, 0x0f, 0x12, 0x14, 0x0a,
    0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x0b, 0x12, 0x03, 0x10, 0x04, 0x15, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x0b, 0x01, 0x12, 0x03, 0x10, 0x04, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x0b, 0x02, 0x12, 0x03, 0x10, 0x12, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x0c,
    0x12, 0x03, 0x11, 0x04, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0c, 0x01, 0x12, 0x03,
    0x11, 0x04, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0c, 0x02, 0x12, 0x03, 0x11, 0x12,
    0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x0d, 0x12, 0x03, 0x12, 0x04, 0x15, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x0d, 0x01, 0x12, 0x03, 0x12, 0x04, 0x0e, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x0d, 0x02, 0x12, 0x03, 0x12, 0x12, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00,
    0x02, 0x0e, 0x12, 0x03, 0x13, 0x04, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0e, 0x01,
    0x12, 0x03, 0x13, 0x04, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0e, 0x02, 0x12, 0x03,
    0x13, 0x12, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x0f, 0x12, 0x03, 0x14, 0x04, 0x15,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0f, 0x01, 0x12, 0x03, 0x14, 0x04, 0x0f, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x0f, 0x02, 0x12, 0x03, 0x14, 0x12, 0x14, 0x0a, 0x0b, 0x0a, 0x04,
    0x05, 0x00, 0x02, 0x10, 0x12, 0x03, 0x15, 0x04, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x10, 0x01, 0x12, 0x03, 0x15, 0x04, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x10, 0x02,
    0x12, 0x03, 0x15, 0x12, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x11, 0x12, 0x03, 0x16,
    0x04, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x11, 0x01, 0x12, 0x03, 0x16, 0x04, 0x0f,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x11, 0x02, 0x12, 0x03, 0x16, 0x12, 0x14, 0x0a, 0x0b,
    0x0a, 0x04, 0x05, 0x00, 0x02, 0x12, 0x12, 0x03, 0x17, 0x04, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x12, 0x01, 0x12, 0x03, 0x17, 0x04, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x12, 0x02, 0x12, 0x03, 0x17, 0x18, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x13, 0x12,
    0x03, 0x18, 0x04, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x13, 0x01, 0x12, 0x03, 0x18,
    0x04, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x13, 0x02, 0x12, 0x03, 0x18, 0x18, 0x1a,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x1b, 0x00, 0x26, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x00, 0x01, 0x12, 0x03, 0x1b, 0x08, 0x0d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00,
    0x12, 0x03, 0x1c, 0x04, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x1c, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x1c, 0x0d,
    0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1c, 0x16, 0x1b, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x1c, 0x20, 0x22, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x1d, 0x04, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x01, 0x04, 0x12, 0x03, 0x1d, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x05, 0x12, 0x03, 0x1d, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x1d, 0x16, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x1d,
    0x20, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x08, 0x12, 0x03, 0x1d, 0x22, 0x2f,
    0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x00, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x1d, 0x23,
    0x2e, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x00, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03,
    0x1d, 0x23, 0x29, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x00, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02,
    0x00, 0x12, 0x03, 0x1d, 0x23, 0x29, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x00, 0x02, 0x01, 0x08, 0xe7,
    0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1d, 0x23, 0x29, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x00,
    0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x1d, 0x2a, 0x2e, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x1e, 0x04, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x02, 0x04, 0x12, 0x03, 0x1e, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05,
    0x12, 0x03, 0x1e, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03,
    0x1e, 0x16, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x1e, 0x20,
    0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x08, 0x12, 0x03, 0x1e, 0x22, 0x2f, 0x0a,
    0x0f, 0x0a, 0x08, 0x04, 0x00, 0x02, 0x02, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x1e, 0x23, 0x2e,
    0x0a, 0x10, 0x0a, 0x09, 0x04, 0x00, 0x02, 0x02, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x1e,
    0x23, 0x29, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x00, 0x02, 0x02, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00,
    0x12, 0x03, 0x1e, 0x23, 0x29, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x00, 0x02, 0x02, 0x08, 0xe7, 0x07,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1e, 0x23, 0x29, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x00, 0x02,
    0x02, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x1e, 0x2a, 0x2e, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x03, 0x12, 0x03, 0x1f, 0x04, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03,
    0x04, 0x12, 0x03, 0x1f, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x05, 0x12,
    0x03, 0x1f, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x1f,
    0x16, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x1f, 0x20, 0x21,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x08, 0x12, 0x03, 0x1f, 0x22, 0x2f, 0x0a, 0x0f,
    0x0a, 0x08, 0x04, 0x00, 0x02, 0x03, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x1f, 0x23, 0x2e, 0x0a,
    0x10, 0x0a, 0x09, 0x04, 0x00, 0x02, 0x03, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x1f, 0x23,
    0x29, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x00, 0x02, 0x03, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x1f, 0x23, 0x29, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x00, 0x02, 0x03, 0x08, 0xe7, 0x07, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x1f, 0x23, 0x29, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x00, 0x02, 0x03,
    0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x1f, 0x2a, 0x2e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x04, 0x12, 0x03, 0x20, 0x04, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x04,
    0x12, 0x03, 0x20, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x05, 0x12, 0x03,
    0x20, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x20, 0x16,
    0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x20, 0x20, 0x21, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x08, 0x12, 0x03, 0x20, 0x22, 0x2f, 0x0a, 0x0f, 0x0a,
    0x08, 0x04, 0x00, 0x02, 0x04, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x20, 0x23, 0x2e, 0x0a, 0x10,
    0x0a, 0x09, 0x04, 0x00, 0x02, 0x04, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x20, 0x23, 0x29,
    0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x00, 0x02, 0x04, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03,
    0x20, 0x23, 0x29, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x00, 0x02, 0x04, 0x08, 0xe7, 0x07, 0x00, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x20, 0x23, 0x29, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x00, 0x02, 0x04, 0x08,
    0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x20, 0x2a, 0x2e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x05, 0x12, 0x03, 0x21, 0x04, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x04, 0x12,
    0x03, 0x21, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x05, 0x12, 0x03, 0x21,
    0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x21, 0x16, 0x1b,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x03, 0x12, 0x03, 0x21, 0x20, 0x21, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x08, 0x12, 0x03, 0x21, 0x22, 0x2f, 0x0a, 0x0f, 0x0a, 0x08,
    0x04, 0x00, 0x02, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x21, 0x23, 0x2e, 0x0a, 0x10, 0x0a,
    0x09, 0x04, 0x00, 0x02, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x21, 0x23, 0x29, 0x0a,
    0x11, 0x0a, 0x0a, 0x04, 0x00, 0x02, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x21,
    0x23, 0x29, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x00, 0x02, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x21, 0x23, 0x29, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x00, 0x02, 0x05, 0x08, 0xe7,
    0x07, 0x00, 0x03, 0x12, 0x03, 0x21, 0x2a, 0x2e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x06,
    0x12, 0x03, 0x22, 0x04, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x04, 0x12, 0x03,
    0x22, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x05, 0x12, 0x03, 0x22, 0x0d,
    0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x22, 0x16, 0x1c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x03, 0x12, 0x03, 0x22, 0x20, 0x21, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x06, 0x08, 0x12, 0x03, 0x22, 0x22, 0x2f, 0x0a, 0x0f, 0x0a, 0x08, 0x04,
    0x00, 0x02, 0x06, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x22, 0x23, 0x2e, 0x0a, 0x10, 0x0a, 0x09,
    0x04, 0x00, 0x02, 0x06, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x22, 0x23, 0x29, 0x0a, 0x11,
    0x0a, 0x0a, 0x04, 0x00, 0x02, 0x06, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x22, 0x23,
    0x29, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x00, 0x02, 0x06, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x22, 0x23, 0x29, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x00, 0x02, 0x06, 0x08, 0xe7, 0x07,
    0x00, 0x03, 0x12, 0x03, 0x22, 0x2a, 0x2e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x07, 0x12,
    0x03, 0x23, 0x04, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x04, 0x12, 0x03, 0x23,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x05, 0x12, 0x03, 0x23, 0x0d, 0x13,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x01, 0x12, 0x03, 0x23, 0x16, 0x1c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x03, 0x12, 0x03, 0x23, 0x20, 0x21, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x07, 0x08, 0x12, 0x03, 0x23, 0x22, 0x2f, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x00,
    0x02, 0x07, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x23, 0x23, 0x2e, 0x0a, 0x10, 0x0a, 0x09, 0x04,
    0x00, 0x02, 0x07, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x23, 0x23, 0x29, 0x0a, 0x11, 0x0a,
    0x0a, 0x04, 0x00, 0x02, 0x07, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x23, 0x23, 0x29,
    0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x00, 0x02, 0x07, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x23, 0x23, 0x29, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x00, 0x02, 0x07, 0x08, 0xe7, 0x07, 0x00,
    0x03, 0x12, 0x03, 0x23, 0x2a, 0x2e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x08, 0x12, 0x03,
    0x24, 0x04, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x04, 0x12, 0x03, 0x24, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x05, 0x12, 0x03, 0x24, 0x0d, 0x13, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x01, 0x12, 0x03, 0x24, 0x16, 0x1c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x08, 0x03, 0x12, 0x03, 0x24, 0x20, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x09, 0x12, 0x03, 0x25, 0x04, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09,
    0x04, 0x12, 0x03, 0x25, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x05, 0x12,
    0x03, 0x25, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x01, 0x12, 0x03, 0x25,
    0x16, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x03, 0x12, 0x03, 0x25, 0x20, 0x21,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x28, 0x00, 0x2c, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x01, 0x01, 0x12, 0x03, 0x28, 0x08, 0x0d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00,
    0x12, 0x03, 0x29, 0x04, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x29, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x29, 0x0d,
    0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x29, 0x14, 0x1a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x29, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x2a, 0x04, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x01, 0x04, 0x12, 0x03, 0x2a, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01,
    0x05, 0x12, 0x03, 0x2a, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x2a, 0x14, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x2a,
    0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x02, 0x12, 0x03, 0x2b, 0x04, 0x1f, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x04, 0x12, 0x03, 0x2b, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x02, 0x05, 0x12, 0x03, 0x2b, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x2b, 0x14, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x02, 0x03, 0x12, 0x03, 0x2b, 0x1d, 0x1e, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x30,
    0x00, 0x37, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x30, 0x08, 0x0f, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x31, 0x04, 0x1f, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x31, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x31, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x31, 0x16, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x31, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x32, 0x04,
    0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x04, 0x12, 0x03, 0x32, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x05, 0x12, 0x03, 0x32, 0x0d, 0x13, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x32, 0x16, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x32, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02,
    0x02, 0x12, 0x03, 0x33, 0x04, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x04, 0x12,
    0x03, 0x33, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x06, 0x12, 0x03, 0x33,
    0x0d, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x01, 0x12, 0x03, 0x33, 0x16, 0x1c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x03, 0x12, 0x03, 0x33, 0x1f, 0x20, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x34, 0x04, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x03, 0x04, 0x12, 0x03, 0x34, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x03, 0x06, 0x12, 0x03, 0x34, 0x0d, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x01,
    0x12, 0x03, 0x34, 0x16, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x03, 0x12, 0x03,
    0x34, 0x20, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x04, 0x12, 0x03, 0x35, 0x04, 0x20,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x04, 0x12, 0x03, 0x35, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x05, 0x12, 0x03, 0x35, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x04, 0x01, 0x12, 0x03, 0x35, 0x16, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x04, 0x03, 0x12, 0x03, 0x35, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x05,
    0x12, 0x03, 0x36, 0x04, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x04, 0x12, 0x03,
    0x36, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x05, 0x12, 0x03, 0x36, 0x0d,
    0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x01, 0x12, 0x03, 0x36, 0x16, 0x1c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x03, 0x12, 0x03, 0x36, 0x1f, 0x20, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x03, 0x12, 0x04, 0x39, 0x00, 0x3e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01,
    0x12, 0x03, 0x39, 0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x3a,
    0x04, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x03, 0x3a, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x05, 0x12, 0x03, 0x3a, 0x0d, 0x13, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x3a, 0x16, 0x1a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x3a, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03,
    0x02, 0x01, 0x12, 0x03, 0x3b, 0x04, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x04,
    0x12, 0x03, 0x3b, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x05, 0x12, 0x03,
    0x3b, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x3b, 0x16,
    0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x3b, 0x1c, 0x1d, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x02, 0x12, 0x03, 0x3c, 0x04, 0x1f, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x02, 0x04, 0x12, 0x03, 0x3c, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x02, 0x06, 0x12, 0x03, 0x3c, 0x0d, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02,
    0x01, 0x12, 0x03, 0x3c, 0x16, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x03, 0x12,
    0x03, 0x3c, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x03, 0x12, 0x03, 0x3d, 0x04,
    0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x04, 0x12, 0x03, 0x3d, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x05, 0x12, 0x03, 0x3d, 0x0d, 0x13, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x03, 0x01, 0x12, 0x03, 0x3d, 0x16, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x03, 0x03, 0x12, 0x03, 0x3d, 0x1d, 0x1e, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12,
    0x04, 0x40, 0x00, 0x45, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x40, 0x08,
    0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x41, 0x04, 0x1f, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x03, 0x41, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x00, 0x05, 0x12, 0x03, 0x41, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x41, 0x16, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x41, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x01, 0x12, 0x03,
    0x42, 0x04, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x04, 0x12, 0x03, 0x42, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x05, 0x12, 0x03, 0x42, 0x0d, 0x13, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x01, 0x12, 0x03, 0x42, 0x16, 0x19, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x01, 0x03, 0x12, 0x03, 0x42, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x04, 0x02, 0x02, 0x12, 0x03, 0x43, 0x04, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02,
    0x04, 0x12, 0x03, 0x43, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x06, 0x12,
    0x03, 0x43, 0x0d, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x43,
    0x16, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x43, 0x1d, 0x1e,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x03, 0x12, 0x03, 0x44, 0x04, 0x22, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x03, 0x04, 0x12, 0x03, 0x44, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x03, 0x06, 0x12, 0x03, 0x44, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x03, 0x01, 0x12, 0x03, 0x44, 0x16, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x03,
    0x12, 0x03, 0x44, 0x20, 0x21, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x01, 0x12, 0x04, 0x49, 0x00, 0x51,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x01, 0x01, 0x12, 0x03, 0x49, 0x05, 0x0c, 0x0a, 0x0b, 0x0a,
    0x04, 0x05, 0x01, 0x02, 0x00, 0x12, 0x03, 0x4a, 0x04, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x4a, 0x04, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x00,
    0x02, 0x12, 0x03, 0x4a, 0x0e, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x01, 0x12, 0x03,
    0x4b, 0x04, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x4b, 0x04,
    0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x01, 0x02, 0x12, 0x03, 0x4b, 0x11, 0x12, 0x0a,
    0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x02, 0x12, 0x03, 0x4c, 0x04, 0x14, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x4c, 0x04, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01,
    0x02, 0x02, 0x02, 0x12, 0x03, 0x4c, 0x12, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x03,
    0x12, 0x03, 0x4d, 0x04, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x03, 0x01, 0x12, 0x03,
    0x4d, 0x04, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x03, 0x02, 0x12, 0x03, 0x4d, 0x13,
    0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x04, 0x12, 0x03, 0x4e, 0x04, 0x13, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x01, 0x02, 0x04, 0x01, 0x12, 0x03, 0x4e, 0x04, 0x0e, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x01, 0x02, 0x04, 0x02, 0x12, 0x03, 0x4e, 0x11, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01,
    0x02, 0x05, 0x12, 0x03, 0x4f, 0x04, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x05, 0x01,
    0x12, 0x03, 0x4f, 0x04, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x05, 0x02, 0x12, 0x03,
    0x4f, 0x11, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x06, 0x12, 0x03, 0x50, 0x04, 0x15,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x06, 0x01, 0x12, 0x03, 0x50, 0x04, 0x0f, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x01, 0x02, 0x06, 0x02, 0x12, 0x03, 0x50, 0x12, 0x14, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x05, 0x12, 0x04, 0x53, 0x00, 0x58, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12,
    0x03, 0x53, 0x08, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00, 0x12, 0x03, 0x54, 0x04,
    0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04, 0x12, 0x03, 0x54, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x05, 0x12, 0x03, 0x54, 0x0d, 0x13, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x54, 0x15, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x54, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02,
    0x01, 0x12, 0x03, 0x55, 0x04, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x04, 0x12,
    0x03, 0x55, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x06, 0x12, 0x03, 0x55,
    0x0d, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x01, 0x12, 0x03, 0x55, 0x15, 0x1a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x03, 0x12, 0x03, 0x55, 0x1d, 0x1e, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x05, 0x02, 0x02, 0x12, 0x03, 0x56, 0x04, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x02, 0x04, 0x12, 0x03, 0x56, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x02, 0x05, 0x12, 0x03, 0x56, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x01,
    0x12, 0x03, 0x56, 0x15, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x03, 0x12, 0x03,
    0x56, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x03, 0x12, 0x03, 0x57, 0x04, 0x1f,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x04, 0x12, 0x03, 0x57, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x06, 0x12, 0x03, 0x57, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x03, 0x01, 0x12, 0x03, 0x57, 0x15, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x03, 0x03, 0x12, 0x03, 0x57, 0x1d, 0x1e, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x02, 0x12, 0x04,
    0x5a, 0x00, 0x5f, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x02, 0x01, 0x12, 0x03, 0x5a, 0x05, 0x0d,
    0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x00, 0x12, 0x03, 0x5b, 0x04, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x5b, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x02, 0x02, 0x00, 0x02, 0x12, 0x03, 0x5b, 0x0f, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02,
    0x01, 0x12, 0x03, 0x5c, 0x04, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x5c, 0x04, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x01, 0x02, 0x12, 0x03, 0x5c,
    0x10, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x02, 0x12, 0x03, 0x5d, 0x04, 0x12, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x02, 0x01, 0x12, 0x03, 0x5d, 0x04, 0x0d, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x02, 0x02, 0x02, 0x02, 0x12, 0x03, 0x5d, 0x10, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x05,
    0x02, 0x02, 0x03, 0x12, 0x03, 0x5e, 0x04, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x03,
    0x01, 0x12, 0x03, 0x5e, 0x04, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x03, 0x02, 0x12,
    0x03, 0x5e, 0x0e, 0x0f, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x06, 0x12, 0x04, 0x61, 0x00, 0x69, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x03, 0x61, 0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x06, 0x02, 0x00, 0x12, 0x03, 0x62, 0x04, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x00, 0x04, 0x12, 0x03, 0x62, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x05,
    0x12, 0x03, 0x62, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x62, 0x16, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x03, 0x12, 0x03, 0x62, 0x1e,
    0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x01, 0x12, 0x03, 0x63, 0x04, 0x20, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x04, 0x12, 0x03, 0x63, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x01, 0x06, 0x12, 0x03, 0x63, 0x0d, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x63, 0x16, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x63, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x02, 0x12, 0x03,
    0x64, 0x04, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x04, 0x12, 0x03, 0x64, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x06, 0x12, 0x03, 0x64, 0x0d, 0x12, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x01, 0x12, 0x03, 0x64, 0x16, 0x1b, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x02, 0x03, 0x12, 0x03, 0x64, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x06, 0x02, 0x03, 0x12, 0x03, 0x65, 0x04, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03,
    0x04, 0x12, 0x03, 0x65, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x06, 0x12,
    0x03, 0x65, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x01, 0x12, 0x03, 0x65,
    0x16, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x03, 0x12, 0x03, 0x65, 0x1e, 0x1f,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x04, 0x12, 0x03, 0x66, 0x04, 0x1f, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x04, 0x04, 0x12, 0x03, 0x66, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x04, 0x06, 0x12, 0x03, 0x66, 0x0d, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x04, 0x01, 0x12, 0x03, 0x66, 0x16, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x04, 0x03,
    0x12, 0x03, 0x66, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x05, 0x12, 0x03, 0x67,
    0x04, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x05, 0x04, 0x12, 0x03, 0x67, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x05, 0x06, 0x12, 0x03, 0x67, 0x0d, 0x15, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x05, 0x01, 0x12, 0x03, 0x67, 0x16, 0x1b, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x05, 0x03, 0x12, 0x03, 0x67, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06,
    0x02, 0x06, 0x12, 0x03, 0x68, 0x04, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x06, 0x04,
    0x12, 0x03, 0x68, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x06, 0x06, 0x12, 0x03,
    0x68, 0x0d, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x06, 0x01, 0x12, 0x03, 0x68, 0x16,
    0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x06, 0x03, 0x12, 0x03, 0x68, 0x1e, 0x1f,
];

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
