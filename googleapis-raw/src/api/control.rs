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
//! Generated file from `google/api/control.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_28_0;

#[derive(PartialEq,Clone,Default)]
pub struct Control {
    // message fields
    pub environment: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Control {
    fn default() -> &'a Control {
        <Control as ::protobuf::Message>::default_instance()
    }
}

impl Control {
    pub fn new() -> Control {
        ::std::default::Default::default()
    }

    // string environment = 1;


    pub fn get_environment(&self) -> &str {
        &self.environment
    }
    pub fn clear_environment(&mut self) {
        self.environment.clear();
    }

    // Param is passed by value, moved
    pub fn set_environment(&mut self, v: ::std::string::String) {
        self.environment = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_environment(&mut self) -> &mut ::std::string::String {
        &mut self.environment
    }

    // Take field
    pub fn take_environment(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.environment, ::std::string::String::new())
    }
}

impl ::protobuf::Message for Control {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.environment)?;
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
        if !self.environment.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.environment);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.environment.is_empty() {
            os.write_string(1, &self.environment)?;
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

    fn new() -> Control {
        Control::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "environment",
                |m: &Control| { &m.environment },
                |m: &mut Control| { &mut m.environment },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Control>(
                "Control",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Control {
        static instance: ::protobuf::rt::LazyV2<Control> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Control::new)
    }
}

impl ::protobuf::Clear for Control {
    fn clear(&mut self) {
        self.environment.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Control {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Control {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x18google/api/control.proto\x12\ngoogle.api\"+\n\x07Control\x12\x20\n\
    \x0benvironment\x18\x01\x20\x01(\tR\x0benvironmentBn\n\x0ecom.google.api\
    B\x0cControlProtoP\x01ZEgoogle.golang.org/genproto/googleapis/api/servic\
    econfig;serviceconfig\xa2\x02\x04GAPIJ\xae\x08\n\x06\x12\x04\x0f\0\x20\
    \x01\n\xbe\x04\n\x01\x0c\x12\x03\x0f\0\x122\xb3\x04\x20Copyright\x202019\
    \x20Google\x20LLC.\n\n\x20Licensed\x20under\x20the\x20Apache\x20License,\
    \x20Version\x202.0\x20(the\x20\"License\");\n\x20you\x20may\x20not\x20us\
    e\x20this\x20file\x20except\x20in\x20compliance\x20with\x20the\x20Licens\
    e.\n\x20You\x20may\x20obtain\x20a\x20copy\x20of\x20the\x20License\x20at\
    \n\n\x20\x20\x20\x20\x20http://www.apache.org/licenses/LICENSE-2.0\n\n\
    \x20Unless\x20required\x20by\x20applicable\x20law\x20or\x20agreed\x20to\
    \x20in\x20writing,\x20software\n\x20distributed\x20under\x20the\x20Licen\
    se\x20is\x20distributed\x20on\x20an\x20\"AS\x20IS\"\x20BASIS,\n\x20WITHO\
    UT\x20WARRANTIES\x20OR\x20CONDITIONS\x20OF\x20ANY\x20KIND,\x20either\x20\
    express\x20or\x20implied.\n\x20See\x20the\x20License\x20for\x20the\x20sp\
    ecific\x20language\x20governing\x20permissions\x20and\n\x20limitations\
    \x20under\x20the\x20License.\n\n\n\x08\n\x01\x02\x12\x03\x11\0\x13\n\x08\
    \n\x01\x08\x12\x03\x13\0\\\n\t\n\x02\x08\x0b\x12\x03\x13\0\\\n\x08\n\x01\
    \x08\x12\x03\x14\0\"\n\t\n\x02\x08\n\x12\x03\x14\0\"\n\x08\n\x01\x08\x12\
    \x03\x15\0-\n\t\n\x02\x08\x08\x12\x03\x15\0-\n\x08\n\x01\x08\x12\x03\x16\
    \0'\n\t\n\x02\x08\x01\x12\x03\x16\0'\n\x08\n\x01\x08\x12\x03\x17\0\"\n\t\
    \n\x02\x08$\x12\x03\x17\0\"\n\xb2\x01\n\x02\x04\0\x12\x04\x1c\0\x20\x01\
    \x1a\xa5\x01\x20Selects\x20and\x20configures\x20the\x20service\x20contro\
    ller\x20used\x20by\x20the\x20service.\x20\x20The\n\x20service\x20control\
    ler\x20handles\x20features\x20like\x20abuse,\x20quota,\x20billing,\x20lo\
    gging,\n\x20monitoring,\x20etc.\n\n\n\n\x03\x04\0\x01\x12\x03\x1c\x08\
    \x0f\n\x84\x01\n\x04\x04\0\x02\0\x12\x03\x1f\x02\x19\x1aw\x20The\x20serv\
    ice\x20control\x20environment\x20to\x20use.\x20If\x20empty,\x20no\x20con\
    trol\x20plane\n\x20feature\x20(like\x20quota\x20and\x20billing)\x20will\
    \x20be\x20enabled.\n\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x1f\x02\x08\n\
    \x0c\n\x05\x04\0\x02\0\x01\x12\x03\x1f\t\x14\n\x0c\n\x05\x04\0\x02\0\x03\
    \x12\x03\x1f\x17\x18b\x06proto3\
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
