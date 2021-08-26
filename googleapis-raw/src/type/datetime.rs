// This file is generated by rust-protobuf 2.25.1. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `google/type/datetime.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_25_1;

#[derive(PartialEq,Clone,Default)]
pub struct DateTime {
    // message fields
    pub year: i32,
    pub month: i32,
    pub day: i32,
    pub hours: i32,
    pub minutes: i32,
    pub seconds: i32,
    pub nanos: i32,
    // message oneof groups
    pub time_offset: ::std::option::Option<DateTime_oneof_time_offset>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a DateTime {
    fn default() -> &'a DateTime {
        <DateTime as ::protobuf::Message>::default_instance()
    }
}

#[derive(Clone,PartialEq,Debug)]
pub enum DateTime_oneof_time_offset {
    utc_offset(::protobuf::well_known_types::Duration),
    time_zone(TimeZone),
}

impl DateTime {
    pub fn new() -> DateTime {
        ::std::default::Default::default()
    }

    // int32 year = 1;


    pub fn get_year(&self) -> i32 {
        self.year
    }
    pub fn clear_year(&mut self) {
        self.year = 0;
    }

    // Param is passed by value, moved
    pub fn set_year(&mut self, v: i32) {
        self.year = v;
    }

    // int32 month = 2;


    pub fn get_month(&self) -> i32 {
        self.month
    }
    pub fn clear_month(&mut self) {
        self.month = 0;
    }

    // Param is passed by value, moved
    pub fn set_month(&mut self, v: i32) {
        self.month = v;
    }

    // int32 day = 3;


    pub fn get_day(&self) -> i32 {
        self.day
    }
    pub fn clear_day(&mut self) {
        self.day = 0;
    }

    // Param is passed by value, moved
    pub fn set_day(&mut self, v: i32) {
        self.day = v;
    }

    // int32 hours = 4;


    pub fn get_hours(&self) -> i32 {
        self.hours
    }
    pub fn clear_hours(&mut self) {
        self.hours = 0;
    }

    // Param is passed by value, moved
    pub fn set_hours(&mut self, v: i32) {
        self.hours = v;
    }

    // int32 minutes = 5;


    pub fn get_minutes(&self) -> i32 {
        self.minutes
    }
    pub fn clear_minutes(&mut self) {
        self.minutes = 0;
    }

    // Param is passed by value, moved
    pub fn set_minutes(&mut self, v: i32) {
        self.minutes = v;
    }

    // int32 seconds = 6;


    pub fn get_seconds(&self) -> i32 {
        self.seconds
    }
    pub fn clear_seconds(&mut self) {
        self.seconds = 0;
    }

    // Param is passed by value, moved
    pub fn set_seconds(&mut self, v: i32) {
        self.seconds = v;
    }

    // int32 nanos = 7;


    pub fn get_nanos(&self) -> i32 {
        self.nanos
    }
    pub fn clear_nanos(&mut self) {
        self.nanos = 0;
    }

    // Param is passed by value, moved
    pub fn set_nanos(&mut self, v: i32) {
        self.nanos = v;
    }

    // .google.protobuf.Duration utc_offset = 8;


