// This file is generated by rust-protobuf 3.2.0. Do not edit
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

//! Generated file from `google/api/log.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_2_0;

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:google.api.LogDescriptor)
pub struct LogDescriptor {
    // message fields
    ///  The name of the log. It must be less than 512 characters long and can
    ///  include the following characters: upper- and lower-case alphanumeric
    ///  characters [A-Za-z0-9], and punctuation characters including
    ///  slash, underscore, hyphen, period [/_-.].
    // @@protoc_insertion_point(field:google.api.LogDescriptor.name)
    pub name: ::std::string::String,
    ///  The set of labels that are available to describe a specific log entry.
    ///  Runtime requests that contain labels not specified here are
    ///  considered invalid.
    // @@protoc_insertion_point(field:google.api.LogDescriptor.labels)
    pub labels: ::std::vec::Vec<super::label::LabelDescriptor>,
    ///  A human-readable description of this log. This information appears in
    ///  the documentation and can contain details.
    // @@protoc_insertion_point(field:google.api.LogDescriptor.description)
    pub description: ::std::string::String,
    ///  The human-readable name for this log. This information appears on
    ///  the user interface and should be concise.
    // @@protoc_insertion_point(field:google.api.LogDescriptor.display_name)
    pub display_name: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:google.api.LogDescriptor.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a LogDescriptor {
    fn default() -> &'a LogDescriptor {
        <LogDescriptor as ::protobuf::Message>::default_instance()
    }
}

