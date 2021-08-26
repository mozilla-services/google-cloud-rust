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
//! Generated file from `google/api/client.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_25_1;

/// Extension fields
pub mod exts {

    pub const method_signature: ::protobuf::ext::ExtFieldRepeated<::protobuf::descriptor::MethodOptions, ::protobuf::types::ProtobufTypeString> = ::protobuf::ext::ExtFieldRepeated { field_number: 1051, phantom: ::std::marker::PhantomData };

    pub const default_host: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::ServiceOptions, ::protobuf::types::ProtobufTypeString> = ::protobuf::ext::ExtFieldOptional { field_number: 1049, phantom: ::std::marker::PhantomData };

    pub const oauth_scopes: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::ServiceOptions, ::protobuf::types::ProtobufTypeString> = ::protobuf::ext::ExtFieldOptional { field_number: 1050, phantom: ::std::marker::PhantomData };
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x17google/api/client.proto\x12\ngoogle.api\x1a\x20google/protobuf/des\
    criptor.proto:J\n\x10method_signature\x18\x9b\x08\x20\x03(\t\x12\x1e.goo\
    gle.protobuf.MethodOptionsR\x0fmethodSignature:C\n\x0cdefault_host\x18\
    \x99\x08\x20\x01(\t\x12\x1f.google.protobuf.ServiceOptionsR\x0bdefaultHo\
    st:C\n\x0coauth_scopes\x18\x9a\x08\x20\x01(\t\x12\x1f.google.protobuf.Se\
    rviceOptionsR\x0boauthScopesBi\n\x0ecom.google.apiB\x0bClientProtoP\x01Z\
    Agoogle.golang.org/genproto/googleapis/api/annotations;annotations\xa2\
    \x02\x04GAPIJ\x92\x17\n\x06\x12\x04\x0e\0b\x01\n\xbc\x04\n\x01\x0c\x12\
    \x03\x0e\0\x122\xb1\x04\x20Copyright\x202018\x20Google\x20LLC\n\n\x20Lic\
    ensed\x20under\x20the\x20Apache\x20License,\x20Version\x202.0\x20(the\
    \x20\"License\");\n\x20you\x20may\x20not\x20use\x20this\x20file\x20excep\
    t\x20in\x20compliance\x20with\x20the\x20License.\n\x20You\x20may\x20obta\
    in\x20a\x20copy\x20of\x20the\x20License\x20at\n\n\x20\x20\x20\x20\x20htt\
    p://www.apache.org/licenses/LICENSE-2.0\n\n\x20Unless\x20required\x20by\
    \x20applicable\x20law\x20or\x20agreed\x20to\x20in\x20writing,\x20softwar\
    e\n\x20distributed\x20under\x20the\x20License\x20is\x20distributed\x20on\
    \x20an\x20\"AS\x20IS\"\x20BASIS,\n\x20WITHOUT\x20WARRANTIES\x20OR\x20CON\
    DITIONS\x20OF\x20ANY\x20KIND,\x20either\x20express\x20or\x20implied.\n\
    \x20See\x20the\x20License\x20for\x20the\x20specific\x20language\x20gover\
    ning\x20permissions\x20and\n\x20limitations\x20under\x20the\x20License.\
    \n\n\x08\n\x01\x02\x12\x03\x10\0\x13\n\t\n\x02\x03\0\x12\x03\x12\0*\n\
    \x08\n\x01\x08\x12\x03\x14\0X\n\t\n\x02\x08\x0b\x12\x03\x14\0X\n\x08\n\
    \x01\x08\x12\x03\x15\0\"\n\t\n\x02\x08\n\x12\x03\x15\0\"\n\x08\n\x01\x08\
    \x12\x03\x16\0,\n\t\n\x02\x08\x08\x12\x03\x16\0,\n\x08\n\x01\x08\x12\x03\
    \x17\0'\n\t\n\x02\x08\x01\x12\x03\x17\0'\n\x08\n\x01\x08\x12\x03\x18\0\"\
    \n\t\n\x02\x08$\x12\x03\x18\0\"\n\t\n\x01\x07\x12\x04\x1a\0?\x01\n\x85\
    \x0b\n\x02\x07\0\x12\x03>\x02*\x1a\xf9\n\x20A\x20definition\x20of\x20a\
    \x20client\x20library\x20method\x20signature.\n\n\x20In\x20client\x20lib\
    raries,\x20each\x20proto\x20RPC\x20corresponds\x20to\x20one\x20or\x20mor\
    e\x20methods\n\x20which\x20the\x20end\x20user\x20is\x20able\x20to\x20cal\
    l,\x20and\x20calls\x20the\x20underlying\x20RPC.\n\x20Normally,\x20this\
    \x20method\x20receives\x20a\x20single\x20argument\x20(a\x20struct\x20or\
    \x20instance\n\x20corresponding\x20to\x20the\x20RPC\x20request\x20object\
    ).\x20Defining\x20this\x20field\x20will\n\x20add\x20one\x20or\x20more\
    \x20overloads\x20providing\x20flattened\x20or\x20simpler\x20method\x20si\
    gnatures\n\x20in\x20some\x20languages.\n\n\x20The\x20fields\x20on\x20the\
    \x20method\x20signature\x20are\x20provided\x20as\x20a\x20comma-separated\
    \n\x20string.\n\n\x20For\x20example,\x20the\x20proto\x20RPC\x20and\x20an\
    notation:\n\n\x20\x20\x20rpc\x20CreateSubscription(CreateSubscriptionReq\
    uest)\n\x20\x20\x20\x20\x20\x20\x20returns\x20(Subscription)\x20{\n\x20\
    \x20\x20\x20\x20option\x20(google.api.method_signature)\x20=\x20\"name,t\
    opic\";\n\x20\x20\x20}\n\n\x20Would\x20add\x20the\x20following\x20Java\
    \x20overload\x20(in\x20addition\x20to\x20the\x20method\x20accepting\n\
    \x20the\x20request\x20object):\n\n\x20\x20\x20public\x20final\x20Subscri\
    ption\x20createSubscription(String\x20name,\x20String\x20topic)\n\n\x20T\
    he\x20following\x20backwards-compatibility\x20guidelines\x20apply:\n\n\
    \x20\x20\x20*\x20Adding\x20this\x20annotation\x20to\x20an\x20unannotated\
    \x20method\x20is\x20backwards\n\x20\x20\x20\x20\x20compatible.\n\x20\x20\
    \x20*\x20Adding\x20this\x20annotation\x20to\x20a\x20method\x20which\x20a\
    lready\x20has\x20existing\n\x20\x20\x20\x20\x20method\x20signature\x20an\
    notations\x20is\x20backwards\x20compatible\x20if\x20and\x20only\x20if\n\
    \x20\x20\x20\x20\x20the\x20new\x20method\x20signature\x20annotation\x20i\
    s\x20last\x20in\x20the\x20sequence.\n\x20\x20\x20*\x20Modifying\x20or\
    \x20removing\x20an\x20existing\x20method\x20signature\x20annotation\x20i\
    s\n\x20\x20\x20\x20\x20a\x20breaking\x20change.\n\x20\x20\x20*\x20Re-ord\
    ering\x20existing\x20method\x20signature\x20annotations\x20is\x20a\x20br\
    eaking\n\x20\x20\x20\x20\x20change.\n\n\n\n\x03\x07\0\x02\x12\x03\x1a\
    \x07$\n\n\n\x03\x07\0\x04\x12\x03>\x02\n\n\n\n\x03\x07\0\x05\x12\x03>\
    \x0b\x11\n\n\n\x03\x07\0\x01\x12\x03>\x12\"\n\n\n\x03\x07\0\x03\x12\x03>\
    %)\n\t\n\x01\x07\x12\x04A\0b\x01\n\xca\x01\n\x02\x07\x01\x12\x03K\x02\
    \x1d\x1a\xbe\x01\x20The\x20hostname\x20for\x20this\x20service.\n\x20This\
    \x20should\x20be\x20specified\x20with\x20no\x20prefix\x20or\x20protocol.\
    \n\n\x20Example:\n\n\x20\x20\x20service\x20Foo\x20{\n\x20\x20\x20\x20\
    \x20option\x20(google.api.default_host)\x20=\x20\"foo.googleapi.com\";\n\
    \x20\x20\x20\x20\x20...\n\x20\x20\x20}\n\n\n\n\x03\x07\x01\x02\x12\x03A\
    \x07%\n\n\n\x03\x07\x01\x05\x12\x03K\x02\x08\n\n\n\x03\x07\x01\x01\x12\
    \x03K\t\x15\n\n\n\x03\x07\x01\x03\x12\x03K\x18\x1c\n\xc3\x03\n\x02\x07\
    \x02\x12\x03a\x02\x1d\x1a\xb7\x03\x20OAuth\x20scopes\x20needed\x20for\
    \x20the\x20client.\n\n\x20Example:\n\n\x20\x20\x20service\x20Foo\x20{\n\
    \x20\x20\x20\x20\x20option\x20(google.api.oauth_scopes)\x20=\x20\\\n\x20\
    \x20\x20\x20\x20\x20\x20\"https://www.googleapis.com/auth/cloud-platform\
    \";\n\x20\x20\x20\x20\x20...\n\x20\x20\x20}\n\n\x20If\x20there\x20is\x20\
    more\x20than\x20one\x20scope,\x20use\x20a\x20comma-separated\x20string:\
    \n\n\x20Example:\n\n\x20\x20\x20service\x20Foo\x20{\n\x20\x20\x20\x20\
    \x20option\x20(google.api.oauth_scopes)\x20=\x20\\\n\x20\x20\x20\x20\x20\
    \x20\x20\"https://www.googleapis.com/auth/cloud-platform,\"\n\x20\x20\
    \x20\x20\x20\x20\x20\"https://www.googleapis.com/auth/monitoring\";\n\
    \x20\x20\x20\x20\x20...\n\x20\x20\x20}\n\n\n\n\x03\x07\x02\x02\x12\x03A\
    \x07%\n\n\n\x03\x07\x02\x05\x12\x03a\x02\x08\n\n\n\x03\x07\x02\x01\x12\
    \x03a\t\x15\n\n\n\x03\x07\x02\x03\x12\x03a\x18\x1cb\x06proto3\
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
