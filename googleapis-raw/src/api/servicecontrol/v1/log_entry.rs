// This file is generated by rust-protobuf 2.27.1. Do not edit
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
//! Generated file from `google/api/servicecontrol/v1/log_entry.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_27_1;

#[derive(PartialEq,Clone,Default)]
pub struct LogEntry {
    // message fields
    pub name: ::std::string::String,
    pub timestamp: ::protobuf::SingularPtrField<::protobuf::well_known_types::Timestamp>,
    pub severity: super::log_severity::LogSeverity,
    pub insert_id: ::std::string::String,
    pub labels: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    // message oneof groups
    pub payload: ::std::option::Option<LogEntry_oneof_payload>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a LogEntry {
    fn default() -> &'a LogEntry {
        <LogEntry as ::protobuf::Message>::default_instance()
    }
}

#[derive(Clone,PartialEq,Debug)]
pub enum LogEntry_oneof_payload {
    proto_payload(::protobuf::well_known_types::Any),
    text_payload(::std::string::String),
    struct_payload(::protobuf::well_known_types::Struct),
}

impl LogEntry {
    pub fn new() -> LogEntry {
        ::std::default::Default::default()
    }

    // string name = 10;


    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    // .google.protobuf.Timestamp timestamp = 11;


    pub fn get_timestamp(&self) -> &::protobuf::well_known_types::Timestamp {
        self.timestamp.as_ref().unwrap_or_else(|| <::protobuf::well_known_types::Timestamp as ::protobuf::Message>::default_instance())
    }
    pub fn clear_timestamp(&mut self) {
        self.timestamp.clear();
    }

