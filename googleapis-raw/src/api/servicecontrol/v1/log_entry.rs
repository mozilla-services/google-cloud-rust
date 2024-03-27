// This file is generated by rust-protobuf 3.4.0. Do not edit
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

//! Generated file from `google/api/servicecontrol/v1/log_entry.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

///  An individual log entry.
// @@protoc_insertion_point(message:google.api.servicecontrol.v1.LogEntry)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct LogEntry {
    // message fields
    ///  Required. The log to which this log entry belongs. Examples: `"syslog"`,
    ///  `"book_log"`.
    // @@protoc_insertion_point(field:google.api.servicecontrol.v1.LogEntry.name)
    pub name: ::std::string::String,
    ///  The time the event described by the log entry occurred. If
    ///  omitted, defaults to operation start time.
    // @@protoc_insertion_point(field:google.api.servicecontrol.v1.LogEntry.timestamp)
    pub timestamp: ::protobuf::MessageField<::protobuf::well_known_types::timestamp::Timestamp>,
    ///  The severity of the log entry. The default value is
    ///  `LogSeverity.DEFAULT`.
    // @@protoc_insertion_point(field:google.api.servicecontrol.v1.LogEntry.severity)
    pub severity: ::protobuf::EnumOrUnknown<super::log_severity::LogSeverity>,
    ///  A unique ID for the log entry used for deduplication. If omitted,
    ///  the implementation will generate one based on operation_id.
    // @@protoc_insertion_point(field:google.api.servicecontrol.v1.LogEntry.insert_id)
    pub insert_id: ::std::string::String,
    ///  A set of user-defined (key, value) data that provides additional
    ///  information about the log entry.
    // @@protoc_insertion_point(field:google.api.servicecontrol.v1.LogEntry.labels)
    pub labels: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    // message oneof groups
    pub payload: ::std::option::Option<log_entry::Payload>,
    // special fields
    // @@protoc_insertion_point(special_field:google.api.servicecontrol.v1.LogEntry.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a LogEntry {
    fn default() -> &'a LogEntry {
        <LogEntry as ::protobuf::Message>::default_instance()
    }
}

impl LogEntry {
    pub fn new() -> LogEntry {
        ::std::default::Default::default()
    }

    // .google.protobuf.Any proto_payload = 2;

