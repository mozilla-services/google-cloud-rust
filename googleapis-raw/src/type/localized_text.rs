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
//! Generated file from `google/type/localized_text.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_25_1;

#[derive(PartialEq,Clone,Default)]
pub struct LocalizedText {
    // message fields
    pub text: ::std::string::String,
    pub language_code: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a LocalizedText {
    fn default() -> &'a LocalizedText {
        <LocalizedText as ::protobuf::Message>::default_instance()
    }
}

impl LocalizedText {
    pub fn new() -> LocalizedText {
        ::std::default::Default::default()
    }

    // string text = 1;


    pub fn get_text(&self) -> &str {
        &self.text
    }
    pub fn clear_text(&mut self) {
        self.text.clear();
    }

    // Param is passed by value, moved
    pub fn set_text(&mut self, v: ::std::string::String) {
        self.text = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_text(&mut self) -> &mut ::std::string::String {
        &mut self.text
    }

    // Take field
    pub fn take_text(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.text, ::std::string::String::new())
    }

    // string language_code = 2;


    pub fn get_language_code(&self) -> &str {
        &self.language_code
    }
    pub fn clear_language_code(&mut self) {
        self.language_code.clear();
    }

    // Param is passed by value, moved
    pub fn set_language_code(&mut self, v: ::std::string::String) {
        self.language_code = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_language_code(&mut self) -> &mut ::std::string::String {
        &mut self.language_code
    }

    // Take field
    pub fn take_language_code(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.language_code, ::std::string::String::new())
    }
}

impl ::protobuf::Message for LocalizedText {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.text)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.language_code)?;
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
        if !self.text.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.text);
        }
        if !self.language_code.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.language_code);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.text.is_empty() {
            os.write_string(1, &self.text)?;
        }
        if !self.language_code.is_empty() {
            os.write_string(2, &self.language_code)?;
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

    fn new() -> LocalizedText {
        LocalizedText::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "text",
                |m: &LocalizedText| { &m.text },
                |m: &mut LocalizedText| { &mut m.text },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "language_code",
                |m: &LocalizedText| { &m.language_code },
                |m: &mut LocalizedText| { &mut m.language_code },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<LocalizedText>(
                "LocalizedText",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static LocalizedText {
        static instance: ::protobuf::rt::LazyV2<LocalizedText> = ::protobuf::rt::LazyV2::INIT;
        instance.get(LocalizedText::new)
    }
}

impl ::protobuf::Clear for LocalizedText {
    fn clear(&mut self) {
        self.text.clear();
        self.language_code.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LocalizedText {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LocalizedText {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x20google/type/localized_text.proto\x12\x0bgoogle.type\"H\n\rLocalize\
    dText\x12\x12\n\x04text\x18\x01\x20\x01(\tR\x04text\x12#\n\rlanguage_cod\
    e\x18\x02\x20\x01(\tR\x0clanguageCodeBz\n\x0fcom.google.typeB\x12Localiz\
    edTextProtoP\x01ZHgoogle.golang.org/genproto/googleapis/type/localized_t\
    ext;localized_text\xf8\x01\x01\xa2\x02\x03GTPJ\xfb\x08\n\x06\x12\x04\x0e\
    \0#\x01\n\xbc\x04\n\x01\x0c\x12\x03\x0e\0\x122\xb1\x04\x20Copyright\x202\
    021\x20Google\x20LLC\n\n\x20Licensed\x20under\x20the\x20Apache\x20Licens\
    e,\x20Version\x202.0\x20(the\x20\"License\");\n\x20you\x20may\x20not\x20\
    use\x20this\x20file\x20except\x20in\x20compliance\x20with\x20the\x20Lice\
    nse.\n\x20You\x20may\x20obtain\x20a\x20copy\x20of\x20the\x20License\x20a\
    t\n\n\x20\x20\x20\x20\x20http://www.apache.org/licenses/LICENSE-2.0\n\n\
    \x20Unless\x20required\x20by\x20applicable\x20law\x20or\x20agreed\x20to\
    \x20in\x20writing,\x20software\n\x20distributed\x20under\x20the\x20Licen\
    se\x20is\x20distributed\x20on\x20an\x20\"AS\x20IS\"\x20BASIS,\n\x20WITHO\
    UT\x20WARRANTIES\x20OR\x20CONDITIONS\x20OF\x20ANY\x20KIND,\x20either\x20\
    express\x20or\x20implied.\n\x20See\x20the\x20License\x20for\x20the\x20sp\
    ecific\x20language\x20governing\x20permissions\x20and\n\x20limitations\
    \x20under\x20the\x20License.\n\n\x08\n\x01\x02\x12\x03\x10\0\x14\n\x08\n\
    \x01\x08\x12\x03\x12\0\x1f\n\t\n\x02\x08\x1f\x12\x03\x12\0\x1f\n\x08\n\
    \x01\x08\x12\x03\x13\0_\n\t\n\x02\x08\x0b\x12\x03\x13\0_\n\x08\n\x01\x08\
    \x12\x03\x14\0\"\n\t\n\x02\x08\n\x12\x03\x14\0\"\n\x08\n\x01\x08\x12\x03\
    \x15\03\n\t\n\x02\x08\x08\x12\x03\x15\03\n\x08\n\x01\x08\x12\x03\x16\0(\
    \n\t\n\x02\x08\x01\x12\x03\x16\0(\n\x08\n\x01\x08\x12\x03\x17\0!\n\t\n\
    \x02\x08$\x12\x03\x17\0!\nC\n\x02\x04\0\x12\x04\x1a\0#\x01\x1a7\x20Local\
    ized\x20variant\x20of\x20a\x20text\x20in\x20a\x20particular\x20language.\
    \n\n\n\n\x03\x04\0\x01\x12\x03\x1a\x08\x15\nW\n\x04\x04\0\x02\0\x12\x03\
    \x1c\x02\x12\x1aJ\x20Localized\x20string\x20in\x20the\x20language\x20cor\
    responding\x20to\x20`language_code'\x20below.\n\n\x0c\n\x05\x04\0\x02\0\
    \x05\x12\x03\x1c\x02\x08\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x1c\t\r\n\
    \x0c\n\x05\x04\0\x02\0\x03\x12\x03\x1c\x10\x11\n\xab\x01\n\x04\x04\0\x02\
    \x01\x12\x03\"\x02\x1b\x1a\x9d\x01\x20The\x20text's\x20BCP-47\x20languag\
    e\x20code,\x20such\x20as\x20\"en-US\"\x20or\x20\"sr-Latn\".\n\n\x20For\
    \x20more\x20information,\x20see\n\x20http://www.unicode.org/reports/tr35\
    /#Unicode_locale_identifier.\n\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\"\
    \x02\x08\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\"\t\x16\n\x0c\n\x05\x04\0\
    \x02\x01\x03\x12\x03\"\x19\x1ab\x06proto3\
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
