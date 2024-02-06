// This file is generated by rust-protobuf 3.3.0. Do not edit
// .proto file is parsed by protoc --rust-out=...
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
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `google/api/label.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_3_0;

///  A description of a label.
// @@protoc_insertion_point(message:google.api.LabelDescriptor)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct LabelDescriptor {
    // message fields
    ///  The label key.
    // @@protoc_insertion_point(field:google.api.LabelDescriptor.key)
    pub key: ::std::string::String,
    ///  The type of data that can be assigned to the label.
    // @@protoc_insertion_point(field:google.api.LabelDescriptor.value_type)
    pub value_type: ::protobuf::EnumOrUnknown<label_descriptor::ValueType>,
    ///  A human-readable description for the label.
    // @@protoc_insertion_point(field:google.api.LabelDescriptor.description)
    pub description: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:google.api.LabelDescriptor.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a LabelDescriptor {
    fn default() -> &'a LabelDescriptor {
        <LabelDescriptor as ::protobuf::Message>::default_instance()
    }
}

impl LabelDescriptor {
    pub fn new() -> LabelDescriptor {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "key",
            |m: &LabelDescriptor| { &m.key },
            |m: &mut LabelDescriptor| { &mut m.key },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "value_type",
            |m: &LabelDescriptor| { &m.value_type },
            |m: &mut LabelDescriptor| { &mut m.value_type },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "description",
            |m: &LabelDescriptor| { &m.description },
            |m: &mut LabelDescriptor| { &mut m.description },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<LabelDescriptor>(
            "LabelDescriptor",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for LabelDescriptor {
    const NAME: &'static str = "LabelDescriptor";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.key = is.read_string()?;
                },
                16 => {
                    self.value_type = is.read_enum_or_unknown()?;
                },
                26 => {
                    self.description = is.read_string()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if !self.key.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.key);
        }
        if self.value_type != ::protobuf::EnumOrUnknown::new(label_descriptor::ValueType::STRING) {
            my_size += ::protobuf::rt::int32_size(2, self.value_type.value());
        }
        if !self.description.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.description);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.key.is_empty() {
            os.write_string(1, &self.key)?;
        }
        if self.value_type != ::protobuf::EnumOrUnknown::new(label_descriptor::ValueType::STRING) {
            os.write_enum(2, ::protobuf::EnumOrUnknown::value(&self.value_type))?;
        }
        if !self.description.is_empty() {
            os.write_string(3, &self.description)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> LabelDescriptor {
        LabelDescriptor::new()
    }

    fn clear(&mut self) {
        self.key.clear();
        self.value_type = ::protobuf::EnumOrUnknown::new(label_descriptor::ValueType::STRING);
        self.description.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static LabelDescriptor {
        static instance: LabelDescriptor = LabelDescriptor {
            key: ::std::string::String::new(),
            value_type: ::protobuf::EnumOrUnknown::from_i32(0),
            description: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for LabelDescriptor {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("LabelDescriptor").unwrap()).clone()
    }
}

impl ::std::fmt::Display for LabelDescriptor {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LabelDescriptor {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `LabelDescriptor`
pub mod label_descriptor {
    ///  Value types that can be used as label values.
    #[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
    // @@protoc_insertion_point(enum:google.api.LabelDescriptor.ValueType)
    pub enum ValueType {
        // @@protoc_insertion_point(enum_value:google.api.LabelDescriptor.ValueType.STRING)
        STRING = 0,
        // @@protoc_insertion_point(enum_value:google.api.LabelDescriptor.ValueType.BOOL)
        BOOL = 1,
        // @@protoc_insertion_point(enum_value:google.api.LabelDescriptor.ValueType.INT64)
        INT64 = 2,
    }

    impl ::protobuf::Enum for ValueType {
        const NAME: &'static str = "ValueType";

        fn value(&self) -> i32 {
            *self as i32
        }

        fn from_i32(value: i32) -> ::std::option::Option<ValueType> {
            match value {
                0 => ::std::option::Option::Some(ValueType::STRING),
                1 => ::std::option::Option::Some(ValueType::BOOL),
                2 => ::std::option::Option::Some(ValueType::INT64),
                _ => ::std::option::Option::None
            }
        }

        fn from_str(str: &str) -> ::std::option::Option<ValueType> {
            match str {
                "STRING" => ::std::option::Option::Some(ValueType::STRING),
                "BOOL" => ::std::option::Option::Some(ValueType::BOOL),
                "INT64" => ::std::option::Option::Some(ValueType::INT64),
                _ => ::std::option::Option::None
            }
        }

        const VALUES: &'static [ValueType] = &[
            ValueType::STRING,
            ValueType::BOOL,
            ValueType::INT64,
        ];
    }

    impl ::protobuf::EnumFull for ValueType {
        fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| super::file_descriptor().enum_by_package_relative_name("LabelDescriptor.ValueType").unwrap()).clone()
        }

        fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
            let index = *self as usize;
            Self::enum_descriptor().value_by_index(index)
        }
    }

    impl ::std::default::Default for ValueType {
        fn default() -> Self {
            ValueType::STRING
        }
    }

    impl ValueType {
        pub(in super) fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
            ::protobuf::reflect::GeneratedEnumDescriptorData::new::<ValueType>("LabelDescriptor.ValueType")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x16google/api/label.proto\x12\ngoogle.api\"\xb9\x01\n\x0fLabelDescrip\
    tor\x12\x10\n\x03key\x18\x01\x20\x01(\tR\x03key\x12D\n\nvalue_type\x18\
    \x02\x20\x01(\x0e2%.google.api.LabelDescriptor.ValueTypeR\tvalueType\x12\
    \x20\n\x0bdescription\x18\x03\x20\x01(\tR\x0bdescription\",\n\tValueType\
    \x12\n\n\x06STRING\x10\0\x12\x08\n\x04BOOL\x10\x01\x12\t\n\x05INT64\x10\
    \x02B_\n\x0ecom.google.apiB\nLabelProtoP\x01Z5google.golang.org/genproto\
    /googleapis/api/label;label\xf8\x01\x01\xa2\x02\x04GAPIJ\xe6\n\n\x06\x12\
    \x04\x0f\00\x01\n\xbe\x04\n\x01\x0c\x12\x03\x0f\0\x122\xb3\x04\x20Copyri\
    ght\x202019\x20Google\x20LLC.\n\n\x20Licensed\x20under\x20the\x20Apache\
    \x20License,\x20Version\x202.0\x20(the\x20\"License\");\n\x20you\x20may\
    \x20not\x20use\x20this\x20file\x20except\x20in\x20compliance\x20with\x20\
    the\x20License.\n\x20You\x20may\x20obtain\x20a\x20copy\x20of\x20the\x20L\
    icense\x20at\n\n\x20\x20\x20\x20\x20http://www.apache.org/licenses/LICEN\
    SE-2.0\n\n\x20Unless\x20required\x20by\x20applicable\x20law\x20or\x20agr\
    eed\x20to\x20in\x20writing,\x20software\n\x20distributed\x20under\x20the\
    \x20License\x20is\x20distributed\x20on\x20an\x20\"AS\x20IS\"\x20BASIS,\n\
    \x20WITHOUT\x20WARRANTIES\x20OR\x20CONDITIONS\x20OF\x20ANY\x20KIND,\x20e\
    ither\x20express\x20or\x20implied.\n\x20See\x20the\x20License\x20for\x20\
    the\x20specific\x20language\x20governing\x20permissions\x20and\n\x20limi\
    tations\x20under\x20the\x20License.\n\n\n\x08\n\x01\x02\x12\x03\x11\0\
    \x13\n\x08\n\x01\x08\x12\x03\x13\0\x1f\n\t\n\x02\x08\x1f\x12\x03\x13\0\
    \x1f\n\x08\n\x01\x08\x12\x03\x14\0L\n\t\n\x02\x08\x0b\x12\x03\x14\0L\n\
    \x08\n\x01\x08\x12\x03\x15\0\"\n\t\n\x02\x08\n\x12\x03\x15\0\"\n\x08\n\
    \x01\x08\x12\x03\x16\0+\n\t\n\x02\x08\x08\x12\x03\x16\0+\n\x08\n\x01\x08\
    \x12\x03\x17\0'\n\t\n\x02\x08\x01\x12\x03\x17\0'\n\x08\n\x01\x08\x12\x03\
    \x18\0\"\n\t\n\x02\x08$\x12\x03\x18\0\"\n'\n\x02\x04\0\x12\x04\x1b\00\
    \x01\x1a\x1b\x20A\x20description\x20of\x20a\x20label.\n\n\n\n\x03\x04\0\
    \x01\x12\x03\x1b\x08\x17\n=\n\x04\x04\0\x04\0\x12\x04\x1d\x02&\x03\x1a/\
    \x20Value\x20types\x20that\x20can\x20be\x20used\x20as\x20label\x20values\
    .\n\n\x0c\n\x05\x04\0\x04\0\x01\x12\x03\x1d\x07\x10\n?\n\x06\x04\0\x04\0\
    \x02\0\x12\x03\x1f\x04\x0f\x1a0\x20A\x20variable-length\x20string.\x20Th\
    is\x20is\x20the\x20default.\n\n\x0e\n\x07\x04\0\x04\0\x02\0\x01\x12\x03\
    \x1f\x04\n\n\x0e\n\x07\x04\0\x04\0\x02\0\x02\x12\x03\x1f\r\x0e\n(\n\x06\
    \x04\0\x04\0\x02\x01\x12\x03\"\x04\r\x1a\x19\x20Boolean;\x20true\x20or\
    \x20false.\n\n\x0e\n\x07\x04\0\x04\0\x02\x01\x01\x12\x03\"\x04\x08\n\x0e\
    \n\x07\x04\0\x04\0\x02\x01\x02\x12\x03\"\x0b\x0c\n)\n\x06\x04\0\x04\0\
    \x02\x02\x12\x03%\x04\x0e\x1a\x1a\x20A\x2064-bit\x20signed\x20integer.\n\
    \n\x0e\n\x07\x04\0\x04\0\x02\x02\x01\x12\x03%\x04\t\n\x0e\n\x07\x04\0\
    \x04\0\x02\x02\x02\x12\x03%\x0c\r\n\x1d\n\x04\x04\0\x02\0\x12\x03)\x02\
    \x11\x1a\x10\x20The\x20label\x20key.\n\n\x0c\n\x05\x04\0\x02\0\x05\x12\
    \x03)\x02\x08\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03)\t\x0c\n\x0c\n\x05\x04\
    \0\x02\0\x03\x12\x03)\x0f\x10\nB\n\x04\x04\0\x02\x01\x12\x03,\x02\x1b\
    \x1a5\x20The\x20type\x20of\x20data\x20that\x20can\x20be\x20assigned\x20t\
    o\x20the\x20label.\n\n\x0c\n\x05\x04\0\x02\x01\x06\x12\x03,\x02\x0b\n\
    \x0c\n\x05\x04\0\x02\x01\x01\x12\x03,\x0c\x16\n\x0c\n\x05\x04\0\x02\x01\
    \x03\x12\x03,\x19\x1a\n:\n\x04\x04\0\x02\x02\x12\x03/\x02\x19\x1a-\x20A\
    \x20human-readable\x20description\x20for\x20the\x20label.\n\n\x0c\n\x05\
    \x04\0\x02\x02\x05\x12\x03/\x02\x08\n\x0c\n\x05\x04\0\x02\x02\x01\x12\
    \x03/\t\x14\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03/\x17\x18b\x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(LabelDescriptor::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(1);
            enums.push(label_descriptor::ValueType::generated_enum_descriptor_data());
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