    pub fn proto_payload(&self) -> &::protobuf::well_known_types::any::Any {
        match self.payload {
            ::std::option::Option::Some(log_entry::Payload::ProtoPayload(ref v)) => v,
            _ => <::protobuf::well_known_types::any::Any as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_proto_payload(&mut self) {
        self.payload = ::std::option::Option::None;
    }

    pub fn has_proto_payload(&self) -> bool {
        match self.payload {
            ::std::option::Option::Some(log_entry::Payload::ProtoPayload(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_proto_payload(&mut self, v: ::protobuf::well_known_types::any::Any) {
        self.payload = ::std::option::Option::Some(log_entry::Payload::ProtoPayload(v))
    }

    // Mutable pointer to the field.
    pub fn mut_proto_payload(&mut self) -> &mut ::protobuf::well_known_types::any::Any {
        if let ::std::option::Option::Some(log_entry::Payload::ProtoPayload(_)) = self.payload {
        } else {
            self.payload = ::std::option::Option::Some(log_entry::Payload::ProtoPayload(::protobuf::well_known_types::any::Any::new()));
        }
        match self.payload {
            ::std::option::Option::Some(log_entry::Payload::ProtoPayload(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_proto_payload(&mut self) -> ::protobuf::well_known_types::any::Any {
        if self.has_proto_payload() {
            match self.payload.take() {
                ::std::option::Option::Some(log_entry::Payload::ProtoPayload(v)) => v,
                _ => panic!(),
            }
        } else {
            ::protobuf::well_known_types::any::Any::new()
        }
    }

    // string text_payload = 3;

    pub fn text_payload(&self) -> &str {
        match self.payload {
            ::std::option::Option::Some(log_entry::Payload::TextPayload(ref v)) => v,
            _ => "",
        }
    }

    pub fn clear_text_payload(&mut self) {
        self.payload = ::std::option::Option::None;
    }

    pub fn has_text_payload(&self) -> bool {
        match self.payload {
            ::std::option::Option::Some(log_entry::Payload::TextPayload(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_text_payload(&mut self, v: ::std::string::String) {
        self.payload = ::std::option::Option::Some(log_entry::Payload::TextPayload(v))
    }

    // Mutable pointer to the field.
    pub fn mut_text_payload(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(log_entry::Payload::TextPayload(_)) = self.payload {
        } else {
            self.payload = ::std::option::Option::Some(log_entry::Payload::TextPayload(::std::string::String::new()));
        }
        match self.payload {
            ::std::option::Option::Some(log_entry::Payload::TextPayload(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_text_payload(&mut self) -> ::std::string::String {
        if self.has_text_payload() {
            match self.payload.take() {
                ::std::option::Option::Some(log_entry::Payload::TextPayload(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    // .google.protobuf.Struct struct_payload = 6;

    pub fn struct_payload(&self) -> &::protobuf::well_known_types::struct_::Struct {
        match self.payload {
            ::std::option::Option::Some(log_entry::Payload::StructPayload(ref v)) => v,
            _ => <::protobuf::well_known_types::struct_::Struct as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_struct_payload(&mut self) {
        self.payload = ::std::option::Option::None;
    }

    pub fn has_struct_payload(&self) -> bool {
        match self.payload {
            ::std::option::Option::Some(log_entry::Payload::StructPayload(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_struct_payload(&mut self, v: ::protobuf::well_known_types::struct_::Struct) {
        self.payload = ::std::option::Option::Some(log_entry::Payload::StructPayload(v))
    }

    // Mutable pointer to the field.
    pub fn mut_struct_payload(&mut self) -> &mut ::protobuf::well_known_types::struct_::Struct {
        if let ::std::option::Option::Some(log_entry::Payload::StructPayload(_)) = self.payload {
        } else {
            self.payload = ::std::option::Option::Some(log_entry::Payload::StructPayload(::protobuf::well_known_types::struct_::Struct::new()));
        }
        match self.payload {
            ::std::option::Option::Some(log_entry::Payload::StructPayload(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_struct_payload(&mut self) -> ::protobuf::well_known_types::struct_::Struct {
        if self.has_struct_payload() {
            match self.payload.take() {
                ::std::option::Option::Some(log_entry::Payload::StructPayload(v)) => v,
                _ => panic!(),
            }
        } else {
            ::protobuf::well_known_types::struct_::Struct::new()
        }
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(8);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "name",
            |m: &LogEntry| { &m.name },
            |m: &mut LogEntry| { &mut m.name },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, ::protobuf::well_known_types::timestamp::Timestamp>(
            "timestamp",
            |m: &LogEntry| { &m.timestamp },
            |m: &mut LogEntry| { &mut m.timestamp },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "severity",
            |m: &LogEntry| { &m.severity },
            |m: &mut LogEntry| { &mut m.severity },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "insert_id",
            |m: &LogEntry| { &m.insert_id },
            |m: &mut LogEntry| { &mut m.insert_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_map_simpler_accessor::<_, _, _>(
            "labels",
            |m: &LogEntry| { &m.labels },
            |m: &mut LogEntry| { &mut m.labels },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, ::protobuf::well_known_types::any::Any>(
            "proto_payload",
            LogEntry::has_proto_payload,
            LogEntry::proto_payload,
            LogEntry::mut_proto_payload,
            LogEntry::set_proto_payload,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_deref_has_get_set_simpler_accessor::<_, _>(
            "text_payload",
            LogEntry::has_text_payload,
            LogEntry::text_payload,
            LogEntry::set_text_payload,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, ::protobuf::well_known_types::struct_::Struct>(
            "struct_payload",
            LogEntry::has_struct_payload,
            LogEntry::struct_payload,
            LogEntry::mut_struct_payload,
            LogEntry::set_struct_payload,
        ));
        oneofs.push(log_entry::Payload::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<LogEntry>(
            "LogEntry",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for LogEntry {
    const NAME: &'static str = "LogEntry";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                82 => {
                    self.name = is.read_string()?;
                },
                90 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.timestamp)?;
                },
                96 => {
                    self.severity = is.read_enum_or_unknown()?;
                },
                34 => {
                    self.insert_id = is.read_string()?;
                },
                106 => {
                    let len = is.read_raw_varint32()?;
                    let old_limit = is.push_limit(len as u64)?;
                    let mut key = ::std::default::Default::default();
                    let mut value = ::std::default::Default::default();
                    while let Some(tag) = is.read_raw_tag_or_eof()? {
                        match tag {
                            10 => key = is.read_string()?,
                            18 => value = is.read_string()?,
                            _ => ::protobuf::rt::skip_field_for_tag(tag, is)?,
                        };
                    }
                    is.pop_limit(old_limit);
                    self.labels.insert(key, value);
                },
                18 => {
                    self.payload = ::std::option::Option::Some(log_entry::Payload::ProtoPayload(is.read_message()?));
                },
                26 => {
                    self.payload = ::std::option::Option::Some(log_entry::Payload::TextPayload(is.read_string()?));
                },
                50 => {
                    self.payload = ::std::option::Option::Some(log_entry::Payload::StructPayload(is.read_message()?));
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
            my_size += ::protobuf::rt::string_size(10, &self.name);
        }
        if let Some(v) = self.timestamp.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.severity != ::protobuf::EnumOrUnknown::new(super::log_severity::LogSeverity::DEFAULT) {
            my_size += ::protobuf::rt::int32_size(12, self.severity.value());
        }
        if !self.insert_id.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.insert_id);
        }
        for (k, v) in &self.labels {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::string_size(1, &k);
            entry_size += ::protobuf::rt::string_size(2, &v);
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(entry_size) + entry_size
        };
        if let ::std::option::Option::Some(ref v) = self.payload {
            match v {
                &log_entry::Payload::ProtoPayload(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &log_entry::Payload::TextPayload(ref v) => {
                    my_size += ::protobuf::rt::string_size(3, &v);
                },
                &log_entry::Payload::StructPayload(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.name.is_empty() {
            os.write_string(10, &self.name)?;
        }
        if let Some(v) = self.timestamp.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
        }
        if self.severity != ::protobuf::EnumOrUnknown::new(super::log_severity::LogSeverity::DEFAULT) {
            os.write_enum(12, ::protobuf::EnumOrUnknown::value(&self.severity))?;
        }
        if !self.insert_id.is_empty() {
            os.write_string(4, &self.insert_id)?;
        }
        for (k, v) in &self.labels {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::string_size(1, &k);
            entry_size += ::protobuf::rt::string_size(2, &v);
            os.write_raw_varint32(106)?; // Tag.
            os.write_raw_varint32(entry_size as u32)?;
            os.write_string(1, &k)?;
            os.write_string(2, &v)?;
        };
        if let ::std::option::Option::Some(ref v) = self.payload {
            match v {
                &log_entry::Payload::ProtoPayload(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
                },
                &log_entry::Payload::TextPayload(ref v) => {
                    os.write_string(3, v)?;
                },
                &log_entry::Payload::StructPayload(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(6, v, os)?;
                },
            };
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

    fn new() -> LogEntry {
        LogEntry::new()
    }

    fn clear(&mut self) {
        self.name.clear();
        self.timestamp.clear();
        self.severity = ::protobuf::EnumOrUnknown::new(super::log_severity::LogSeverity::DEFAULT);
        self.insert_id.clear();
        self.labels.clear();
        self.payload = ::std::option::Option::None;
        self.payload = ::std::option::Option::None;
        self.payload = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static LogEntry {
        static instance: ::protobuf::rt::Lazy<LogEntry> = ::protobuf::rt::Lazy::new();
        instance.get(LogEntry::new)
    }
}

impl ::protobuf::MessageFull for LogEntry {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("LogEntry").unwrap()).clone()
    }
}

impl ::std::fmt::Display for LogEntry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LogEntry {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `LogEntry`
pub mod log_entry {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:google.api.servicecontrol.v1.LogEntry.payload)
    pub enum Payload {
        // @@protoc_insertion_point(oneof_field:google.api.servicecontrol.v1.LogEntry.proto_payload)
        ProtoPayload(::protobuf::well_known_types::any::Any),
        // @@protoc_insertion_point(oneof_field:google.api.servicecontrol.v1.LogEntry.text_payload)
        TextPayload(::std::string::String),
        // @@protoc_insertion_point(oneof_field:google.api.servicecontrol.v1.LogEntry.struct_payload)
        StructPayload(::protobuf::well_known_types::struct_::Struct),
    }

    impl ::protobuf::Oneof for Payload {
    }

    impl ::protobuf::OneofFull for Payload {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::LogEntry as ::protobuf::MessageFull>::descriptor().oneof_by_name("payload").unwrap()).clone()
        }
    }

    impl Payload {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<Payload>("payload")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n,google/api/servicecontrol/v1/log_entry.proto\x12\x1cgoogle.api.servic\
    econtrol.v1\x1a\x1cgoogle/api/annotations.proto\x1a&google/logging/type/\
    log_severity.proto\x1a\x19google/protobuf/any.proto\x1a\x1cgoogle/protob\
    uf/struct.proto\x1a\x1fgoogle/protobuf/timestamp.proto\"\xe9\x03\n\x08Lo\
    gEntry\x12\x12\n\x04name\x18\n\x20\x01(\tR\x04name\x128\n\ttimestamp\x18\
    \x0b\x20\x01(\x0b2\x1a.google.protobuf.TimestampR\ttimestamp\x12<\n\x08s\
    everity\x18\x0c\x20\x01(\x0e2\x20.google.logging.type.LogSeverityR\x08se\
    verity\x12\x1b\n\tinsert_id\x18\x04\x20\x01(\tR\x08insertId\x12J\n\x06la\
    bels\x18\r\x20\x03(\x0b22.google.api.servicecontrol.v1.LogEntry.LabelsEn\
    tryR\x06labels\x12;\n\rproto_payload\x18\x02\x20\x01(\x0b2\x14.google.pr\
    otobuf.AnyH\0R\x0cprotoPayload\x12#\n\x0ctext_payload\x18\x03\x20\x01(\t\
    H\0R\x0btextPayload\x12@\n\x0estruct_payload\x18\x06\x20\x01(\x0b2\x17.g\
    oogle.protobuf.StructH\0R\rstructPayload\x1a9\n\x0bLabelsEntry\x12\x10\n\
    \x03key\x18\x01\x20\x01(\tR\x03key\x12\x14\n\x05value\x18\x02\x20\x01(\t\
    R\x05value:\x028\x01B\t\n\x07payloadB\x7f\n\x20com.google.api.servicecon\
    trol.v1B\rLogEntryProtoP\x01ZJgoogle.golang.org/genproto/googleapis/api/\
    servicecontrol/v1;servicecontrolJ\xee\x10\n\x06\x12\x04\x0e\0A\x01\n\xbd\
    \x04\n\x01\x0c\x12\x03\x0e\0\x122\xb2\x04\x20Copyright\x202017\x20Google\
    \x20Inc.\n\n\x20Licensed\x20under\x20the\x20Apache\x20License,\x20Versio\
    n\x202.0\x20(the\x20\"License\");\n\x20you\x20may\x20not\x20use\x20this\
    \x20file\x20except\x20in\x20compliance\x20with\x20the\x20License.\n\x20Y\
    ou\x20may\x20obtain\x20a\x20copy\x20of\x20the\x20License\x20at\n\n\x20\
    \x20\x20\x20\x20http://www.apache.org/licenses/LICENSE-2.0\n\n\x20Unless\
    \x20required\x20by\x20applicable\x20law\x20or\x20agreed\x20to\x20in\x20w\
    riting,\x20software\n\x20distributed\x20under\x20the\x20License\x20is\
    \x20distributed\x20on\x20an\x20\"AS\x20IS\"\x20BASIS,\n\x20WITHOUT\x20WA\
    RRANTIES\x20OR\x20CONDITIONS\x20OF\x20ANY\x20KIND,\x20either\x20express\
    \x20or\x20implied.\n\x20See\x20the\x20License\x20for\x20the\x20specific\
    \x20language\x20governing\x20permissions\x20and\n\x20limitations\x20unde\
    r\x20the\x20License.\n\n\x08\n\x01\x02\x12\x03\x10\0%\n\t\n\x02\x03\0\
    \x12\x03\x12\0&\n\t\n\x02\x03\x01\x12\x03\x13\00\n\t\n\x02\x03\x02\x12\
    \x03\x14\0#\n\t\n\x02\x03\x03\x12\x03\x15\0&\n\t\n\x02\x03\x04\x12\x03\
    \x16\0)\n\x08\n\x01\x08\x12\x03\x18\0a\n\t\n\x02\x08\x0b\x12\x03\x18\0a\
    \n\x08\n\x01\x08\x12\x03\x19\0\"\n\t\n\x02\x08\n\x12\x03\x19\0\"\n\x08\n\
    \x01\x08\x12\x03\x1a\0.\n\t\n\x02\x08\x08\x12\x03\x1a\0.\n\x08\n\x01\x08\
    \x12\x03\x1b\09\n\t\n\x02\x08\x01\x12\x03\x1b\09\n&\n\x02\x04\0\x12\x04\
    \x1e\0A\x01\x1a\x1a\x20An\x20individual\x20log\x20entry.\n\n\n\n\x03\x04\
    \0\x01\x12\x03\x1e\x08\x10\nf\n\x04\x04\0\x02\0\x12\x03!\x02\x13\x1aY\
    \x20Required.\x20The\x20log\x20to\x20which\x20this\x20log\x20entry\x20be\
    longs.\x20Examples:\x20`\"syslog\"`,\n\x20`\"book_log\"`.\n\n\x0c\n\x05\
    \x04\0\x02\0\x05\x12\x03!\x02\x08\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03!\t\
    \r\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03!\x10\x12\nu\n\x04\x04\0\x02\x01\
    \x12\x03%\x02+\x1ah\x20The\x20time\x20the\x20event\x20described\x20by\
    \x20the\x20log\x20entry\x20occurred.\x20If\n\x20omitted,\x20defaults\x20\
    to\x20operation\x20start\x20time.\n\n\x0c\n\x05\x04\0\x02\x01\x06\x12\
    \x03%\x02\x1b\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03%\x1c%\n\x0c\n\x05\
    \x04\0\x02\x01\x03\x12\x03%(*\nZ\n\x04\x04\0\x02\x02\x12\x03)\x020\x1aM\
    \x20The\x20severity\x20of\x20the\x20log\x20entry.\x20The\x20default\x20v\
    alue\x20is\n\x20`LogSeverity.DEFAULT`.\n\n\x0c\n\x05\x04\0\x02\x02\x06\
    \x12\x03)\x02!\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03)\"*\n\x0c\n\x05\x04\
    \0\x02\x02\x03\x12\x03)-/\n\x8e\x01\n\x04\x04\0\x02\x03\x12\x03-\x02\x17\
    \x1a\x80\x01\x20A\x20unique\x20ID\x20for\x20the\x20log\x20entry\x20used\
    \x20for\x20deduplication.\x20If\x20omitted,\n\x20the\x20implementation\
    \x20will\x20generate\x20one\x20based\x20on\x20operation_id.\n\n\x0c\n\
    \x05\x04\0\x02\x03\x05\x12\x03-\x02\x08\n\x0c\n\x05\x04\0\x02\x03\x01\
    \x12\x03-\t\x12\n\x0c\n\x05\x04\0\x02\x03\x03\x12\x03-\x15\x16\nq\n\x04\
    \x04\0\x02\x04\x12\x031\x02\"\x1ad\x20A\x20set\x20of\x20user-defined\x20\
    (key,\x20value)\x20data\x20that\x20provides\x20additional\n\x20informati\
    on\x20about\x20the\x20log\x20entry.\n\n\x0c\n\x05\x04\0\x02\x04\x06\x12\
    \x031\x02\x15\n\x0c\n\x05\x04\0\x02\x04\x01\x12\x031\x16\x1c\n\x0c\n\x05\
    \x04\0\x02\x04\x03\x12\x031\x1f!\nJ\n\x04\x04\0\x08\0\x12\x044\x02@\x03\
    \x1a<\x20The\x20log\x20entry\x20payload,\x20which\x20can\x20be\x20one\
    \x20of\x20multiple\x20types.\n\n\x0c\n\x05\x04\0\x08\0\x01\x12\x034\x08\
    \x0f\n\xba\x01\n\x04\x04\0\x02\x05\x12\x038\x04*\x1a\xac\x01\x20The\x20l\
    og\x20entry\x20payload,\x20represented\x20as\x20a\x20protocol\x20buffer\
    \x20that\x20is\n\x20expressed\x20as\x20a\x20JSON\x20object.\x20The\x20on\
    ly\x20accepted\x20type\x20currently\x20is\n\x20[AuditLog][google.cloud.a\
    udit.AuditLog].\n\n\x0c\n\x05\x04\0\x02\x05\x06\x12\x038\x04\x17\n\x0c\n\
    \x05\x04\0\x02\x05\x01\x12\x038\x18%\n\x0c\n\x05\x04\0\x02\x05\x03\x12\
    \x038()\nN\n\x04\x04\0\x02\x06\x12\x03;\x04\x1c\x1aA\x20The\x20log\x20en\
    try\x20payload,\x20represented\x20as\x20a\x20Unicode\x20string\x20(UTF-8\
    ).\n\n\x0c\n\x05\x04\0\x02\x06\x05\x12\x03;\x04\n\n\x0c\n\x05\x04\0\x02\
    \x06\x01\x12\x03;\x0b\x17\n\x0c\n\x05\x04\0\x02\x06\x03\x12\x03;\x1a\x1b\
    \ne\n\x04\x04\0\x02\x07\x12\x03?\x04.\x1aX\x20The\x20log\x20entry\x20pay\
    load,\x20represented\x20as\x20a\x20structure\x20that\n\x20is\x20expresse\
    d\x20as\x20a\x20JSON\x20object.\n\n\x0c\n\x05\x04\0\x02\x07\x06\x12\x03?\
    \x04\x1a\n\x0c\n\x05\x04\0\x02\x07\x01\x12\x03?\x1b)\n\x0c\n\x05\x04\0\
    \x02\x07\x03\x12\x03?,-b\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(5);
            deps.push(super::annotations::file_descriptor().clone());
            deps.push(super::log_severity::file_descriptor().clone());
            deps.push(::protobuf::well_known_types::any::file_descriptor().clone());
            deps.push(::protobuf::well_known_types::struct_::file_descriptor().clone());
            deps.push(::protobuf::well_known_types::timestamp::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(LogEntry::generated_message_descriptor_data());
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
