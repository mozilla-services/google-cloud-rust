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
//! Generated file from `google/identity/accesscontextmanager/type/device_resources.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_28_0;

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum DeviceEncryptionStatus {
    ENCRYPTION_UNSPECIFIED = 0,
    ENCRYPTION_UNSUPPORTED = 1,
    UNENCRYPTED = 2,
    ENCRYPTED = 3,
}

impl ::protobuf::ProtobufEnum for DeviceEncryptionStatus {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<DeviceEncryptionStatus> {
        match value {
            0 => ::std::option::Option::Some(DeviceEncryptionStatus::ENCRYPTION_UNSPECIFIED),
            1 => ::std::option::Option::Some(DeviceEncryptionStatus::ENCRYPTION_UNSUPPORTED),
            2 => ::std::option::Option::Some(DeviceEncryptionStatus::UNENCRYPTED),
            3 => ::std::option::Option::Some(DeviceEncryptionStatus::ENCRYPTED),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [DeviceEncryptionStatus] = &[
            DeviceEncryptionStatus::ENCRYPTION_UNSPECIFIED,
            DeviceEncryptionStatus::ENCRYPTION_UNSUPPORTED,
            DeviceEncryptionStatus::UNENCRYPTED,
            DeviceEncryptionStatus::ENCRYPTED,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<DeviceEncryptionStatus>("DeviceEncryptionStatus", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for DeviceEncryptionStatus {
}

impl ::std::default::Default for DeviceEncryptionStatus {
    fn default() -> Self {
        DeviceEncryptionStatus::ENCRYPTION_UNSPECIFIED
    }
}

impl ::protobuf::reflect::ProtobufValue for DeviceEncryptionStatus {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum OsType {
    OS_UNSPECIFIED = 0,
    DESKTOP_MAC = 1,
    DESKTOP_WINDOWS = 2,
    DESKTOP_LINUX = 3,
    DESKTOP_CHROME_OS = 6,
    ANDROID = 4,
    IOS = 5,
}

impl ::protobuf::ProtobufEnum for OsType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<OsType> {
        match value {
            0 => ::std::option::Option::Some(OsType::OS_UNSPECIFIED),
            1 => ::std::option::Option::Some(OsType::DESKTOP_MAC),
            2 => ::std::option::Option::Some(OsType::DESKTOP_WINDOWS),
            3 => ::std::option::Option::Some(OsType::DESKTOP_LINUX),
            6 => ::std::option::Option::Some(OsType::DESKTOP_CHROME_OS),
            4 => ::std::option::Option::Some(OsType::ANDROID),
            5 => ::std::option::Option::Some(OsType::IOS),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [OsType] = &[
            OsType::OS_UNSPECIFIED,
            OsType::DESKTOP_MAC,
            OsType::DESKTOP_WINDOWS,
            OsType::DESKTOP_LINUX,
            OsType::DESKTOP_CHROME_OS,
            OsType::ANDROID,
            OsType::IOS,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<OsType>("OsType", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for OsType {
}

impl ::std::default::Default for OsType {
    fn default() -> Self {
        OsType::OS_UNSPECIFIED
    }
}

impl ::protobuf::reflect::ProtobufValue for OsType {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum DeviceManagementLevel {
    MANAGEMENT_UNSPECIFIED = 0,
    NONE = 1,
    BASIC = 2,
    COMPLETE = 3,
}

impl ::protobuf::ProtobufEnum for DeviceManagementLevel {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<DeviceManagementLevel> {
        match value {
            0 => ::std::option::Option::Some(DeviceManagementLevel::MANAGEMENT_UNSPECIFIED),
            1 => ::std::option::Option::Some(DeviceManagementLevel::NONE),
            2 => ::std::option::Option::Some(DeviceManagementLevel::BASIC),
            3 => ::std::option::Option::Some(DeviceManagementLevel::COMPLETE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [DeviceManagementLevel] = &[
            DeviceManagementLevel::MANAGEMENT_UNSPECIFIED,
            DeviceManagementLevel::NONE,
            DeviceManagementLevel::BASIC,
            DeviceManagementLevel::COMPLETE,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<DeviceManagementLevel>("DeviceManagementLevel", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for DeviceManagementLevel {
}

impl ::std::default::Default for DeviceManagementLevel {
    fn default() -> Self {
        DeviceManagementLevel::MANAGEMENT_UNSPECIFIED
    }
}

impl ::protobuf::reflect::ProtobufValue for DeviceManagementLevel {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n@google/identity/accesscontextmanager/type/device_resources.proto\x12)\
    google.identity.accesscontextmanager.type*p\n\x16DeviceEncryptionStatus\
    \x12\x1a\n\x16ENCRYPTION_UNSPECIFIED\x10\0\x12\x1a\n\x16ENCRYPTION_UNSUP\
    PORTED\x10\x01\x12\x0f\n\x0bUNENCRYPTED\x10\x02\x12\r\n\tENCRYPTED\x10\
    \x03*\x82\x01\n\x06OsType\x12\x12\n\x0eOS_UNSPECIFIED\x10\0\x12\x0f\n\
    \x0bDESKTOP_MAC\x10\x01\x12\x13\n\x0fDESKTOP_WINDOWS\x10\x02\x12\x11\n\r\
    DESKTOP_LINUX\x10\x03\x12\x15\n\x11DESKTOP_CHROME_OS\x10\x06\x12\x0b\n\
    \x07ANDROID\x10\x04\x12\x07\n\x03IOS\x10\x05*V\n\x15DeviceManagementLeve\
    l\x12\x1a\n\x16MANAGEMENT_UNSPECIFIED\x10\0\x12\x08\n\x04NONE\x10\x01\
    \x12\t\n\x05BASIC\x10\x02\x12\x0c\n\x08COMPLETE\x10\x03B\x8d\x02\n-com.g\
    oogle.identity.accesscontextmanager.typeB\tTypeProtoP\x01ZHgoogle.golang\
    .org/genproto/googleapis/identity/accesscontextmanager/type\xaa\x02)Goog\
    le.Identity.AccessContextManager.Type\xca\x02)Google\\Identity\\AccessCo\
    ntextManager\\Type\xea\x02,Google::Identity::AccessContextManager::TypeJ\
    \xb8\x13\n\x06\x12\x04\x0e\0R\x01\n\xbc\x04\n\x01\x0c\x12\x03\x0e\0\x122\
    \xb1\x04\x20Copyright\x202020\x20Google\x20LLC\n\n\x20Licensed\x20under\
    \x20the\x20Apache\x20License,\x20Version\x202.0\x20(the\x20\"License\");\
    \n\x20you\x20may\x20not\x20use\x20this\x20file\x20except\x20in\x20compli\
    ance\x20with\x20the\x20License.\n\x20You\x20may\x20obtain\x20a\x20copy\
    \x20of\x20the\x20License\x20at\n\n\x20\x20\x20\x20\x20http://www.apache.\
    org/licenses/LICENSE-2.0\n\n\x20Unless\x20required\x20by\x20applicable\
    \x20law\x20or\x20agreed\x20to\x20in\x20writing,\x20software\n\x20distrib\
    uted\x20under\x20the\x20License\x20is\x20distributed\x20on\x20an\x20\"AS\
    \x20IS\"\x20BASIS,\n\x20WITHOUT\x20WARRANTIES\x20OR\x20CONDITIONS\x20OF\
    \x20ANY\x20KIND,\x20either\x20express\x20or\x20implied.\n\x20See\x20the\
    \x20License\x20for\x20the\x20specific\x20language\x20governing\x20permis\
    sions\x20and\n\x20limitations\x20under\x20the\x20License.\n\n\x08\n\x01\
    \x02\x12\x03\x10\02\n\x08\n\x01\x08\x12\x03\x12\0F\n\t\n\x02\x08%\x12\
    \x03\x12\0F\n\x08\n\x01\x08\x12\x03\x13\0_\n\t\n\x02\x08\x0b\x12\x03\x13\
    \0_\n\x08\n\x01\x08\x12\x03\x14\0F\n\t\n\x02\x08\x01\x12\x03\x14\0F\n\
    \x08\n\x01\x08\x12\x03\x15\0\"\n\t\n\x02\x08\n\x12\x03\x15\0\"\n\x08\n\
    \x01\x08\x12\x03\x16\0*\n\t\n\x02\x08\x08\x12\x03\x16\0*\n\x08\n\x01\x08\
    \x12\x03\x17\0F\n\t\n\x02\x08)\x12\x03\x17\0F\n\x08\n\x01\x08\x12\x03\
    \x18\0E\n\t\n\x02\x08-\x12\x03\x18\0E\n1\n\x02\x05\0\x12\x04\x1b\0'\x01\
    \x1a%\x20The\x20encryption\x20state\x20of\x20the\x20device.\n\n\n\n\x03\
    \x05\0\x01\x12\x03\x1b\x05\x1b\nQ\n\x04\x05\0\x02\0\x12\x03\x1d\x02\x1d\
    \x1aD\x20The\x20encryption\x20status\x20of\x20the\x20device\x20is\x20not\
    \x20specified\x20or\x20not\x20known.\n\n\x0c\n\x05\x05\0\x02\0\x01\x12\
    \x03\x1d\x02\x18\n\x0c\n\x05\x05\0\x02\0\x02\x12\x03\x1d\x1b\x1c\n6\n\
    \x04\x05\0\x02\x01\x12\x03\x20\x02\x1d\x1a)\x20The\x20device\x20does\x20\
    not\x20support\x20encryption.\n\n\x0c\n\x05\x05\0\x02\x01\x01\x12\x03\
    \x20\x02\x18\n\x0c\n\x05\x05\0\x02\x01\x02\x12\x03\x20\x1b\x1c\nL\n\x04\
    \x05\0\x02\x02\x12\x03#\x02\x12\x1a?\x20The\x20device\x20supports\x20enc\
    ryption,\x20but\x20is\x20currently\x20unencrypted.\n\n\x0c\n\x05\x05\0\
    \x02\x02\x01\x12\x03#\x02\r\n\x0c\n\x05\x05\0\x02\x02\x02\x12\x03#\x10\
    \x11\n'\n\x04\x05\0\x02\x03\x12\x03&\x02\x10\x1a\x1a\x20The\x20device\
    \x20is\x20encrypted.\n\n\x0c\n\x05\x05\0\x02\x03\x01\x12\x03&\x02\x0b\n\
    \x0c\n\x05\x05\0\x02\x03\x02\x12\x03&\x0e\x0f\nB\n\x02\x05\x01\x12\x04+\
    \0@\x01\x1a6\x20The\x20operating\x20system\x20type\x20of\x20the\x20devic\
    e.\n\x20Next\x20id:\x207\n\n\n\n\x03\x05\x01\x01\x12\x03+\x05\x0b\nP\n\
    \x04\x05\x01\x02\0\x12\x03-\x02\x15\x1aC\x20The\x20operating\x20system\
    \x20of\x20the\x20device\x20is\x20not\x20specified\x20or\x20not\x20known.\
    \n\n\x0c\n\x05\x05\x01\x02\0\x01\x12\x03-\x02\x10\n\x0c\n\x05\x05\x01\
    \x02\0\x02\x12\x03-\x13\x14\n.\n\x04\x05\x01\x02\x01\x12\x030\x02\x12\
    \x1a!\x20A\x20desktop\x20Mac\x20operating\x20system.\n\n\x0c\n\x05\x05\
    \x01\x02\x01\x01\x12\x030\x02\r\n\x0c\n\x05\x05\x01\x02\x01\x02\x12\x030\
    \x10\x11\n2\n\x04\x05\x01\x02\x02\x12\x033\x02\x16\x1a%\x20A\x20desktop\
    \x20Windows\x20operating\x20system.\n\n\x0c\n\x05\x05\x01\x02\x02\x01\
    \x12\x033\x02\x11\n\x0c\n\x05\x05\x01\x02\x02\x02\x12\x033\x14\x15\n0\n\
    \x04\x05\x01\x02\x03\x12\x036\x02\x14\x1a#\x20A\x20desktop\x20Linux\x20o\
    perating\x20system.\n\n\x0c\n\x05\x05\x01\x02\x03\x01\x12\x036\x02\x0f\n\
    \x0c\n\x05\x05\x01\x02\x03\x02\x12\x036\x12\x13\n3\n\x04\x05\x01\x02\x04\
    \x12\x039\x02\x18\x1a&\x20A\x20desktop\x20ChromeOS\x20operating\x20syste\
    m.\n\n\x0c\n\x05\x05\x01\x02\x04\x01\x12\x039\x02\x13\n\x0c\n\x05\x05\
    \x01\x02\x04\x02\x12\x039\x16\x17\n+\n\x04\x05\x01\x02\x05\x12\x03<\x02\
    \x0e\x1a\x1e\x20An\x20Android\x20operating\x20system.\n\n\x0c\n\x05\x05\
    \x01\x02\x05\x01\x12\x03<\x02\t\n\x0c\n\x05\x05\x01\x02\x05\x02\x12\x03<\
    \x0c\r\n'\n\x04\x05\x01\x02\x06\x12\x03?\x02\n\x1a\x1a\x20An\x20iOS\x20o\
    perating\x20system.\n\n\x0c\n\x05\x05\x01\x02\x06\x01\x12\x03?\x02\x05\n\
    \x0c\n\x05\x05\x01\x02\x06\x02\x12\x03?\x08\t\nR\n\x02\x05\x02\x12\x04C\
    \0R\x01\x1aF\x20The\x20degree\x20to\x20which\x20the\x20device\x20is\x20m\
    anaged\x20by\x20the\x20Cloud\x20organization.\n\n\n\n\x03\x05\x02\x01\
    \x12\x03C\x05\x1a\nK\n\x04\x05\x02\x02\0\x12\x03E\x02\x1d\x1a>\x20The\
    \x20device's\x20management\x20level\x20is\x20not\x20specified\x20or\x20n\
    ot\x20known.\n\n\x0c\n\x05\x05\x02\x02\0\x01\x12\x03E\x02\x18\n\x0c\n\
    \x05\x05\x02\x02\0\x02\x12\x03E\x1b\x1c\n)\n\x04\x05\x02\x02\x01\x12\x03\
    H\x02\x0b\x1a\x1c\x20The\x20device\x20is\x20not\x20managed.\n\n\x0c\n\
    \x05\x05\x02\x02\x01\x01\x12\x03H\x02\x06\n\x0c\n\x05\x05\x02\x02\x01\
    \x02\x12\x03H\t\n\nw\n\x04\x05\x02\x02\x02\x12\x03L\x02\x0c\x1aj\x20Basi\
    c\x20management\x20is\x20enabled,\x20which\x20is\x20generally\x20limited\
    \x20to\x20monitoring\x20and\n\x20wiping\x20the\x20corporate\x20account.\
    \n\n\x0c\n\x05\x05\x02\x02\x02\x01\x12\x03L\x02\x07\n\x0c\n\x05\x05\x02\
    \x02\x02\x02\x12\x03L\n\x0b\n\xd8\x01\n\x04\x05\x02\x02\x03\x12\x03Q\x02\
    \x0f\x1a\xca\x01\x20Complete\x20device\x20management.\x20This\x20include\
    s\x20more\x20thorough\x20monitoring\x20and\x20the\n\x20ability\x20to\x20\
    directly\x20manage\x20the\x20device\x20(such\x20as\x20remote\x20wiping).\
    \x20This\x20can\x20be\n\x20enabled\x20through\x20the\x20Android\x20Enter\
    prise\x20Platform.\n\n\x0c\n\x05\x05\x02\x02\x03\x01\x12\x03Q\x02\n\n\
    \x0c\n\x05\x05\x02\x02\x03\x02\x12\x03Q\r\x0eb\x06proto3\
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