impl LogDescriptor {
    pub fn new() -> LogDescriptor {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "name",
            |m: &LogDescriptor| { &m.name },
            |m: &mut LogDescriptor| { &mut m.name },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "labels",
            |m: &LogDescriptor| { &m.labels },
            |m: &mut LogDescriptor| { &mut m.labels },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "description",
            |m: &LogDescriptor| { &m.description },
            |m: &mut LogDescriptor| { &mut m.description },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "display_name",
            |m: &LogDescriptor| { &m.display_name },
            |m: &mut LogDescriptor| { &mut m.display_name },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<LogDescriptor>(
            "LogDescriptor",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for LogDescriptor {
    const NAME: &'static str = "LogDescriptor";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.name = is.read_string()?;
                },
                18 => {
                    self.labels.push(is.read_message()?);
                },
                26 => {
                    self.description = is.read_string()?;
                },
                34 => {
                    self.display_name = is.read_string()?;
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
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.name);
        }
        for value in &self.labels {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if !self.description.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.description);
        }
        if !self.display_name.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.display_name);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        for v in &self.labels {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        };
        if !self.description.is_empty() {
            os.write_string(3, &self.description)?;
        }
        if !self.display_name.is_empty() {
            os.write_string(4, &self.display_name)?;
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

    fn new() -> LogDescriptor {
        LogDescriptor::new()
    }

    fn clear(&mut self) {
        self.name.clear();
        self.labels.clear();
        self.description.clear();
        self.display_name.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static LogDescriptor {
        static instance: LogDescriptor = LogDescriptor {
            name: ::std::string::String::new(),
            labels: ::std::vec::Vec::new(),
            description: ::std::string::String::new(),
            display_name: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for LogDescriptor {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("LogDescriptor").unwrap()).clone()
    }
}

impl ::std::fmt::Display for LogDescriptor {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LogDescriptor {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x14google/api/log.proto\x12\ngoogle.api\x1a\x16google/api/label.proto\
    \"\x9d\x01\n\rLogDescriptor\x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04nam\
    e\x123\n\x06labels\x18\x02\x20\x03(\x0b2\x1b.google.api.LabelDescriptorR\
    \x06labels\x12\x20\n\x0bdescription\x18\x03\x20\x01(\tR\x0bdescription\
    \x12!\n\x0cdisplay_name\x18\x04\x20\x01(\tR\x0bdisplayNameBj\n\x0ecom.go\
    ogle.apiB\x08LogProtoP\x01ZEgoogle.golang.org/genproto/googleapis/api/se\
    rviceconfig;serviceconfig\xa2\x02\x04GAPIJ\x80\x0f\n\x06\x12\x04\x0f\06\
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
    \x20under\x20the\x20License.\n\n\n\x08\n\x01\x02\x12\x03\x11\0\x13\n\t\n\
    \x02\x03\0\x12\x03\x13\0\x20\n\x08\n\x01\x08\x12\x03\x15\0\\\n\t\n\x02\
    \x08\x0b\x12\x03\x15\0\\\n\x08\n\x01\x08\x12\x03\x16\0\"\n\t\n\x02\x08\n\
    \x12\x03\x16\0\"\n\x08\n\x01\x08\x12\x03\x17\0)\n\t\n\x02\x08\x08\x12\
    \x03\x17\0)\n\x08\n\x01\x08\x12\x03\x18\0'\n\t\n\x02\x08\x01\x12\x03\x18\
    \0'\n\x08\n\x01\x08\x12\x03\x19\0\"\n\t\n\x02\x08$\x12\x03\x19\0\"\n\xc2\
    \x02\n\x02\x04\0\x12\x04#\06\x01\x1a\xb5\x02\x20A\x20description\x20of\
    \x20a\x20log\x20type.\x20Example\x20in\x20YAML\x20format:\n\n\x20\x20\
    \x20\x20\x20-\x20name:\x20library.googleapis.com/activity_history\n\x20\
    \x20\x20\x20\x20\x20\x20description:\x20The\x20history\x20of\x20borrowin\
    g\x20and\x20returning\x20library\x20items.\n\x20\x20\x20\x20\x20\x20\x20\
    display_name:\x20Activity\n\x20\x20\x20\x20\x20\x20\x20labels:\n\x20\x20\
    \x20\x20\x20\x20\x20-\x20key:\x20/customer_id\n\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20description:\x20Identifier\x20of\x20a\x20library\x20customer\
    \n\n\n\n\x03\x04\0\x01\x12\x03#\x08\x15\n\x84\x02\n\x04\x04\0\x02\0\x12\
    \x03(\x02\x12\x1a\xf6\x01\x20The\x20name\x20of\x20the\x20log.\x20It\x20m\
    ust\x20be\x20less\x20than\x20512\x20characters\x20long\x20and\x20can\n\
    \x20include\x20the\x20following\x20characters:\x20upper-\x20and\x20lower\
    -case\x20alphanumeric\n\x20characters\x20[A-Za-z0-9],\x20and\x20punctuat\
    ion\x20characters\x20including\n\x20slash,\x20underscore,\x20hyphen,\x20\
    period\x20[/_-.].\n\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03(\x02\x08\n\x0c\n\
    \x05\x04\0\x02\0\x01\x12\x03(\t\r\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03(\
    \x10\x11\n\xa8\x01\n\x04\x04\0\x02\x01\x12\x03-\x02&\x1a\x9a\x01\x20The\
    \x20set\x20of\x20labels\x20that\x20are\x20available\x20to\x20describe\
    \x20a\x20specific\x20log\x20entry.\n\x20Runtime\x20requests\x20that\x20c\
    ontain\x20labels\x20not\x20specified\x20here\x20are\n\x20considered\x20i\
    nvalid.\n\n\x0c\n\x05\x04\0\x02\x01\x04\x12\x03-\x02\n\n\x0c\n\x05\x04\0\
    \x02\x01\x06\x12\x03-\x0b\x1a\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03-\x1b\
    !\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03-$%\n\x80\x01\n\x04\x04\0\x02\x02\
    \x12\x031\x02\x19\x1as\x20A\x20human-readable\x20description\x20of\x20th\
    is\x20log.\x20This\x20information\x20appears\x20in\n\x20the\x20documenta\
    tion\x20and\x20can\x20contain\x20details.\n\n\x0c\n\x05\x04\0\x02\x02\
    \x05\x12\x031\x02\x08\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x031\t\x14\n\x0c\
    \n\x05\x04\0\x02\x02\x03\x12\x031\x17\x18\n{\n\x04\x04\0\x02\x03\x12\x03\
    5\x02\x1a\x1an\x20The\x20human-readable\x20name\x20for\x20this\x20log.\
    \x20This\x20information\x20appears\x20on\n\x20the\x20user\x20interface\
    \x20and\x20should\x20be\x20concise.\n\n\x0c\n\x05\x04\0\x02\x03\x05\x12\
    \x035\x02\x08\n\x0c\n\x05\x04\0\x02\x03\x01\x12\x035\t\x15\n\x0c\n\x05\
    \x04\0\x02\x03\x03\x12\x035\x18\x19b\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(1);
            deps.push(super::label::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(LogDescriptor::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(0);
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
