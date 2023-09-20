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
//! Generated file from `google/type/dayofweek.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_28_0;

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum DayOfWeek {
    DAY_OF_WEEK_UNSPECIFIED = 0,
    MONDAY = 1,
    TUESDAY = 2,
    WEDNESDAY = 3,
    THURSDAY = 4,
    FRIDAY = 5,
    SATURDAY = 6,
    SUNDAY = 7,
}

impl ::protobuf::ProtobufEnum for DayOfWeek {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<DayOfWeek> {
        match value {
            0 => ::std::option::Option::Some(DayOfWeek::DAY_OF_WEEK_UNSPECIFIED),
            1 => ::std::option::Option::Some(DayOfWeek::MONDAY),
            2 => ::std::option::Option::Some(DayOfWeek::TUESDAY),
            3 => ::std::option::Option::Some(DayOfWeek::WEDNESDAY),
            4 => ::std::option::Option::Some(DayOfWeek::THURSDAY),
            5 => ::std::option::Option::Some(DayOfWeek::FRIDAY),
            6 => ::std::option::Option::Some(DayOfWeek::SATURDAY),
            7 => ::std::option::Option::Some(DayOfWeek::SUNDAY),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [DayOfWeek] = &[
            DayOfWeek::DAY_OF_WEEK_UNSPECIFIED,
            DayOfWeek::MONDAY,
            DayOfWeek::TUESDAY,
            DayOfWeek::WEDNESDAY,
            DayOfWeek::THURSDAY,
            DayOfWeek::FRIDAY,
            DayOfWeek::SATURDAY,
            DayOfWeek::SUNDAY,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<DayOfWeek>("DayOfWeek", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for DayOfWeek {
}

impl ::std::default::Default for DayOfWeek {
    fn default() -> Self {
        DayOfWeek::DAY_OF_WEEK_UNSPECIFIED
    }
}

impl ::protobuf::reflect::ProtobufValue for DayOfWeek {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1bgoogle/type/dayofweek.proto\x12\x0bgoogle.type*\x84\x01\n\tDayOfWe\
    ek\x12\x1b\n\x17DAY_OF_WEEK_UNSPECIFIED\x10\0\x12\n\n\x06MONDAY\x10\x01\
    \x12\x0b\n\x07TUESDAY\x10\x02\x12\r\n\tWEDNESDAY\x10\x03\x12\x0c\n\x08TH\
    URSDAY\x10\x04\x12\n\n\x06FRIDAY\x10\x05\x12\x0c\n\x08SATURDAY\x10\x06\
    \x12\n\n\x06SUNDAY\x10\x07Bi\n\x0fcom.google.typeB\x0eDayOfWeekProtoP\
    \x01Z>google.golang.org/genproto/googleapis/type/dayofweek;dayofweek\xa2\
    \x02\x03GTPJ\xb0\t\n\x06\x12\x04\x0e\01\x01\n\xbc\x04\n\x01\x0c\x12\x03\
    \x0e\0\x122\xb1\x04\x20Copyright\x202021\x20Google\x20LLC\n\n\x20License\
    d\x20under\x20the\x20Apache\x20License,\x20Version\x202.0\x20(the\x20\"L\
    icense\");\n\x20you\x20may\x20not\x20use\x20this\x20file\x20except\x20in\
    \x20compliance\x20with\x20the\x20License.\n\x20You\x20may\x20obtain\x20a\
    \x20copy\x20of\x20the\x20License\x20at\n\n\x20\x20\x20\x20\x20http://www\
    .apache.org/licenses/LICENSE-2.0\n\n\x20Unless\x20required\x20by\x20appl\
    icable\x20law\x20or\x20agreed\x20to\x20in\x20writing,\x20software\n\x20d\
    istributed\x20under\x20the\x20License\x20is\x20distributed\x20on\x20an\
    \x20\"AS\x20IS\"\x20BASIS,\n\x20WITHOUT\x20WARRANTIES\x20OR\x20CONDITION\
    S\x20OF\x20ANY\x20KIND,\x20either\x20express\x20or\x20implied.\n\x20See\
    \x20the\x20License\x20for\x20the\x20specific\x20language\x20governing\
    \x20permissions\x20and\n\x20limitations\x20under\x20the\x20License.\n\n\
    \x08\n\x01\x02\x12\x03\x10\0\x14\n\x08\n\x01\x08\x12\x03\x12\0U\n\t\n\
    \x02\x08\x0b\x12\x03\x12\0U\n\x08\n\x01\x08\x12\x03\x13\0\"\n\t\n\x02\
    \x08\n\x12\x03\x13\0\"\n\x08\n\x01\x08\x12\x03\x14\0/\n\t\n\x02\x08\x08\
    \x12\x03\x14\0/\n\x08\n\x01\x08\x12\x03\x15\0(\n\t\n\x02\x08\x01\x12\x03\
    \x15\0(\n\x08\n\x01\x08\x12\x03\x16\0!\n\t\n\x02\x08$\x12\x03\x16\0!\n+\
    \n\x02\x05\0\x12\x04\x19\01\x01\x1a\x1f\x20Represents\x20a\x20day\x20of\
    \x20the\x20week.\n\n\n\n\x03\x05\0\x01\x12\x03\x19\x05\x0e\n2\n\x04\x05\
    \0\x02\0\x12\x03\x1b\x02\x1e\x1a%\x20The\x20day\x20of\x20the\x20week\x20\
    is\x20unspecified.\n\n\x0c\n\x05\x05\0\x02\0\x01\x12\x03\x1b\x02\x19\n\
    \x0c\n\x05\x05\0\x02\0\x02\x12\x03\x1b\x1c\x1d\n\x15\n\x04\x05\0\x02\x01\
    \x12\x03\x1e\x02\r\x1a\x08\x20Monday\n\n\x0c\n\x05\x05\0\x02\x01\x01\x12\
    \x03\x1e\x02\x08\n\x0c\n\x05\x05\0\x02\x01\x02\x12\x03\x1e\x0b\x0c\n\x16\
    \n\x04\x05\0\x02\x02\x12\x03!\x02\x0e\x1a\t\x20Tuesday\n\n\x0c\n\x05\x05\
    \0\x02\x02\x01\x12\x03!\x02\t\n\x0c\n\x05\x05\0\x02\x02\x02\x12\x03!\x0c\
    \r\n\x18\n\x04\x05\0\x02\x03\x12\x03$\x02\x10\x1a\x0b\x20Wednesday\n\n\
    \x0c\n\x05\x05\0\x02\x03\x01\x12\x03$\x02\x0b\n\x0c\n\x05\x05\0\x02\x03\
    \x02\x12\x03$\x0e\x0f\n\x17\n\x04\x05\0\x02\x04\x12\x03'\x02\x0f\x1a\n\
    \x20Thursday\n\n\x0c\n\x05\x05\0\x02\x04\x01\x12\x03'\x02\n\n\x0c\n\x05\
    \x05\0\x02\x04\x02\x12\x03'\r\x0e\n\x15\n\x04\x05\0\x02\x05\x12\x03*\x02\
    \r\x1a\x08\x20Friday\n\n\x0c\n\x05\x05\0\x02\x05\x01\x12\x03*\x02\x08\n\
    \x0c\n\x05\x05\0\x02\x05\x02\x12\x03*\x0b\x0c\n\x17\n\x04\x05\0\x02\x06\
    \x12\x03-\x02\x0f\x1a\n\x20Saturday\n\n\x0c\n\x05\x05\0\x02\x06\x01\x12\
    \x03-\x02\n\n\x0c\n\x05\x05\0\x02\x06\x02\x12\x03-\r\x0e\n\x15\n\x04\x05\
    \0\x02\x07\x12\x030\x02\r\x1a\x08\x20Sunday\n\n\x0c\n\x05\x05\0\x02\x07\
    \x01\x12\x030\x02\x08\n\x0c\n\x05\x05\0\x02\x07\x02\x12\x030\x0b\x0cb\
    \x06proto3\
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