    pub fn has_timestamp(&self) -> bool {
        self.timestamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_timestamp(&mut self, v: ::protobuf::well_known_types::Timestamp) {
        self.timestamp = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_timestamp(&mut self) -> &mut ::protobuf::well_known_types::Timestamp {
        if self.timestamp.is_none() {
            self.timestamp.set_default();
        }
        self.timestamp.as_mut().unwrap()
    }

    // Take field
    pub fn take_timestamp(&mut self) -> ::protobuf::well_known_types::Timestamp {
        self.timestamp.take().unwrap_or_else(|| ::protobuf::well_known_types::Timestamp::new())
    }

    // .google.logging.type.LogSeverity severity = 12;


    pub fn get_severity(&self) -> super::log_severity::LogSeverity {
        self.severity
    }
    pub fn clear_severity(&mut self) {
        self.severity = super::log_severity::LogSeverity::DEFAULT;
    }

    // Param is passed by value, moved
    pub fn set_severity(&mut self, v: super::log_severity::LogSeverity) {
        self.severity = v;
    }

    // string insert_id = 4;


    pub fn get_insert_id(&self) -> &str {
        &self.insert_id
    }
    pub fn clear_insert_id(&mut self) {
        self.insert_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_insert_id(&mut self, v: ::std::string::String) {
        self.insert_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_insert_id(&mut self) -> &mut ::std::string::String {
        &mut self.insert_id
    }

    // Take field
    pub fn take_insert_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.insert_id, ::std::string::String::new())
    }

    // repeated .google.api.servicecontrol.v1.LogEntry.LabelsEntry labels = 13;


    pub fn get_labels(&self) -> &::std::collections::HashMap<::std::string::String, ::std::string::String> {
        &self.labels
    }
    pub fn clear_labels(&mut self) {
        self.labels.clear();
    }

    // Param is passed by value, moved
    pub fn set_labels(&mut self, v: ::std::collections::HashMap<::std::string::String, ::std::string::String>) {
        self.labels = v;
    }

    // Mutable pointer to the field.
    pub fn mut_labels(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, ::std::string::String> {
        &mut self.labels
    }

    // Take field
    pub fn take_labels(&mut self) -> ::std::collections::HashMap<::std::string::String, ::std::string::String> {
        ::std::mem::replace(&mut self.labels, ::std::collections::HashMap::new())
    }

    // .google.protobuf.Any proto_payload = 2;


    pub fn get_proto_payload(&self) -> &::protobuf::well_known_types::Any {
        match self.payload {
            ::std::option::Option::Some(LogEntry_oneof_payload::proto_payload(ref v)) => v,
            _ => <::protobuf::well_known_types::Any as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn clear_proto_payload(&mut self) {
        self.payload = ::std::option::Option::None;
    }

    pub fn has_proto_payload(&self) -> bool {
        match self.payload {
            ::std::option::Option::Some(LogEntry_oneof_payload::proto_payload(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_proto_payload(&mut self, v: ::protobuf::well_known_types::Any) {
        self.payload = ::std::option::Option::Some(LogEntry_oneof_payload::proto_payload(v))
    }

    // Mutable pointer to the field.
    pub fn mut_proto_payload(&mut self) -> &mut ::protobuf::well_known_types::Any {
        if let ::std::option::Option::Some(LogEntry_oneof_payload::proto_payload(_)) = self.payload {
        } else {
            self.payload = ::std::option::Option::Some(LogEntry_oneof_payload::proto_payload(::protobuf::well_known_types::Any::new()));
        }
        match self.payload {
            ::std::option::Option::Some(LogEntry_oneof_payload::proto_payload(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_proto_payload(&mut self) -> ::protobuf::well_known_types::Any {
        if self.has_proto_payload() {
            match self.payload.take() {
                ::std::option::Option::Some(LogEntry_oneof_payload::proto_payload(v)) => v,
                _ => panic!(),
            }
        } else {
            ::protobuf::well_known_types::Any::new()
        }
    }

    // string text_payload = 3;


    pub fn get_text_payload(&self) -> &str {
        match self.payload {
            ::std::option::Option::Some(LogEntry_oneof_payload::text_payload(ref v)) => v,
            _ => "",
        }
    }
    pub fn clear_text_payload(&mut self) {
        self.payload = ::std::option::Option::None;
    }

    pub fn has_text_payload(&self) -> bool {
        match self.payload {
            ::std::option::Option::Some(LogEntry_oneof_payload::text_payload(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_text_payload(&mut self, v: ::std::string::String) {
        self.payload = ::std::option::Option::Some(LogEntry_oneof_payload::text_payload(v))
    }

    // Mutable pointer to the field.
    pub fn mut_text_payload(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(LogEntry_oneof_payload::text_payload(_)) = self.payload {
        } else {
            self.payload = ::std::option::Option::Some(LogEntry_oneof_payload::text_payload(::std::string::String::new()));
        }
        match self.payload {
            ::std::option::Option::Some(LogEntry_oneof_payload::text_payload(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_text_payload(&mut self) -> ::std::string::String {
        if self.has_text_payload() {
            match self.payload.take() {
                ::std::option::Option::Some(LogEntry_oneof_payload::text_payload(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    // .google.protobuf.Struct struct_payload = 6;


    pub fn get_struct_payload(&self) -> &::protobuf::well_known_types::Struct {
        match self.payload {
            ::std::option::Option::Some(LogEntry_oneof_payload::struct_payload(ref v)) => v,
            _ => <::protobuf::well_known_types::Struct as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn clear_struct_payload(&mut self) {
        self.payload = ::std::option::Option::None;
    }

    pub fn has_struct_payload(&self) -> bool {
        match self.payload {
            ::std::option::Option::Some(LogEntry_oneof_payload::struct_payload(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_struct_payload(&mut self, v: ::protobuf::well_known_types::Struct) {
        self.payload = ::std::option::Option::Some(LogEntry_oneof_payload::struct_payload(v))
    }

    // Mutable pointer to the field.
    pub fn mut_struct_payload(&mut self) -> &mut ::protobuf::well_known_types::Struct {
        if let ::std::option::Option::Some(LogEntry_oneof_payload::struct_payload(_)) = self.payload {
        } else {
            self.payload = ::std::option::Option::Some(LogEntry_oneof_payload::struct_payload(::protobuf::well_known_types::Struct::new()));
        }
        match self.payload {
            ::std::option::Option::Some(LogEntry_oneof_payload::struct_payload(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_struct_payload(&mut self) -> ::protobuf::well_known_types::Struct {
        if self.has_struct_payload() {
            match self.payload.take() {
                ::std::option::Option::Some(LogEntry_oneof_payload::struct_payload(v)) => v,
                _ => panic!(),
            }
        } else {
            ::protobuf::well_known_types::Struct::new()
        }
    }
}

impl ::protobuf::Message for LogEntry {
    fn is_initialized(&self) -> bool {
        for v in &self.timestamp {
            if !v.is_initialized() {
                return false;
            }
        };
        if let Some(LogEntry_oneof_payload::proto_payload(ref v)) = self.payload {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(LogEntry_oneof_payload::struct_payload(ref v)) = self.payload {
            if !v.is_initialized() {
                return false;
            }
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                10 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                11 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.timestamp)?;
                },
                12 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.severity, 12, &mut self.unknown_fields)?
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.insert_id)?;
                },
                13 => {
                    ::protobuf::rt::read_map_into::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(wire_type, is, &mut self.labels)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.payload = ::std::option::Option::Some(LogEntry_oneof_payload::proto_payload(is.read_message()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.payload = ::std::option::Option::Some(LogEntry_oneof_payload::text_payload(is.read_string()?));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.payload = ::std::option::Option::Some(LogEntry_oneof_payload::struct_payload(is.read_message()?));
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
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(10, &self.name);
        }
        if let Some(ref v) = self.timestamp.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.severity != super::log_severity::LogSeverity::DEFAULT {
            my_size += ::protobuf::rt::enum_size(12, self.severity);
        }
        if !self.insert_id.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.insert_id);
        }
        my_size += ::protobuf::rt::compute_map_size::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(13, &self.labels);
        if let ::std::option::Option::Some(ref v) = self.payload {
            match v {
                &LogEntry_oneof_payload::proto_payload(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &LogEntry_oneof_payload::text_payload(ref v) => {
                    my_size += ::protobuf::rt::string_size(3, &v);
                },
                &LogEntry_oneof_payload::struct_payload(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(10, &self.name)?;
        }
        if let Some(ref v) = self.timestamp.as_ref() {
            os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.severity != super::log_severity::LogSeverity::DEFAULT {
            os.write_enum(12, ::protobuf::ProtobufEnum::value(&self.severity))?;
        }
        if !self.insert_id.is_empty() {
            os.write_string(4, &self.insert_id)?;
        }
        ::protobuf::rt::write_map_with_cached_sizes::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(13, &self.labels, os)?;
        if let ::std::option::Option::Some(ref v) = self.payload {
            match v {
                &LogEntry_oneof_payload::proto_payload(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &LogEntry_oneof_payload::text_payload(ref v) => {
                    os.write_string(3, v)?;
                },
                &LogEntry_oneof_payload::struct_payload(ref v) => {
                    os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
            };
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

    fn new() -> LogEntry {
        LogEntry::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "name",
                |m: &LogEntry| { &m.name },
                |m: &mut LogEntry| { &mut m.name },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Timestamp>>(
                "timestamp",
                |m: &LogEntry| { &m.timestamp },
                |m: &mut LogEntry| { &mut m.timestamp },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::log_severity::LogSeverity>>(
                "severity",
                |m: &LogEntry| { &m.severity },
                |m: &mut LogEntry| { &mut m.severity },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "insert_id",
                |m: &LogEntry| { &m.insert_id },
                |m: &mut LogEntry| { &mut m.insert_id },
            ));
            fields.push(::protobuf::reflect::accessor::make_map_accessor::<_, ::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(
                "labels",
                |m: &LogEntry| { &m.labels },
                |m: &mut LogEntry| { &mut m.labels },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ::protobuf::well_known_types::Any>(
                "proto_payload",
                LogEntry::has_proto_payload,
                LogEntry::get_proto_payload,
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                "text_payload",
                LogEntry::has_text_payload,
                LogEntry::get_text_payload,
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ::protobuf::well_known_types::Struct>(
                "struct_payload",
                LogEntry::has_struct_payload,
                LogEntry::get_struct_payload,
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<LogEntry>(
                "LogEntry",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static LogEntry {
        static instance: ::protobuf::rt::LazyV2<LogEntry> = ::protobuf::rt::LazyV2::INIT;
        instance.get(LogEntry::new)
    }
}

impl ::protobuf::Clear for LogEntry {
    fn clear(&mut self) {
        self.name.clear();
        self.timestamp.clear();
        self.severity = super::log_severity::LogSeverity::DEFAULT;
        self.insert_id.clear();
        self.labels.clear();
        self.payload = ::std::option::Option::None;
        self.payload = ::std::option::Option::None;
        self.payload = ::std::option::Option::None;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LogEntry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LogEntry {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
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

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
