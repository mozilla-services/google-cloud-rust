// This file is generated by rust-protobuf 2.23.0. Do not edit
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
//! Generated file from `google/api/httpbody.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_23_0;

#[derive(PartialEq,Clone,Default)]
pub struct HttpBody {
    // message fields
    pub content_type: ::std::string::String,
    pub data: ::std::vec::Vec<u8>,
    pub extensions: ::protobuf::RepeatedField<::protobuf::well_known_types::Any>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a HttpBody {
    fn default() -> &'a HttpBody {
        <HttpBody as ::protobuf::Message>::default_instance()
    }
}

impl HttpBody {
    pub fn new() -> HttpBody {
        ::std::default::Default::default()
    }

    // string content_type = 1;


    pub fn get_content_type(&self) -> &str {
        &self.content_type
    }
    pub fn clear_content_type(&mut self) {
        self.content_type.clear();
    }

    // Param is passed by value, moved
    pub fn set_content_type(&mut self, v: ::std::string::String) {
        self.content_type = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_content_type(&mut self) -> &mut ::std::string::String {
        &mut self.content_type
    }

    // Take field
    pub fn take_content_type(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.content_type, ::std::string::String::new())
    }

    // bytes data = 2;


    pub fn get_data(&self) -> &[u8] {
        &self.data
    }
    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.data = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.data
    }

    // Take field
    pub fn take_data(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.data, ::std::vec::Vec::new())
    }

    // repeated .google.protobuf.Any extensions = 3;


    pub fn get_extensions(&self) -> &[::protobuf::well_known_types::Any] {
        &self.extensions
    }
    pub fn clear_extensions(&mut self) {
        self.extensions.clear();
    }

    // Param is passed by value, moved
    pub fn set_extensions(&mut self, v: ::protobuf::RepeatedField<::protobuf::well_known_types::Any>) {
        self.extensions = v;
    }

    // Mutable pointer to the field.
    pub fn mut_extensions(&mut self) -> &mut ::protobuf::RepeatedField<::protobuf::well_known_types::Any> {
        &mut self.extensions
    }

    // Take field
    pub fn take_extensions(&mut self) -> ::protobuf::RepeatedField<::protobuf::well_known_types::Any> {
        ::std::mem::replace(&mut self.extensions, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for HttpBody {
    fn is_initialized(&self) -> bool {
        for v in &self.extensions {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.content_type)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.data)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.extensions)?;
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
        if !self.content_type.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.content_type);
        }
        if !self.data.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.data);
        }
        for value in &self.extensions {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.content_type.is_empty() {
            os.write_string(1, &self.content_type)?;
        }
        if !self.data.is_empty() {
            os.write_bytes(2, &self.data)?;
        }
        for v in &self.extensions {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

    fn new() -> HttpBody {
        HttpBody::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "content_type",
                |m: &HttpBody| { &m.content_type },
                |m: &mut HttpBody| { &mut m.content_type },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "data",
                |m: &HttpBody| { &m.data },
                |m: &mut HttpBody| { &mut m.data },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Any>>(
                "extensions",
                |m: &HttpBody| { &m.extensions },
                |m: &mut HttpBody| { &mut m.extensions },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<HttpBody>(
                "HttpBody",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static HttpBody {
        static instance: ::protobuf::rt::LazyV2<HttpBody> = ::protobuf::rt::LazyV2::INIT;
        instance.get(HttpBody::new)
    }
}

impl ::protobuf::Clear for HttpBody {
    fn clear(&mut self) {
        self.content_type.clear();
        self.data.clear();
        self.extensions.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for HttpBody {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HttpBody {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x19google/api/httpbody.proto\x12\ngoogle.api\x1a\x19google/protobuf/a\
    ny.proto\"w\n\x08HttpBody\x12!\n\x0ccontent_type\x18\x01\x20\x01(\tR\x0b\
    contentType\x12\x12\n\x04data\x18\x02\x20\x01(\x0cR\x04data\x124\n\nexte\
    nsions\x18\x03\x20\x03(\x0b2\x14.google.protobuf.AnyR\nextensionsBh\n\
    \x0ecom.google.apiB\rHttpBodyProtoP\x01Z;google.golang.org/genproto/goog\
    leapis/api/httpbody;httpbody\xf8\x01\x01\xa2\x02\x04GAPIJ\xc2\x13\n\x06\
    \x12\x04\x0f\0M\x01\n\xbe\x04\n\x01\x0c\x12\x03\x0f\0\x122\xb3\x04\x20Co\
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
    \x11\0\x13\n\t\n\x02\x03\0\x12\x03\x13\0#\n\x08\n\x01\x08\x12\x03\x15\0\
    \x1f\n\t\n\x02\x08\x1f\x12\x03\x15\0\x1f\n\x08\n\x01\x08\x12\x03\x16\0R\
    \n\t\n\x02\x08\x0b\x12\x03\x16\0R\n\x08\n\x01\x08\x12\x03\x17\0\"\n\t\n\
    \x02\x08\n\x12\x03\x17\0\"\n\x08\n\x01\x08\x12\x03\x18\0.\n\t\n\x02\x08\
    \x08\x12\x03\x18\0.\n\x08\n\x01\x08\x12\x03\x19\0'\n\t\n\x02\x08\x01\x12\
    \x03\x19\0'\n\x08\n\x01\x08\x12\x03\x1a\0\"\n\t\n\x02\x08$\x12\x03\x1a\0\
    \"\n\xa4\n\n\x02\x04\0\x12\x04C\0M\x01\x1a\x97\n\x20Message\x20that\x20r\
    epresents\x20an\x20arbitrary\x20HTTP\x20body.\x20It\x20should\x20only\
    \x20be\x20used\x20for\n\x20payload\x20formats\x20that\x20can't\x20be\x20\
    represented\x20as\x20JSON,\x20such\x20as\x20raw\x20binary\x20or\n\x20an\
    \x20HTML\x20page.\n\n\n\x20This\x20message\x20can\x20be\x20used\x20both\
    \x20in\x20streaming\x20and\x20non-streaming\x20API\x20methods\x20in\n\
    \x20the\x20request\x20as\x20well\x20as\x20the\x20response.\n\n\x20It\x20\
    can\x20be\x20used\x20as\x20a\x20top-level\x20request\x20field,\x20which\
    \x20is\x20convenient\x20if\x20one\n\x20wants\x20to\x20extract\x20paramet\
    ers\x20from\x20either\x20the\x20URL\x20or\x20HTTP\x20template\x20into\
    \x20the\n\x20request\x20fields\x20and\x20also\x20want\x20access\x20to\
    \x20the\x20raw\x20HTTP\x20body.\n\n\x20Example:\n\n\x20\x20\x20\x20\x20m\
    essage\x20GetResourceRequest\x20{\n\x20\x20\x20\x20\x20\x20\x20//\x20A\
    \x20unique\x20request\x20id.\n\x20\x20\x20\x20\x20\x20\x20string\x20requ\
    est_id\x20=\x201;\n\n\x20\x20\x20\x20\x20\x20\x20//\x20The\x20raw\x20HTT\
    P\x20body\x20is\x20bound\x20to\x20this\x20field.\n\x20\x20\x20\x20\x20\
    \x20\x20google.api.HttpBody\x20http_body\x20=\x202;\n\x20\x20\x20\x20\
    \x20}\n\n\x20\x20\x20\x20\x20service\x20ResourceService\x20{\n\x20\x20\
    \x20\x20\x20\x20\x20rpc\x20GetResource(GetResourceRequest)\x20returns\
    \x20(google.api.HttpBody);\n\x20\x20\x20\x20\x20\x20\x20rpc\x20UpdateRes\
    ource(google.api.HttpBody)\x20returns\n\x20\x20\x20\x20\x20\x20\x20(goog\
    le.protobuf.Empty);\n\x20\x20\x20\x20\x20}\n\n\x20Example\x20with\x20str\
    eaming\x20methods:\n\n\x20\x20\x20\x20\x20service\x20CaldavService\x20{\
    \n\x20\x20\x20\x20\x20\x20\x20rpc\x20GetCalendar(stream\x20google.api.Ht\
    tpBody)\n\x20\x20\x20\x20\x20\x20\x20\x20\x20returns\x20(stream\x20googl\
    e.api.HttpBody);\n\x20\x20\x20\x20\x20\x20\x20rpc\x20UpdateCalendar(stre\
    am\x20google.api.HttpBody)\n\x20\x20\x20\x20\x20\x20\x20\x20\x20returns\
    \x20(stream\x20google.api.HttpBody);\n\x20\x20\x20\x20\x20}\n\n\x20Use\
    \x20of\x20this\x20type\x20only\x20changes\x20how\x20the\x20request\x20an\
    d\x20response\x20bodies\x20are\n\x20handled,\x20all\x20other\x20features\
    \x20will\x20continue\x20to\x20work\x20unchanged.\n\n\n\n\x03\x04\0\x01\
    \x12\x03C\x08\x10\nZ\n\x04\x04\0\x02\0\x12\x03E\x02\x1a\x1aM\x20The\x20H\
    TTP\x20Content-Type\x20header\x20value\x20specifying\x20the\x20content\
    \x20type\x20of\x20the\x20body.\n\n\r\n\x05\x04\0\x02\0\x04\x12\x04E\x02C\
    \x12\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03E\x02\x08\n\x0c\n\x05\x04\0\x02\
    \0\x01\x12\x03E\t\x15\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03E\x18\x19\n<\n\
    \x04\x04\0\x02\x01\x12\x03H\x02\x11\x1a/\x20The\x20HTTP\x20request/respo\
    nse\x20body\x20as\x20raw\x20binary.\n\n\r\n\x05\x04\0\x02\x01\x04\x12\
    \x04H\x02E\x1a\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03H\x02\x07\n\x0c\n\
    \x05\x04\0\x02\x01\x01\x12\x03H\x08\x0c\n\x0c\n\x05\x04\0\x02\x01\x03\
    \x12\x03H\x0f\x10\nm\n\x04\x04\0\x02\x02\x12\x03L\x02.\x1a`\x20Applicati\
    on\x20specific\x20response\x20metadata.\x20Must\x20be\x20set\x20in\x20th\
    e\x20first\x20response\n\x20for\x20streaming\x20APIs.\n\n\x0c\n\x05\x04\
    \0\x02\x02\x04\x12\x03L\x02\n\n\x0c\n\x05\x04\0\x02\x02\x06\x12\x03L\x0b\
    \x1e\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03L\x1f)\n\x0c\n\x05\x04\0\x02\
    \x02\x03\x12\x03L,-b\x06proto3\
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
