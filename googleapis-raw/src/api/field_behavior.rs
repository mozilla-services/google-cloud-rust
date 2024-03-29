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
//! Generated file from `google/api/field_behavior.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_28_0;

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum FieldBehavior {
    FIELD_BEHAVIOR_UNSPECIFIED = 0,
    OPTIONAL = 1,
    REQUIRED = 2,
    OUTPUT_ONLY = 3,
    INPUT_ONLY = 4,
    IMMUTABLE = 5,
    UNORDERED_LIST = 6,
    NON_EMPTY_DEFAULT = 7,
    IDENTIFIER = 8,
}

impl ::protobuf::ProtobufEnum for FieldBehavior {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<FieldBehavior> {
        match value {
            0 => ::std::option::Option::Some(FieldBehavior::FIELD_BEHAVIOR_UNSPECIFIED),
            1 => ::std::option::Option::Some(FieldBehavior::OPTIONAL),
            2 => ::std::option::Option::Some(FieldBehavior::REQUIRED),
            3 => ::std::option::Option::Some(FieldBehavior::OUTPUT_ONLY),
            4 => ::std::option::Option::Some(FieldBehavior::INPUT_ONLY),
            5 => ::std::option::Option::Some(FieldBehavior::IMMUTABLE),
            6 => ::std::option::Option::Some(FieldBehavior::UNORDERED_LIST),
            7 => ::std::option::Option::Some(FieldBehavior::NON_EMPTY_DEFAULT),
            8 => ::std::option::Option::Some(FieldBehavior::IDENTIFIER),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [FieldBehavior] = &[
            FieldBehavior::FIELD_BEHAVIOR_UNSPECIFIED,
            FieldBehavior::OPTIONAL,
            FieldBehavior::REQUIRED,
            FieldBehavior::OUTPUT_ONLY,
            FieldBehavior::INPUT_ONLY,
            FieldBehavior::IMMUTABLE,
            FieldBehavior::UNORDERED_LIST,
            FieldBehavior::NON_EMPTY_DEFAULT,
            FieldBehavior::IDENTIFIER,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<FieldBehavior>("FieldBehavior", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for FieldBehavior {
}

impl ::std::default::Default for FieldBehavior {
    fn default() -> Self {
        FieldBehavior::FIELD_BEHAVIOR_UNSPECIFIED
    }
}

impl ::protobuf::reflect::ProtobufValue for FieldBehavior {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

/// Extension fields
pub mod exts {

    pub const field_behavior: ::protobuf::ext::ExtFieldRepeated<::protobuf::descriptor::FieldOptions, ::protobuf::types::ProtobufTypeEnum<super::FieldBehavior>> = ::protobuf::ext::ExtFieldRepeated { field_number: 1052, phantom: ::std::marker::PhantomData };
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1fgoogle/api/field_behavior.proto\x12\ngoogle.api\x1a\x20google/prot\
    obuf/descriptor.proto*\xb6\x01\n\rFieldBehavior\x12\x1e\n\x1aFIELD_BEHAV\
    IOR_UNSPECIFIED\x10\0\x12\x0c\n\x08OPTIONAL\x10\x01\x12\x0c\n\x08REQUIRE\
    D\x10\x02\x12\x0f\n\x0bOUTPUT_ONLY\x10\x03\x12\x0e\n\nINPUT_ONLY\x10\x04\
    \x12\r\n\tIMMUTABLE\x10\x05\x12\x12\n\x0eUNORDERED_LIST\x10\x06\x12\x15\
    \n\x11NON_EMPTY_DEFAULT\x10\x07\x12\x0e\n\nIDENTIFIER\x10\x08:`\n\x0efie\
    ld_behavior\x18\x9c\x08\x20\x03(\x0e2\x19.google.api.FieldBehavior\x12\
    \x1d.google.protobuf.FieldOptionsR\rfieldBehaviorBp\n\x0ecom.google.apiB\
    \x12FieldBehaviorProtoP\x01ZAgoogle.golang.org/genproto/googleapis/api/a\
    nnotations;annotations\xa2\x02\x04GAPIJ\xe2\x1f\n\x06\x12\x04\x0e\0g\x01\
    \n\xbc\x04\n\x01\x0c\x12\x03\x0e\0\x122\xb1\x04\x20Copyright\x202023\x20\
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
    r\x20the\x20License.\n\n\x08\n\x01\x02\x12\x03\x10\0\x13\n\t\n\x02\x03\0\
    \x12\x03\x12\0*\n\x08\n\x01\x08\x12\x03\x14\0X\n\t\n\x02\x08\x0b\x12\x03\
    \x14\0X\n\x08\n\x01\x08\x12\x03\x15\0\"\n\t\n\x02\x08\n\x12\x03\x15\0\"\
    \n\x08\n\x01\x08\x12\x03\x16\03\n\t\n\x02\x08\x08\x12\x03\x16\03\n\x08\n\
    \x01\x08\x12\x03\x17\0'\n\t\n\x02\x08\x01\x12\x03\x17\0'\n\x08\n\x01\x08\
    \x12\x03\x18\0\"\n\t\n\x02\x08$\x12\x03\x18\0\"\n\t\n\x01\x07\x12\x04\
    \x1a\0(\x01\n\xda\x03\n\x02\x07\0\x12\x03'\x02:\x1a\xce\x03\x20A\x20desi\
    gnation\x20of\x20a\x20specific\x20field\x20behavior\x20(required,\x20out\
    put\x20only,\x20etc.)\n\x20in\x20protobuf\x20messages.\n\n\x20Examples:\
    \n\n\x20\x20\x20string\x20name\x20=\x201\x20[(google.api.field_behavior)\
    \x20=\x20REQUIRED];\n\x20\x20\x20State\x20state\x20=\x201\x20[(google.ap\
    i.field_behavior)\x20=\x20OUTPUT_ONLY];\n\x20\x20\x20google.protobuf.Dur\
    ation\x20ttl\x20=\x201\n\x20\x20\x20\x20\x20[(google.api.field_behavior)\
    \x20=\x20INPUT_ONLY];\n\x20\x20\x20google.protobuf.Timestamp\x20expire_t\
    ime\x20=\x201\n\x20\x20\x20\x20\x20[(google.api.field_behavior)\x20=\x20\
    OUTPUT_ONLY,\n\x20\x20\x20\x20\x20\x20(google.api.field_behavior)\x20=\
    \x20IMMUTABLE];\n\n\n\n\x03\x07\0\x02\x12\x03\x1a\x07#\n\n\n\x03\x07\0\
    \x04\x12\x03'\x02\n\n\n\n\x03\x07\0\x06\x12\x03'\x0b#\n\n\n\x03\x07\0\
    \x01\x12\x03'$2\n\n\n\x03\x07\0\x03\x12\x03'59\n\xea\x02\n\x02\x05\0\x12\
    \x040\0g\x01\x1a\xdd\x02\x20An\x20indicator\x20of\x20the\x20behavior\x20\
    of\x20a\x20given\x20field\x20(for\x20example,\x20that\x20a\x20field\n\
    \x20is\x20required\x20in\x20requests,\x20or\x20given\x20as\x20output\x20\
    but\x20ignored\x20as\x20input).\n\x20This\x20**does\x20not**\x20change\
    \x20the\x20behavior\x20in\x20protocol\x20buffers\x20itself;\x20it\x20onl\
    y\n\x20denotes\x20the\x20behavior\x20and\x20may\x20affect\x20how\x20API\
    \x20tooling\x20handles\x20the\x20field.\n\n\x20Note:\x20This\x20enum\x20\
    **may**\x20receive\x20new\x20values\x20in\x20the\x20future.\n\n\n\n\x03\
    \x05\0\x01\x12\x030\x05\x12\n?\n\x04\x05\0\x02\0\x12\x032\x02!\x1a2\x20C\
    onventional\x20default\x20for\x20enums.\x20Do\x20not\x20use\x20this.\n\n\
    \x0c\n\x05\x05\0\x02\0\x01\x12\x032\x02\x1c\n\x0c\n\x05\x05\0\x02\0\x02\
    \x12\x032\x1f\x20\n\xa1\x01\n\x04\x05\0\x02\x01\x12\x037\x02\x0f\x1a\x93\
    \x01\x20Specifically\x20denotes\x20a\x20field\x20as\x20optional.\n\x20Wh\
    ile\x20all\x20fields\x20in\x20protocol\x20buffers\x20are\x20optional,\
    \x20this\x20may\x20be\x20specified\n\x20for\x20emphasis\x20if\x20appropr\
    iate.\n\n\x0c\n\x05\x05\0\x02\x01\x01\x12\x037\x02\n\n\x0c\n\x05\x05\0\
    \x02\x01\x02\x12\x037\r\x0e\n\xc0\x01\n\x04\x05\0\x02\x02\x12\x03<\x02\
    \x0f\x1a\xb2\x01\x20Denotes\x20a\x20field\x20as\x20required.\n\x20This\
    \x20indicates\x20that\x20the\x20field\x20**must**\x20be\x20provided\x20a\
    s\x20part\x20of\x20the\x20request,\n\x20and\x20failure\x20to\x20do\x20so\
    \x20will\x20cause\x20an\x20error\x20(usually\x20`INVALID_ARGUMENT`).\n\n\
    \x0c\n\x05\x05\0\x02\x02\x01\x12\x03<\x02\n\n\x0c\n\x05\x05\0\x02\x02\
    \x02\x12\x03<\r\x0e\n\xfd\x01\n\x04\x05\0\x02\x03\x12\x03B\x02\x12\x1a\
    \xef\x01\x20Denotes\x20a\x20field\x20as\x20output\x20only.\n\x20This\x20\
    indicates\x20that\x20the\x20field\x20is\x20provided\x20in\x20responses,\
    \x20but\x20including\x20the\n\x20field\x20in\x20a\x20request\x20does\x20\
    nothing\x20(the\x20server\x20*must*\x20ignore\x20it\x20and\n\x20*must\
    \x20not*\x20throw\x20an\x20error\x20as\x20a\x20result\x20of\x20the\x20fi\
    eld's\x20presence).\n\n\x0c\n\x05\x05\0\x02\x03\x01\x12\x03B\x02\r\n\x0c\
    \n\x05\x05\0\x02\x03\x02\x12\x03B\x10\x11\n\x9e\x01\n\x04\x05\0\x02\x04\
    \x12\x03G\x02\x11\x1a\x90\x01\x20Denotes\x20a\x20field\x20as\x20input\
    \x20only.\n\x20This\x20indicates\x20that\x20the\x20field\x20is\x20provid\
    ed\x20in\x20requests,\x20and\x20the\n\x20corresponding\x20field\x20is\
    \x20not\x20included\x20in\x20output.\n\n\x0c\n\x05\x05\0\x02\x04\x01\x12\
    \x03G\x02\x0c\n\x0c\n\x05\x05\0\x02\x04\x02\x12\x03G\x0f\x10\n\xa3\x01\n\
    \x04\x05\0\x02\x05\x12\x03L\x02\x10\x1a\x95\x01\x20Denotes\x20a\x20field\
    \x20as\x20immutable.\n\x20This\x20indicates\x20that\x20the\x20field\x20m\
    ay\x20be\x20set\x20once\x20in\x20a\x20request\x20to\x20create\x20a\n\x20\
    resource,\x20but\x20may\x20not\x20be\x20changed\x20thereafter.\n\n\x0c\n\
    \x05\x05\0\x02\x05\x01\x12\x03L\x02\x0b\n\x0c\n\x05\x05\0\x02\x05\x02\
    \x12\x03L\x0e\x0f\n\x93\x02\n\x04\x05\0\x02\x06\x12\x03R\x02\x15\x1a\x85\
    \x02\x20Denotes\x20that\x20a\x20(repeated)\x20field\x20is\x20an\x20unord\
    ered\x20list.\n\x20This\x20indicates\x20that\x20the\x20service\x20may\
    \x20provide\x20the\x20elements\x20of\x20the\x20list\n\x20in\x20any\x20ar\
    bitrary\x20\x20order,\x20rather\x20than\x20the\x20order\x20the\x20user\
    \x20originally\n\x20provided.\x20Additionally,\x20the\x20list's\x20order\
    \x20may\x20or\x20may\x20not\x20be\x20stable.\n\n\x0c\n\x05\x05\0\x02\x06\
    \x01\x12\x03R\x02\x10\n\x0c\n\x05\x05\0\x02\x06\x02\x12\x03R\x13\x14\n\
    \x81\x02\n\x04\x05\0\x02\x07\x12\x03X\x02\x18\x1a\xf3\x01\x20Denotes\x20\
    that\x20this\x20field\x20returns\x20a\x20non-empty\x20default\x20value\
    \x20if\x20not\x20set.\n\x20This\x20indicates\x20that\x20if\x20the\x20use\
    r\x20provides\x20the\x20empty\x20value\x20in\x20a\x20request,\n\x20a\x20\
    non-empty\x20value\x20will\x20be\x20returned.\x20The\x20user\x20will\x20\
    not\x20be\x20aware\x20of\x20what\n\x20non-empty\x20value\x20to\x20expect\
    .\n\n\x0c\n\x05\x05\0\x02\x07\x01\x12\x03X\x02\x13\n\x0c\n\x05\x05\0\x02\
    \x07\x02\x12\x03X\x16\x17\n\xf8\x04\n\x04\x05\0\x02\x08\x12\x03f\x02\x11\
    \x1a\xea\x04\x20Denotes\x20that\x20the\x20field\x20in\x20a\x20resource\
    \x20(a\x20message\x20annotated\x20with\n\x20google.api.resource)\x20is\
    \x20used\x20in\x20the\x20resource\x20name\x20to\x20uniquely\x20identify\
    \x20the\n\x20resource.\x20For\x20AIP-compliant\x20APIs,\x20this\x20shoul\
    d\x20only\x20be\x20applied\x20to\x20the\n\x20`name`\x20field\x20on\x20th\
    e\x20resource.\n\n\x20This\x20behavior\x20should\x20not\x20be\x20applied\
    \x20to\x20references\x20to\x20other\x20resources\x20within\n\x20the\x20m\
    essage.\n\n\x20The\x20identifier\x20field\x20of\x20resources\x20often\
    \x20have\x20different\x20field\x20behavior\n\x20depending\x20on\x20the\
    \x20request\x20it\x20is\x20embedded\x20in\x20(e.g.\x20for\x20Create\x20m\
    ethods\x20name\n\x20is\x20optional\x20and\x20unused,\x20while\x20for\x20\
    Update\x20methods\x20it\x20is\x20required).\x20Instead\n\x20of\x20method\
    -specific\x20annotations,\x20only\x20`IDENTIFIER`\x20is\x20required.\n\n\
    \x0c\n\x05\x05\0\x02\x08\x01\x12\x03f\x02\x0c\n\x0c\n\x05\x05\0\x02\x08\
    \x02\x12\x03f\x0f\x10b\x06proto3\
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