    pub fn get_utc_offset(&self) -> &::protobuf::well_known_types::Duration {
        match self.time_offset {
            ::std::option::Option::Some(DateTime_oneof_time_offset::utc_offset(ref v)) => v,
            _ => <::protobuf::well_known_types::Duration as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn clear_utc_offset(&mut self) {
        self.time_offset = ::std::option::Option::None;
    }

    pub fn has_utc_offset(&self) -> bool {
        match self.time_offset {
            ::std::option::Option::Some(DateTime_oneof_time_offset::utc_offset(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_utc_offset(&mut self, v: ::protobuf::well_known_types::Duration) {
        self.time_offset = ::std::option::Option::Some(DateTime_oneof_time_offset::utc_offset(v))
    }

    // Mutable pointer to the field.
    pub fn mut_utc_offset(&mut self) -> &mut ::protobuf::well_known_types::Duration {
        if let ::std::option::Option::Some(DateTime_oneof_time_offset::utc_offset(_)) = self.time_offset {
        } else {
            self.time_offset = ::std::option::Option::Some(DateTime_oneof_time_offset::utc_offset(::protobuf::well_known_types::Duration::new()));
        }
        match self.time_offset {
            ::std::option::Option::Some(DateTime_oneof_time_offset::utc_offset(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_utc_offset(&mut self) -> ::protobuf::well_known_types::Duration {
        if self.has_utc_offset() {
            match self.time_offset.take() {
                ::std::option::Option::Some(DateTime_oneof_time_offset::utc_offset(v)) => v,
                _ => panic!(),
            }
        } else {
            ::protobuf::well_known_types::Duration::new()
        }
    }

    // .google.type.TimeZone time_zone = 9;


    pub fn get_time_zone(&self) -> &TimeZone {
        match self.time_offset {
            ::std::option::Option::Some(DateTime_oneof_time_offset::time_zone(ref v)) => v,
            _ => <TimeZone as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn clear_time_zone(&mut self) {
        self.time_offset = ::std::option::Option::None;
    }

    pub fn has_time_zone(&self) -> bool {
        match self.time_offset {
            ::std::option::Option::Some(DateTime_oneof_time_offset::time_zone(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_time_zone(&mut self, v: TimeZone) {
        self.time_offset = ::std::option::Option::Some(DateTime_oneof_time_offset::time_zone(v))
    }

    // Mutable pointer to the field.
    pub fn mut_time_zone(&mut self) -> &mut TimeZone {
        if let ::std::option::Option::Some(DateTime_oneof_time_offset::time_zone(_)) = self.time_offset {
        } else {
            self.time_offset = ::std::option::Option::Some(DateTime_oneof_time_offset::time_zone(TimeZone::new()));
        }
        match self.time_offset {
            ::std::option::Option::Some(DateTime_oneof_time_offset::time_zone(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_time_zone(&mut self) -> TimeZone {
        if self.has_time_zone() {
            match self.time_offset.take() {
                ::std::option::Option::Some(DateTime_oneof_time_offset::time_zone(v)) => v,
                _ => panic!(),
            }
        } else {
            TimeZone::new()
        }
    }
}

impl ::protobuf::Message for DateTime {
    fn is_initialized(&self) -> bool {
        if let Some(DateTime_oneof_time_offset::utc_offset(ref v)) = self.time_offset {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(DateTime_oneof_time_offset::time_zone(ref v)) = self.time_offset {
            if !v.is_initialized() {
                return false;
            }
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.year = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.month = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.day = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.hours = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.minutes = tmp;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.seconds = tmp;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.nanos = tmp;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.time_offset = ::std::option::Option::Some(DateTime_oneof_time_offset::utc_offset(is.read_message()?));
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.time_offset = ::std::option::Option::Some(DateTime_oneof_time_offset::time_zone(is.read_message()?));
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
        if self.year != 0 {
            my_size += ::protobuf::rt::value_size(1, self.year, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.month != 0 {
            my_size += ::protobuf::rt::value_size(2, self.month, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.day != 0 {
            my_size += ::protobuf::rt::value_size(3, self.day, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.hours != 0 {
            my_size += ::protobuf::rt::value_size(4, self.hours, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.minutes != 0 {
            my_size += ::protobuf::rt::value_size(5, self.minutes, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.seconds != 0 {
            my_size += ::protobuf::rt::value_size(6, self.seconds, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.nanos != 0 {
            my_size += ::protobuf::rt::value_size(7, self.nanos, ::protobuf::wire_format::WireTypeVarint);
        }
        if let ::std::option::Option::Some(ref v) = self.time_offset {
            match v {
                &DateTime_oneof_time_offset::utc_offset(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &DateTime_oneof_time_offset::time_zone(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.year != 0 {
            os.write_int32(1, self.year)?;
        }
        if self.month != 0 {
            os.write_int32(2, self.month)?;
        }
        if self.day != 0 {
            os.write_int32(3, self.day)?;
        }
        if self.hours != 0 {
            os.write_int32(4, self.hours)?;
        }
        if self.minutes != 0 {
            os.write_int32(5, self.minutes)?;
        }
        if self.seconds != 0 {
            os.write_int32(6, self.seconds)?;
        }
        if self.nanos != 0 {
            os.write_int32(7, self.nanos)?;
        }
        if let ::std::option::Option::Some(ref v) = self.time_offset {
            match v {
                &DateTime_oneof_time_offset::utc_offset(ref v) => {
                    os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &DateTime_oneof_time_offset::time_zone(ref v) => {
                    os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
            };
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

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> DateTime {
        DateTime::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                "year",
                |m: &DateTime| { &m.year },
                |m: &mut DateTime| { &mut m.year },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                "month",
                |m: &DateTime| { &m.month },
                |m: &mut DateTime| { &mut m.month },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                "day",
                |m: &DateTime| { &m.day },
                |m: &mut DateTime| { &mut m.day },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                "hours",
                |m: &DateTime| { &m.hours },
                |m: &mut DateTime| { &mut m.hours },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                "minutes",
                |m: &DateTime| { &m.minutes },
                |m: &mut DateTime| { &mut m.minutes },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                "seconds",
                |m: &DateTime| { &m.seconds },
                |m: &mut DateTime| { &mut m.seconds },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                "nanos",
                |m: &DateTime| { &m.nanos },
                |m: &mut DateTime| { &mut m.nanos },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ::protobuf::well_known_types::Duration>(
                "utc_offset",
                DateTime::has_utc_offset,
                DateTime::get_utc_offset,
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, TimeZone>(
                "time_zone",
                DateTime::has_time_zone,
                DateTime::get_time_zone,
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<DateTime>(
                "DateTime",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static DateTime {
        static instance: ::protobuf::rt::LazyV2<DateTime> = ::protobuf::rt::LazyV2::INIT;
        instance.get(DateTime::new)
    }
}

impl ::protobuf::Clear for DateTime {
    fn clear(&mut self) {
        self.year = 0;
        self.month = 0;
        self.day = 0;
        self.hours = 0;
        self.minutes = 0;
        self.seconds = 0;
        self.nanos = 0;
        self.time_offset = ::std::option::Option::None;
        self.time_offset = ::std::option::Option::None;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DateTime {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DateTime {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TimeZone {
    // message fields
    pub id: ::std::string::String,
    pub version: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a TimeZone {
    fn default() -> &'a TimeZone {
        <TimeZone as ::protobuf::Message>::default_instance()
    }
}

impl TimeZone {
    pub fn new() -> TimeZone {
        ::std::default::Default::default()
    }

    // string id = 1;


    pub fn get_id(&self) -> &str {
        &self.id
    }
    pub fn clear_id(&mut self) {
        self.id.clear();
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_id(&mut self) -> &mut ::std::string::String {
        &mut self.id
    }

    // Take field
    pub fn take_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.id, ::std::string::String::new())
    }

    // string version = 2;


    pub fn get_version(&self) -> &str {
        &self.version
    }
    pub fn clear_version(&mut self) {
        self.version.clear();
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: ::std::string::String) {
        self.version = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_version(&mut self) -> &mut ::std::string::String {
        &mut self.version
    }

    // Take field
    pub fn take_version(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.version, ::std::string::String::new())
    }
}

impl ::protobuf::Message for TimeZone {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.id)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.version)?;
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
        if !self.id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.id);
        }
        if !self.version.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.version);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.id.is_empty() {
            os.write_string(1, &self.id)?;
        }
        if !self.version.is_empty() {
            os.write_string(2, &self.version)?;
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

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> TimeZone {
        TimeZone::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "id",
                |m: &TimeZone| { &m.id },
                |m: &mut TimeZone| { &mut m.id },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "version",
                |m: &TimeZone| { &m.version },
                |m: &mut TimeZone| { &mut m.version },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<TimeZone>(
                "TimeZone",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static TimeZone {
        static instance: ::protobuf::rt::LazyV2<TimeZone> = ::protobuf::rt::LazyV2::INIT;
        instance.get(TimeZone::new)
    }
}

impl ::protobuf::Clear for TimeZone {
    fn clear(&mut self) {
        self.id.clear();
        self.version.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TimeZone {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TimeZone {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1agoogle/type/datetime.proto\x12\x0bgoogle.type\x1a\x1egoogle/protob\
    uf/duration.proto\"\xa7\x02\n\x08DateTime\x12\x12\n\x04year\x18\x01\x20\
    \x01(\x05R\x04year\x12\x14\n\x05month\x18\x02\x20\x01(\x05R\x05month\x12\
    \x10\n\x03day\x18\x03\x20\x01(\x05R\x03day\x12\x14\n\x05hours\x18\x04\
    \x20\x01(\x05R\x05hours\x12\x18\n\x07minutes\x18\x05\x20\x01(\x05R\x07mi\
    nutes\x12\x18\n\x07seconds\x18\x06\x20\x01(\x05R\x07seconds\x12\x14\n\
    \x05nanos\x18\x07\x20\x01(\x05R\x05nanos\x12:\n\nutc_offset\x18\x08\x20\
    \x01(\x0b2\x19.google.protobuf.DurationH\0R\tutcOffset\x124\n\ttime_zone\
    \x18\t\x20\x01(\x0b2\x15.google.type.TimeZoneH\0R\x08timeZoneB\r\n\x0bti\
    me_offset\"4\n\x08TimeZone\x12\x0e\n\x02id\x18\x01\x20\x01(\tR\x02id\x12\
    \x18\n\x07version\x18\x02\x20\x01(\tR\x07versionBi\n\x0fcom.google.typeB\
    \rDateTimeProtoP\x01Z<google.golang.org/genproto/googleapis/type/datetim\
    e;datetime\xf8\x01\x01\xa2\x02\x03GTPJ\xd8\x1e\n\x06\x12\x04\x0e\0g\x01\
    \n\xbc\x04\n\x01\x0c\x12\x03\x0e\0\x122\xb1\x04\x20Copyright\x202021\x20\
    Google\x20LLC\n\n\x20Licensed\x20under\x20the\x20Apache\x20License,\x20V\
    ersion\x202.0\x20(the\x20\"License\");\n\x20you\x20may\x20not\x20use\x20\
    this\x20file\x20except\x20in\x20compliance\x20with\x20the\x20License.\n\
    \x20You\x20may\x20obtain\x20a\x20copy\x20of\x20the\x20License\x20at\n\n\
    \x20\x20\x20\x20\x20http://www.apache.org/licenses/LICENSE-2.0\n\n\x20Un\
    less\x20required\x20by\x20applicable\x20law\x20or\x20agreed\x20to\x20in\
    \x20writing,\x20software\n\x20distributed\x20under\x20the\x20License\x20\
    is\x20distributed\x20on\x20an\x20\"AS\x20IS\"\x20BASIS,\n\x20WITHOUT\x20\
    WARRANTIES\x20OR\x20CONDITIONS\x20OF\x20ANY\x20KIND,\x20either\x20expres\
    s\x20or\x20implied.\n\x20See\x20the\x20License\x20for\x20the\x20specific\
    \x20language\x20governing\x20permissions\x20and\n\x20limitations\x20unde\
    r\x20the\x20License.\n\n\x08\n\x01\x02\x12\x03\x10\0\x14\n\t\n\x02\x03\0\
    \x12\x03\x12\0(\n\x08\n\x01\x08\x12\x03\x14\0\x1f\n\t\n\x02\x08\x1f\x12\
    \x03\x14\0\x1f\n\x08\n\x01\x08\x12\x03\x15\0S\n\t\n\x02\x08\x0b\x12\x03\
    \x15\0S\n\x08\n\x01\x08\x12\x03\x16\0\"\n\t\n\x02\x08\n\x12\x03\x16\0\"\
    \n\x08\n\x01\x08\x12\x03\x17\0.\n\t\n\x02\x08\x08\x12\x03\x17\0.\n\x08\n\
    \x01\x08\x12\x03\x18\0(\n\t\n\x02\x08\x01\x12\x03\x18\0(\n\x08\n\x01\x08\
    \x12\x03\x19\0!\n\t\n\x02\x08$\x12\x03\x19\0!\n\xca\x08\n\x02\x04\0\x12\
    \x043\0]\x01\x1a\xbd\x08\x20Represents\x20civil\x20time\x20(or\x20occasi\
    onally\x20physical\x20time).\n\n\x20This\x20type\x20can\x20represent\x20\
    a\x20civil\x20time\x20in\x20one\x20of\x20a\x20few\x20possible\x20ways:\n\
    \n\x20\x20*\x20When\x20utc_offset\x20is\x20set\x20and\x20time_zone\x20is\
    \x20unset:\x20a\x20civil\x20time\x20on\x20a\x20calendar\n\x20\x20\x20\
    \x20day\x20with\x20a\x20particular\x20offset\x20from\x20UTC.\n\x20\x20*\
    \x20When\x20time_zone\x20is\x20set\x20and\x20utc_offset\x20is\x20unset:\
    \x20a\x20civil\x20time\x20on\x20a\x20calendar\n\x20\x20\x20\x20day\x20in\
    \x20a\x20particular\x20time\x20zone.\n\x20\x20*\x20When\x20neither\x20ti\
    me_zone\x20nor\x20utc_offset\x20is\x20set:\x20a\x20civil\x20time\x20on\
    \x20a\x20calendar\n\x20\x20\x20\x20day\x20in\x20local\x20time.\n\n\x20Th\
    e\x20date\x20is\x20relative\x20to\x20the\x20Proleptic\x20Gregorian\x20Ca\
    lendar.\n\n\x20If\x20year\x20is\x200,\x20the\x20DateTime\x20is\x20consid\
    ered\x20not\x20to\x20have\x20a\x20specific\x20year.\x20month\n\x20and\
    \x20day\x20must\x20have\x20valid,\x20non-zero\x20values.\n\n\x20This\x20\
    type\x20may\x20also\x20be\x20used\x20to\x20represent\x20a\x20physical\
    \x20time\x20if\x20all\x20the\x20date\x20and\n\x20time\x20fields\x20are\
    \x20set\x20and\x20either\x20case\x20of\x20the\x20`time_offset`\x20oneof\
    \x20is\x20set.\n\x20Consider\x20using\x20`Timestamp`\x20message\x20for\
    \x20physical\x20time\x20instead.\x20If\x20your\x20use\n\x20case\x20also\
    \x20would\x20like\x20to\x20store\x20the\x20user's\x20timezone,\x20that\
    \x20can\x20be\x20done\x20in\n\x20another\x20field.\n\n\x20This\x20type\
    \x20is\x20more\x20flexible\x20than\x20some\x20applications\x20may\x20wan\
    t.\x20Make\x20sure\x20to\n\x20document\x20and\x20validate\x20your\x20app\
    lication's\x20limitations.\n\n\n\n\x03\x04\0\x01\x12\x033\x08\x10\nm\n\
    \x04\x04\0\x02\0\x12\x036\x02\x11\x1a`\x20Optional.\x20Year\x20of\x20dat\
    e.\x20Must\x20be\x20from\x201\x20to\x209999,\x20or\x200\x20if\x20specify\
    ing\x20a\n\x20datetime\x20without\x20a\x20year.\n\n\x0c\n\x05\x04\0\x02\
    \0\x05\x12\x036\x02\x07\n\x0c\n\x05\x04\0\x02\0\x01\x12\x036\x08\x0c\n\
    \x0c\n\x05\x04\0\x02\0\x03\x12\x036\x0f\x10\n=\n\x04\x04\0\x02\x01\x12\
    \x039\x02\x12\x1a0\x20Required.\x20Month\x20of\x20year.\x20Must\x20be\
    \x20from\x201\x20to\x2012.\n\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x039\x02\
    \x07\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x039\x08\r\n\x0c\n\x05\x04\0\x02\
    \x01\x03\x12\x039\x10\x11\n^\n\x04\x04\0\x02\x02\x12\x03=\x02\x10\x1aQ\
    \x20Required.\x20Day\x20of\x20month.\x20Must\x20be\x20from\x201\x20to\
    \x2031\x20and\x20valid\x20for\x20the\x20year\x20and\n\x20month.\n\n\x0c\
    \n\x05\x04\0\x02\x02\x05\x12\x03=\x02\x07\n\x0c\n\x05\x04\0\x02\x02\x01\
    \x12\x03=\x08\x0b\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03=\x0e\x0f\n\xad\
    \x01\n\x04\x04\0\x02\x03\x12\x03B\x02\x12\x1a\x9f\x01\x20Required.\x20Ho\
    urs\x20of\x20day\x20in\x2024\x20hour\x20format.\x20Should\x20be\x20from\
    \x200\x20to\x2023.\x20An\x20API\n\x20may\x20choose\x20to\x20allow\x20the\
    \x20value\x20\"24:00:00\"\x20for\x20scenarios\x20like\x20business\n\x20c\
    losing\x20time.\n\n\x0c\n\x05\x04\0\x02\x03\x05\x12\x03B\x02\x07\n\x0c\n\
    \x05\x04\0\x02\x03\x01\x12\x03B\x08\r\n\x0c\n\x05\x04\0\x02\x03\x03\x12\
    \x03B\x10\x11\nF\n\x04\x04\0\x02\x04\x12\x03E\x02\x14\x1a9\x20Required.\
    \x20Minutes\x20of\x20hour\x20of\x20day.\x20Must\x20be\x20from\x200\x20to\
    \x2059.\n\n\x0c\n\x05\x04\0\x02\x04\x05\x12\x03E\x02\x07\n\x0c\n\x05\x04\
    \0\x02\x04\x01\x12\x03E\x08\x0f\n\x0c\n\x05\x04\0\x02\x04\x03\x12\x03E\
    \x12\x13\n\x92\x01\n\x04\x04\0\x02\x05\x12\x03I\x02\x14\x1a\x84\x01\x20R\
    equired.\x20Seconds\x20of\x20minutes\x20of\x20the\x20time.\x20Must\x20no\
    rmally\x20be\x20from\x200\x20to\x2059.\x20An\n\x20API\x20may\x20allow\
    \x20the\x20value\x2060\x20if\x20it\x20allows\x20leap-seconds.\n\n\x0c\n\
    \x05\x04\0\x02\x05\x05\x12\x03I\x02\x07\n\x0c\n\x05\x04\0\x02\x05\x01\
    \x12\x03I\x08\x0f\n\x0c\n\x05\x04\0\x02\x05\x03\x12\x03I\x12\x13\n]\n\
    \x04\x04\0\x02\x06\x12\x03M\x02\x12\x1aP\x20Required.\x20Fractions\x20of\
    \x20seconds\x20in\x20nanoseconds.\x20Must\x20be\x20from\x200\x20to\n\x20\
    999,999,999.\n\n\x0c\n\x05\x04\0\x02\x06\x05\x12\x03M\x02\x07\n\x0c\n\
    \x05\x04\0\x02\x06\x01\x12\x03M\x08\r\n\x0c\n\x05\x04\0\x02\x06\x03\x12\
    \x03M\x10\x11\n\xf5\x02\n\x04\x04\0\x08\0\x12\x04T\x02\\\x03\x1a\xe6\x02\
    \x20Optional.\x20Specifies\x20either\x20the\x20UTC\x20offset\x20or\x20th\
    e\x20time\x20zone\x20of\x20the\x20DateTime.\n\x20Choose\x20carefully\x20\
    between\x20them,\x20considering\x20that\x20time\x20zone\x20data\x20may\
    \x20change\n\x20in\x20the\x20future\x20(for\x20example,\x20a\x20country\
    \x20modifies\x20their\x20DST\x20start/end\x20dates,\n\x20and\x20future\
    \x20DateTimes\x20in\x20the\x20affected\x20range\x20had\x20already\x20bee\
    n\x20stored).\n\x20If\x20omitted,\x20the\x20DateTime\x20is\x20considered\
    \x20to\x20be\x20in\x20local\x20time.\n\n\x0c\n\x05\x04\0\x08\0\x01\x12\
    \x03T\x08\x13\n\xa5\x01\n\x04\x04\0\x02\x07\x12\x03X\x04,\x1a\x97\x01\
    \x20UTC\x20offset.\x20Must\x20be\x20whole\x20seconds,\x20between\x20-18\
    \x20hours\x20and\x20+18\x20hours.\n\x20For\x20example,\x20a\x20UTC\x20of\
    fset\x20of\x20-4:00\x20would\x20be\x20represented\x20as\n\x20{\x20second\
    s:\x20-14400\x20}.\n\n\x0c\n\x05\x04\0\x02\x07\x06\x12\x03X\x04\x1c\n\
    \x0c\n\x05\x04\0\x02\x07\x01\x12\x03X\x1d'\n\x0c\n\x05\x04\0\x02\x07\x03\
    \x12\x03X*+\n\x19\n\x04\x04\0\x02\x08\x12\x03[\x04\x1b\x1a\x0c\x20Time\
    \x20zone.\n\n\x0c\n\x05\x04\0\x02\x08\x06\x12\x03[\x04\x0c\n\x0c\n\x05\
    \x04\0\x02\x08\x01\x12\x03[\r\x16\n\x0c\n\x05\x04\0\x02\x08\x03\x12\x03[\
    \x19\x1a\nj\n\x02\x04\x01\x12\x04a\0g\x01\x1a^\x20Represents\x20a\x20tim\
    e\x20zone\x20from\x20the\n\x20[IANA\x20Time\x20Zone\x20Database](https:/\
    /www.iana.org/time-zones).\n\n\n\n\x03\x04\x01\x01\x12\x03a\x08\x10\nJ\n\
    \x04\x04\x01\x02\0\x12\x03c\x02\x10\x1a=\x20IANA\x20Time\x20Zone\x20Data\
    base\x20time\x20zone,\x20e.g.\x20\"America/New_York\".\n\n\x0c\n\x05\x04\
    \x01\x02\0\x05\x12\x03c\x02\x08\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03c\t\
    \x0b\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03c\x0e\x0f\nN\n\x04\x04\x01\x02\
    \x01\x12\x03f\x02\x15\x1aA\x20Optional.\x20IANA\x20Time\x20Zone\x20Datab\
    ase\x20version\x20number,\x20e.g.\x20\"2019a\".\n\n\x0c\n\x05\x04\x01\
    \x02\x01\x05\x12\x03f\x02\x08\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\x03f\t\
    \x10\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03f\x13\x14b\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
