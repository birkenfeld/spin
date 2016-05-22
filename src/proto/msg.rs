// This file is generated. Do not edit

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default)]
pub struct BoolArr {
    // message fields
    bool: ::std::vec::Vec<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl BoolArr {
    pub fn new() -> BoolArr {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BoolArr {
        static mut instance: ::protobuf::lazy::Lazy<BoolArr> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BoolArr,
        };
        unsafe {
            instance.get(|| {
                BoolArr {
                    bool: ::std::vec::Vec::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
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
    pub fn mut_bool<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<bool> {
        &mut self.bool
    }

    // Take field
    pub fn take_bool(&mut self) -> ::std::vec::Vec<bool> {
        ::std::mem::replace(&mut self.bool, ::std::vec::Vec::new())
    }

    pub fn get_bool<'a>(&'a self) -> &'a [bool] {
        &self.bool
    }
}

impl ::protobuf::Message for BoolArr {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_bool_into(wire_type, is, &mut self.bool));
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.bool.is_empty() {
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(self.bool.len() as u32) + (self.bool.len() * 1) as u32;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.bool.is_empty() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            // TODO: Data size is computed again, it should be cached
            try!(os.write_raw_varint32((self.bool.len() * 1) as u32));
            for v in self.bool.iter() {
                try!(os.write_bool_no_tag(*v));
            };
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<BoolArr>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for BoolArr {
    fn new() -> BoolArr {
        BoolArr::new()
    }

    fn descriptor_static(_: ::std::option::Option<BoolArr>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_bool_accessor(
                    "bool",
                    BoolArr::get_bool,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BoolArr>(
                    "BoolArr",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BoolArr {
    fn clear(&mut self) {
        self.clear_bool();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for BoolArr {
    fn eq(&self, other: &BoolArr) -> bool {
        self.bool == other.bool &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for BoolArr {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct DoubleArr {
    // message fields
    double: ::std::vec::Vec<f64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl DoubleArr {
    pub fn new() -> DoubleArr {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DoubleArr {
        static mut instance: ::protobuf::lazy::Lazy<DoubleArr> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DoubleArr,
        };
        unsafe {
            instance.get(|| {
                DoubleArr {
                    double: ::std::vec::Vec::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
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
    pub fn mut_double<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<f64> {
        &mut self.double
    }

    // Take field
    pub fn take_double(&mut self) -> ::std::vec::Vec<f64> {
        ::std::mem::replace(&mut self.double, ::std::vec::Vec::new())
    }

    pub fn get_double<'a>(&'a self) -> &'a [f64] {
        &self.double
    }
}

impl ::protobuf::Message for DoubleArr {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                2 => {
                    try!(::protobuf::rt::read_repeated_double_into(wire_type, is, &mut self.double));
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.double.is_empty() {
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(self.double.len() as u32) + (self.double.len() * 8) as u32;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.double.is_empty() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            // TODO: Data size is computed again, it should be cached
            try!(os.write_raw_varint32((self.double.len() * 8) as u32));
            for v in self.double.iter() {
                try!(os.write_double_no_tag(*v));
            };
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<DoubleArr>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DoubleArr {
    fn new() -> DoubleArr {
        DoubleArr::new()
    }

    fn descriptor_static(_: ::std::option::Option<DoubleArr>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_f64_accessor(
                    "double",
                    DoubleArr::get_double,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DoubleArr>(
                    "DoubleArr",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DoubleArr {
    fn clear(&mut self) {
        self.clear_double();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for DoubleArr {
    fn eq(&self, other: &DoubleArr) -> bool {
        self.double == other.double &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for DoubleArr {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct FloatArr {
    // message fields
    float: ::std::vec::Vec<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl FloatArr {
    pub fn new() -> FloatArr {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FloatArr {
        static mut instance: ::protobuf::lazy::Lazy<FloatArr> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FloatArr,
        };
        unsafe {
            instance.get(|| {
                FloatArr {
                    float: ::std::vec::Vec::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
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
    pub fn mut_float<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<f32> {
        &mut self.float
    }

    // Take field
    pub fn take_float(&mut self) -> ::std::vec::Vec<f32> {
        ::std::mem::replace(&mut self.float, ::std::vec::Vec::new())
    }

    pub fn get_float<'a>(&'a self) -> &'a [f32] {
        &self.float
    }
}

impl ::protobuf::Message for FloatArr {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                3 => {
                    try!(::protobuf::rt::read_repeated_float_into(wire_type, is, &mut self.float));
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.float.is_empty() {
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(self.float.len() as u32) + (self.float.len() * 4) as u32;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.float.is_empty() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            // TODO: Data size is computed again, it should be cached
            try!(os.write_raw_varint32((self.float.len() * 4) as u32));
            for v in self.float.iter() {
                try!(os.write_float_no_tag(*v));
            };
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<FloatArr>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for FloatArr {
    fn new() -> FloatArr {
        FloatArr::new()
    }

    fn descriptor_static(_: ::std::option::Option<FloatArr>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_f32_accessor(
                    "float",
                    FloatArr::get_float,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FloatArr>(
                    "FloatArr",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FloatArr {
    fn clear(&mut self) {
        self.clear_float();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for FloatArr {
    fn eq(&self, other: &FloatArr) -> bool {
        self.float == other.float &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for FloatArr {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Int32Arr {
    // message fields
    int32: ::std::vec::Vec<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl Int32Arr {
    pub fn new() -> Int32Arr {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Int32Arr {
        static mut instance: ::protobuf::lazy::Lazy<Int32Arr> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Int32Arr,
        };
        unsafe {
            instance.get(|| {
                Int32Arr {
                    int32: ::std::vec::Vec::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
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
    pub fn mut_int32<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<i32> {
        &mut self.int32
    }

    // Take field
    pub fn take_int32(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.int32, ::std::vec::Vec::new())
    }

    pub fn get_int32<'a>(&'a self) -> &'a [i32] {
        &self.int32
    }
}

impl ::protobuf::Message for Int32Arr {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                4 => {
                    try!(::protobuf::rt::read_repeated_sint32_into(wire_type, is, &mut self.int32));
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.int32.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_zigzag_size(4, &self.int32);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.int32.is_empty() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            // TODO: Data size is computed again, it should be cached
            try!(os.write_raw_varint32(::protobuf::rt::vec_packed_varint_zigzag_data_size(&self.int32)));
            for v in self.int32.iter() {
                try!(os.write_sint32_no_tag(*v));
            };
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Int32Arr>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Int32Arr {
    fn new() -> Int32Arr {
        Int32Arr::new()
    }

    fn descriptor_static(_: ::std::option::Option<Int32Arr>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_i32_accessor(
                    "int32",
                    Int32Arr::get_int32,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Int32Arr>(
                    "Int32Arr",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Int32Arr {
    fn clear(&mut self) {
        self.clear_int32();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Int32Arr {
    fn eq(&self, other: &Int32Arr) -> bool {
        self.int32 == other.int32 &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Int32Arr {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Int64Arr {
    // message fields
    int64: ::std::vec::Vec<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl Int64Arr {
    pub fn new() -> Int64Arr {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Int64Arr {
        static mut instance: ::protobuf::lazy::Lazy<Int64Arr> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Int64Arr,
        };
        unsafe {
            instance.get(|| {
                Int64Arr {
                    int64: ::std::vec::Vec::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
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
    pub fn mut_int64<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<i64> {
        &mut self.int64
    }

    // Take field
    pub fn take_int64(&mut self) -> ::std::vec::Vec<i64> {
        ::std::mem::replace(&mut self.int64, ::std::vec::Vec::new())
    }

    pub fn get_int64<'a>(&'a self) -> &'a [i64] {
        &self.int64
    }
}

impl ::protobuf::Message for Int64Arr {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                5 => {
                    try!(::protobuf::rt::read_repeated_sint64_into(wire_type, is, &mut self.int64));
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.int64.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_zigzag_size(5, &self.int64);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.int64.is_empty() {
            try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
            // TODO: Data size is computed again, it should be cached
            try!(os.write_raw_varint32(::protobuf::rt::vec_packed_varint_zigzag_data_size(&self.int64)));
            for v in self.int64.iter() {
                try!(os.write_sint64_no_tag(*v));
            };
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Int64Arr>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Int64Arr {
    fn new() -> Int64Arr {
        Int64Arr::new()
    }

    fn descriptor_static(_: ::std::option::Option<Int64Arr>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_i64_accessor(
                    "int64",
                    Int64Arr::get_int64,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Int64Arr>(
                    "Int64Arr",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Int64Arr {
    fn clear(&mut self) {
        self.clear_int64();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Int64Arr {
    fn eq(&self, other: &Int64Arr) -> bool {
        self.int64 == other.int64 &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Int64Arr {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct UInt32Arr {
    // message fields
    uint32: ::std::vec::Vec<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl UInt32Arr {
    pub fn new() -> UInt32Arr {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UInt32Arr {
        static mut instance: ::protobuf::lazy::Lazy<UInt32Arr> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UInt32Arr,
        };
        unsafe {
            instance.get(|| {
                UInt32Arr {
                    uint32: ::std::vec::Vec::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
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
    pub fn mut_uint32<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u32> {
        &mut self.uint32
    }

    // Take field
    pub fn take_uint32(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.uint32, ::std::vec::Vec::new())
    }

    pub fn get_uint32<'a>(&'a self) -> &'a [u32] {
        &self.uint32
    }
}

impl ::protobuf::Message for UInt32Arr {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                6 => {
                    try!(::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.uint32));
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.uint32.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(6, &self.uint32);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.uint32.is_empty() {
            try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
            // TODO: Data size is computed again, it should be cached
            try!(os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(&self.uint32)));
            for v in self.uint32.iter() {
                try!(os.write_uint32_no_tag(*v));
            };
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<UInt32Arr>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for UInt32Arr {
    fn new() -> UInt32Arr {
        UInt32Arr::new()
    }

    fn descriptor_static(_: ::std::option::Option<UInt32Arr>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_u32_accessor(
                    "uint32",
                    UInt32Arr::get_uint32,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UInt32Arr>(
                    "UInt32Arr",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UInt32Arr {
    fn clear(&mut self) {
        self.clear_uint32();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for UInt32Arr {
    fn eq(&self, other: &UInt32Arr) -> bool {
        self.uint32 == other.uint32 &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for UInt32Arr {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct UInt64Arr {
    // message fields
    uint64: ::std::vec::Vec<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl UInt64Arr {
    pub fn new() -> UInt64Arr {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UInt64Arr {
        static mut instance: ::protobuf::lazy::Lazy<UInt64Arr> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UInt64Arr,
        };
        unsafe {
            instance.get(|| {
                UInt64Arr {
                    uint64: ::std::vec::Vec::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
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
    pub fn mut_uint64<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u64> {
        &mut self.uint64
    }

    // Take field
    pub fn take_uint64(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.uint64, ::std::vec::Vec::new())
    }

    pub fn get_uint64<'a>(&'a self) -> &'a [u64] {
        &self.uint64
    }
}

impl ::protobuf::Message for UInt64Arr {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                7 => {
                    try!(::protobuf::rt::read_repeated_uint64_into(wire_type, is, &mut self.uint64));
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.uint64.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(7, &self.uint64);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.uint64.is_empty() {
            try!(os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited));
            // TODO: Data size is computed again, it should be cached
            try!(os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(&self.uint64)));
            for v in self.uint64.iter() {
                try!(os.write_uint64_no_tag(*v));
            };
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<UInt64Arr>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for UInt64Arr {
    fn new() -> UInt64Arr {
        UInt64Arr::new()
    }

    fn descriptor_static(_: ::std::option::Option<UInt64Arr>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_u64_accessor(
                    "uint64",
                    UInt64Arr::get_uint64,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UInt64Arr>(
                    "UInt64Arr",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UInt64Arr {
    fn clear(&mut self) {
        self.clear_uint64();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for UInt64Arr {
    fn eq(&self, other: &UInt64Arr) -> bool {
        self.uint64 == other.uint64 &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for UInt64Arr {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct StringArr {
    // message fields
    string: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl StringArr {
    pub fn new() -> StringArr {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StringArr {
        static mut instance: ::protobuf::lazy::Lazy<StringArr> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StringArr,
        };
        unsafe {
            instance.get(|| {
                StringArr {
                    string: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
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
    pub fn mut_string<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.string
    }

    // Take field
    pub fn take_string(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.string, ::protobuf::RepeatedField::new())
    }

    pub fn get_string<'a>(&'a self) -> &'a [::std::string::String] {
        &self.string
    }
}

impl ::protobuf::Message for StringArr {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                8 => {
                    try!(::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.string));
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.string.iter() {
            my_size += ::protobuf::rt::string_size(8, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in self.string.iter() {
            try!(os.write_string(8, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<StringArr>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for StringArr {
    fn new() -> StringArr {
        StringArr::new()
    }

    fn descriptor_static(_: ::std::option::Option<StringArr>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_string_accessor(
                    "string",
                    StringArr::get_string,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StringArr>(
                    "StringArr",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StringArr {
    fn clear(&mut self) {
        self.clear_string();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for StringArr {
    fn eq(&self, other: &StringArr) -> bool {
        self.string == other.string &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for StringArr {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Int32StringArr {
    // message fields
    int32: ::std::vec::Vec<i32>,
    string: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl Int32StringArr {
    pub fn new() -> Int32StringArr {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Int32StringArr {
        static mut instance: ::protobuf::lazy::Lazy<Int32StringArr> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Int32StringArr,
        };
        unsafe {
            instance.get(|| {
                Int32StringArr {
                    int32: ::std::vec::Vec::new(),
                    string: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated sint32 int32 = 1;

    pub fn clear_int32(&mut self) {
        self.int32.clear();
    }

    // Param is passed by value, moved
    pub fn set_int32(&mut self, v: ::std::vec::Vec<i32>) {
        self.int32 = v;
    }

    // Mutable pointer to the field.
    pub fn mut_int32<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<i32> {
        &mut self.int32
    }

    // Take field
    pub fn take_int32(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.int32, ::std::vec::Vec::new())
    }

    pub fn get_int32<'a>(&'a self) -> &'a [i32] {
        &self.int32
    }

    // repeated string string = 2;

    pub fn clear_string(&mut self) {
        self.string.clear();
    }

    // Param is passed by value, moved
    pub fn set_string(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.string = v;
    }

    // Mutable pointer to the field.
    pub fn mut_string<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.string
    }

    // Take field
    pub fn take_string(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.string, ::protobuf::RepeatedField::new())
    }

    pub fn get_string<'a>(&'a self) -> &'a [::std::string::String] {
        &self.string
    }
}

impl ::protobuf::Message for Int32StringArr {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_sint32_into(wire_type, is, &mut self.int32));
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.string));
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.int32.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_zigzag_size(1, &self.int32);
        };
        for value in self.string.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.int32.is_empty() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            // TODO: Data size is computed again, it should be cached
            try!(os.write_raw_varint32(::protobuf::rt::vec_packed_varint_zigzag_data_size(&self.int32)));
            for v in self.int32.iter() {
                try!(os.write_sint32_no_tag(*v));
            };
        };
        for v in self.string.iter() {
            try!(os.write_string(2, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Int32StringArr>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Int32StringArr {
    fn new() -> Int32StringArr {
        Int32StringArr::new()
    }

    fn descriptor_static(_: ::std::option::Option<Int32StringArr>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_i32_accessor(
                    "int32",
                    Int32StringArr::get_int32,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_string_accessor(
                    "string",
                    Int32StringArr::get_string,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Int32StringArr>(
                    "Int32StringArr",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Int32StringArr {
    fn clear(&mut self) {
        self.clear_int32();
        self.clear_string();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Int32StringArr {
    fn eq(&self, other: &Int32StringArr) -> bool {
        self.int32 == other.int32 &&
        self.string == other.string &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Int32StringArr {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct DoubleStringArr {
    // message fields
    double: ::std::vec::Vec<f64>,
    string: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl DoubleStringArr {
    pub fn new() -> DoubleStringArr {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DoubleStringArr {
        static mut instance: ::protobuf::lazy::Lazy<DoubleStringArr> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DoubleStringArr,
        };
        unsafe {
            instance.get(|| {
                DoubleStringArr {
                    double: ::std::vec::Vec::new(),
                    string: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated double double = 1;

    pub fn clear_double(&mut self) {
        self.double.clear();
    }

    // Param is passed by value, moved
    pub fn set_double(&mut self, v: ::std::vec::Vec<f64>) {
        self.double = v;
    }

    // Mutable pointer to the field.
    pub fn mut_double<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<f64> {
        &mut self.double
    }

    // Take field
    pub fn take_double(&mut self) -> ::std::vec::Vec<f64> {
        ::std::mem::replace(&mut self.double, ::std::vec::Vec::new())
    }

    pub fn get_double<'a>(&'a self) -> &'a [f64] {
        &self.double
    }

    // repeated string string = 2;

    pub fn clear_string(&mut self) {
        self.string.clear();
    }

    // Param is passed by value, moved
    pub fn set_string(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.string = v;
    }

    // Mutable pointer to the field.
    pub fn mut_string<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.string
    }

    // Take field
    pub fn take_string(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.string, ::protobuf::RepeatedField::new())
    }

    pub fn get_string<'a>(&'a self) -> &'a [::std::string::String] {
        &self.string
    }
}

impl ::protobuf::Message for DoubleStringArr {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_double_into(wire_type, is, &mut self.double));
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.string));
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.double.is_empty() {
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(self.double.len() as u32) + (self.double.len() * 8) as u32;
        };
        for value in self.string.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.double.is_empty() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            // TODO: Data size is computed again, it should be cached
            try!(os.write_raw_varint32((self.double.len() * 8) as u32));
            for v in self.double.iter() {
                try!(os.write_double_no_tag(*v));
            };
        };
        for v in self.string.iter() {
            try!(os.write_string(2, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<DoubleStringArr>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DoubleStringArr {
    fn new() -> DoubleStringArr {
        DoubleStringArr::new()
    }

    fn descriptor_static(_: ::std::option::Option<DoubleStringArr>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_f64_accessor(
                    "double",
                    DoubleStringArr::get_double,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_string_accessor(
                    "string",
                    DoubleStringArr::get_string,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DoubleStringArr>(
                    "DoubleStringArr",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DoubleStringArr {
    fn clear(&mut self) {
        self.clear_double();
        self.clear_string();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for DoubleStringArr {
    fn eq(&self, other: &DoubleStringArr) -> bool {
        self.double == other.double &&
        self.string == other.string &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for DoubleStringArr {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Value {
    // message fields
    // message oneof groups
    value: ::std::option::Option<Value_oneof_value>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

#[derive(Clone,PartialEq)]
pub enum Value_oneof_value {
    void(bool),
    bool(bool),
    double(f64),
    float(f32),
    int32(i32),
    int64(i64),
    uint32(u32),
    uint64(u64),
    string(::std::string::String),
    bool_arr(BoolArr),
    double_arr(DoubleArr),
    float_arr(FloatArr),
    int32_arr(Int32Arr),
    int64_arr(Int64Arr),
    uint32_arr(UInt32Arr),
    uint64_arr(UInt64Arr),
    string_arr(StringArr),
    int32string_arr(Int32StringArr),
    doublestring_arr(DoubleStringArr),
}

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
                    value: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bool void = 1;

    pub fn clear_void(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_void(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Value_oneof_value::void(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_void(&mut self, v: bool) {
        self.value = ::std::option::Option::Some(Value_oneof_value::void(v))
    }

    pub fn get_void<'a>(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Value_oneof_value::void(v)) => v,
            _ => false,
        }
    }

    // optional bool bool = 2;

    pub fn clear_bool(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_bool(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Value_oneof_value::bool(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_bool(&mut self, v: bool) {
        self.value = ::std::option::Option::Some(Value_oneof_value::bool(v))
    }

    pub fn get_bool<'a>(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Value_oneof_value::bool(v)) => v,
            _ => false,
        }
    }

    // optional double double = 3;

    pub fn clear_double(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_double(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Value_oneof_value::double(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_double(&mut self, v: f64) {
        self.value = ::std::option::Option::Some(Value_oneof_value::double(v))
    }

    pub fn get_double<'a>(&self) -> f64 {
        match self.value {
            ::std::option::Option::Some(Value_oneof_value::double(v)) => v,
            _ => 0.,
        }
    }

    // optional float float = 4;

    pub fn clear_float(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_float(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Value_oneof_value::float(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_float(&mut self, v: f32) {
        self.value = ::std::option::Option::Some(Value_oneof_value::float(v))
    }

    pub fn get_float<'a>(&self) -> f32 {
        match self.value {
            ::std::option::Option::Some(Value_oneof_value::float(v)) => v,
            _ => 0.,
        }
    }

    // optional sint32 int32 = 5;

    pub fn clear_int32(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_int32(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Value_oneof_value::int32(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_int32(&mut self, v: i32) {
        self.value = ::std::option::Option::Some(Value_oneof_value::int32(v))
    }

    pub fn get_int32<'a>(&self) -> i32 {
        match self.value {
            ::std::option::Option::Some(Value_oneof_value::int32(v)) => v,
            _ => 0,
        }
    }

    // optional sint64 int64 = 6;

    pub fn clear_int64(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_int64(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Value_oneof_value::int64(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_int64(&mut self, v: i64) {
        self.value = ::std::option::Option::Some(Value_oneof_value::int64(v))
    }

    pub fn get_int64<'a>(&self) -> i64 {
        match self.value {
            ::std::option::Option::Some(Value_oneof_value::int64(v)) => v,
            _ => 0,
        }
    }

    // optional uint32 uint32 = 7;

    pub fn clear_uint32(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_uint32(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Value_oneof_value::uint32(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_uint32(&mut self, v: u32) {
        self.value = ::std::option::Option::Some(Value_oneof_value::uint32(v))
    }

    pub fn get_uint32<'a>(&self) -> u32 {
        match self.value {
            ::std::option::Option::Some(Value_oneof_value::uint32(v)) => v,
            _ => 0,
        }
    }

    // optional uint64 uint64 = 8;

    pub fn clear_uint64(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_uint64(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Value_oneof_value::uint64(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_uint64(&mut self, v: u64) {
        self.value = ::std::option::Option::Some(Value_oneof_value::uint64(v))
    }

    pub fn get_uint64<'a>(&self) -> u64 {
        match self.value {
            ::std::option::Option::Some(Value_oneof_value::uint64(v)) => v,
            _ => 0,
        }
    }

    // optional string string = 9;

    pub fn clear_string(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_string(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Value_oneof_value::string(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_string(&mut self, v: ::std::string::String) {
        self.value = ::std::option::Option::Some(Value_oneof_value::string(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_string<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if let ::std::option::Option::Some(Value_oneof_value::string(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(Value_oneof_value::string(::std::string::String::new()));
        }
        match self.value {
            ::std::option::Option::Some(Value_oneof_value::string(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_string(&mut self) -> ::std::string::String {
        if self.has_string() {
            match self.value.take() {
                ::std::option::Option::Some(Value_oneof_value::string(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_string<'a>(&'a self) -> &'a str {
        match self.value {
            ::std::option::Option::Some(Value_oneof_value::string(ref v)) => v,
            _ => "",
        }
    }

    // optional .BoolArr bool_arr = 11;

    pub fn clear_bool_arr(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_bool_arr(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Value_oneof_value::bool_arr(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_bool_arr(&mut self, v: BoolArr) {
        self.value = ::std::option::Option::Some(Value_oneof_value::bool_arr(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_bool_arr<'a>(&'a mut self) -> &'a mut BoolArr {
        if let ::std::option::Option::Some(Value_oneof_value::bool_arr(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(Value_oneof_value::bool_arr(BoolArr::new()));
        }
        match self.value {
            ::std::option::Option::Some(Value_oneof_value::bool_arr(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_bool_arr(&mut self) -> BoolArr {
        if self.has_bool_arr() {
            match self.value.take() {
                ::std::option::Option::Some(Value_oneof_value::bool_arr(v)) => v,
                _ => panic!(),
            }
        } else {
            BoolArr::new()
        }
    }

    pub fn get_bool_arr<'a>(&'a self) -> &'a BoolArr {
        match self.value {
            ::std::option::Option::Some(Value_oneof_value::bool_arr(ref v)) => v,
            _ => BoolArr::default_instance(),
        }
    }

    // optional .DoubleArr double_arr = 12;

    pub fn clear_double_arr(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_double_arr(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Value_oneof_value::double_arr(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_double_arr(&mut self, v: DoubleArr) {
        self.value = ::std::option::Option::Some(Value_oneof_value::double_arr(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_double_arr<'a>(&'a mut self) -> &'a mut DoubleArr {
        if let ::std::option::Option::Some(Value_oneof_value::double_arr(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(Value_oneof_value::double_arr(DoubleArr::new()));
        }
        match self.value {
            ::std::option::Option::Some(Value_oneof_value::double_arr(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_double_arr(&mut self) -> DoubleArr {
        if self.has_double_arr() {
            match self.value.take() {
                ::std::option::Option::Some(Value_oneof_value::double_arr(v)) => v,
                _ => panic!(),
            }
        } else {
            DoubleArr::new()
        }
    }

    pub fn get_double_arr<'a>(&'a self) -> &'a DoubleArr {
        match self.value {
            ::std::option::Option::Some(Value_oneof_value::double_arr(ref v)) => v,
            _ => DoubleArr::default_instance(),
        }
    }

    // optional .FloatArr float_arr = 13;

    pub fn clear_float_arr(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_float_arr(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Value_oneof_value::float_arr(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_float_arr(&mut self, v: FloatArr) {
        self.value = ::std::option::Option::Some(Value_oneof_value::float_arr(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_float_arr<'a>(&'a mut self) -> &'a mut FloatArr {
        if let ::std::option::Option::Some(Value_oneof_value::float_arr(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(Value_oneof_value::float_arr(FloatArr::new()));
        }
        match self.value {
            ::std::option::Option::Some(Value_oneof_value::float_arr(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_float_arr(&mut self) -> FloatArr {
        if self.has_float_arr() {
            match self.value.take() {
                ::std::option::Option::Some(Value_oneof_value::float_arr(v)) => v,
                _ => panic!(),
            }
        } else {
            FloatArr::new()
        }
    }

    pub fn get_float_arr<'a>(&'a self) -> &'a FloatArr {
        match self.value {
            ::std::option::Option::Some(Value_oneof_value::float_arr(ref v)) => v,
            _ => FloatArr::default_instance(),
        }
    }

    // optional .Int32Arr int32_arr = 14;

    pub fn clear_int32_arr(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_int32_arr(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Value_oneof_value::int32_arr(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_int32_arr(&mut self, v: Int32Arr) {
        self.value = ::std::option::Option::Some(Value_oneof_value::int32_arr(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_int32_arr<'a>(&'a mut self) -> &'a mut Int32Arr {
        if let ::std::option::Option::Some(Value_oneof_value::int32_arr(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(Value_oneof_value::int32_arr(Int32Arr::new()));
        }
        match self.value {
            ::std::option::Option::Some(Value_oneof_value::int32_arr(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_int32_arr(&mut self) -> Int32Arr {
        if self.has_int32_arr() {
            match self.value.take() {
                ::std::option::Option::Some(Value_oneof_value::int32_arr(v)) => v,
                _ => panic!(),
            }
        } else {
            Int32Arr::new()
        }
    }

    pub fn get_int32_arr<'a>(&'a self) -> &'a Int32Arr {
        match self.value {
            ::std::option::Option::Some(Value_oneof_value::int32_arr(ref v)) => v,
            _ => Int32Arr::default_instance(),
        }
    }

    // optional .Int64Arr int64_arr = 15;

    pub fn clear_int64_arr(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_int64_arr(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Value_oneof_value::int64_arr(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_int64_arr(&mut self, v: Int64Arr) {
        self.value = ::std::option::Option::Some(Value_oneof_value::int64_arr(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_int64_arr<'a>(&'a mut self) -> &'a mut Int64Arr {
        if let ::std::option::Option::Some(Value_oneof_value::int64_arr(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(Value_oneof_value::int64_arr(Int64Arr::new()));
        }
        match self.value {
            ::std::option::Option::Some(Value_oneof_value::int64_arr(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_int64_arr(&mut self) -> Int64Arr {
        if self.has_int64_arr() {
            match self.value.take() {
                ::std::option::Option::Some(Value_oneof_value::int64_arr(v)) => v,
                _ => panic!(),
            }
        } else {
            Int64Arr::new()
        }
    }

    pub fn get_int64_arr<'a>(&'a self) -> &'a Int64Arr {
        match self.value {
            ::std::option::Option::Some(Value_oneof_value::int64_arr(ref v)) => v,
            _ => Int64Arr::default_instance(),
        }
    }

    // optional .UInt32Arr uint32_arr = 16;

    pub fn clear_uint32_arr(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_uint32_arr(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Value_oneof_value::uint32_arr(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_uint32_arr(&mut self, v: UInt32Arr) {
        self.value = ::std::option::Option::Some(Value_oneof_value::uint32_arr(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_uint32_arr<'a>(&'a mut self) -> &'a mut UInt32Arr {
        if let ::std::option::Option::Some(Value_oneof_value::uint32_arr(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(Value_oneof_value::uint32_arr(UInt32Arr::new()));
        }
        match self.value {
            ::std::option::Option::Some(Value_oneof_value::uint32_arr(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_uint32_arr(&mut self) -> UInt32Arr {
        if self.has_uint32_arr() {
            match self.value.take() {
                ::std::option::Option::Some(Value_oneof_value::uint32_arr(v)) => v,
                _ => panic!(),
            }
        } else {
            UInt32Arr::new()
        }
    }

    pub fn get_uint32_arr<'a>(&'a self) -> &'a UInt32Arr {
        match self.value {
            ::std::option::Option::Some(Value_oneof_value::uint32_arr(ref v)) => v,
            _ => UInt32Arr::default_instance(),
        }
    }

    // optional .UInt64Arr uint64_arr = 17;

    pub fn clear_uint64_arr(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_uint64_arr(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Value_oneof_value::uint64_arr(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_uint64_arr(&mut self, v: UInt64Arr) {
        self.value = ::std::option::Option::Some(Value_oneof_value::uint64_arr(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_uint64_arr<'a>(&'a mut self) -> &'a mut UInt64Arr {
        if let ::std::option::Option::Some(Value_oneof_value::uint64_arr(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(Value_oneof_value::uint64_arr(UInt64Arr::new()));
        }
        match self.value {
            ::std::option::Option::Some(Value_oneof_value::uint64_arr(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_uint64_arr(&mut self) -> UInt64Arr {
        if self.has_uint64_arr() {
            match self.value.take() {
                ::std::option::Option::Some(Value_oneof_value::uint64_arr(v)) => v,
                _ => panic!(),
            }
        } else {
            UInt64Arr::new()
        }
    }

    pub fn get_uint64_arr<'a>(&'a self) -> &'a UInt64Arr {
        match self.value {
            ::std::option::Option::Some(Value_oneof_value::uint64_arr(ref v)) => v,
            _ => UInt64Arr::default_instance(),
        }
    }

    // optional .StringArr string_arr = 18;

    pub fn clear_string_arr(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_string_arr(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Value_oneof_value::string_arr(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_string_arr(&mut self, v: StringArr) {
        self.value = ::std::option::Option::Some(Value_oneof_value::string_arr(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_string_arr<'a>(&'a mut self) -> &'a mut StringArr {
        if let ::std::option::Option::Some(Value_oneof_value::string_arr(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(Value_oneof_value::string_arr(StringArr::new()));
        }
        match self.value {
            ::std::option::Option::Some(Value_oneof_value::string_arr(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_string_arr(&mut self) -> StringArr {
        if self.has_string_arr() {
            match self.value.take() {
                ::std::option::Option::Some(Value_oneof_value::string_arr(v)) => v,
                _ => panic!(),
            }
        } else {
            StringArr::new()
        }
    }

    pub fn get_string_arr<'a>(&'a self) -> &'a StringArr {
        match self.value {
            ::std::option::Option::Some(Value_oneof_value::string_arr(ref v)) => v,
            _ => StringArr::default_instance(),
        }
    }

    // optional .Int32StringArr int32string_arr = 21;

    pub fn clear_int32string_arr(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_int32string_arr(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Value_oneof_value::int32string_arr(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_int32string_arr(&mut self, v: Int32StringArr) {
        self.value = ::std::option::Option::Some(Value_oneof_value::int32string_arr(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_int32string_arr<'a>(&'a mut self) -> &'a mut Int32StringArr {
        if let ::std::option::Option::Some(Value_oneof_value::int32string_arr(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(Value_oneof_value::int32string_arr(Int32StringArr::new()));
        }
        match self.value {
            ::std::option::Option::Some(Value_oneof_value::int32string_arr(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_int32string_arr(&mut self) -> Int32StringArr {
        if self.has_int32string_arr() {
            match self.value.take() {
                ::std::option::Option::Some(Value_oneof_value::int32string_arr(v)) => v,
                _ => panic!(),
            }
        } else {
            Int32StringArr::new()
        }
    }

    pub fn get_int32string_arr<'a>(&'a self) -> &'a Int32StringArr {
        match self.value {
            ::std::option::Option::Some(Value_oneof_value::int32string_arr(ref v)) => v,
            _ => Int32StringArr::default_instance(),
        }
    }

    // optional .DoubleStringArr doublestring_arr = 22;

    pub fn clear_doublestring_arr(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_doublestring_arr(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Value_oneof_value::doublestring_arr(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_doublestring_arr(&mut self, v: DoubleStringArr) {
        self.value = ::std::option::Option::Some(Value_oneof_value::doublestring_arr(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_doublestring_arr<'a>(&'a mut self) -> &'a mut DoubleStringArr {
        if let ::std::option::Option::Some(Value_oneof_value::doublestring_arr(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(Value_oneof_value::doublestring_arr(DoubleStringArr::new()));
        }
        match self.value {
            ::std::option::Option::Some(Value_oneof_value::doublestring_arr(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_doublestring_arr(&mut self) -> DoubleStringArr {
        if self.has_doublestring_arr() {
            match self.value.take() {
                ::std::option::Option::Some(Value_oneof_value::doublestring_arr(v)) => v,
                _ => panic!(),
            }
        } else {
            DoubleStringArr::new()
        }
    }

    pub fn get_doublestring_arr<'a>(&'a self) -> &'a DoubleStringArr {
        match self.value {
            ::std::option::Option::Some(Value_oneof_value::doublestring_arr(ref v)) => v,
            _ => DoubleStringArr::default_instance(),
        }
    }
}

impl ::protobuf::Message for Value {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(Value_oneof_value::void(try!(is.read_bool())));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(Value_oneof_value::bool(try!(is.read_bool())));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(Value_oneof_value::double(try!(is.read_double())));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(Value_oneof_value::float(try!(is.read_float())));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(Value_oneof_value::int32(try!(is.read_sint32())));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(Value_oneof_value::int64(try!(is.read_sint64())));
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(Value_oneof_value::uint32(try!(is.read_uint32())));
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(Value_oneof_value::uint64(try!(is.read_uint64())));
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(Value_oneof_value::string(try!(is.read_string())));
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(Value_oneof_value::bool_arr(try!(is.read_message())));
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(Value_oneof_value::double_arr(try!(is.read_message())));
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(Value_oneof_value::float_arr(try!(is.read_message())));
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(Value_oneof_value::int32_arr(try!(is.read_message())));
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(Value_oneof_value::int64_arr(try!(is.read_message())));
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(Value_oneof_value::uint32_arr(try!(is.read_message())));
                },
                17 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(Value_oneof_value::uint64_arr(try!(is.read_message())));
                },
                18 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(Value_oneof_value::string_arr(try!(is.read_message())));
                },
                21 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(Value_oneof_value::int32string_arr(try!(is.read_message())));
                },
                22 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(Value_oneof_value::doublestring_arr(try!(is.read_message())));
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let ::std::option::Option::Some(ref v) = self.value {
            match v {
                &Value_oneof_value::void(v) => {
                    my_size += 2;
                },
                &Value_oneof_value::bool(v) => {
                    my_size += 2;
                },
                &Value_oneof_value::double(v) => {
                    my_size += 9;
                },
                &Value_oneof_value::float(v) => {
                    my_size += 5;
                },
                &Value_oneof_value::int32(v) => {
                    my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
                },
                &Value_oneof_value::int64(v) => {
                    my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
                },
                &Value_oneof_value::uint32(v) => {
                    my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
                },
                &Value_oneof_value::uint64(v) => {
                    my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
                },
                &Value_oneof_value::string(ref v) => {
                    my_size += ::protobuf::rt::string_size(9, &v);
                },
                &Value_oneof_value::bool_arr(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Value_oneof_value::double_arr(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Value_oneof_value::float_arr(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Value_oneof_value::int32_arr(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Value_oneof_value::int64_arr(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Value_oneof_value::uint32_arr(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Value_oneof_value::uint64_arr(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Value_oneof_value::string_arr(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Value_oneof_value::int32string_arr(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Value_oneof_value::doublestring_arr(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.value {
            match v {
                &Value_oneof_value::void(v) => {
                    try!(os.write_bool(1, v));
                },
                &Value_oneof_value::bool(v) => {
                    try!(os.write_bool(2, v));
                },
                &Value_oneof_value::double(v) => {
                    try!(os.write_double(3, v));
                },
                &Value_oneof_value::float(v) => {
                    try!(os.write_float(4, v));
                },
                &Value_oneof_value::int32(v) => {
                    try!(os.write_sint32(5, v));
                },
                &Value_oneof_value::int64(v) => {
                    try!(os.write_sint64(6, v));
                },
                &Value_oneof_value::uint32(v) => {
                    try!(os.write_uint32(7, v));
                },
                &Value_oneof_value::uint64(v) => {
                    try!(os.write_uint64(8, v));
                },
                &Value_oneof_value::string(ref v) => {
                    try!(os.write_string(9, v));
                },
                &Value_oneof_value::bool_arr(ref v) => {
                    try!(os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &Value_oneof_value::double_arr(ref v) => {
                    try!(os.write_tag(12, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &Value_oneof_value::float_arr(ref v) => {
                    try!(os.write_tag(13, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &Value_oneof_value::int32_arr(ref v) => {
                    try!(os.write_tag(14, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &Value_oneof_value::int64_arr(ref v) => {
                    try!(os.write_tag(15, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &Value_oneof_value::uint32_arr(ref v) => {
                    try!(os.write_tag(16, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &Value_oneof_value::uint64_arr(ref v) => {
                    try!(os.write_tag(17, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &Value_oneof_value::string_arr(ref v) => {
                    try!(os.write_tag(18, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &Value_oneof_value::int32string_arr(ref v) => {
                    try!(os.write_tag(21, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &Value_oneof_value::doublestring_arr(ref v) => {
                    try!(os.write_tag(22, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
            };
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
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
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "void",
                    Value::has_void,
                    Value::get_void,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "bool",
                    Value::has_bool,
                    Value::get_bool,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "double",
                    Value::has_double,
                    Value::get_double,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "float",
                    Value::has_float,
                    Value::get_float,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "int32",
                    Value::has_int32,
                    Value::get_int32,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "int64",
                    Value::has_int64,
                    Value::get_int64,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "uint32",
                    Value::has_uint32,
                    Value::get_uint32,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "uint64",
                    Value::has_uint64,
                    Value::get_uint64,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "string",
                    Value::has_string,
                    Value::get_string,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "bool_arr",
                    Value::has_bool_arr,
                    Value::get_bool_arr,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "double_arr",
                    Value::has_double_arr,
                    Value::get_double_arr,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "float_arr",
                    Value::has_float_arr,
                    Value::get_float_arr,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "int32_arr",
                    Value::has_int32_arr,
                    Value::get_int32_arr,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "int64_arr",
                    Value::has_int64_arr,
                    Value::get_int64_arr,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "uint32_arr",
                    Value::has_uint32_arr,
                    Value::get_uint32_arr,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "uint64_arr",
                    Value::has_uint64_arr,
                    Value::get_uint64_arr,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "string_arr",
                    Value::has_string_arr,
                    Value::get_string_arr,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "int32string_arr",
                    Value::has_int32string_arr,
                    Value::get_int32string_arr,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "doublestring_arr",
                    Value::has_doublestring_arr,
                    Value::get_doublestring_arr,
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
        self.clear_void();
        self.clear_bool();
        self.clear_double();
        self.clear_float();
        self.clear_int32();
        self.clear_int64();
        self.clear_uint32();
        self.clear_uint64();
        self.clear_string();
        self.clear_bool_arr();
        self.clear_double_arr();
        self.clear_float_arr();
        self.clear_int32_arr();
        self.clear_int64_arr();
        self.clear_uint32_arr();
        self.clear_uint64_arr();
        self.clear_string_arr();
        self.clear_int32string_arr();
        self.clear_doublestring_arr();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Value {
    fn eq(&self, other: &Value) -> bool {
        self.value == other.value &&
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
    pub fn mut_reason<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.reason.is_none() {
            self.reason.set_default();
        };
        self.reason.as_mut().unwrap()
    }

    // Take field
    pub fn take_reason(&mut self) -> ::std::string::String {
        self.reason.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_reason<'a>(&'a self) -> &'a str {
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
    pub fn mut_desc<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.desc.is_none() {
            self.desc.set_default();
        };
        self.desc.as_mut().unwrap()
    }

    // Take field
    pub fn take_desc(&mut self) -> ::std::string::String {
        self.desc.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_desc<'a>(&'a self) -> &'a str {
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
    pub fn mut_origin<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.origin.is_none() {
            self.origin.set_default();
        };
        self.origin.as_mut().unwrap()
    }

    // Take field
    pub fn take_origin(&mut self) -> ::std::string::String {
        self.origin.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_origin<'a>(&'a self) -> &'a str {
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
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.reason.set_default();
                    try!(is.read_string_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.desc.set_default();
                    try!(is.read_string_into(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.origin.set_default();
                    try!(is.read_string_into(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
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

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
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
pub struct ExecCommand {
    // message fields
    cmd: ::protobuf::SingularField<::std::string::String>,
    value: ::protobuf::SingularPtrField<Value>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl ExecCommand {
    pub fn new() -> ExecCommand {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ExecCommand {
        static mut instance: ::protobuf::lazy::Lazy<ExecCommand> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ExecCommand,
        };
        unsafe {
            instance.get(|| {
                ExecCommand {
                    cmd: ::protobuf::SingularField::none(),
                    value: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string cmd = 1;

    pub fn clear_cmd(&mut self) {
        self.cmd.clear();
    }

    pub fn has_cmd(&self) -> bool {
        self.cmd.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cmd(&mut self, v: ::std::string::String) {
        self.cmd = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cmd<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.cmd.is_none() {
            self.cmd.set_default();
        };
        self.cmd.as_mut().unwrap()
    }

    // Take field
    pub fn take_cmd(&mut self) -> ::std::string::String {
        self.cmd.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_cmd<'a>(&'a self) -> &'a str {
        match self.cmd.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional .Value value = 2;

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
    pub fn mut_value<'a>(&'a mut self) -> &'a mut Value {
        if self.value.is_none() {
            self.value.set_default();
        };
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> Value {
        self.value.take().unwrap_or_else(|| Value::new())
    }

    pub fn get_value<'a>(&'a self) -> &'a Value {
        self.value.as_ref().unwrap_or_else(|| Value::default_instance())
    }
}

impl ::protobuf::Message for ExecCommand {
    fn is_initialized(&self) -> bool {
        if self.cmd.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.cmd.set_default();
                    try!(is.read_string_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.value.set_default();
                    try!(is.merge_message(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.cmd.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
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
        if let Some(v) = self.cmd.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.value.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ExecCommand>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ExecCommand {
    fn new() -> ExecCommand {
        ExecCommand::new()
    }

    fn descriptor_static(_: ::std::option::Option<ExecCommand>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "cmd",
                    ExecCommand::has_cmd,
                    ExecCommand::get_cmd,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "value",
                    ExecCommand::has_value,
                    ExecCommand::get_value,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ExecCommand>(
                    "ExecCommand",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ExecCommand {
    fn clear(&mut self) {
        self.clear_cmd();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ExecCommand {
    fn eq(&self, other: &ExecCommand) -> bool {
        self.cmd == other.cmd &&
        self.value == other.value &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ExecCommand {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ReadAttribute {
    // message fields
    attr: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl ReadAttribute {
    pub fn new() -> ReadAttribute {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ReadAttribute {
        static mut instance: ::protobuf::lazy::Lazy<ReadAttribute> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ReadAttribute,
        };
        unsafe {
            instance.get(|| {
                ReadAttribute {
                    attr: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string attr = 1;

    pub fn clear_attr(&mut self) {
        self.attr.clear();
    }

    pub fn has_attr(&self) -> bool {
        self.attr.is_some()
    }

    // Param is passed by value, moved
    pub fn set_attr(&mut self, v: ::std::string::String) {
        self.attr = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_attr<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.attr.is_none() {
            self.attr.set_default();
        };
        self.attr.as_mut().unwrap()
    }

    // Take field
    pub fn take_attr(&mut self) -> ::std::string::String {
        self.attr.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_attr<'a>(&'a self) -> &'a str {
        match self.attr.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for ReadAttribute {
    fn is_initialized(&self) -> bool {
        if self.attr.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.attr.set_default();
                    try!(is.read_string_into(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.attr.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.attr.as_ref() {
            try!(os.write_string(1, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ReadAttribute>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ReadAttribute {
    fn new() -> ReadAttribute {
        ReadAttribute::new()
    }

    fn descriptor_static(_: ::std::option::Option<ReadAttribute>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "attr",
                    ReadAttribute::has_attr,
                    ReadAttribute::get_attr,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ReadAttribute>(
                    "ReadAttribute",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ReadAttribute {
    fn clear(&mut self) {
        self.clear_attr();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ReadAttribute {
    fn eq(&self, other: &ReadAttribute) -> bool {
        self.attr == other.attr &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ReadAttribute {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct WriteAttribute {
    // message fields
    attr: ::protobuf::SingularField<::std::string::String>,
    value: ::protobuf::SingularPtrField<Value>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl WriteAttribute {
    pub fn new() -> WriteAttribute {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static WriteAttribute {
        static mut instance: ::protobuf::lazy::Lazy<WriteAttribute> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const WriteAttribute,
        };
        unsafe {
            instance.get(|| {
                WriteAttribute {
                    attr: ::protobuf::SingularField::none(),
                    value: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string attr = 1;

    pub fn clear_attr(&mut self) {
        self.attr.clear();
    }

    pub fn has_attr(&self) -> bool {
        self.attr.is_some()
    }

    // Param is passed by value, moved
    pub fn set_attr(&mut self, v: ::std::string::String) {
        self.attr = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_attr<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.attr.is_none() {
            self.attr.set_default();
        };
        self.attr.as_mut().unwrap()
    }

    // Take field
    pub fn take_attr(&mut self) -> ::std::string::String {
        self.attr.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_attr<'a>(&'a self) -> &'a str {
        match self.attr.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required .Value value = 2;

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
    pub fn mut_value<'a>(&'a mut self) -> &'a mut Value {
        if self.value.is_none() {
            self.value.set_default();
        };
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> Value {
        self.value.take().unwrap_or_else(|| Value::new())
    }

    pub fn get_value<'a>(&'a self) -> &'a Value {
        self.value.as_ref().unwrap_or_else(|| Value::default_instance())
    }
}

impl ::protobuf::Message for WriteAttribute {
    fn is_initialized(&self) -> bool {
        if self.attr.is_none() {
            return false;
        };
        if self.value.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.attr.set_default();
                    try!(is.read_string_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.value.set_default();
                    try!(is.merge_message(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.attr.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
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
        if let Some(v) = self.attr.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.value.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<WriteAttribute>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for WriteAttribute {
    fn new() -> WriteAttribute {
        WriteAttribute::new()
    }

    fn descriptor_static(_: ::std::option::Option<WriteAttribute>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "attr",
                    WriteAttribute::has_attr,
                    WriteAttribute::get_attr,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "value",
                    WriteAttribute::has_value,
                    WriteAttribute::get_value,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<WriteAttribute>(
                    "WriteAttribute",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for WriteAttribute {
    fn clear(&mut self) {
        self.clear_attr();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for WriteAttribute {
    fn eq(&self, other: &WriteAttribute) -> bool {
        self.attr == other.attr &&
        self.value == other.value &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for WriteAttribute {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct QueryAPI {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl QueryAPI {
    pub fn new() -> QueryAPI {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static QueryAPI {
        static mut instance: ::protobuf::lazy::Lazy<QueryAPI> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const QueryAPI,
        };
        unsafe {
            instance.get(|| {
                QueryAPI {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for QueryAPI {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<QueryAPI>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for QueryAPI {
    fn new() -> QueryAPI {
        QueryAPI::new()
    }

    fn descriptor_static(_: ::std::option::Option<QueryAPI>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<QueryAPI>(
                    "QueryAPI",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for QueryAPI {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for QueryAPI {
    fn eq(&self, other: &QueryAPI) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for QueryAPI {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Request {
    // message fields
    seqno: ::std::option::Option<u32>,
    // message oneof groups
    request: ::std::option::Option<Request_oneof_request>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

#[derive(Clone,PartialEq)]
pub enum Request_oneof_request {
    exec(ExecCommand),
    rattr(ReadAttribute),
    wattr(WriteAttribute),
    qapi(QueryAPI),
}

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
                    request: ::std::option::Option::None,
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

    pub fn get_seqno<'a>(&self) -> u32 {
        self.seqno.unwrap_or(0)
    }

    // optional .ExecCommand exec = 2;

    pub fn clear_exec(&mut self) {
        self.request = ::std::option::Option::None;
    }

    pub fn has_exec(&self) -> bool {
        match self.request {
            ::std::option::Option::Some(Request_oneof_request::exec(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_exec(&mut self, v: ExecCommand) {
        self.request = ::std::option::Option::Some(Request_oneof_request::exec(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_exec<'a>(&'a mut self) -> &'a mut ExecCommand {
        if let ::std::option::Option::Some(Request_oneof_request::exec(_)) = self.request {
        } else {
            self.request = ::std::option::Option::Some(Request_oneof_request::exec(ExecCommand::new()));
        }
        match self.request {
            ::std::option::Option::Some(Request_oneof_request::exec(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_exec(&mut self) -> ExecCommand {
        if self.has_exec() {
            match self.request.take() {
                ::std::option::Option::Some(Request_oneof_request::exec(v)) => v,
                _ => panic!(),
            }
        } else {
            ExecCommand::new()
        }
    }

    pub fn get_exec<'a>(&'a self) -> &'a ExecCommand {
        match self.request {
            ::std::option::Option::Some(Request_oneof_request::exec(ref v)) => v,
            _ => ExecCommand::default_instance(),
        }
    }

    // optional .ReadAttribute rattr = 3;

    pub fn clear_rattr(&mut self) {
        self.request = ::std::option::Option::None;
    }

    pub fn has_rattr(&self) -> bool {
        match self.request {
            ::std::option::Option::Some(Request_oneof_request::rattr(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_rattr(&mut self, v: ReadAttribute) {
        self.request = ::std::option::Option::Some(Request_oneof_request::rattr(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_rattr<'a>(&'a mut self) -> &'a mut ReadAttribute {
        if let ::std::option::Option::Some(Request_oneof_request::rattr(_)) = self.request {
        } else {
            self.request = ::std::option::Option::Some(Request_oneof_request::rattr(ReadAttribute::new()));
        }
        match self.request {
            ::std::option::Option::Some(Request_oneof_request::rattr(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_rattr(&mut self) -> ReadAttribute {
        if self.has_rattr() {
            match self.request.take() {
                ::std::option::Option::Some(Request_oneof_request::rattr(v)) => v,
                _ => panic!(),
            }
        } else {
            ReadAttribute::new()
        }
    }

    pub fn get_rattr<'a>(&'a self) -> &'a ReadAttribute {
        match self.request {
            ::std::option::Option::Some(Request_oneof_request::rattr(ref v)) => v,
            _ => ReadAttribute::default_instance(),
        }
    }

    // optional .WriteAttribute wattr = 4;

    pub fn clear_wattr(&mut self) {
        self.request = ::std::option::Option::None;
    }

    pub fn has_wattr(&self) -> bool {
        match self.request {
            ::std::option::Option::Some(Request_oneof_request::wattr(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_wattr(&mut self, v: WriteAttribute) {
        self.request = ::std::option::Option::Some(Request_oneof_request::wattr(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_wattr<'a>(&'a mut self) -> &'a mut WriteAttribute {
        if let ::std::option::Option::Some(Request_oneof_request::wattr(_)) = self.request {
        } else {
            self.request = ::std::option::Option::Some(Request_oneof_request::wattr(WriteAttribute::new()));
        }
        match self.request {
            ::std::option::Option::Some(Request_oneof_request::wattr(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_wattr(&mut self) -> WriteAttribute {
        if self.has_wattr() {
            match self.request.take() {
                ::std::option::Option::Some(Request_oneof_request::wattr(v)) => v,
                _ => panic!(),
            }
        } else {
            WriteAttribute::new()
        }
    }

    pub fn get_wattr<'a>(&'a self) -> &'a WriteAttribute {
        match self.request {
            ::std::option::Option::Some(Request_oneof_request::wattr(ref v)) => v,
            _ => WriteAttribute::default_instance(),
        }
    }

    // optional .QueryAPI qapi = 6;

    pub fn clear_qapi(&mut self) {
        self.request = ::std::option::Option::None;
    }

    pub fn has_qapi(&self) -> bool {
        match self.request {
            ::std::option::Option::Some(Request_oneof_request::qapi(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_qapi(&mut self, v: QueryAPI) {
        self.request = ::std::option::Option::Some(Request_oneof_request::qapi(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_qapi<'a>(&'a mut self) -> &'a mut QueryAPI {
        if let ::std::option::Option::Some(Request_oneof_request::qapi(_)) = self.request {
        } else {
            self.request = ::std::option::Option::Some(Request_oneof_request::qapi(QueryAPI::new()));
        }
        match self.request {
            ::std::option::Option::Some(Request_oneof_request::qapi(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_qapi(&mut self) -> QueryAPI {
        if self.has_qapi() {
            match self.request.take() {
                ::std::option::Option::Some(Request_oneof_request::qapi(v)) => v,
                _ => panic!(),
            }
        } else {
            QueryAPI::new()
        }
    }

    pub fn get_qapi<'a>(&'a self) -> &'a QueryAPI {
        match self.request {
            ::std::option::Option::Some(Request_oneof_request::qapi(ref v)) => v,
            _ => QueryAPI::default_instance(),
        }
    }
}

impl ::protobuf::Message for Request {
    fn is_initialized(&self) -> bool {
        if self.seqno.is_none() {
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
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.seqno = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.request = ::std::option::Option::Some(Request_oneof_request::exec(try!(is.read_message())));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.request = ::std::option::Option::Some(Request_oneof_request::rattr(try!(is.read_message())));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.request = ::std::option::Option::Some(Request_oneof_request::wattr(try!(is.read_message())));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.request = ::std::option::Option::Some(Request_oneof_request::qapi(try!(is.read_message())));
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
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
        if let ::std::option::Option::Some(ref v) = self.request {
            match v {
                &Request_oneof_request::exec(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Request_oneof_request::rattr(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Request_oneof_request::wattr(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Request_oneof_request::qapi(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.seqno {
            try!(os.write_uint32(1, v));
        };
        if let ::std::option::Option::Some(ref v) = self.request {
            match v {
                &Request_oneof_request::exec(ref v) => {
                    try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &Request_oneof_request::rattr(ref v) => {
                    try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &Request_oneof_request::wattr(ref v) => {
                    try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &Request_oneof_request::qapi(ref v) => {
                    try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
            };
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
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
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "exec",
                    Request::has_exec,
                    Request::get_exec,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "rattr",
                    Request::has_rattr,
                    Request::get_rattr,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "wattr",
                    Request::has_wattr,
                    Request::get_wattr,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "qapi",
                    Request::has_qapi,
                    Request::get_qapi,
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
        self.clear_exec();
        self.clear_rattr();
        self.clear_wattr();
        self.clear_qapi();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Request {
    fn eq(&self, other: &Request) -> bool {
        self.seqno == other.seqno &&
        self.request == other.request &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Request {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ExecCommandResult {
    // message fields
    cmd: ::protobuf::SingularField<::std::string::String>,
    // message oneof groups
    result: ::std::option::Option<ExecCommandResult_oneof_result>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

#[derive(Clone,PartialEq)]
pub enum ExecCommandResult_oneof_result {
    value(Value),
    error(Error),
}

impl ExecCommandResult {
    pub fn new() -> ExecCommandResult {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ExecCommandResult {
        static mut instance: ::protobuf::lazy::Lazy<ExecCommandResult> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ExecCommandResult,
        };
        unsafe {
            instance.get(|| {
                ExecCommandResult {
                    cmd: ::protobuf::SingularField::none(),
                    result: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string cmd = 1;

    pub fn clear_cmd(&mut self) {
        self.cmd.clear();
    }

    pub fn has_cmd(&self) -> bool {
        self.cmd.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cmd(&mut self, v: ::std::string::String) {
        self.cmd = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cmd<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.cmd.is_none() {
            self.cmd.set_default();
        };
        self.cmd.as_mut().unwrap()
    }

    // Take field
    pub fn take_cmd(&mut self) -> ::std::string::String {
        self.cmd.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_cmd<'a>(&'a self) -> &'a str {
        match self.cmd.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional .Value value = 2;

    pub fn clear_value(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_value(&self) -> bool {
        match self.result {
            ::std::option::Option::Some(ExecCommandResult_oneof_result::value(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: Value) {
        self.result = ::std::option::Option::Some(ExecCommandResult_oneof_result::value(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value<'a>(&'a mut self) -> &'a mut Value {
        if let ::std::option::Option::Some(ExecCommandResult_oneof_result::value(_)) = self.result {
        } else {
            self.result = ::std::option::Option::Some(ExecCommandResult_oneof_result::value(Value::new()));
        }
        match self.result {
            ::std::option::Option::Some(ExecCommandResult_oneof_result::value(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_value(&mut self) -> Value {
        if self.has_value() {
            match self.result.take() {
                ::std::option::Option::Some(ExecCommandResult_oneof_result::value(v)) => v,
                _ => panic!(),
            }
        } else {
            Value::new()
        }
    }

    pub fn get_value<'a>(&'a self) -> &'a Value {
        match self.result {
            ::std::option::Option::Some(ExecCommandResult_oneof_result::value(ref v)) => v,
            _ => Value::default_instance(),
        }
    }

    // optional .Error error = 3;

    pub fn clear_error(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_error(&self) -> bool {
        match self.result {
            ::std::option::Option::Some(ExecCommandResult_oneof_result::error(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: Error) {
        self.result = ::std::option::Option::Some(ExecCommandResult_oneof_result::error(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error<'a>(&'a mut self) -> &'a mut Error {
        if let ::std::option::Option::Some(ExecCommandResult_oneof_result::error(_)) = self.result {
        } else {
            self.result = ::std::option::Option::Some(ExecCommandResult_oneof_result::error(Error::new()));
        }
        match self.result {
            ::std::option::Option::Some(ExecCommandResult_oneof_result::error(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_error(&mut self) -> Error {
        if self.has_error() {
            match self.result.take() {
                ::std::option::Option::Some(ExecCommandResult_oneof_result::error(v)) => v,
                _ => panic!(),
            }
        } else {
            Error::new()
        }
    }

    pub fn get_error<'a>(&'a self) -> &'a Error {
        match self.result {
            ::std::option::Option::Some(ExecCommandResult_oneof_result::error(ref v)) => v,
            _ => Error::default_instance(),
        }
    }
}

impl ::protobuf::Message for ExecCommandResult {
    fn is_initialized(&self) -> bool {
        if self.cmd.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.cmd.set_default();
                    try!(is.read_string_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.result = ::std::option::Option::Some(ExecCommandResult_oneof_result::value(try!(is.read_message())));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.result = ::std::option::Option::Some(ExecCommandResult_oneof_result::error(try!(is.read_message())));
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.cmd.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        if let ::std::option::Option::Some(ref v) = self.result {
            match v {
                &ExecCommandResult_oneof_result::value(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ExecCommandResult_oneof_result::error(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.cmd.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let ::std::option::Option::Some(ref v) = self.result {
            match v {
                &ExecCommandResult_oneof_result::value(ref v) => {
                    try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &ExecCommandResult_oneof_result::error(ref v) => {
                    try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
            };
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ExecCommandResult>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ExecCommandResult {
    fn new() -> ExecCommandResult {
        ExecCommandResult::new()
    }

    fn descriptor_static(_: ::std::option::Option<ExecCommandResult>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "cmd",
                    ExecCommandResult::has_cmd,
                    ExecCommandResult::get_cmd,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "value",
                    ExecCommandResult::has_value,
                    ExecCommandResult::get_value,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "error",
                    ExecCommandResult::has_error,
                    ExecCommandResult::get_error,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ExecCommandResult>(
                    "ExecCommandResult",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ExecCommandResult {
    fn clear(&mut self) {
        self.clear_cmd();
        self.clear_value();
        self.clear_error();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ExecCommandResult {
    fn eq(&self, other: &ExecCommandResult) -> bool {
        self.cmd == other.cmd &&
        self.result == other.result &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ExecCommandResult {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ReadAttributeResult {
    // message fields
    attr: ::protobuf::SingularField<::std::string::String>,
    // message oneof groups
    result: ::std::option::Option<ReadAttributeResult_oneof_result>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

#[derive(Clone,PartialEq)]
pub enum ReadAttributeResult_oneof_result {
    value(Value),
    error(Error),
}

impl ReadAttributeResult {
    pub fn new() -> ReadAttributeResult {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ReadAttributeResult {
        static mut instance: ::protobuf::lazy::Lazy<ReadAttributeResult> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ReadAttributeResult,
        };
        unsafe {
            instance.get(|| {
                ReadAttributeResult {
                    attr: ::protobuf::SingularField::none(),
                    result: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string attr = 1;

    pub fn clear_attr(&mut self) {
        self.attr.clear();
    }

    pub fn has_attr(&self) -> bool {
        self.attr.is_some()
    }

    // Param is passed by value, moved
    pub fn set_attr(&mut self, v: ::std::string::String) {
        self.attr = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_attr<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.attr.is_none() {
            self.attr.set_default();
        };
        self.attr.as_mut().unwrap()
    }

    // Take field
    pub fn take_attr(&mut self) -> ::std::string::String {
        self.attr.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_attr<'a>(&'a self) -> &'a str {
        match self.attr.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional .Value value = 2;

    pub fn clear_value(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_value(&self) -> bool {
        match self.result {
            ::std::option::Option::Some(ReadAttributeResult_oneof_result::value(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: Value) {
        self.result = ::std::option::Option::Some(ReadAttributeResult_oneof_result::value(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value<'a>(&'a mut self) -> &'a mut Value {
        if let ::std::option::Option::Some(ReadAttributeResult_oneof_result::value(_)) = self.result {
        } else {
            self.result = ::std::option::Option::Some(ReadAttributeResult_oneof_result::value(Value::new()));
        }
        match self.result {
            ::std::option::Option::Some(ReadAttributeResult_oneof_result::value(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_value(&mut self) -> Value {
        if self.has_value() {
            match self.result.take() {
                ::std::option::Option::Some(ReadAttributeResult_oneof_result::value(v)) => v,
                _ => panic!(),
            }
        } else {
            Value::new()
        }
    }

    pub fn get_value<'a>(&'a self) -> &'a Value {
        match self.result {
            ::std::option::Option::Some(ReadAttributeResult_oneof_result::value(ref v)) => v,
            _ => Value::default_instance(),
        }
    }

    // optional .Error error = 3;

    pub fn clear_error(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_error(&self) -> bool {
        match self.result {
            ::std::option::Option::Some(ReadAttributeResult_oneof_result::error(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: Error) {
        self.result = ::std::option::Option::Some(ReadAttributeResult_oneof_result::error(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error<'a>(&'a mut self) -> &'a mut Error {
        if let ::std::option::Option::Some(ReadAttributeResult_oneof_result::error(_)) = self.result {
        } else {
            self.result = ::std::option::Option::Some(ReadAttributeResult_oneof_result::error(Error::new()));
        }
        match self.result {
            ::std::option::Option::Some(ReadAttributeResult_oneof_result::error(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_error(&mut self) -> Error {
        if self.has_error() {
            match self.result.take() {
                ::std::option::Option::Some(ReadAttributeResult_oneof_result::error(v)) => v,
                _ => panic!(),
            }
        } else {
            Error::new()
        }
    }

    pub fn get_error<'a>(&'a self) -> &'a Error {
        match self.result {
            ::std::option::Option::Some(ReadAttributeResult_oneof_result::error(ref v)) => v,
            _ => Error::default_instance(),
        }
    }
}

impl ::protobuf::Message for ReadAttributeResult {
    fn is_initialized(&self) -> bool {
        if self.attr.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.attr.set_default();
                    try!(is.read_string_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.result = ::std::option::Option::Some(ReadAttributeResult_oneof_result::value(try!(is.read_message())));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.result = ::std::option::Option::Some(ReadAttributeResult_oneof_result::error(try!(is.read_message())));
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.attr.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        if let ::std::option::Option::Some(ref v) = self.result {
            match v {
                &ReadAttributeResult_oneof_result::value(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ReadAttributeResult_oneof_result::error(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.attr.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let ::std::option::Option::Some(ref v) = self.result {
            match v {
                &ReadAttributeResult_oneof_result::value(ref v) => {
                    try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &ReadAttributeResult_oneof_result::error(ref v) => {
                    try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
            };
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ReadAttributeResult>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ReadAttributeResult {
    fn new() -> ReadAttributeResult {
        ReadAttributeResult::new()
    }

    fn descriptor_static(_: ::std::option::Option<ReadAttributeResult>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "attr",
                    ReadAttributeResult::has_attr,
                    ReadAttributeResult::get_attr,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "value",
                    ReadAttributeResult::has_value,
                    ReadAttributeResult::get_value,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "error",
                    ReadAttributeResult::has_error,
                    ReadAttributeResult::get_error,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ReadAttributeResult>(
                    "ReadAttributeResult",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ReadAttributeResult {
    fn clear(&mut self) {
        self.clear_attr();
        self.clear_value();
        self.clear_error();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ReadAttributeResult {
    fn eq(&self, other: &ReadAttributeResult) -> bool {
        self.attr == other.attr &&
        self.result == other.result &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ReadAttributeResult {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct WriteAttributeResult {
    // message fields
    attr: ::protobuf::SingularField<::std::string::String>,
    error: ::protobuf::SingularPtrField<Error>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl WriteAttributeResult {
    pub fn new() -> WriteAttributeResult {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static WriteAttributeResult {
        static mut instance: ::protobuf::lazy::Lazy<WriteAttributeResult> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const WriteAttributeResult,
        };
        unsafe {
            instance.get(|| {
                WriteAttributeResult {
                    attr: ::protobuf::SingularField::none(),
                    error: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string attr = 1;

    pub fn clear_attr(&mut self) {
        self.attr.clear();
    }

    pub fn has_attr(&self) -> bool {
        self.attr.is_some()
    }

    // Param is passed by value, moved
    pub fn set_attr(&mut self, v: ::std::string::String) {
        self.attr = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_attr<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.attr.is_none() {
            self.attr.set_default();
        };
        self.attr.as_mut().unwrap()
    }

    // Take field
    pub fn take_attr(&mut self) -> ::std::string::String {
        self.attr.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_attr<'a>(&'a self) -> &'a str {
        match self.attr.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional .Error error = 2;

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
    pub fn mut_error<'a>(&'a mut self) -> &'a mut Error {
        if self.error.is_none() {
            self.error.set_default();
        };
        self.error.as_mut().unwrap()
    }

    // Take field
    pub fn take_error(&mut self) -> Error {
        self.error.take().unwrap_or_else(|| Error::new())
    }

    pub fn get_error<'a>(&'a self) -> &'a Error {
        self.error.as_ref().unwrap_or_else(|| Error::default_instance())
    }
}

impl ::protobuf::Message for WriteAttributeResult {
    fn is_initialized(&self) -> bool {
        if self.attr.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.attr.set_default();
                    try!(is.read_string_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.error.set_default();
                    try!(is.merge_message(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.attr.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in self.error.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.attr.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.error.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<WriteAttributeResult>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for WriteAttributeResult {
    fn new() -> WriteAttributeResult {
        WriteAttributeResult::new()
    }

    fn descriptor_static(_: ::std::option::Option<WriteAttributeResult>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "attr",
                    WriteAttributeResult::has_attr,
                    WriteAttributeResult::get_attr,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "error",
                    WriteAttributeResult::has_error,
                    WriteAttributeResult::get_error,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<WriteAttributeResult>(
                    "WriteAttributeResult",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for WriteAttributeResult {
    fn clear(&mut self) {
        self.clear_attr();
        self.clear_error();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for WriteAttributeResult {
    fn eq(&self, other: &WriteAttributeResult) -> bool {
        self.attr == other.attr &&
        self.error == other.error &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for WriteAttributeResult {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CommandInfo {
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

impl CommandInfo {
    pub fn new() -> CommandInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CommandInfo {
        static mut instance: ::protobuf::lazy::Lazy<CommandInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CommandInfo,
        };
        unsafe {
            instance.get(|| {
                CommandInfo {
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
    pub fn mut_name<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name<'a>(&'a self) -> &'a str {
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
    pub fn mut_doc<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.doc.is_none() {
            self.doc.set_default();
        };
        self.doc.as_mut().unwrap()
    }

    // Take field
    pub fn take_doc(&mut self) -> ::std::string::String {
        self.doc.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_doc<'a>(&'a self) -> &'a str {
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

    pub fn get_intype<'a>(&self) -> DataType {
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

    pub fn get_outtype<'a>(&self) -> DataType {
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
    pub fn mut_indoc<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.indoc.is_none() {
            self.indoc.set_default();
        };
        self.indoc.as_mut().unwrap()
    }

    // Take field
    pub fn take_indoc(&mut self) -> ::std::string::String {
        self.indoc.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_indoc<'a>(&'a self) -> &'a str {
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
    pub fn mut_outdoc<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.outdoc.is_none() {
            self.outdoc.set_default();
        };
        self.outdoc.as_mut().unwrap()
    }

    // Take field
    pub fn take_outdoc(&mut self) -> ::std::string::String {
        self.outdoc.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_outdoc<'a>(&'a self) -> &'a str {
        match self.outdoc.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for CommandInfo {
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
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.name.set_default();
                    try!(is.read_string_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.doc.set_default();
                    try!(is.read_string_into(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_enum());
                    self.intype = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_enum());
                    self.outtype = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.indoc.set_default();
                    try!(is.read_string_into(tmp))
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.outdoc.set_default();
                    try!(is.read_string_into(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
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
            try!(os.write_enum(3, v as i32));
        };
        if let Some(v) = self.outtype {
            try!(os.write_enum(4, v as i32));
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

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<CommandInfo>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CommandInfo {
    fn new() -> CommandInfo {
        CommandInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<CommandInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "name",
                    CommandInfo::has_name,
                    CommandInfo::get_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "doc",
                    CommandInfo::has_doc,
                    CommandInfo::get_doc,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "intype",
                    CommandInfo::has_intype,
                    CommandInfo::get_intype,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "outtype",
                    CommandInfo::has_outtype,
                    CommandInfo::get_outtype,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "indoc",
                    CommandInfo::has_indoc,
                    CommandInfo::get_indoc,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "outdoc",
                    CommandInfo::has_outdoc,
                    CommandInfo::get_outdoc,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CommandInfo>(
                    "CommandInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CommandInfo {
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

impl ::std::cmp::PartialEq for CommandInfo {
    fn eq(&self, other: &CommandInfo) -> bool {
        self.name == other.name &&
        self.doc == other.doc &&
        self.intype == other.intype &&
        self.outtype == other.outtype &&
        self.indoc == other.indoc &&
        self.outdoc == other.outdoc &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CommandInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct AttributeInfo {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    doc: ::protobuf::SingularField<::std::string::String>,
    field_type: ::std::option::Option<DataType>,
    unit: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl AttributeInfo {
    pub fn new() -> AttributeInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AttributeInfo {
        static mut instance: ::protobuf::lazy::Lazy<AttributeInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AttributeInfo,
        };
        unsafe {
            instance.get(|| {
                AttributeInfo {
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
    pub fn mut_name<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name<'a>(&'a self) -> &'a str {
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
    pub fn mut_doc<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.doc.is_none() {
            self.doc.set_default();
        };
        self.doc.as_mut().unwrap()
    }

    // Take field
    pub fn take_doc(&mut self) -> ::std::string::String {
        self.doc.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_doc<'a>(&'a self) -> &'a str {
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

    pub fn get_field_type<'a>(&self) -> DataType {
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
    pub fn mut_unit<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.unit.is_none() {
            self.unit.set_default();
        };
        self.unit.as_mut().unwrap()
    }

    // Take field
    pub fn take_unit(&mut self) -> ::std::string::String {
        self.unit.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_unit<'a>(&'a self) -> &'a str {
        match self.unit.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for AttributeInfo {
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
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.name.set_default();
                    try!(is.read_string_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.doc.set_default();
                    try!(is.read_string_into(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_enum());
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.unit.set_default();
                    try!(is.read_string_into(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
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
            try!(os.write_enum(3, v as i32));
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

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<AttributeInfo>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AttributeInfo {
    fn new() -> AttributeInfo {
        AttributeInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<AttributeInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "name",
                    AttributeInfo::has_name,
                    AttributeInfo::get_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "doc",
                    AttributeInfo::has_doc,
                    AttributeInfo::get_doc,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "field_type",
                    AttributeInfo::has_field_type,
                    AttributeInfo::get_field_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "unit",
                    AttributeInfo::has_unit,
                    AttributeInfo::get_unit,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AttributeInfo>(
                    "AttributeInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AttributeInfo {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_doc();
        self.clear_field_type();
        self.clear_unit();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for AttributeInfo {
    fn eq(&self, other: &AttributeInfo) -> bool {
        self.name == other.name &&
        self.doc == other.doc &&
        self.field_type == other.field_type &&
        self.unit == other.unit &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for AttributeInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct QueryAPIResult {
    // message fields
    cmd: ::protobuf::RepeatedField<CommandInfo>,
    attr: ::protobuf::RepeatedField<AttributeInfo>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl QueryAPIResult {
    pub fn new() -> QueryAPIResult {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static QueryAPIResult {
        static mut instance: ::protobuf::lazy::Lazy<QueryAPIResult> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const QueryAPIResult,
        };
        unsafe {
            instance.get(|| {
                QueryAPIResult {
                    cmd: ::protobuf::RepeatedField::new(),
                    attr: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .CommandInfo cmd = 1;

    pub fn clear_cmd(&mut self) {
        self.cmd.clear();
    }

    // Param is passed by value, moved
    pub fn set_cmd(&mut self, v: ::protobuf::RepeatedField<CommandInfo>) {
        self.cmd = v;
    }

    // Mutable pointer to the field.
    pub fn mut_cmd<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<CommandInfo> {
        &mut self.cmd
    }

    // Take field
    pub fn take_cmd(&mut self) -> ::protobuf::RepeatedField<CommandInfo> {
        ::std::mem::replace(&mut self.cmd, ::protobuf::RepeatedField::new())
    }

    pub fn get_cmd<'a>(&'a self) -> &'a [CommandInfo] {
        &self.cmd
    }

    // repeated .AttributeInfo attr = 2;

    pub fn clear_attr(&mut self) {
        self.attr.clear();
    }

    // Param is passed by value, moved
    pub fn set_attr(&mut self, v: ::protobuf::RepeatedField<AttributeInfo>) {
        self.attr = v;
    }

    // Mutable pointer to the field.
    pub fn mut_attr<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<AttributeInfo> {
        &mut self.attr
    }

    // Take field
    pub fn take_attr(&mut self) -> ::protobuf::RepeatedField<AttributeInfo> {
        ::std::mem::replace(&mut self.attr, ::protobuf::RepeatedField::new())
    }

    pub fn get_attr<'a>(&'a self) -> &'a [AttributeInfo] {
        &self.attr
    }
}

impl ::protobuf::Message for QueryAPIResult {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.cmd));
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.attr));
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.cmd.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.attr.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in self.cmd.iter() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in self.attr.iter() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<QueryAPIResult>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for QueryAPIResult {
    fn new() -> QueryAPIResult {
        QueryAPIResult::new()
    }

    fn descriptor_static(_: ::std::option::Option<QueryAPIResult>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "cmd",
                    QueryAPIResult::get_cmd,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "attr",
                    QueryAPIResult::get_attr,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<QueryAPIResult>(
                    "QueryAPIResult",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for QueryAPIResult {
    fn clear(&mut self) {
        self.clear_cmd();
        self.clear_attr();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for QueryAPIResult {
    fn eq(&self, other: &QueryAPIResult) -> bool {
        self.cmd == other.cmd &&
        self.attr == other.attr &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for QueryAPIResult {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Response {
    // message fields
    seqno: ::std::option::Option<u32>,
    // message oneof groups
    response: ::std::option::Option<Response_oneof_response>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

#[derive(Clone,PartialEq)]
pub enum Response_oneof_response {
    exec(ExecCommandResult),
    rattr(ReadAttributeResult),
    wattr(WriteAttributeResult),
    qapi(QueryAPIResult),
}

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
                    response: ::std::option::Option::None,
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

    pub fn get_seqno<'a>(&self) -> u32 {
        self.seqno.unwrap_or(0)
    }

    // optional .ExecCommandResult exec = 2;

    pub fn clear_exec(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_exec(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(Response_oneof_response::exec(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_exec(&mut self, v: ExecCommandResult) {
        self.response = ::std::option::Option::Some(Response_oneof_response::exec(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_exec<'a>(&'a mut self) -> &'a mut ExecCommandResult {
        if let ::std::option::Option::Some(Response_oneof_response::exec(_)) = self.response {
        } else {
            self.response = ::std::option::Option::Some(Response_oneof_response::exec(ExecCommandResult::new()));
        }
        match self.response {
            ::std::option::Option::Some(Response_oneof_response::exec(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_exec(&mut self) -> ExecCommandResult {
        if self.has_exec() {
            match self.response.take() {
                ::std::option::Option::Some(Response_oneof_response::exec(v)) => v,
                _ => panic!(),
            }
        } else {
            ExecCommandResult::new()
        }
    }

    pub fn get_exec<'a>(&'a self) -> &'a ExecCommandResult {
        match self.response {
            ::std::option::Option::Some(Response_oneof_response::exec(ref v)) => v,
            _ => ExecCommandResult::default_instance(),
        }
    }

    // optional .ReadAttributeResult rattr = 3;

    pub fn clear_rattr(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_rattr(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(Response_oneof_response::rattr(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_rattr(&mut self, v: ReadAttributeResult) {
        self.response = ::std::option::Option::Some(Response_oneof_response::rattr(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_rattr<'a>(&'a mut self) -> &'a mut ReadAttributeResult {
        if let ::std::option::Option::Some(Response_oneof_response::rattr(_)) = self.response {
        } else {
            self.response = ::std::option::Option::Some(Response_oneof_response::rattr(ReadAttributeResult::new()));
        }
        match self.response {
            ::std::option::Option::Some(Response_oneof_response::rattr(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_rattr(&mut self) -> ReadAttributeResult {
        if self.has_rattr() {
            match self.response.take() {
                ::std::option::Option::Some(Response_oneof_response::rattr(v)) => v,
                _ => panic!(),
            }
        } else {
            ReadAttributeResult::new()
        }
    }

    pub fn get_rattr<'a>(&'a self) -> &'a ReadAttributeResult {
        match self.response {
            ::std::option::Option::Some(Response_oneof_response::rattr(ref v)) => v,
            _ => ReadAttributeResult::default_instance(),
        }
    }

    // optional .WriteAttributeResult wattr = 4;

    pub fn clear_wattr(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_wattr(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(Response_oneof_response::wattr(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_wattr(&mut self, v: WriteAttributeResult) {
        self.response = ::std::option::Option::Some(Response_oneof_response::wattr(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_wattr<'a>(&'a mut self) -> &'a mut WriteAttributeResult {
        if let ::std::option::Option::Some(Response_oneof_response::wattr(_)) = self.response {
        } else {
            self.response = ::std::option::Option::Some(Response_oneof_response::wattr(WriteAttributeResult::new()));
        }
        match self.response {
            ::std::option::Option::Some(Response_oneof_response::wattr(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_wattr(&mut self) -> WriteAttributeResult {
        if self.has_wattr() {
            match self.response.take() {
                ::std::option::Option::Some(Response_oneof_response::wattr(v)) => v,
                _ => panic!(),
            }
        } else {
            WriteAttributeResult::new()
        }
    }

    pub fn get_wattr<'a>(&'a self) -> &'a WriteAttributeResult {
        match self.response {
            ::std::option::Option::Some(Response_oneof_response::wattr(ref v)) => v,
            _ => WriteAttributeResult::default_instance(),
        }
    }

    // optional .QueryAPIResult qapi = 6;

    pub fn clear_qapi(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_qapi(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(Response_oneof_response::qapi(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_qapi(&mut self, v: QueryAPIResult) {
        self.response = ::std::option::Option::Some(Response_oneof_response::qapi(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_qapi<'a>(&'a mut self) -> &'a mut QueryAPIResult {
        if let ::std::option::Option::Some(Response_oneof_response::qapi(_)) = self.response {
        } else {
            self.response = ::std::option::Option::Some(Response_oneof_response::qapi(QueryAPIResult::new()));
        }
        match self.response {
            ::std::option::Option::Some(Response_oneof_response::qapi(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_qapi(&mut self) -> QueryAPIResult {
        if self.has_qapi() {
            match self.response.take() {
                ::std::option::Option::Some(Response_oneof_response::qapi(v)) => v,
                _ => panic!(),
            }
        } else {
            QueryAPIResult::new()
        }
    }

    pub fn get_qapi<'a>(&'a self) -> &'a QueryAPIResult {
        match self.response {
            ::std::option::Option::Some(Response_oneof_response::qapi(ref v)) => v,
            _ => QueryAPIResult::default_instance(),
        }
    }
}

impl ::protobuf::Message for Response {
    fn is_initialized(&self) -> bool {
        if self.seqno.is_none() {
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
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.seqno = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.response = ::std::option::Option::Some(Response_oneof_response::exec(try!(is.read_message())));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.response = ::std::option::Option::Some(Response_oneof_response::rattr(try!(is.read_message())));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.response = ::std::option::Option::Some(Response_oneof_response::wattr(try!(is.read_message())));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.response = ::std::option::Option::Some(Response_oneof_response::qapi(try!(is.read_message())));
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
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
        if let ::std::option::Option::Some(ref v) = self.response {
            match v {
                &Response_oneof_response::exec(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Response_oneof_response::rattr(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Response_oneof_response::wattr(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Response_oneof_response::qapi(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.seqno {
            try!(os.write_uint32(1, v));
        };
        if let ::std::option::Option::Some(ref v) = self.response {
            match v {
                &Response_oneof_response::exec(ref v) => {
                    try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &Response_oneof_response::rattr(ref v) => {
                    try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &Response_oneof_response::wattr(ref v) => {
                    try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &Response_oneof_response::qapi(ref v) => {
                    try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
            };
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
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
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "exec",
                    Response::has_exec,
                    Response::get_exec,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "rattr",
                    Response::has_rattr,
                    Response::get_rattr,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "wattr",
                    Response::has_wattr,
                    Response::get_wattr,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "qapi",
                    Response::has_qapi,
                    Response::get_qapi,
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
        self.clear_exec();
        self.clear_rattr();
        self.clear_wattr();
        self.clear_qapi();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Response {
    fn eq(&self, other: &Response) -> bool {
        self.seqno == other.seqno &&
        self.response == other.response &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Response {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug)]
pub enum DataType {
    Void = 0,
    Bool = 2,
    Double = 3,
    Float = 4,
    Int32 = 5,
    Int64 = 6,
    UInt32 = 7,
    UInt64 = 8,
    String = 9,
    BoolArray = 11,
    DoubleArray = 12,
    FloatArray = 13,
    Int32Array = 14,
    Int64Array = 15,
    UInt32Array = 16,
    UInt64Array = 17,
    StringArray = 18,
    Int32StringArray = 21,
    DoubleStringArray = 22,
}

impl ::protobuf::ProtobufEnum for DataType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<DataType> {
        match value {
            0 => ::std::option::Option::Some(DataType::Void),
            2 => ::std::option::Option::Some(DataType::Bool),
            3 => ::std::option::Option::Some(DataType::Double),
            4 => ::std::option::Option::Some(DataType::Float),
            5 => ::std::option::Option::Some(DataType::Int32),
            6 => ::std::option::Option::Some(DataType::Int64),
            7 => ::std::option::Option::Some(DataType::UInt32),
            8 => ::std::option::Option::Some(DataType::UInt64),
            9 => ::std::option::Option::Some(DataType::String),
            11 => ::std::option::Option::Some(DataType::BoolArray),
            12 => ::std::option::Option::Some(DataType::DoubleArray),
            13 => ::std::option::Option::Some(DataType::FloatArray),
            14 => ::std::option::Option::Some(DataType::Int32Array),
            15 => ::std::option::Option::Some(DataType::Int64Array),
            16 => ::std::option::Option::Some(DataType::UInt32Array),
            17 => ::std::option::Option::Some(DataType::UInt64Array),
            18 => ::std::option::Option::Some(DataType::StringArray),
            21 => ::std::option::Option::Some(DataType::Int32StringArray),
            22 => ::std::option::Option::Some(DataType::DoubleStringArray),
            _ => ::std::option::Option::None
        }
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

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x09, 0x6d, 0x73, 0x67, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x1b, 0x0a, 0x07, 0x42,
    0x6f, 0x6f, 0x6c, 0x41, 0x72, 0x72, 0x12, 0x10, 0x0a, 0x04, 0x62, 0x6f, 0x6f, 0x6c, 0x18, 0x01,
    0x20, 0x03, 0x28, 0x08, 0x42, 0x02, 0x10, 0x01, 0x22, 0x1f, 0x0a, 0x09, 0x44, 0x6f, 0x75, 0x62,
    0x6c, 0x65, 0x41, 0x72, 0x72, 0x12, 0x12, 0x0a, 0x06, 0x64, 0x6f, 0x75, 0x62, 0x6c, 0x65, 0x18,
    0x02, 0x20, 0x03, 0x28, 0x01, 0x42, 0x02, 0x10, 0x01, 0x22, 0x1d, 0x0a, 0x08, 0x46, 0x6c, 0x6f,
    0x61, 0x74, 0x41, 0x72, 0x72, 0x12, 0x11, 0x0a, 0x05, 0x66, 0x6c, 0x6f, 0x61, 0x74, 0x18, 0x03,
    0x20, 0x03, 0x28, 0x02, 0x42, 0x02, 0x10, 0x01, 0x22, 0x1d, 0x0a, 0x08, 0x49, 0x6e, 0x74, 0x33,
    0x32, 0x41, 0x72, 0x72, 0x12, 0x11, 0x0a, 0x05, 0x69, 0x6e, 0x74, 0x33, 0x32, 0x18, 0x04, 0x20,
    0x03, 0x28, 0x11, 0x42, 0x02, 0x10, 0x01, 0x22, 0x1d, 0x0a, 0x08, 0x49, 0x6e, 0x74, 0x36, 0x34,
    0x41, 0x72, 0x72, 0x12, 0x11, 0x0a, 0x05, 0x69, 0x6e, 0x74, 0x36, 0x34, 0x18, 0x05, 0x20, 0x03,
    0x28, 0x12, 0x42, 0x02, 0x10, 0x01, 0x22, 0x1f, 0x0a, 0x09, 0x55, 0x49, 0x6e, 0x74, 0x33, 0x32,
    0x41, 0x72, 0x72, 0x12, 0x12, 0x0a, 0x06, 0x75, 0x69, 0x6e, 0x74, 0x33, 0x32, 0x18, 0x06, 0x20,
    0x03, 0x28, 0x0d, 0x42, 0x02, 0x10, 0x01, 0x22, 0x1f, 0x0a, 0x09, 0x55, 0x49, 0x6e, 0x74, 0x36,
    0x34, 0x41, 0x72, 0x72, 0x12, 0x12, 0x0a, 0x06, 0x75, 0x69, 0x6e, 0x74, 0x36, 0x34, 0x18, 0x07,
    0x20, 0x03, 0x28, 0x04, 0x42, 0x02, 0x10, 0x01, 0x22, 0x1b, 0x0a, 0x09, 0x53, 0x74, 0x72, 0x69,
    0x6e, 0x67, 0x41, 0x72, 0x72, 0x12, 0x0e, 0x0a, 0x06, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x18,
    0x08, 0x20, 0x03, 0x28, 0x09, 0x22, 0x33, 0x0a, 0x0e, 0x49, 0x6e, 0x74, 0x33, 0x32, 0x53, 0x74,
    0x72, 0x69, 0x6e, 0x67, 0x41, 0x72, 0x72, 0x12, 0x11, 0x0a, 0x05, 0x69, 0x6e, 0x74, 0x33, 0x32,
    0x18, 0x01, 0x20, 0x03, 0x28, 0x11, 0x42, 0x02, 0x10, 0x01, 0x12, 0x0e, 0x0a, 0x06, 0x73, 0x74,
    0x72, 0x69, 0x6e, 0x67, 0x18, 0x02, 0x20, 0x03, 0x28, 0x09, 0x22, 0x35, 0x0a, 0x0f, 0x44, 0x6f,
    0x75, 0x62, 0x6c, 0x65, 0x53, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x41, 0x72, 0x72, 0x12, 0x12, 0x0a,
    0x06, 0x64, 0x6f, 0x75, 0x62, 0x6c, 0x65, 0x18, 0x01, 0x20, 0x03, 0x28, 0x01, 0x42, 0x02, 0x10,
    0x01, 0x12, 0x0e, 0x0a, 0x06, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x18, 0x02, 0x20, 0x03, 0x28,
    0x09, 0x22, 0x8b, 0x04, 0x0a, 0x05, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x12, 0x0e, 0x0a, 0x04, 0x76,
    0x6f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x08, 0x48, 0x00, 0x12, 0x0e, 0x0a, 0x04, 0x62,
    0x6f, 0x6f, 0x6c, 0x18, 0x02, 0x20, 0x01, 0x28, 0x08, 0x48, 0x00, 0x12, 0x10, 0x0a, 0x06, 0x64,
    0x6f, 0x75, 0x62, 0x6c, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x01, 0x48, 0x00, 0x12, 0x0f, 0x0a,
    0x05, 0x66, 0x6c, 0x6f, 0x61, 0x74, 0x18, 0x04, 0x20, 0x01, 0x28, 0x02, 0x48, 0x00, 0x12, 0x0f,
    0x0a, 0x05, 0x69, 0x6e, 0x74, 0x33, 0x32, 0x18, 0x05, 0x20, 0x01, 0x28, 0x11, 0x48, 0x00, 0x12,
    0x0f, 0x0a, 0x05, 0x69, 0x6e, 0x74, 0x36, 0x34, 0x18, 0x06, 0x20, 0x01, 0x28, 0x12, 0x48, 0x00,
    0x12, 0x10, 0x0a, 0x06, 0x75, 0x69, 0x6e, 0x74, 0x33, 0x32, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0d,
    0x48, 0x00, 0x12, 0x10, 0x0a, 0x06, 0x75, 0x69, 0x6e, 0x74, 0x36, 0x34, 0x18, 0x08, 0x20, 0x01,
    0x28, 0x04, 0x48, 0x00, 0x12, 0x10, 0x0a, 0x06, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x18, 0x09,
    0x20, 0x01, 0x28, 0x09, 0x48, 0x00, 0x12, 0x1c, 0x0a, 0x08, 0x62, 0x6f, 0x6f, 0x6c, 0x5f, 0x61,
    0x72, 0x72, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x08, 0x2e, 0x42, 0x6f, 0x6f, 0x6c, 0x41,
    0x72, 0x72, 0x48, 0x00, 0x12, 0x20, 0x0a, 0x0a, 0x64, 0x6f, 0x75, 0x62, 0x6c, 0x65, 0x5f, 0x61,
    0x72, 0x72, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0a, 0x2e, 0x44, 0x6f, 0x75, 0x62, 0x6c,
    0x65, 0x41, 0x72, 0x72, 0x48, 0x00, 0x12, 0x1e, 0x0a, 0x09, 0x66, 0x6c, 0x6f, 0x61, 0x74, 0x5f,
    0x61, 0x72, 0x72, 0x18, 0x0d, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x09, 0x2e, 0x46, 0x6c, 0x6f, 0x61,
    0x74, 0x41, 0x72, 0x72, 0x48, 0x00, 0x12, 0x1e, 0x0a, 0x09, 0x69, 0x6e, 0x74, 0x33, 0x32, 0x5f,
    0x61, 0x72, 0x72, 0x18, 0x0e, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x09, 0x2e, 0x49, 0x6e, 0x74, 0x33,
    0x32, 0x41, 0x72, 0x72, 0x48, 0x00, 0x12, 0x1e, 0x0a, 0x09, 0x69, 0x6e, 0x74, 0x36, 0x34, 0x5f,
    0x61, 0x72, 0x72, 0x18, 0x0f, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x09, 0x2e, 0x49, 0x6e, 0x74, 0x36,
    0x34, 0x41, 0x72, 0x72, 0x48, 0x00, 0x12, 0x20, 0x0a, 0x0a, 0x75, 0x69, 0x6e, 0x74, 0x33, 0x32,
    0x5f, 0x61, 0x72, 0x72, 0x18, 0x10, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0a, 0x2e, 0x55, 0x49, 0x6e,
    0x74, 0x33, 0x32, 0x41, 0x72, 0x72, 0x48, 0x00, 0x12, 0x20, 0x0a, 0x0a, 0x75, 0x69, 0x6e, 0x74,
    0x36, 0x34, 0x5f, 0x61, 0x72, 0x72, 0x18, 0x11, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0a, 0x2e, 0x55,
    0x49, 0x6e, 0x74, 0x36, 0x34, 0x41, 0x72, 0x72, 0x48, 0x00, 0x12, 0x20, 0x0a, 0x0a, 0x73, 0x74,
    0x72, 0x69, 0x6e, 0x67, 0x5f, 0x61, 0x72, 0x72, 0x18, 0x12, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0a,
    0x2e, 0x53, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x41, 0x72, 0x72, 0x48, 0x00, 0x12, 0x2a, 0x0a, 0x0f,
    0x69, 0x6e, 0x74, 0x33, 0x32, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x5f, 0x61, 0x72, 0x72, 0x18,
    0x15, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0f, 0x2e, 0x49, 0x6e, 0x74, 0x33, 0x32, 0x53, 0x74, 0x72,
    0x69, 0x6e, 0x67, 0x41, 0x72, 0x72, 0x48, 0x00, 0x12, 0x2c, 0x0a, 0x10, 0x64, 0x6f, 0x75, 0x62,
    0x6c, 0x65, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x5f, 0x61, 0x72, 0x72, 0x18, 0x16, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x10, 0x2e, 0x44, 0x6f, 0x75, 0x62, 0x6c, 0x65, 0x53, 0x74, 0x72, 0x69, 0x6e,
    0x67, 0x41, 0x72, 0x72, 0x48, 0x00, 0x42, 0x07, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x22,
    0x35, 0x0a, 0x05, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x12, 0x0e, 0x0a, 0x06, 0x72, 0x65, 0x61, 0x73,
    0x6f, 0x6e, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x12, 0x0c, 0x0a, 0x04, 0x64, 0x65, 0x73, 0x63,
    0x18, 0x02, 0x20, 0x02, 0x28, 0x09, 0x12, 0x0e, 0x0a, 0x06, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e,
    0x18, 0x03, 0x20, 0x02, 0x28, 0x09, 0x22, 0x31, 0x0a, 0x0b, 0x45, 0x78, 0x65, 0x63, 0x43, 0x6f,
    0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x12, 0x0b, 0x0a, 0x03, 0x63, 0x6d, 0x64, 0x18, 0x01, 0x20, 0x02,
    0x28, 0x09, 0x12, 0x15, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x06, 0x2e, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x22, 0x1d, 0x0a, 0x0d, 0x52, 0x65, 0x61,
    0x64, 0x41, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x12, 0x0c, 0x0a, 0x04, 0x61, 0x74,
    0x74, 0x72, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x22, 0x35, 0x0a, 0x0e, 0x57, 0x72, 0x69, 0x74,
    0x65, 0x41, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x12, 0x0c, 0x0a, 0x04, 0x61, 0x74,
    0x74, 0x72, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x12, 0x15, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75,
    0x65, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x06, 0x2e, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x22,
    0x0a, 0x0a, 0x08, 0x51, 0x75, 0x65, 0x72, 0x79, 0x41, 0x50, 0x49, 0x22, 0x9f, 0x01, 0x0a, 0x07,
    0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x0d, 0x0a, 0x05, 0x73, 0x65, 0x71, 0x6e, 0x6f,
    0x18, 0x01, 0x20, 0x02, 0x28, 0x0d, 0x12, 0x1c, 0x0a, 0x04, 0x65, 0x78, 0x65, 0x63, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x0c, 0x2e, 0x45, 0x78, 0x65, 0x63, 0x43, 0x6f, 0x6d, 0x6d, 0x61,
    0x6e, 0x64, 0x48, 0x00, 0x12, 0x1f, 0x0a, 0x05, 0x72, 0x61, 0x74, 0x74, 0x72, 0x18, 0x03, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x52, 0x65, 0x61, 0x64, 0x41, 0x74, 0x74, 0x72, 0x69, 0x62,
    0x75, 0x74, 0x65, 0x48, 0x00, 0x12, 0x20, 0x0a, 0x05, 0x77, 0x61, 0x74, 0x74, 0x72, 0x18, 0x04,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x0f, 0x2e, 0x57, 0x72, 0x69, 0x74, 0x65, 0x41, 0x74, 0x74, 0x72,
    0x69, 0x62, 0x75, 0x74, 0x65, 0x48, 0x00, 0x12, 0x19, 0x0a, 0x04, 0x71, 0x61, 0x70, 0x69, 0x18,
    0x06, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x09, 0x2e, 0x51, 0x75, 0x65, 0x72, 0x79, 0x41, 0x50, 0x49,
    0x48, 0x00, 0x42, 0x09, 0x0a, 0x07, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x22, 0x5c, 0x0a,
    0x11, 0x45, 0x78, 0x65, 0x63, 0x43, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x52, 0x65, 0x73, 0x75,
    0x6c, 0x74, 0x12, 0x0b, 0x0a, 0x03, 0x63, 0x6d, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x12,
    0x17, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x06,
    0x2e, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x48, 0x00, 0x12, 0x17, 0x0a, 0x05, 0x65, 0x72, 0x72, 0x6f,
    0x72, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x06, 0x2e, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x48,
    0x00, 0x42, 0x08, 0x0a, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x22, 0x5f, 0x0a, 0x13, 0x52,
    0x65, 0x61, 0x64, 0x41, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x52, 0x65, 0x73, 0x75,
    0x6c, 0x74, 0x12, 0x0c, 0x0a, 0x04, 0x61, 0x74, 0x74, 0x72, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09,
    0x12, 0x17, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x06, 0x2e, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x48, 0x00, 0x12, 0x17, 0x0a, 0x05, 0x65, 0x72, 0x72,
    0x6f, 0x72, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x06, 0x2e, 0x45, 0x72, 0x72, 0x6f, 0x72,
    0x48, 0x00, 0x42, 0x08, 0x0a, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x22, 0x3b, 0x0a, 0x14,
    0x57, 0x72, 0x69, 0x74, 0x65, 0x41, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x52, 0x65,
    0x73, 0x75, 0x6c, 0x74, 0x12, 0x0c, 0x0a, 0x04, 0x61, 0x74, 0x74, 0x72, 0x18, 0x01, 0x20, 0x02,
    0x28, 0x09, 0x12, 0x15, 0x0a, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x06, 0x2e, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x22, 0x7e, 0x0a, 0x0b, 0x43, 0x6f, 0x6d,
    0x6d, 0x61, 0x6e, 0x64, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x0c, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65,
    0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x12, 0x0b, 0x0a, 0x03, 0x64, 0x6f, 0x63, 0x18, 0x02, 0x20,
    0x02, 0x28, 0x09, 0x12, 0x19, 0x0a, 0x06, 0x69, 0x6e, 0x74, 0x79, 0x70, 0x65, 0x18, 0x03, 0x20,
    0x02, 0x28, 0x0e, 0x32, 0x09, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x54, 0x79, 0x70, 0x65, 0x12, 0x1a,
    0x0a, 0x07, 0x6f, 0x75, 0x74, 0x74, 0x79, 0x70, 0x65, 0x18, 0x04, 0x20, 0x02, 0x28, 0x0e, 0x32,
    0x09, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x54, 0x79, 0x70, 0x65, 0x12, 0x0d, 0x0a, 0x05, 0x69, 0x6e,
    0x64, 0x6f, 0x63, 0x18, 0x05, 0x20, 0x02, 0x28, 0x09, 0x12, 0x0e, 0x0a, 0x06, 0x6f, 0x75, 0x74,
    0x64, 0x6f, 0x63, 0x18, 0x06, 0x20, 0x02, 0x28, 0x09, 0x22, 0x51, 0x0a, 0x0d, 0x41, 0x74, 0x74,
    0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x0c, 0x0a, 0x04, 0x6e, 0x61,
    0x6d, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x12, 0x0b, 0x0a, 0x03, 0x64, 0x6f, 0x63, 0x18,
    0x02, 0x20, 0x02, 0x28, 0x09, 0x12, 0x17, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x03, 0x20,
    0x02, 0x28, 0x0e, 0x32, 0x09, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x54, 0x79, 0x70, 0x65, 0x12, 0x0c,
    0x0a, 0x04, 0x75, 0x6e, 0x69, 0x74, 0x18, 0x04, 0x20, 0x02, 0x28, 0x09, 0x22, 0x49, 0x0a, 0x0e,
    0x51, 0x75, 0x65, 0x72, 0x79, 0x41, 0x50, 0x49, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x19,
    0x0a, 0x03, 0x63, 0x6d, 0x64, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0c, 0x2e, 0x43, 0x6f,
    0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x1c, 0x0a, 0x04, 0x61, 0x74, 0x74,
    0x72, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x41, 0x74, 0x74, 0x72, 0x69, 0x62,
    0x75, 0x74, 0x65, 0x49, 0x6e, 0x66, 0x6f, 0x22, 0xb9, 0x01, 0x0a, 0x08, 0x52, 0x65, 0x73, 0x70,
    0x6f, 0x6e, 0x73, 0x65, 0x12, 0x0d, 0x0a, 0x05, 0x73, 0x65, 0x71, 0x6e, 0x6f, 0x18, 0x01, 0x20,
    0x02, 0x28, 0x0d, 0x12, 0x22, 0x0a, 0x04, 0x65, 0x78, 0x65, 0x63, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x12, 0x2e, 0x45, 0x78, 0x65, 0x63, 0x43, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x52,
    0x65, 0x73, 0x75, 0x6c, 0x74, 0x48, 0x00, 0x12, 0x25, 0x0a, 0x05, 0x72, 0x61, 0x74, 0x74, 0x72,
    0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x14, 0x2e, 0x52, 0x65, 0x61, 0x64, 0x41, 0x74, 0x74,
    0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x48, 0x00, 0x12, 0x26,
    0x0a, 0x05, 0x77, 0x61, 0x74, 0x74, 0x72, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x15, 0x2e,
    0x57, 0x72, 0x69, 0x74, 0x65, 0x41, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x52, 0x65,
    0x73, 0x75, 0x6c, 0x74, 0x48, 0x00, 0x12, 0x1f, 0x0a, 0x04, 0x71, 0x61, 0x70, 0x69, 0x18, 0x06,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x0f, 0x2e, 0x51, 0x75, 0x65, 0x72, 0x79, 0x41, 0x50, 0x49, 0x52,
    0x65, 0x73, 0x75, 0x6c, 0x74, 0x48, 0x00, 0x42, 0x0a, 0x0a, 0x08, 0x72, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x2a, 0x9f, 0x02, 0x0a, 0x08, 0x44, 0x61, 0x74, 0x61, 0x54, 0x79, 0x70, 0x65,
    0x12, 0x08, 0x0a, 0x04, 0x56, 0x6f, 0x69, 0x64, 0x10, 0x00, 0x12, 0x08, 0x0a, 0x04, 0x42, 0x6f,
    0x6f, 0x6c, 0x10, 0x02, 0x12, 0x0a, 0x0a, 0x06, 0x44, 0x6f, 0x75, 0x62, 0x6c, 0x65, 0x10, 0x03,
    0x12, 0x09, 0x0a, 0x05, 0x46, 0x6c, 0x6f, 0x61, 0x74, 0x10, 0x04, 0x12, 0x09, 0x0a, 0x05, 0x49,
    0x6e, 0x74, 0x33, 0x32, 0x10, 0x05, 0x12, 0x09, 0x0a, 0x05, 0x49, 0x6e, 0x74, 0x36, 0x34, 0x10,
    0x06, 0x12, 0x0a, 0x0a, 0x06, 0x55, 0x49, 0x6e, 0x74, 0x33, 0x32, 0x10, 0x07, 0x12, 0x0a, 0x0a,
    0x06, 0x55, 0x49, 0x6e, 0x74, 0x36, 0x34, 0x10, 0x08, 0x12, 0x0a, 0x0a, 0x06, 0x53, 0x74, 0x72,
    0x69, 0x6e, 0x67, 0x10, 0x09, 0x12, 0x0d, 0x0a, 0x09, 0x42, 0x6f, 0x6f, 0x6c, 0x41, 0x72, 0x72,
    0x61, 0x79, 0x10, 0x0b, 0x12, 0x0f, 0x0a, 0x0b, 0x44, 0x6f, 0x75, 0x62, 0x6c, 0x65, 0x41, 0x72,
    0x72, 0x61, 0x79, 0x10, 0x0c, 0x12, 0x0e, 0x0a, 0x0a, 0x46, 0x6c, 0x6f, 0x61, 0x74, 0x41, 0x72,
    0x72, 0x61, 0x79, 0x10, 0x0d, 0x12, 0x0e, 0x0a, 0x0a, 0x49, 0x6e, 0x74, 0x33, 0x32, 0x41, 0x72,
    0x72, 0x61, 0x79, 0x10, 0x0e, 0x12, 0x0e, 0x0a, 0x0a, 0x49, 0x6e, 0x74, 0x36, 0x34, 0x41, 0x72,
    0x72, 0x61, 0x79, 0x10, 0x0f, 0x12, 0x0f, 0x0a, 0x0b, 0x55, 0x49, 0x6e, 0x74, 0x33, 0x32, 0x41,
    0x72, 0x72, 0x61, 0x79, 0x10, 0x10, 0x12, 0x0f, 0x0a, 0x0b, 0x55, 0x49, 0x6e, 0x74, 0x36, 0x34,
    0x41, 0x72, 0x72, 0x61, 0x79, 0x10, 0x11, 0x12, 0x0f, 0x0a, 0x0b, 0x53, 0x74, 0x72, 0x69, 0x6e,
    0x67, 0x41, 0x72, 0x72, 0x61, 0x79, 0x10, 0x12, 0x12, 0x14, 0x0a, 0x10, 0x49, 0x6e, 0x74, 0x33,
    0x32, 0x53, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x41, 0x72, 0x72, 0x61, 0x79, 0x10, 0x15, 0x12, 0x15,
    0x0a, 0x11, 0x44, 0x6f, 0x75, 0x62, 0x6c, 0x65, 0x53, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x41, 0x72,
    0x72, 0x61, 0x79, 0x10, 0x16, 0x4a, 0xf1, 0x36, 0x0a, 0x07, 0x12, 0x05, 0x05, 0x00, 0xbf, 0x01,
    0x01, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x00, 0x12, 0x04, 0x05, 0x00, 0x19, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x05, 0x00, 0x01, 0x12, 0x03, 0x05, 0x05, 0x0d, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02,
    0x00, 0x12, 0x03, 0x06, 0x04, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x06, 0x04, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x06,
    0x0d, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x01, 0x12, 0x03, 0x07, 0x04, 0x0f, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x07, 0x04, 0x08, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x07, 0x0d, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x05,
    0x00, 0x02, 0x02, 0x12, 0x03, 0x08, 0x04, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02,
    0x01, 0x12, 0x03, 0x08, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x02, 0x12,
    0x03, 0x08, 0x0d, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x03, 0x12, 0x03, 0x09, 0x04,
    0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x09, 0x04, 0x09, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03, 0x02, 0x12, 0x03, 0x09, 0x0d, 0x0e, 0x0a, 0x0b, 0x0a,
    0x04, 0x05, 0x00, 0x02, 0x04, 0x12, 0x03, 0x0a, 0x04, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x04, 0x01, 0x12, 0x03, 0x0a, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x04,
    0x02, 0x12, 0x03, 0x0a, 0x0d, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x05, 0x12, 0x03,
    0x0b, 0x04, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x0b, 0x04,
    0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x05, 0x02, 0x12, 0x03, 0x0b, 0x0d, 0x0e, 0x0a,
    0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x06, 0x12, 0x03, 0x0c, 0x04, 0x0f, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x0c, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x06, 0x02, 0x12, 0x03, 0x0c, 0x0d, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x07,
    0x12, 0x03, 0x0d, 0x04, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x07, 0x01, 0x12, 0x03,
    0x0d, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x07, 0x02, 0x12, 0x03, 0x0d, 0x0d,
    0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x08, 0x12, 0x03, 0x0e, 0x04, 0x0f, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x08, 0x01, 0x12, 0x03, 0x0e, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x08, 0x02, 0x12, 0x03, 0x0e, 0x0d, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00,
    0x02, 0x09, 0x12, 0x03, 0x0f, 0x04, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x09, 0x01,
    0x12, 0x03, 0x0f, 0x04, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x09, 0x02, 0x12, 0x03,
    0x0f, 0x12, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x0a, 0x12, 0x03, 0x10, 0x04, 0x15,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x10, 0x04, 0x0f, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x0a, 0x02, 0x12, 0x03, 0x10, 0x12, 0x14, 0x0a, 0x0b, 0x0a, 0x04,
    0x05, 0x00, 0x02, 0x0b, 0x12, 0x03, 0x11, 0x04, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x0b, 0x01, 0x12, 0x03, 0x11, 0x04, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0b, 0x02,
    0x12, 0x03, 0x11, 0x12, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x0c, 0x12, 0x03, 0x12,
    0x04, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0c, 0x01, 0x12, 0x03, 0x12, 0x04, 0x0e,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0c, 0x02, 0x12, 0x03, 0x12, 0x12, 0x14, 0x0a, 0x0b,
    0x0a, 0x04, 0x05, 0x00, 0x02, 0x0d, 0x12, 0x03, 0x13, 0x04, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x0d, 0x01, 0x12, 0x03, 0x13, 0x04, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x0d, 0x02, 0x12, 0x03, 0x13, 0x12, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x0e, 0x12,
    0x03, 0x14, 0x04, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0e, 0x01, 0x12, 0x03, 0x14,
    0x04, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0e, 0x02, 0x12, 0x03, 0x14, 0x12, 0x14,
    0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x0f, 0x12, 0x03, 0x15, 0x04, 0x15, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x0f, 0x01, 0x12, 0x03, 0x15, 0x04, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x0f, 0x02, 0x12, 0x03, 0x15, 0x12, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02,
    0x10, 0x12, 0x03, 0x16, 0x04, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x10, 0x01, 0x12,
    0x03, 0x16, 0x04, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x10, 0x02, 0x12, 0x03, 0x16,
    0x12, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x11, 0x12, 0x03, 0x17, 0x04, 0x1b, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x11, 0x01, 0x12, 0x03, 0x17, 0x04, 0x14, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x11, 0x02, 0x12, 0x03, 0x17, 0x18, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x05,
    0x00, 0x02, 0x12, 0x12, 0x03, 0x18, 0x04, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x12,
    0x01, 0x12, 0x03, 0x18, 0x04, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x12, 0x02, 0x12,
    0x03, 0x18, 0x18, 0x1a, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x1b, 0x00, 0x1d, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x1b, 0x08, 0x0f, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x1c, 0x04, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x04, 0x12, 0x03, 0x1c, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05,
    0x12, 0x03, 0x1c, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x1c, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x1c, 0x19,
    0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x08, 0x12, 0x03, 0x1c, 0x1b, 0x28, 0x0a,
    0x0f, 0x0a, 0x08, 0x04, 0x00, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x1c, 0x1c, 0x27,
    0x0a, 0x10, 0x0a, 0x09, 0x04, 0x00, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x1c,
    0x1c, 0x22, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x00, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00,
    0x12, 0x03, 0x1c, 0x1c, 0x22, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x00, 0x02, 0x00, 0x08, 0xe7, 0x07,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1c, 0x1c, 0x22, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x00, 0x02,
    0x00, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x1c, 0x23, 0x27, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x01, 0x12, 0x04, 0x1f, 0x00, 0x21, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03,
    0x1f, 0x08, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x20, 0x04, 0x2d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x20, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x20, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x20, 0x14, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x20, 0x1d, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00,
    0x08, 0x12, 0x03, 0x20, 0x1f, 0x2c, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x01, 0x02, 0x00, 0x08, 0xe7,
    0x07, 0x00, 0x12, 0x03, 0x20, 0x20, 0x2b, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x02, 0x00, 0x08,
    0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x20, 0x20, 0x26, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x01, 0x02,
    0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x20, 0x20, 0x26, 0x0a, 0x12, 0x0a, 0x0b,
    0x04, 0x01, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x20, 0x20, 0x26,
    0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x20,
    0x27, 0x2b, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x23, 0x00, 0x25, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x23, 0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02,
    0x02, 0x00, 0x12, 0x03, 0x24, 0x04, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x24, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x24, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x24, 0x13,
    0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x24, 0x1b, 0x1c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x08, 0x12, 0x03, 0x24, 0x1d, 0x2a, 0x0a, 0x0f, 0x0a,
    0x08, 0x04, 0x02, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x24, 0x1e, 0x29, 0x0a, 0x10,
    0x0a, 0x09, 0x04, 0x02, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x24, 0x1e, 0x24,
    0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x02, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03,
    0x24, 0x1e, 0x24, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x02, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x24, 0x1e, 0x24, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x02, 0x02, 0x00, 0x08,
    0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x24, 0x25, 0x29, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12,
    0x04, 0x27, 0x00, 0x29, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x27, 0x08,
    0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x28, 0x04, 0x2c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x03, 0x28, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x00, 0x05, 0x12, 0x03, 0x28, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x28, 0x14, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x28, 0x1c, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x08, 0x12,
    0x03, 0x28, 0x1e, 0x2b, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x03, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00,
    0x12, 0x03, 0x28, 0x1f, 0x2a, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x03, 0x02, 0x00, 0x08, 0xe7, 0x07,
    0x00, 0x02, 0x12, 0x03, 0x28, 0x1f, 0x25, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x03, 0x02, 0x00, 0x08,
    0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x28, 0x1f, 0x25, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x03,
    0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x28, 0x1f, 0x25, 0x0a, 0x10,
    0x0a, 0x09, 0x04, 0x03, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x28, 0x26, 0x2a,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x2b, 0x00, 0x2d, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x04, 0x01, 0x12, 0x03, 0x2b, 0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00,
    0x12, 0x03, 0x2c, 0x04, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x2c, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x05, 0x12, 0x03, 0x2c, 0x0d,
    0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2c, 0x14, 0x19, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2c, 0x1c, 0x1d, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x00, 0x08, 0x12, 0x03, 0x2c, 0x1e, 0x2b, 0x0a, 0x0f, 0x0a, 0x08, 0x04,
    0x04, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x2c, 0x1f, 0x2a, 0x0a, 0x10, 0x0a, 0x09,
    0x04, 0x04, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x2c, 0x1f, 0x25, 0x0a, 0x11,
    0x0a, 0x0a, 0x04, 0x04, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x2c, 0x1f,
    0x25, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x04, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x2c, 0x1f, 0x25, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x04, 0x02, 0x00, 0x08, 0xe7, 0x07,
    0x00, 0x03, 0x12, 0x03, 0x2c, 0x26, 0x2a, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x2f,
    0x00, 0x31, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x2f, 0x08, 0x11, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00, 0x12, 0x03, 0x30, 0x04, 0x2d, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x00, 0x04, 0x12, 0x03, 0x30, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x30, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x30, 0x14, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x30, 0x1d, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x08, 0x12, 0x03, 0x30,
    0x1f, 0x2c, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x05, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03,
    0x30, 0x20, 0x2b, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x05, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02,
    0x12, 0x03, 0x30, 0x20, 0x26, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x05, 0x02, 0x00, 0x08, 0xe7, 0x07,
    0x00, 0x02, 0x00, 0x12, 0x03, 0x30, 0x20, 0x26, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x05, 0x02, 0x00,
    0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x30, 0x20, 0x26, 0x0a, 0x10, 0x0a, 0x09,
    0x04, 0x05, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x30, 0x27, 0x2b, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x06, 0x12, 0x04, 0x33, 0x00, 0x35, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06,
    0x01, 0x12, 0x03, 0x33, 0x08, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x00, 0x12, 0x03,
    0x34, 0x04, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x04, 0x12, 0x03, 0x34, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x05, 0x12, 0x03, 0x34, 0x0d, 0x13, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x01, 0x12, 0x03, 0x34, 0x14, 0x1a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x00, 0x03, 0x12, 0x03, 0x34, 0x1d, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x00, 0x08, 0x12, 0x03, 0x34, 0x1f, 0x2c, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x06, 0x02,
    0x00, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x34, 0x20, 0x2b, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x06,
    0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x34, 0x20, 0x26, 0x0a, 0x11, 0x0a, 0x0a,
    0x04, 0x06, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x34, 0x20, 0x26, 0x0a,
    0x12, 0x0a, 0x0b, 0x04, 0x06, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x34, 0x20, 0x26, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x06, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x03,
    0x12, 0x03, 0x34, 0x27, 0x2b, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x07, 0x12, 0x04, 0x37, 0x00, 0x39,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x07, 0x01, 0x12, 0x03, 0x37, 0x08, 0x11, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x07, 0x02, 0x00, 0x12, 0x03, 0x38, 0x04, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07,
    0x02, 0x00, 0x04, 0x12, 0x03, 0x38, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x38, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x38, 0x14, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x03, 0x12, 0x03, 0x38,
    0x1d, 0x1e, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x08, 0x12, 0x04, 0x3b, 0x00, 0x3e, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x08, 0x01, 0x12, 0x03, 0x3b, 0x08, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08,
    0x02, 0x00, 0x12, 0x03, 0x3c, 0x04, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x3c, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x3c, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x01, 0x12, 0x03, 0x3c, 0x14,
    0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x03, 0x12, 0x03, 0x3c, 0x1c, 0x1d, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x08, 0x12, 0x03, 0x3c, 0x1e, 0x2b, 0x0a, 0x0f, 0x0a,
    0x08, 0x04, 0x08, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x3c, 0x1f, 0x2a, 0x0a, 0x10,
    0x0a, 0x09, 0x04, 0x08, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x3c, 0x1f, 0x25,
    0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x08, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03,
    0x3c, 0x1f, 0x25, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x08, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x3c, 0x1f, 0x25, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x08, 0x02, 0x00, 0x08,
    0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x3c, 0x26, 0x2a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02,
    0x01, 0x12, 0x03, 0x3d, 0x04, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x04, 0x12,
    0x03, 0x3d, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x05, 0x12, 0x03, 0x3d,
    0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x01, 0x12, 0x03, 0x3d, 0x14, 0x1a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x03, 0x12, 0x03, 0x3d, 0x1d, 0x1e, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x09, 0x12, 0x04, 0x40, 0x00, 0x43, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x09,
    0x01, 0x12, 0x03, 0x40, 0x08, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x00, 0x12, 0x03,
    0x41, 0x04, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x04, 0x12, 0x03, 0x41, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x05, 0x12, 0x03, 0x41, 0x0d, 0x13, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x01, 0x12, 0x03, 0x41, 0x14, 0x1a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x00, 0x03, 0x12, 0x03, 0x41, 0x1d, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x09, 0x02, 0x00, 0x08, 0x12, 0x03, 0x41, 0x1f, 0x2c, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x09, 0x02,
    0x00, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x41, 0x20, 0x2b, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x09,
    0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x41, 0x20, 0x26, 0x0a, 0x11, 0x0a, 0x0a,
    0x04, 0x09, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x41, 0x20, 0x26, 0x0a,
    0x12, 0x0a, 0x0b, 0x04, 0x09, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x41, 0x20, 0x26, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x09, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x03,
    0x12, 0x03, 0x41, 0x27, 0x2b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x01, 0x12, 0x03, 0x42,
    0x04, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x04, 0x12, 0x03, 0x42, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x05, 0x12, 0x03, 0x42, 0x0d, 0x13, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x01, 0x12, 0x03, 0x42, 0x14, 0x1a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x01, 0x03, 0x12, 0x03, 0x42, 0x1d, 0x1e, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0a,
    0x12, 0x04, 0x45, 0x00, 0x5b, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0a, 0x01, 0x12, 0x03, 0x45,
    0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0a, 0x08, 0x00, 0x12, 0x04, 0x46, 0x04, 0x5a, 0x05,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x08, 0x00, 0x01, 0x12, 0x03, 0x46, 0x0a, 0x0f, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x0a, 0x02, 0x00, 0x12, 0x03, 0x47, 0x08, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0a, 0x02, 0x00, 0x05, 0x12, 0x03, 0x47, 0x08, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x47, 0x0f, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x47, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x01, 0x12, 0x03, 0x48,
    0x08, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x05, 0x12, 0x03, 0x48, 0x08, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x01, 0x12, 0x03, 0x48, 0x0f, 0x13, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x03, 0x12, 0x03, 0x48, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x0a, 0x02, 0x02, 0x12, 0x03, 0x49, 0x08, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02,
    0x02, 0x05, 0x12, 0x03, 0x49, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x02, 0x01,
    0x12, 0x03, 0x49, 0x0f, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x02, 0x03, 0x12, 0x03,
    0x49, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x03, 0x12, 0x03, 0x4a, 0x08, 0x21,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x03, 0x05, 0x12, 0x03, 0x4a, 0x08, 0x0d, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x4a, 0x0f, 0x14, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0a, 0x02, 0x03, 0x03, 0x12, 0x03, 0x4a, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0a,
    0x02, 0x04, 0x12, 0x03, 0x4b, 0x08, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x04, 0x05,
    0x12, 0x03, 0x4b, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x03,
    0x4b, 0x0f, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x03, 0x4b, 0x1f,
    0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x05, 0x12, 0x03, 0x4c, 0x08, 0x21, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0a, 0x02, 0x05, 0x05, 0x12, 0x03, 0x4c, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0a, 0x02, 0x05, 0x01, 0x12, 0x03, 0x4c, 0x0f, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a,
    0x02, 0x05, 0x03, 0x12, 0x03, 0x4c, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x06,
    0x12, 0x03, 0x4d, 0x08, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x06, 0x05, 0x12, 0x03,
    0x4d, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x06, 0x01, 0x12, 0x03, 0x4d, 0x0f,
    0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x06, 0x03, 0x12, 0x03, 0x4d, 0x1f, 0x20, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x07, 0x12, 0x03, 0x4e, 0x08, 0x21, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0a, 0x02, 0x07, 0x05, 0x12, 0x03, 0x4e, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a,
    0x02, 0x07, 0x01, 0x12, 0x03, 0x4e, 0x0f, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x07,
    0x03, 0x12, 0x03, 0x4e, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x08, 0x12, 0x03,
    0x4f, 0x08, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x08, 0x05, 0x12, 0x03, 0x4f, 0x08,
    0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x08, 0x01, 0x12, 0x03, 0x4f, 0x0f, 0x15, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x08, 0x03, 0x12, 0x03, 0x4f, 0x1f, 0x20, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x0a, 0x02, 0x09, 0x12, 0x03, 0x50, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a,
    0x02, 0x09, 0x06, 0x12, 0x03, 0x50, 0x08, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x09,
    0x01, 0x12, 0x03, 0x50, 0x12, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x09, 0x03, 0x12,
    0x03, 0x50, 0x1f, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x0a, 0x12, 0x03, 0x51, 0x08,
    0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x0a, 0x06, 0x12, 0x03, 0x51, 0x08, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x51, 0x12, 0x1c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0a, 0x02, 0x0a, 0x03, 0x12, 0x03, 0x51, 0x1f, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x0a, 0x02, 0x0b, 0x12, 0x03, 0x52, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x0b,
    0x06, 0x12, 0x03, 0x52, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x0b, 0x01, 0x12,
    0x03, 0x52, 0x12, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x0b, 0x03, 0x12, 0x03, 0x52,
    0x1f, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x0c, 0x12, 0x03, 0x53, 0x08, 0x22, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x0c, 0x06, 0x12, 0x03, 0x53, 0x08, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0a, 0x02, 0x0c, 0x01, 0x12, 0x03, 0x53, 0x12, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0a, 0x02, 0x0c, 0x03, 0x12, 0x03, 0x53, 0x1f, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0a, 0x02,
    0x0d, 0x12, 0x03, 0x54, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x0d, 0x06, 0x12,
    0x03, 0x54, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x0d, 0x01, 0x12, 0x03, 0x54,
    0x12, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x0d, 0x03, 0x12, 0x03, 0x54, 0x1f, 0x21,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x0e, 0x12, 0x03, 0x55, 0x08, 0x22, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0a, 0x02, 0x0e, 0x06, 0x12, 0x03, 0x55, 0x08, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0a, 0x02, 0x0e, 0x01, 0x12, 0x03, 0x55, 0x12, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02,
    0x0e, 0x03, 0x12, 0x03, 0x55, 0x1f, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x0f, 0x12,
    0x03, 0x56, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x0f, 0x06, 0x12, 0x03, 0x56,
    0x08, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x0f, 0x01, 0x12, 0x03, 0x56, 0x12, 0x1c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x0f, 0x03, 0x12, 0x03, 0x56, 0x1f, 0x21, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x0a, 0x02, 0x10, 0x12, 0x03, 0x57, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0a, 0x02, 0x10, 0x06, 0x12, 0x03, 0x57, 0x08, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02,
    0x10, 0x01, 0x12, 0x03, 0x57, 0x12, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x10, 0x03,
    0x12, 0x03, 0x57, 0x1f, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x11, 0x12, 0x03, 0x58,
    0x08, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x11, 0x06, 0x12, 0x03, 0x58, 0x08, 0x16,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x11, 0x01, 0x12, 0x03, 0x58, 0x18, 0x27, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0a, 0x02, 0x11, 0x03, 0x12, 0x03, 0x58, 0x2b, 0x2d, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x0a, 0x02, 0x12, 0x12, 0x03, 0x59, 0x08, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02,
    0x12, 0x06, 0x12, 0x03, 0x59, 0x08, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x12, 0x01,
    0x12, 0x03, 0x59, 0x18, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x12, 0x03, 0x12, 0x03,
    0x59, 0x2b, 0x2d, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0b, 0x12, 0x04, 0x5d, 0x00, 0x61, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x0b, 0x01, 0x12, 0x03, 0x5d, 0x08, 0x0d, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x0b, 0x02, 0x00, 0x12, 0x03, 0x5e, 0x04, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x5e, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x5e, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x01, 0x12, 0x03, 0x5e,
    0x14, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x03, 0x12, 0x03, 0x5e, 0x1d, 0x1e,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x01, 0x12, 0x03, 0x5f, 0x04, 0x1d, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0b, 0x02, 0x01, 0x04, 0x12, 0x03, 0x5f, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0b, 0x02, 0x01, 0x05, 0x12, 0x03, 0x5f, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x5f, 0x14, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x5f, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x02, 0x12, 0x03, 0x60,
    0x04, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x02, 0x04, 0x12, 0x03, 0x60, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x02, 0x05, 0x12, 0x03, 0x60, 0x0d, 0x13, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0b, 0x02, 0x02, 0x01, 0x12, 0x03, 0x60, 0x14, 0x1a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0b, 0x02, 0x02, 0x03, 0x12, 0x03, 0x60, 0x1d, 0x1e, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0c,
    0x12, 0x04, 0x65, 0x00, 0x68, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0c, 0x01, 0x12, 0x03, 0x65,
    0x08, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x00, 0x12, 0x03, 0x66, 0x04, 0x1c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x04, 0x12, 0x03, 0x66, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0c, 0x02, 0x00, 0x05, 0x12, 0x03, 0x66, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0c, 0x02, 0x00, 0x01, 0x12, 0x03, 0x66, 0x14, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x66, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x01, 0x12,
    0x03, 0x67, 0x04, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x04, 0x12, 0x03, 0x67,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x06, 0x12, 0x03, 0x67, 0x0d, 0x12,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x01, 0x12, 0x03, 0x67, 0x13, 0x18, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x03, 0x12, 0x03, 0x67, 0x1b, 0x1c, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x0d, 0x12, 0x04, 0x6a, 0x00, 0x6c, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0d, 0x01, 0x12,
    0x03, 0x6a, 0x08, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x00, 0x12, 0x03, 0x6b, 0x04,
    0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x04, 0x12, 0x03, 0x6b, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x05, 0x12, 0x03, 0x6b, 0x0d, 0x13, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0d, 0x02, 0x00, 0x01, 0x12, 0x03, 0x6b, 0x14, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0d, 0x02, 0x00, 0x03, 0x12, 0x03, 0x6b, 0x1b, 0x1c, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0e, 0x12,
    0x04, 0x6e, 0x00, 0x71, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0e, 0x01, 0x12, 0x03, 0x6e, 0x08,
    0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x00, 0x12, 0x03, 0x6f, 0x04, 0x1d, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x04, 0x12, 0x03, 0x6f, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0e, 0x02, 0x00, 0x05, 0x12, 0x03, 0x6f, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x6f, 0x14, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x6f, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x01, 0x12, 0x03,
    0x70, 0x04, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x04, 0x12, 0x03, 0x70, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x06, 0x12, 0x03, 0x70, 0x0d, 0x12, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x01, 0x12, 0x03, 0x70, 0x13, 0x18, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0e, 0x02, 0x01, 0x03, 0x12, 0x03, 0x70, 0x1b, 0x1c, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x0f, 0x12, 0x04, 0x77, 0x00, 0x78, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0f, 0x01, 0x12, 0x03,
    0x77, 0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x02, 0x04, 0x10, 0x12, 0x05, 0x7a, 0x00, 0x83, 0x01, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x10, 0x01, 0x12, 0x03, 0x7a, 0x08, 0x0f, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x10, 0x02, 0x00, 0x12, 0x03, 0x7b, 0x04, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02,
    0x00, 0x04, 0x12, 0x03, 0x7b, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x05,
    0x12, 0x03, 0x7b, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x7b, 0x14, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x03, 0x12, 0x03, 0x7b, 0x1c,
    0x1d, 0x0a, 0x0d, 0x0a, 0x04, 0x04, 0x10, 0x08, 0x00, 0x12, 0x05, 0x7c, 0x04, 0x82, 0x01, 0x05,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10, 0x08, 0x00, 0x01, 0x12, 0x03, 0x7c, 0x0a, 0x11, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x10, 0x02, 0x01, 0x12, 0x03, 0x7d, 0x08, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x10, 0x02, 0x01, 0x06, 0x12, 0x03, 0x7d, 0x08, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x7d, 0x14, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x7d, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x02, 0x12, 0x03, 0x7e,
    0x08, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x02, 0x06, 0x12, 0x03, 0x7e, 0x08, 0x15,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x02, 0x01, 0x12, 0x03, 0x7e, 0x16, 0x1b, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x10, 0x02, 0x02, 0x03, 0x12, 0x03, 0x7e, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x10, 0x02, 0x03, 0x12, 0x03, 0x7f, 0x08, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02,
    0x03, 0x06, 0x12, 0x03, 0x7f, 0x08, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x03, 0x01,
    0x12, 0x03, 0x7f, 0x17, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x03, 0x03, 0x12, 0x03,
    0x7f, 0x1f, 0x20, 0x0a, 0x28, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x04, 0x12, 0x04, 0x81, 0x01, 0x08,
    0x1a, 0x1a, 0x1a, 0x51, 0x75, 0x65, 0x72, 0x79, 0x41, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74,
    0x65, 0x20, 0x71, 0x61, 0x74, 0x74, 0x72, 0x20, 0x3d, 0x20, 0x35, 0x3b, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x10, 0x02, 0x04, 0x06, 0x12, 0x04, 0x81, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x10, 0x02, 0x04, 0x01, 0x12, 0x04, 0x81, 0x01, 0x11, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x10, 0x02, 0x04, 0x03, 0x12, 0x04, 0x81, 0x01, 0x18, 0x19, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x11,
    0x12, 0x06, 0x87, 0x01, 0x00, 0x8d, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x11, 0x01, 0x12,
    0x04, 0x87, 0x01, 0x08, 0x19, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x00, 0x12, 0x04, 0x88,
    0x01, 0x04, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x04, 0x12, 0x04, 0x88, 0x01,
    0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x05, 0x12, 0x04, 0x88, 0x01, 0x0d,
    0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x01, 0x12, 0x04, 0x88, 0x01, 0x14, 0x17,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x03, 0x12, 0x04, 0x88, 0x01, 0x1a, 0x1b, 0x0a,
    0x0e, 0x0a, 0x04, 0x04, 0x11, 0x08, 0x00, 0x12, 0x06, 0x89, 0x01, 0x04, 0x8c, 0x01, 0x05, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x11, 0x08, 0x00, 0x01, 0x12, 0x04, 0x89, 0x01, 0x0a, 0x10, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x11, 0x02, 0x01, 0x12, 0x04, 0x8a, 0x01, 0x08, 0x18, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x11, 0x02, 0x01, 0x06, 0x12, 0x04, 0x8a, 0x01, 0x08, 0x0d, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x11, 0x02, 0x01, 0x01, 0x12, 0x04, 0x8a, 0x01, 0x0e, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11,
    0x02, 0x01, 0x03, 0x12, 0x04, 0x8a, 0x01, 0x16, 0x17, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02,
    0x02, 0x12, 0x04, 0x8b, 0x01, 0x08, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x02, 0x06,
    0x12, 0x04, 0x8b, 0x01, 0x08, 0x0d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x02, 0x01, 0x12,
    0x04, 0x8b, 0x01, 0x0e, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x02, 0x03, 0x12, 0x04,
    0x8b, 0x01, 0x16, 0x17, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x12, 0x12, 0x06, 0x8f, 0x01, 0x00, 0x95,
    0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x12, 0x01, 0x12, 0x04, 0x8f, 0x01, 0x08, 0x1b, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x12, 0x02, 0x00, 0x12, 0x04, 0x90, 0x01, 0x04, 0x1d, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x12, 0x02, 0x00, 0x04, 0x12, 0x04, 0x90, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x12, 0x02, 0x00, 0x05, 0x12, 0x04, 0x90, 0x01, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x12, 0x02, 0x00, 0x01, 0x12, 0x04, 0x90, 0x01, 0x14, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12,
    0x02, 0x00, 0x03, 0x12, 0x04, 0x90, 0x01, 0x1b, 0x1c, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x12, 0x08,
    0x00, 0x12, 0x06, 0x91, 0x01, 0x04, 0x94, 0x01, 0x05, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x08,
    0x00, 0x01, 0x12, 0x04, 0x91, 0x01, 0x0a, 0x10, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x12, 0x02, 0x01,
    0x12, 0x04, 0x92, 0x01, 0x08, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x01, 0x06, 0x12,
    0x04, 0x92, 0x01, 0x08, 0x0d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x01, 0x01, 0x12, 0x04,
    0x92, 0x01, 0x0e, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x01, 0x03, 0x12, 0x04, 0x92,
    0x01, 0x16, 0x17, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x12, 0x02, 0x02, 0x12, 0x04, 0x93, 0x01, 0x08,
    0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x02, 0x06, 0x12, 0x04, 0x93, 0x01, 0x08, 0x0d,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x02, 0x01, 0x12, 0x04, 0x93, 0x01, 0x0e, 0x13, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x02, 0x03, 0x12, 0x04, 0x93, 0x01, 0x16, 0x17, 0x0a, 0x0c,
    0x0a, 0x02, 0x04, 0x13, 0x12, 0x06, 0x97, 0x01, 0x00, 0x9a, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03,
    0x04, 0x13, 0x01, 0x12, 0x04, 0x97, 0x01, 0x08, 0x1c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x13, 0x02,
    0x00, 0x12, 0x04, 0x98, 0x01, 0x04, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x04,
    0x12, 0x04, 0x98, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x05, 0x12,
    0x04, 0x98, 0x01, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x01, 0x12, 0x04,
    0x98, 0x01, 0x14, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x03, 0x12, 0x04, 0x98,
    0x01, 0x1b, 0x1c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x01, 0x12, 0x04, 0x99, 0x01, 0x04,
    0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x01, 0x04, 0x12, 0x04, 0x99, 0x01, 0x04, 0x0c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x01, 0x06, 0x12, 0x04, 0x99, 0x01, 0x0d, 0x12, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x01, 0x01, 0x12, 0x04, 0x99, 0x01, 0x13, 0x18, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x13, 0x02, 0x01, 0x03, 0x12, 0x04, 0x99, 0x01, 0x1b, 0x1c, 0x0a, 0x0c, 0x0a,
    0x02, 0x04, 0x14, 0x12, 0x06, 0x9c, 0x01, 0x00, 0xa3, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04,
    0x14, 0x01, 0x12, 0x04, 0x9c, 0x01, 0x08, 0x13, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x00,
    0x12, 0x04, 0x9d, 0x01, 0x04, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x04, 0x12,
    0x04, 0x9d, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x05, 0x12, 0x04,
    0x9d, 0x01, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x01, 0x12, 0x04, 0x9d,
    0x01, 0x14, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x03, 0x12, 0x04, 0x9d, 0x01,
    0x1b, 0x1c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x01, 0x12, 0x04, 0x9e, 0x01, 0x04, 0x1c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x01, 0x04, 0x12, 0x04, 0x9e, 0x01, 0x04, 0x0c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x01, 0x05, 0x12, 0x04, 0x9e, 0x01, 0x0d, 0x13, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x14, 0x02, 0x01, 0x01, 0x12, 0x04, 0x9e, 0x01, 0x14, 0x17, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x14, 0x02, 0x01, 0x03, 0x12, 0x04, 0x9e, 0x01, 0x1a, 0x1b, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x14, 0x02, 0x02, 0x12, 0x04, 0x9f, 0x01, 0x04, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14,
    0x02, 0x02, 0x04, 0x12, 0x04, 0x9f, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02,
    0x02, 0x06, 0x12, 0x04, 0x9f, 0x01, 0x0d, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x02,
    0x01, 0x12, 0x04, 0x9f, 0x01, 0x16, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x02, 0x03,
    0x12, 0x04, 0x9f, 0x01, 0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x03, 0x12, 0x04,
    0xa0, 0x01, 0x04, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x03, 0x04, 0x12, 0x04, 0xa0,
    0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x03, 0x06, 0x12, 0x04, 0xa0, 0x01,
    0x0d, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x03, 0x01, 0x12, 0x04, 0xa0, 0x01, 0x16,
    0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x03, 0x03, 0x12, 0x04, 0xa0, 0x01, 0x20, 0x21,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x04, 0x12, 0x04, 0xa1, 0x01, 0x04, 0x1e, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x14, 0x02, 0x04, 0x04, 0x12, 0x04, 0xa1, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x14, 0x02, 0x04, 0x05, 0x12, 0x04, 0xa1, 0x01, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x14, 0x02, 0x04, 0x01, 0x12, 0x04, 0xa1, 0x01, 0x14, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x14, 0x02, 0x04, 0x03, 0x12, 0x04, 0xa1, 0x01, 0x1c, 0x1d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14,
    0x02, 0x05, 0x12, 0x04, 0xa2, 0x01, 0x04, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x05,
    0x04, 0x12, 0x04, 0xa2, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x05, 0x05,
    0x12, 0x04, 0xa2, 0x01, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x05, 0x01, 0x12,
    0x04, 0xa2, 0x01, 0x14, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x05, 0x03, 0x12, 0x04,
    0xa2, 0x01, 0x1d, 0x1e, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x15, 0x12, 0x06, 0xa5, 0x01, 0x00, 0xaa,
    0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x15, 0x01, 0x12, 0x04, 0xa5, 0x01, 0x08, 0x15, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x00, 0x12, 0x04, 0xa6, 0x01, 0x04, 0x1d, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x15, 0x02, 0x00, 0x04, 0x12, 0x04, 0xa6, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x15, 0x02, 0x00, 0x05, 0x12, 0x04, 0xa6, 0x01, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x15, 0x02, 0x00, 0x01, 0x12, 0x04, 0xa6, 0x01, 0x14, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15,
    0x02, 0x00, 0x03, 0x12, 0x04, 0xa6, 0x01, 0x1b, 0x1c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02,
    0x01, 0x12, 0x04, 0xa7, 0x01, 0x04, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x01, 0x04,
    0x12, 0x04, 0xa7, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x01, 0x05, 0x12,
    0x04, 0xa7, 0x01, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x01, 0x01, 0x12, 0x04,
    0xa7, 0x01, 0x14, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x01, 0x03, 0x12, 0x04, 0xa7,
    0x01, 0x1a, 0x1b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x02, 0x12, 0x04, 0xa8, 0x01, 0x04,
    0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x02, 0x04, 0x12, 0x04, 0xa8, 0x01, 0x04, 0x0c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x02, 0x06, 0x12, 0x04, 0xa8, 0x01, 0x0d, 0x15, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x02, 0x01, 0x12, 0x04, 0xa8, 0x01, 0x16, 0x1a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x15, 0x02, 0x02, 0x03, 0x12, 0x04, 0xa8, 0x01, 0x1d, 0x1e, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x15, 0x02, 0x03, 0x12, 0x04, 0xa9, 0x01, 0x04, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x15, 0x02, 0x03, 0x04, 0x12, 0x04, 0xa9, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15,
    0x02, 0x03, 0x05, 0x12, 0x04, 0xa9, 0x01, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02,
    0x03, 0x01, 0x12, 0x04, 0xa9, 0x01, 0x14, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x03,
    0x03, 0x12, 0x04, 0xa9, 0x01, 0x1b, 0x1c, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x16, 0x12, 0x06, 0xb1,
    0x01, 0x00, 0xb4, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x16, 0x01, 0x12, 0x04, 0xb1, 0x01,
    0x08, 0x16, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x16, 0x02, 0x00, 0x12, 0x04, 0xb2, 0x01, 0x04, 0x21,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x00, 0x04, 0x12, 0x04, 0xb2, 0x01, 0x04, 0x0c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x00, 0x06, 0x12, 0x04, 0xb2, 0x01, 0x0d, 0x18, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x16, 0x02, 0x00, 0x01, 0x12, 0x04, 0xb2, 0x01, 0x19, 0x1c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x16, 0x02, 0x00, 0x03, 0x12, 0x04, 0xb2, 0x01, 0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x16, 0x02, 0x01, 0x12, 0x04, 0xb3, 0x01, 0x04, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16,
    0x02, 0x01, 0x04, 0x12, 0x04, 0xb3, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02,
    0x01, 0x06, 0x12, 0x04, 0xb3, 0x01, 0x0d, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x01,
    0x01, 0x12, 0x04, 0xb3, 0x01, 0x1b, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x01, 0x03,
    0x12, 0x04, 0xb3, 0x01, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x17, 0x12, 0x06, 0xb6, 0x01,
    0x00, 0xbf, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x17, 0x01, 0x12, 0x04, 0xb6, 0x01, 0x08,
    0x10, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x17, 0x02, 0x00, 0x12, 0x04, 0xb7, 0x01, 0x04, 0x1e, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x04, 0x12, 0x04, 0xb7, 0x01, 0x04, 0x0c, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x05, 0x12, 0x04, 0xb7, 0x01, 0x0d, 0x13, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x17, 0x02, 0x00, 0x01, 0x12, 0x04, 0xb7, 0x01, 0x14, 0x19, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x17, 0x02, 0x00, 0x03, 0x12, 0x04, 0xb7, 0x01, 0x1c, 0x1d, 0x0a, 0x0e, 0x0a, 0x04, 0x04,
    0x17, 0x08, 0x00, 0x12, 0x06, 0xb8, 0x01, 0x04, 0xbe, 0x01, 0x05, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x17, 0x08, 0x00, 0x01, 0x12, 0x04, 0xb8, 0x01, 0x0a, 0x12, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x17,
    0x02, 0x01, 0x12, 0x04, 0xb9, 0x01, 0x08, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x01,
    0x06, 0x12, 0x04, 0xb9, 0x01, 0x08, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x01, 0x01,
    0x12, 0x04, 0xb9, 0x01, 0x1a, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x01, 0x03, 0x12,
    0x04, 0xb9, 0x01, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x17, 0x02, 0x02, 0x12, 0x04, 0xba,
    0x01, 0x08, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x02, 0x06, 0x12, 0x04, 0xba, 0x01,
    0x08, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x02, 0x01, 0x12, 0x04, 0xba, 0x01, 0x1c,
    0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x02, 0x03, 0x12, 0x04, 0xba, 0x01, 0x24, 0x25,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x17, 0x02, 0x03, 0x12, 0x04, 0xbb, 0x01, 0x08, 0x27, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x17, 0x02, 0x03, 0x06, 0x12, 0x04, 0xbb, 0x01, 0x08, 0x1c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x17, 0x02, 0x03, 0x01, 0x12, 0x04, 0xbb, 0x01, 0x1d, 0x22, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x17, 0x02, 0x03, 0x03, 0x12, 0x04, 0xbb, 0x01, 0x25, 0x26, 0x0a, 0x2f, 0x0a, 0x04, 0x04,
    0x17, 0x02, 0x04, 0x12, 0x04, 0xbd, 0x01, 0x08, 0x20, 0x1a, 0x21, 0x20, 0x51, 0x75, 0x65, 0x72,
    0x79, 0x41, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74,
    0x20, 0x71, 0x61, 0x74, 0x74, 0x72, 0x20, 0x3d, 0x20, 0x35, 0x3b, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x17, 0x02, 0x04, 0x06, 0x12, 0x04, 0xbd, 0x01, 0x08, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x17, 0x02, 0x04, 0x01, 0x12, 0x04, 0xbd, 0x01, 0x17, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17,
    0x02, 0x04, 0x03, 0x12, 0x04, 0xbd, 0x01, 0x1e, 0x1f,
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
