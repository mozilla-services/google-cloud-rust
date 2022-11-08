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
//! Generated file from `google/bigtable/admin/v2/common.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_28_0;

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum StorageType {
    STORAGE_TYPE_UNSPECIFIED = 0,
    SSD = 1,
    HDD = 2,
}

impl ::protobuf::ProtobufEnum for StorageType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<StorageType> {
        match value {
            0 => ::std::option::Option::Some(StorageType::STORAGE_TYPE_UNSPECIFIED),
            1 => ::std::option::Option::Some(StorageType::SSD),
            2 => ::std::option::Option::Some(StorageType::HDD),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [StorageType] = &[
            StorageType::STORAGE_TYPE_UNSPECIFIED,
            StorageType::SSD,
            StorageType::HDD,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<StorageType>("StorageType", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for StorageType {
}

impl ::std::default::Default for StorageType {
    fn default() -> Self {
        StorageType::STORAGE_TYPE_UNSPECIFIED
    }
}

impl ::protobuf::reflect::ProtobufValue for StorageType {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n%google/bigtable/admin/v2/common.proto\x12\x18google.bigtable.admin.v2\
    \x1a\x1cgoogle/api/annotations.proto\x1a\x1fgoogle/protobuf/timestamp.pr\
    oto*=\n\x0bStorageType\x12\x1c\n\x18STORAGE_TYPE_UNSPECIFIED\x10\0\x12\
    \x07\n\x03SSD\x10\x01\x12\x07\n\x03HDD\x10\x02B\xae\x01\n\x1ccom.google.\
    bigtable.admin.v2B\x0bCommonProtoP\x01Z=google.golang.org/genproto/googl\
    eapis/bigtable/admin/v2;admin\xaa\x02\x1eGoogle.Cloud.Bigtable.Admin.V2\
    \xca\x02\x1eGoogle\\Cloud\\Bigtable\\Admin\\V2J\xb2\x08\n\x06\x12\x04\
    \x0f\0'\x01\n\xbe\x04\n\x01\x0c\x12\x03\x0f\0\x122\xb3\x04\x20Copyright\
    \x202018\x20Google\x20LLC.\n\n\x20Licensed\x20under\x20the\x20Apache\x20\
    License,\x20Version\x202.0\x20(the\x20\"License\");\n\x20you\x20may\x20n\
    ot\x20use\x20this\x20file\x20except\x20in\x20compliance\x20with\x20the\
    \x20License.\n\x20You\x20may\x20obtain\x20a\x20copy\x20of\x20the\x20Lice\
    nse\x20at\n\n\x20\x20\x20\x20\x20http://www.apache.org/licenses/LICENSE-\
    2.0\n\n\x20Unless\x20required\x20by\x20applicable\x20law\x20or\x20agreed\
    \x20to\x20in\x20writing,\x20software\n\x20distributed\x20under\x20the\
    \x20License\x20is\x20distributed\x20on\x20an\x20\"AS\x20IS\"\x20BASIS,\n\
    \x20WITHOUT\x20WARRANTIES\x20OR\x20CONDITIONS\x20OF\x20ANY\x20KIND,\x20e\
    ither\x20express\x20or\x20implied.\n\x20See\x20the\x20License\x20for\x20\
    the\x20specific\x20language\x20governing\x20permissions\x20and\n\x20limi\
    tations\x20under\x20the\x20License.\n\n\n\x08\n\x01\x02\x12\x03\x11\0!\n\
    \t\n\x02\x03\0\x12\x03\x13\0&\n\t\n\x02\x03\x01\x12\x03\x14\0)\n\x08\n\
    \x01\x08\x12\x03\x16\0;\n\t\n\x02\x08%\x12\x03\x16\0;\n\x08\n\x01\x08\
    \x12\x03\x17\0T\n\t\n\x02\x08\x0b\x12\x03\x17\0T\n\x08\n\x01\x08\x12\x03\
    \x18\0\"\n\t\n\x02\x08\n\x12\x03\x18\0\"\n\x08\n\x01\x08\x12\x03\x19\0,\
    \n\t\n\x02\x08\x08\x12\x03\x19\0,\n\x08\n\x01\x08\x12\x03\x1a\05\n\t\n\
    \x02\x08\x01\x12\x03\x1a\05\n\x08\n\x01\x08\x12\x03\x1b\0<\n\t\n\x02\x08\
    )\x12\x03\x1b\0<\n?\n\x02\x05\0\x12\x04\x1e\0'\x01\x1a3\x20Storage\x20me\
    dia\x20types\x20for\x20persisting\x20Bigtable\x20data.\n\n\n\n\x03\x05\0\
    \x01\x12\x03\x1e\x05\x10\n7\n\x04\x05\0\x02\0\x12\x03\x20\x02\x1f\x1a*\
    \x20The\x20user\x20did\x20not\x20specify\x20a\x20storage\x20type.\n\n\
    \x0c\n\x05\x05\0\x02\0\x01\x12\x03\x20\x02\x1a\n\x0c\n\x05\x05\0\x02\0\
    \x02\x12\x03\x20\x1d\x1e\n2\n\x04\x05\0\x02\x01\x12\x03#\x02\n\x1a%\x20F\
    lash\x20(SSD)\x20storage\x20should\x20be\x20used.\n\n\x0c\n\x05\x05\0\
    \x02\x01\x01\x12\x03#\x02\x05\n\x0c\n\x05\x05\0\x02\x01\x02\x12\x03#\x08\
    \t\n;\n\x04\x05\0\x02\x02\x12\x03&\x02\n\x1a.\x20Magnetic\x20drive\x20(H\
    DD)\x20storage\x20should\x20be\x20used.\n\n\x0c\n\x05\x05\0\x02\x02\x01\
    \x12\x03&\x02\x05\n\x0c\n\x05\x05\0\x02\x02\x02\x12\x03&\x08\tb\x06proto\
    3\
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
