// This file is generated by rust-protobuf 2.28.0. Do not edit
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
//! Generated file from `google/type/timeofday.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_28_0;

#[derive(PartialEq,Clone,Default)]
pub struct TimeOfDay {
    // message fields
    pub hours: i32,
    pub minutes: i32,
    pub seconds: i32,
    pub nanos: i32,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a TimeOfDay {
    fn default() -> &'a TimeOfDay {
        <TimeOfDay as ::protobuf::Message>::default_instance()
    }
}

impl TimeOfDay {
    pub fn new() -> TimeOfDay {
        ::std::default::Default::default()
    }

    // int32 hours = 1;


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

    // int32 minutes = 2;


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

    // int32 seconds = 3;


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

    // int32 nanos = 4;


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
}

impl ::protobuf::Message for TimeOfDay {
    fn is_initialized(&self) -> bool {
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
                    self.hours = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.minutes = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.seconds = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.nanos = tmp;
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
        if self.hours != 0 {
            my_size += ::protobuf::rt::value_size(1, self.hours, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.minutes != 0 {
            my_size += ::protobuf::rt::value_size(2, self.minutes, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.seconds != 0 {
            my_size += ::protobuf::rt::value_size(3, self.seconds, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.nanos != 0 {
            my_size += ::protobuf::rt::value_size(4, self.nanos, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.hours != 0 {
            os.write_int32(1, self.hours)?;
        }
        if self.minutes != 0 {
            os.write_int32(2, self.minutes)?;
        }
        if self.seconds != 0 {
            os.write_int32(3, self.seconds)?;
        }
        if self.nanos != 0 {
            os.write_int32(4, self.nanos)?;
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

    fn new() -> TimeOfDay {
        TimeOfDay::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                "hours",
                |m: &TimeOfDay| { &m.hours },
                |m: &mut TimeOfDay| { &mut m.hours },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                "minutes",
                |m: &TimeOfDay| { &m.minutes },
                |m: &mut TimeOfDay| { &mut m.minutes },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                "seconds",
                |m: &TimeOfDay| { &m.seconds },
                |m: &mut TimeOfDay| { &mut m.seconds },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                "nanos",
                |m: &TimeOfDay| { &m.nanos },
                |m: &mut TimeOfDay| { &mut m.nanos },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<TimeOfDay>(
                "TimeOfDay",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static TimeOfDay {
        static instance: ::protobuf::rt::LazyV2<TimeOfDay> = ::protobuf::rt::LazyV2::INIT;
        instance.get(TimeOfDay::new)
    }
}

impl ::protobuf::Clear for TimeOfDay {
    fn clear(&mut self) {
        self.hours = 0;
        self.minutes = 0;
        self.seconds = 0;
        self.nanos = 0;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TimeOfDay {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TimeOfDay {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1bgoogle/type/timeofday.proto\x12\x0bgoogle.type\"k\n\tTimeOfDay\x12\
    \x14\n\x05hours\x18\x01\x20\x01(\x05R\x05hours\x12\x18\n\x07minutes\x18\
    \x02\x20\x01(\x05R\x07minutes\x12\x18\n\x07seconds\x18\x03\x20\x01(\x05R\
    \x07seconds\x12\x14\n\x05nanos\x18\x04\x20\x01(\x05R\x05nanosBl\n\x0fcom\
    .google.typeB\x0eTimeOfDayProtoP\x01Z>google.golang.org/genproto/googlea\
    pis/type/timeofday;timeofday\xf8\x01\x01\xa2\x02\x03GTPJ\xc2\x0c\n\x06\
    \x12\x04\x0f\0+\x01\n\xbe\x04\n\x01\x0c\x12\x03\x0f\0\x122\xb3\x04\x20Co\
    pyright\x202019\x20Google\x20LLC.\n\n\x20Licensed\x20under\x20the\x20Apa\
    che\x20License,\x20Version\x202.0\x20(the\x20\"License\");\n\x20you\x20m\
    ay\x20not\x20use\x20this\x20file\x20except\x20in\x20compliance\x20with\
    \x20the\x20License.\n\x20You\x20may\x20obtain\x20a\x20copy\x20of\x20the\
    \x20License\x20at\n\n\x20\x20\x20\x20\x20http://www.apache.org/licenses/\
    LICENSE-2.0\n\n\x20Unless\x20required\x20by\x20applicable\x20law\x20or\
    \x20agreed\x20to\x20in\x20writing,\x20software\n\x20distributed\x20under\
    \x20the\x20License\x20is\x20distributed\x20on\x20an\x20\"AS\x20IS\"\x20B\
    ASIS,\n\x20WITHOUT\x20WARRANTIES\x20OR\x20CONDITIONS\x20OF\x20ANY\x20KIN\
    D,\x20either\x20express\x20or\x20implied.\n\x20See\x20the\x20License\x20\
    for\x20the\x20specific\x20language\x20governing\x20permissions\x20and\n\
    \x20limitations\x20under\x20the\x20License.\n\n\n\x08\n\x01\x02\x12\x03\
    \x11\0\x14\n\x08\n\x01\x08\x12\x03\x13\0\x1f\n\t\n\x02\x08\x1f\x12\x03\
    \x13\0\x1f\n\x08\n\x01\x08\x12\x03\x14\0U\n\t\n\x02\x08\x0b\x12\x03\x14\
    \0U\n\x08\n\x01\x08\x12\x03\x15\0\"\n\t\n\x02\x08\n\x12\x03\x15\0\"\n\
    \x08\n\x01\x08\x12\x03\x16\0/\n\t\n\x02\x08\x08\x12\x03\x16\0/\n\x08\n\
    \x01\x08\x12\x03\x17\0(\n\t\n\x02\x08\x01\x12\x03\x17\0(\n\x08\n\x01\x08\
    \x12\x03\x18\0!\n\t\n\x02\x08$\x12\x03\x18\0!\n\xf9\x01\n\x02\x04\0\x12\
    \x04\x1d\0+\x01\x1a\xec\x01\x20Represents\x20a\x20time\x20of\x20day.\x20\
    The\x20date\x20and\x20time\x20zone\x20are\x20either\x20not\x20significan\
    t\n\x20or\x20are\x20specified\x20elsewhere.\x20An\x20API\x20may\x20choos\
    e\x20to\x20allow\x20leap\x20seconds.\x20Related\n\x20types\x20are\x20[go\
    ogle.type.Date][google.type.Date]\x20and\x20`google.protobuf.Timestamp`.\
    \n\n\n\n\x03\x04\0\x01\x12\x03\x1d\x08\x11\n\xa2\x01\n\x04\x04\0\x02\0\
    \x12\x03\x20\x02\x12\x1a\x94\x01\x20Hours\x20of\x20day\x20in\x2024\x20ho\
    ur\x20format.\x20Should\x20be\x20from\x200\x20to\x2023.\x20An\x20API\x20\
    may\x20choose\n\x20to\x20allow\x20the\x20value\x20\"24:00:00\"\x20for\
    \x20scenarios\x20like\x20business\x20closing\x20time.\n\n\x0c\n\x05\x04\
    \0\x02\0\x05\x12\x03\x20\x02\x07\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x20\
    \x08\r\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x20\x10\x11\n<\n\x04\x04\0\
    \x02\x01\x12\x03#\x02\x14\x1a/\x20Minutes\x20of\x20hour\x20of\x20day.\
    \x20Must\x20be\x20from\x200\x20to\x2059.\n\n\x0c\n\x05\x04\0\x02\x01\x05\
    \x12\x03#\x02\x07\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03#\x08\x0f\n\x0c\n\
    \x05\x04\0\x02\x01\x03\x12\x03#\x12\x13\n\x87\x01\n\x04\x04\0\x02\x02\
    \x12\x03'\x02\x14\x1az\x20Seconds\x20of\x20minutes\x20of\x20the\x20time.\
    \x20Must\x20normally\x20be\x20from\x200\x20to\x2059.\x20An\x20API\x20may\
    \n\x20allow\x20the\x20value\x2060\x20if\x20it\x20allows\x20leap-seconds.\
    \n\n\x0c\n\x05\x04\0\x02\x02\x05\x12\x03'\x02\x07\n\x0c\n\x05\x04\0\x02\
    \x02\x01\x12\x03'\x08\x0f\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03'\x12\x13\
    \nR\n\x04\x04\0\x02\x03\x12\x03*\x02\x12\x1aE\x20Fractions\x20of\x20seco\
    nds\x20in\x20nanoseconds.\x20Must\x20be\x20from\x200\x20to\x20999,999,99\
    9.\n\n\x0c\n\x05\x04\0\x02\x03\x05\x12\x03*\x02\x07\n\x0c\n\x05\x04\0\
    \x02\x03\x01\x12\x03*\x08\r\n\x0c\n\x05\x04\0\x02\x03\x03\x12\x03*\x10\
    \x11b\x06proto3\
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
