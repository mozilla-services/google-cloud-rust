// This file is generated by rust-protobuf 2.22.0. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![rustfmt::skip]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `google/api/context.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_22_0;

#[derive(PartialEq,Clone,Default)]
pub struct Context {
    // message fields
    pub rules: ::protobuf::RepeatedField<ContextRule>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Context {
    fn default() -> &'a Context {
        <Context as ::protobuf::Message>::default_instance()
    }
}

impl Context {
    pub fn new() -> Context {
        ::std::default::Default::default()
    }

    // repeated .google.api.ContextRule rules = 1;


    pub fn get_rules(&self) -> &[ContextRule] {
        &self.rules
    }
    pub fn clear_rules(&mut self) {
        self.rules.clear();
    }

    // Param is passed by value, moved
    pub fn set_rules(&mut self, v: ::protobuf::RepeatedField<ContextRule>) {
        self.rules = v;
    }

    // Mutable pointer to the field.
    pub fn mut_rules(&mut self) -> &mut ::protobuf::RepeatedField<ContextRule> {
        &mut self.rules
    }

    // Take field
    pub fn take_rules(&mut self) -> ::protobuf::RepeatedField<ContextRule> {
        ::std::mem::replace(&mut self.rules, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for Context {
    fn is_initialized(&self) -> bool {
        for v in &self.rules {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.rules)?;
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
        for value in &self.rules {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        for v in &self.rules {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

    fn new() -> Context {
        Context::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ContextRule>>(
                "rules",
                |m: &Context| { &m.rules },
                |m: &mut Context| { &mut m.rules },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Context>(
                "Context",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Context {
        static instance: ::protobuf::rt::LazyV2<Context> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Context::new)
    }
}

impl ::protobuf::Clear for Context {
    fn clear(&mut self) {
        self.rules.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Context {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Context {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ContextRule {
    // message fields
    pub selector: ::std::string::String,
    pub requested: ::protobuf::RepeatedField<::std::string::String>,
    pub provided: ::protobuf::RepeatedField<::std::string::String>,
    pub allowed_request_extensions: ::protobuf::RepeatedField<::std::string::String>,
    pub allowed_response_extensions: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a ContextRule {
    fn default() -> &'a ContextRule {
        <ContextRule as ::protobuf::Message>::default_instance()
    }
}

impl ContextRule {
    pub fn new() -> ContextRule {
        ::std::default::Default::default()
    }

    // string selector = 1;


    pub fn get_selector(&self) -> &str {
        &self.selector
    }
    pub fn clear_selector(&mut self) {
        self.selector.clear();
    }

    // Param is passed by value, moved
    pub fn set_selector(&mut self, v: ::std::string::String) {
        self.selector = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_selector(&mut self) -> &mut ::std::string::String {
        &mut self.selector
    }

    // Take field
    pub fn take_selector(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.selector, ::std::string::String::new())
    }

    // repeated string requested = 2;


    pub fn get_requested(&self) -> &[::std::string::String] {
        &self.requested
    }
    pub fn clear_requested(&mut self) {
        self.requested.clear();
    }

    // Param is passed by value, moved
    pub fn set_requested(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.requested = v;
    }

    // Mutable pointer to the field.
    pub fn mut_requested(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.requested
    }

    // Take field
    pub fn take_requested(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.requested, ::protobuf::RepeatedField::new())
    }

    // repeated string provided = 3;


    pub fn get_provided(&self) -> &[::std::string::String] {
        &self.provided
    }
    pub fn clear_provided(&mut self) {
        self.provided.clear();
    }

    // Param is passed by value, moved
    pub fn set_provided(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.provided = v;
    }

    // Mutable pointer to the field.
    pub fn mut_provided(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.provided
    }

    // Take field
    pub fn take_provided(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.provided, ::protobuf::RepeatedField::new())
    }

    // repeated string allowed_request_extensions = 4;


    pub fn get_allowed_request_extensions(&self) -> &[::std::string::String] {
        &self.allowed_request_extensions
    }
    pub fn clear_allowed_request_extensions(&mut self) {
        self.allowed_request_extensions.clear();
    }

    // Param is passed by value, moved
    pub fn set_allowed_request_extensions(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.allowed_request_extensions = v;
    }

    // Mutable pointer to the field.
    pub fn mut_allowed_request_extensions(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.allowed_request_extensions
    }

    // Take field
    pub fn take_allowed_request_extensions(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.allowed_request_extensions, ::protobuf::RepeatedField::new())
    }

    // repeated string allowed_response_extensions = 5;


    pub fn get_allowed_response_extensions(&self) -> &[::std::string::String] {
        &self.allowed_response_extensions
    }
    pub fn clear_allowed_response_extensions(&mut self) {
        self.allowed_response_extensions.clear();
    }

    // Param is passed by value, moved
    pub fn set_allowed_response_extensions(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.allowed_response_extensions = v;
    }

    // Mutable pointer to the field.
    pub fn mut_allowed_response_extensions(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.allowed_response_extensions
    }

    // Take field
    pub fn take_allowed_response_extensions(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.allowed_response_extensions, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for ContextRule {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.selector)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.requested)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.provided)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.allowed_request_extensions)?;
                },
                5 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.allowed_response_extensions)?;
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
        if !self.selector.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.selector);
        }
        for value in &self.requested {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in &self.provided {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        for value in &self.allowed_request_extensions {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        for value in &self.allowed_response_extensions {
            my_size += ::protobuf::rt::string_size(5, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.selector.is_empty() {
            os.write_string(1, &self.selector)?;
        }
        for v in &self.requested {
            os.write_string(2, &v)?;
        };
        for v in &self.provided {
            os.write_string(3, &v)?;
        };
        for v in &self.allowed_request_extensions {
            os.write_string(4, &v)?;
        };
        for v in &self.allowed_response_extensions {
            os.write_string(5, &v)?;
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

    fn new() -> ContextRule {
        ContextRule::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "selector",
                |m: &ContextRule| { &m.selector },
                |m: &mut ContextRule| { &mut m.selector },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "requested",
                |m: &ContextRule| { &m.requested },
                |m: &mut ContextRule| { &mut m.requested },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "provided",
                |m: &ContextRule| { &m.provided },
                |m: &mut ContextRule| { &mut m.provided },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "allowed_request_extensions",
                |m: &ContextRule| { &m.allowed_request_extensions },
                |m: &mut ContextRule| { &mut m.allowed_request_extensions },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "allowed_response_extensions",
                |m: &ContextRule| { &m.allowed_response_extensions },
                |m: &mut ContextRule| { &mut m.allowed_response_extensions },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<ContextRule>(
                "ContextRule",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static ContextRule {
        static instance: ::protobuf::rt::LazyV2<ContextRule> = ::protobuf::rt::LazyV2::INIT;
        instance.get(ContextRule::new)
    }
}

impl ::protobuf::Clear for ContextRule {
    fn clear(&mut self) {
        self.selector.clear();
        self.requested.clear();
        self.provided.clear();
        self.allowed_request_extensions.clear();
        self.allowed_response_extensions.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ContextRule {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ContextRule {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x18google/api/context.proto\x12\ngoogle.api\"8\n\x07Context\x12-\n\
    \x05rules\x18\x01\x20\x03(\x0b2\x17.google.api.ContextRuleR\x05rules\"\
    \xe1\x01\n\x0bContextRule\x12\x1a\n\x08selector\x18\x01\x20\x01(\tR\x08s\
    elector\x12\x1c\n\trequested\x18\x02\x20\x03(\tR\trequested\x12\x1a\n\
    \x08provided\x18\x03\x20\x03(\tR\x08provided\x12<\n\x1aallowed_request_e\
    xtensions\x18\x04\x20\x03(\tR\x18allowedRequestExtensions\x12>\n\x1ballo\
    wed_response_extensions\x18\x05\x20\x03(\tR\x19allowedResponseExtensions\
    Bn\n\x0ecom.google.apiB\x0cContextProtoP\x01ZEgoogle.golang.org/genproto\
    /googleapis/api/serviceconfig;serviceconfig\xa2\x02\x04GAPIJ\x9c\x17\n\
    \x06\x12\x04\x0f\0Z\x01\n\xbe\x04\n\x01\x0c\x12\x03\x0f\0\x122\xb3\x04\
    \x20Copyright\x202018\x20Google\x20LLC.\n\n\x20Licensed\x20under\x20the\
    \x20Apache\x20License,\x20Version\x202.0\x20(the\x20\"License\");\n\x20y\
    ou\x20may\x20not\x20use\x20this\x20file\x20except\x20in\x20compliance\
    \x20with\x20the\x20License.\n\x20You\x20may\x20obtain\x20a\x20copy\x20of\
    \x20the\x20License\x20at\n\n\x20\x20\x20\x20\x20http://www.apache.org/li\
    censes/LICENSE-2.0\n\n\x20Unless\x20required\x20by\x20applicable\x20law\
    \x20or\x20agreed\x20to\x20in\x20writing,\x20software\n\x20distributed\
    \x20under\x20the\x20License\x20is\x20distributed\x20on\x20an\x20\"AS\x20\
    IS\"\x20BASIS,\n\x20WITHOUT\x20WARRANTIES\x20OR\x20CONDITIONS\x20OF\x20A\
    NY\x20KIND,\x20either\x20express\x20or\x20implied.\n\x20See\x20the\x20Li\
    cense\x20for\x20the\x20specific\x20language\x20governing\x20permissions\
    \x20and\n\x20limitations\x20under\x20the\x20License.\n\n\n\x08\n\x01\x02\
    \x12\x03\x11\0\x13\n\x08\n\x01\x08\x12\x03\x13\0\\\n\t\n\x02\x08\x0b\x12\
    \x03\x13\0\\\n\x08\n\x01\x08\x12\x03\x14\0\"\n\t\n\x02\x08\n\x12\x03\x14\
    \0\"\n\x08\n\x01\x08\x12\x03\x15\0-\n\t\n\x02\x08\x08\x12\x03\x15\0-\n\
    \x08\n\x01\x08\x12\x03\x16\0'\n\t\n\x02\x08\x01\x12\x03\x16\0'\n\x08\n\
    \x01\x08\x12\x03\x17\0\"\n\t\n\x02\x08$\x12\x03\x17\0\"\n\xdf\x08\n\x02\
    \x04\0\x12\x04>\0C\x01\x1a\xd2\x08\x20`Context`\x20defines\x20which\x20c\
    ontexts\x20an\x20API\x20requests.\n\n\x20Example:\n\n\x20\x20\x20\x20\
    \x20context:\n\x20\x20\x20\x20\x20\x20\x20rules:\n\x20\x20\x20\x20\x20\
    \x20\x20-\x20selector:\x20\"*\"\n\x20\x20\x20\x20\x20\x20\x20\x20\x20req\
    uested:\n\x20\x20\x20\x20\x20\x20\x20\x20\x20-\x20google.rpc.context.Pro\
    jectContext\n\x20\x20\x20\x20\x20\x20\x20\x20\x20-\x20google.rpc.context\
    .OriginContext\n\n\x20The\x20above\x20specifies\x20that\x20all\x20method\
    s\x20in\x20the\x20API\x20request\n\x20`google.rpc.context.ProjectContext\
    `\x20and\n\x20`google.rpc.context.OriginContext`.\n\n\x20Available\x20co\
    ntext\x20types\x20are\x20defined\x20in\x20package\n\x20`google.rpc.conte\
    xt`.\n\n\x20This\x20also\x20provides\x20mechanism\x20to\x20whitelist\x20\
    any\x20protobuf\x20message\x20extension\x20that\n\x20can\x20be\x20sent\
    \x20in\x20grpc\x20metadata\x20using\x20\xe2\x80\x9cx-goog-ext-<extension\
    _id>-bin\xe2\x80\x9d\x20and\n\x20\xe2\x80\x9cx-goog-ext-<extension_id>-j\
    spb\xe2\x80\x9d\x20format.\x20For\x20example,\x20list\x20any\x20service\
    \n\x20specific\x20protobuf\x20types\x20that\x20can\x20appear\x20in\x20gr\
    pc\x20metadata\x20as\x20follows\x20in\x20your\n\x20yaml\x20file:\n\n\x20\
    Example:\n\n\x20\x20\x20\x20\x20context:\n\x20\x20\x20\x20\x20\x20\x20ru\
    les:\n\x20\x20\x20\x20\x20\x20\x20\x20-\x20selector:\x20\"google.example\
    .library.v1.LibraryService.CreateBook\"\n\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20allowed_request_extensions:\n\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20-\x20google.foo.v1.NewExtension\n\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20allowed_response_extensions:\n\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20-\x20google.foo.v1.NewExtension\n\n\x20You\x20can\x20als\
    o\x20specify\x20extension\x20ID\x20instead\x20of\x20fully\x20qualified\
    \x20extension\x20name\n\x20here.\n\n\n\n\x03\x04\0\x01\x12\x03>\x08\x0f\
    \n\x9b\x01\n\x04\x04\0\x02\0\x12\x03B\x02!\x1a\x8d\x01\x20A\x20list\x20o\
    f\x20RPC\x20context\x20rules\x20that\x20apply\x20to\x20individual\x20API\
    \x20methods.\n\n\x20**NOTE:**\x20All\x20service\x20configuration\x20rule\
    s\x20follow\x20\"last\x20one\x20wins\"\x20order.\n\n\x0c\n\x05\x04\0\x02\
    \0\x04\x12\x03B\x02\n\n\x0c\n\x05\x04\0\x02\0\x06\x12\x03B\x0b\x16\n\x0c\
    \n\x05\x04\0\x02\0\x01\x12\x03B\x17\x1c\n\x0c\n\x05\x04\0\x02\0\x03\x12\
    \x03B\x1f\x20\nc\n\x02\x04\x01\x12\x04G\0Z\x01\x1aW\x20A\x20context\x20r\
    ule\x20provides\x20information\x20about\x20the\x20context\x20for\x20an\
    \x20individual\x20API\n\x20element.\n\n\n\n\x03\x04\x01\x01\x12\x03G\x08\
    \x13\n\x90\x01\n\x04\x04\x01\x02\0\x12\x03K\x02\x16\x1a\x82\x01\x20Selec\
    ts\x20the\x20methods\x20to\x20which\x20this\x20rule\x20applies.\n\n\x20R\
    efer\x20to\x20[selector][google.api.DocumentationRule.selector]\x20for\
    \x20syntax\x20details.\n\n\r\n\x05\x04\x01\x02\0\x04\x12\x04K\x02G\x15\n\
    \x0c\n\x05\x04\x01\x02\0\x05\x12\x03K\x02\x08\n\x0c\n\x05\x04\x01\x02\0\
    \x01\x12\x03K\t\x11\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03K\x14\x15\n?\n\
    \x04\x04\x01\x02\x01\x12\x03N\x02\x20\x1a2\x20A\x20list\x20of\x20full\
    \x20type\x20names\x20of\x20requested\x20contexts.\n\n\x0c\n\x05\x04\x01\
    \x02\x01\x04\x12\x03N\x02\n\n\x0c\n\x05\x04\x01\x02\x01\x05\x12\x03N\x0b\
    \x11\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\x03N\x12\x1b\n\x0c\n\x05\x04\
    \x01\x02\x01\x03\x12\x03N\x1e\x1f\n>\n\x04\x04\x01\x02\x02\x12\x03Q\x02\
    \x1f\x1a1\x20A\x20list\x20of\x20full\x20type\x20names\x20of\x20provided\
    \x20contexts.\n\n\x0c\n\x05\x04\x01\x02\x02\x04\x12\x03Q\x02\n\n\x0c\n\
    \x05\x04\x01\x02\x02\x05\x12\x03Q\x0b\x11\n\x0c\n\x05\x04\x01\x02\x02\
    \x01\x12\x03Q\x12\x1a\n\x0c\n\x05\x04\x01\x02\x02\x03\x12\x03Q\x1d\x1e\n\
    }\n\x04\x04\x01\x02\x03\x12\x03U\x021\x1ap\x20A\x20list\x20of\x20full\
    \x20type\x20names\x20or\x20extension\x20IDs\x20of\x20extensions\x20allow\
    ed\x20in\x20grpc\n\x20side\x20channel\x20from\x20client\x20to\x20backend\
    .\n\n\x0c\n\x05\x04\x01\x02\x03\x04\x12\x03U\x02\n\n\x0c\n\x05\x04\x01\
    \x02\x03\x05\x12\x03U\x0b\x11\n\x0c\n\x05\x04\x01\x02\x03\x01\x12\x03U\
    \x12,\n\x0c\n\x05\x04\x01\x02\x03\x03\x12\x03U/0\n}\n\x04\x04\x01\x02\
    \x04\x12\x03Y\x022\x1ap\x20A\x20list\x20of\x20full\x20type\x20names\x20o\
    r\x20extension\x20IDs\x20of\x20extensions\x20allowed\x20in\x20grpc\n\x20\
    side\x20channel\x20from\x20backend\x20to\x20client.\n\n\x0c\n\x05\x04\
    \x01\x02\x04\x04\x12\x03Y\x02\n\n\x0c\n\x05\x04\x01\x02\x04\x05\x12\x03Y\
    \x0b\x11\n\x0c\n\x05\x04\x01\x02\x04\x01\x12\x03Y\x12-\n\x0c\n\x05\x04\
    \x01\x02\x04\x03\x12\x03Y01b\x06proto3\
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
