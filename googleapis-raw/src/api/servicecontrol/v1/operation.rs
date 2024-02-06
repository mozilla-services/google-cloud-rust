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

//! Generated file from `google/api/servicecontrol/v1/operation.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_3_0;

///  Represents information regarding an operation.
// @@protoc_insertion_point(message:google.api.servicecontrol.v1.Operation)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct Operation {
    // message fields
    ///  Identity of the operation. This must be unique within the scope of the
    ///  service that generated the operation. If the service calls
    ///  Check() and Report() on the same operation, the two calls should carry
    ///  the same id.
    ///
    ///  UUID version 4 is recommended, though not required.
    ///  In scenarios where an operation is computed from existing information
    ///  and an idempotent id is desirable for deduplication purpose, UUID version 5
    ///  is recommended. See RFC 4122 for details.
    // @@protoc_insertion_point(field:google.api.servicecontrol.v1.Operation.operation_id)
    pub operation_id: ::std::string::String,
    ///  Fully qualified name of the operation. Reserved for future use.
    // @@protoc_insertion_point(field:google.api.servicecontrol.v1.Operation.operation_name)
    pub operation_name: ::std::string::String,
    ///  Identity of the consumer who is using the service.
    ///  This field should be filled in for the operations initiated by a
    ///  consumer, but not for service-initiated operations that are
    ///  not related to a specific consumer.
    ///
    ///  This can be in one of the following formats:
    ///    project:<project_id>,
    ///    project_number:<project_number>,
    ///    api_key:<api_key>.
    // @@protoc_insertion_point(field:google.api.servicecontrol.v1.Operation.consumer_id)
    pub consumer_id: ::std::string::String,
    ///  Required. Start time of the operation.
    // @@protoc_insertion_point(field:google.api.servicecontrol.v1.Operation.start_time)
    pub start_time: ::protobuf::MessageField<::protobuf::well_known_types::timestamp::Timestamp>,
    ///  End time of the operation.
    ///  Required when the operation is used in
    ///  [ServiceController.Report][google.api.servicecontrol.v1.ServiceController.Report],
    ///  but optional when the operation is used in
    ///  [ServiceController.Check][google.api.servicecontrol.v1.ServiceController.Check].
    // @@protoc_insertion_point(field:google.api.servicecontrol.v1.Operation.end_time)
    pub end_time: ::protobuf::MessageField<::protobuf::well_known_types::timestamp::Timestamp>,
    // @@protoc_insertion_point(field:google.api.servicecontrol.v1.Operation.labels)
    pub labels: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    ///  Represents information about this operation. Each MetricValueSet
    ///  corresponds to a metric defined in the service configuration.
    ///  The data type used in the MetricValueSet must agree with
    ///  the data type specified in the metric definition.
    ///
    ///  Within a single operation, it is not allowed to have more than one
    ///  MetricValue instances that have the same metric names and identical
    ///  label value combinations. If a request has such duplicated MetricValue
    ///  instances, the entire request is rejected with
    ///  an invalid argument error.
    // @@protoc_insertion_point(field:google.api.servicecontrol.v1.Operation.metric_value_sets)
    pub metric_value_sets: ::std::vec::Vec<super::metric_value::MetricValueSet>,
    ///  Represents information to be logged.
    // @@protoc_insertion_point(field:google.api.servicecontrol.v1.Operation.log_entries)
    pub log_entries: ::std::vec::Vec<super::log_entry::LogEntry>,
    ///  DO NOT USE. This is an experimental field.
    // @@protoc_insertion_point(field:google.api.servicecontrol.v1.Operation.importance)
    pub importance: ::protobuf::EnumOrUnknown<operation::Importance>,
    // special fields
    // @@protoc_insertion_point(special_field:google.api.servicecontrol.v1.Operation.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Operation {
    fn default() -> &'a Operation {
        <Operation as ::protobuf::Message>::default_instance()
    }
}

impl Operation {
    pub fn new() -> Operation {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(9);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "operation_id",
            |m: &Operation| { &m.operation_id },
            |m: &mut Operation| { &mut m.operation_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "operation_name",
            |m: &Operation| { &m.operation_name },
            |m: &mut Operation| { &mut m.operation_name },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "consumer_id",
            |m: &Operation| { &m.consumer_id },
            |m: &mut Operation| { &mut m.consumer_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, ::protobuf::well_known_types::timestamp::Timestamp>(
            "start_time",
            |m: &Operation| { &m.start_time },
            |m: &mut Operation| { &mut m.start_time },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, ::protobuf::well_known_types::timestamp::Timestamp>(
            "end_time",
            |m: &Operation| { &m.end_time },
            |m: &mut Operation| { &mut m.end_time },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_map_simpler_accessor::<_, _, _>(
            "labels",
            |m: &Operation| { &m.labels },
            |m: &mut Operation| { &mut m.labels },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "metric_value_sets",
            |m: &Operation| { &m.metric_value_sets },
            |m: &mut Operation| { &mut m.metric_value_sets },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "log_entries",
            |m: &Operation| { &m.log_entries },
            |m: &mut Operation| { &mut m.log_entries },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "importance",
            |m: &Operation| { &m.importance },
            |m: &mut Operation| { &mut m.importance },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Operation>(
            "Operation",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Operation {
    const NAME: &'static str = "Operation";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.operation_id = is.read_string()?;
                },
                18 => {
                    self.operation_name = is.read_string()?;
                },
                26 => {
                    self.consumer_id = is.read_string()?;
                },
                34 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.start_time)?;
                },
                42 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.end_time)?;
                },
                50 => {
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
                58 => {
                    self.metric_value_sets.push(is.read_message()?);
                },
                66 => {
                    self.log_entries.push(is.read_message()?);
                },
                88 => {
                    self.importance = is.read_enum_or_unknown()?;
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
        if !self.operation_id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.operation_id);
        }
        if !self.operation_name.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.operation_name);
        }
        if !self.consumer_id.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.consumer_id);
        }
        if let Some(v) = self.start_time.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.end_time.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        for (k, v) in &self.labels {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::string_size(1, &k);
            entry_size += ::protobuf::rt::string_size(2, &v);
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(entry_size) + entry_size
        };
        for value in &self.metric_value_sets {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.log_entries {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.importance != ::protobuf::EnumOrUnknown::new(operation::Importance::LOW) {
            my_size += ::protobuf::rt::int32_size(11, self.importance.value());
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.operation_id.is_empty() {
            os.write_string(1, &self.operation_id)?;
        }
        if !self.operation_name.is_empty() {
            os.write_string(2, &self.operation_name)?;
        }
        if !self.consumer_id.is_empty() {
            os.write_string(3, &self.consumer_id)?;
        }
        if let Some(v) = self.start_time.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        }
        if let Some(v) = self.end_time.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
        }
        for (k, v) in &self.labels {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::string_size(1, &k);
            entry_size += ::protobuf::rt::string_size(2, &v);
            os.write_raw_varint32(50)?; // Tag.
            os.write_raw_varint32(entry_size as u32)?;
            os.write_string(1, &k)?;
            os.write_string(2, &v)?;
        };
        for v in &self.metric_value_sets {
            ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
        };
        for v in &self.log_entries {
            ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
        };
        if self.importance != ::protobuf::EnumOrUnknown::new(operation::Importance::LOW) {
            os.write_enum(11, ::protobuf::EnumOrUnknown::value(&self.importance))?;
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

    fn new() -> Operation {
        Operation::new()
    }

    fn clear(&mut self) {
        self.operation_id.clear();
        self.operation_name.clear();
        self.consumer_id.clear();
        self.start_time.clear();
        self.end_time.clear();
        self.labels.clear();
        self.metric_value_sets.clear();
        self.log_entries.clear();
        self.importance = ::protobuf::EnumOrUnknown::new(operation::Importance::LOW);
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Operation {
        static instance: ::protobuf::rt::Lazy<Operation> = ::protobuf::rt::Lazy::new();
        instance.get(Operation::new)
    }
}

impl ::protobuf::MessageFull for Operation {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Operation").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Operation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Operation {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `Operation`
pub mod operation {
    ///  Defines the importance of the data contained in the operation.
    #[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
    // @@protoc_insertion_point(enum:google.api.servicecontrol.v1.Operation.Importance)
    pub enum Importance {
        // @@protoc_insertion_point(enum_value:google.api.servicecontrol.v1.Operation.Importance.LOW)
        LOW = 0,
        // @@protoc_insertion_point(enum_value:google.api.servicecontrol.v1.Operation.Importance.HIGH)
        HIGH = 1,
    }

    impl ::protobuf::Enum for Importance {
        const NAME: &'static str = "Importance";

        fn value(&self) -> i32 {
            *self as i32
        }

        fn from_i32(value: i32) -> ::std::option::Option<Importance> {
            match value {
                0 => ::std::option::Option::Some(Importance::LOW),
                1 => ::std::option::Option::Some(Importance::HIGH),
                _ => ::std::option::Option::None
            }
        }

        fn from_str(str: &str) -> ::std::option::Option<Importance> {
            match str {
                "LOW" => ::std::option::Option::Some(Importance::LOW),
                "HIGH" => ::std::option::Option::Some(Importance::HIGH),
                _ => ::std::option::Option::None
            }
        }

        const VALUES: &'static [Importance] = &[
            Importance::LOW,
            Importance::HIGH,
        ];
    }

    impl ::protobuf::EnumFull for Importance {
        fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| super::file_descriptor().enum_by_package_relative_name("Operation.Importance").unwrap()).clone()
        }

        fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
            let index = *self as usize;
            Self::enum_descriptor().value_by_index(index)
        }
    }

    impl ::std::default::Default for Importance {
        fn default() -> Self {
            Importance::LOW
        }
    }

    impl Importance {
        pub(in super) fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
            ::protobuf::reflect::GeneratedEnumDescriptorData::new::<Importance>("Operation.Importance")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n,google/api/servicecontrol/v1/operation.proto\x12\x1cgoogle.api.servic\
    econtrol.v1\x1a\x1cgoogle/api/annotations.proto\x1a,google/api/serviceco\
    ntrol/v1/log_entry.proto\x1a/google/api/servicecontrol/v1/metric_value.p\
    roto\x1a\x1fgoogle/protobuf/timestamp.proto\"\x88\x05\n\tOperation\x12!\
    \n\x0coperation_id\x18\x01\x20\x01(\tR\x0boperationId\x12%\n\x0eoperatio\
    n_name\x18\x02\x20\x01(\tR\roperationName\x12\x1f\n\x0bconsumer_id\x18\
    \x03\x20\x01(\tR\nconsumerId\x129\n\nstart_time\x18\x04\x20\x01(\x0b2\
    \x1a.google.protobuf.TimestampR\tstartTime\x125\n\x08end_time\x18\x05\
    \x20\x01(\x0b2\x1a.google.protobuf.TimestampR\x07endTime\x12K\n\x06label\
    s\x18\x06\x20\x03(\x0b23.google.api.servicecontrol.v1.Operation.LabelsEn\
    tryR\x06labels\x12X\n\x11metric_value_sets\x18\x07\x20\x03(\x0b2,.google\
    .api.servicecontrol.v1.MetricValueSetR\x0fmetricValueSets\x12G\n\x0blog_\
    entries\x18\x08\x20\x03(\x0b2&.google.api.servicecontrol.v1.LogEntryR\nl\
    ogEntries\x12R\n\nimportance\x18\x0b\x20\x01(\x0e22.google.api.serviceco\
    ntrol.v1.Operation.ImportanceR\nimportance\x1a9\n\x0bLabelsEntry\x12\x10\
    \n\x03key\x18\x01\x20\x01(\tR\x03key\x12\x14\n\x05value\x18\x02\x20\x01(\
    \tR\x05value:\x028\x01\"\x1f\n\nImportance\x12\x07\n\x03LOW\x10\0\x12\
    \x08\n\x04HIGH\x10\x01B\x83\x01\n\x20com.google.api.servicecontrol.v1B\
    \x0eOperationProtoP\x01ZJgoogle.golang.org/genproto/googleapis/api/servi\
    cecontrol/v1;servicecontrol\xf8\x01\x01J\xe0\"\n\x06\x12\x04\x0e\0p\x01\
    \n\xbd\x04\n\x01\x0c\x12\x03\x0e\0\x122\xb2\x04\x20Copyright\x202017\x20\
    Google\x20Inc.\n\n\x20Licensed\x20under\x20the\x20Apache\x20License,\x20\
    Version\x202.0\x20(the\x20\"License\");\n\x20you\x20may\x20not\x20use\
    \x20this\x20file\x20except\x20in\x20compliance\x20with\x20the\x20License\
    .\n\x20You\x20may\x20obtain\x20a\x20copy\x20of\x20the\x20License\x20at\n\
    \n\x20\x20\x20\x20\x20http://www.apache.org/licenses/LICENSE-2.0\n\n\x20\
    Unless\x20required\x20by\x20applicable\x20law\x20or\x20agreed\x20to\x20i\
    n\x20writing,\x20software\n\x20distributed\x20under\x20the\x20License\
    \x20is\x20distributed\x20on\x20an\x20\"AS\x20IS\"\x20BASIS,\n\x20WITHOUT\
    \x20WARRANTIES\x20OR\x20CONDITIONS\x20OF\x20ANY\x20KIND,\x20either\x20ex\
    press\x20or\x20implied.\n\x20See\x20the\x20License\x20for\x20the\x20spec\
    ific\x20language\x20governing\x20permissions\x20and\n\x20limitations\x20\
    under\x20the\x20License.\n\n\x08\n\x01\x02\x12\x03\x10\0%\n\t\n\x02\x03\
    \0\x12\x03\x12\0&\n\t\n\x02\x03\x01\x12\x03\x13\06\n\t\n\x02\x03\x02\x12\
    \x03\x14\09\n\t\n\x02\x03\x03\x12\x03\x15\0)\n\x08\n\x01\x08\x12\x03\x17\
    \0\x1f\n\t\n\x02\x08\x1f\x12\x03\x17\0\x1f\n\x08\n\x01\x08\x12\x03\x18\0\
    a\n\t\n\x02\x08\x0b\x12\x03\x18\0a\n\x08\n\x01\x08\x12\x03\x19\0\"\n\t\n\
    \x02\x08\n\x12\x03\x19\0\"\n\x08\n\x01\x08\x12\x03\x1a\0/\n\t\n\x02\x08\
    \x08\x12\x03\x1a\0/\n\x08\n\x01\x08\x12\x03\x1b\09\n\t\n\x02\x08\x01\x12\
    \x03\x1b\09\n<\n\x02\x04\0\x12\x04\x1e\0p\x01\x1a0\x20Represents\x20info\
    rmation\x20regarding\x20an\x20operation.\n\n\n\n\x03\x04\0\x01\x12\x03\
    \x1e\x08\x11\nN\n\x04\x04\0\x04\0\x12\x04\x20\x02)\x03\x1a@\x20Defines\
    \x20the\x20importance\x20of\x20the\x20data\x20contained\x20in\x20the\x20\
    operation.\n\n\x0c\n\x05\x04\0\x04\0\x01\x12\x03\x20\x07\x11\n\x90\x01\n\
    \x06\x04\0\x04\0\x02\0\x12\x03#\x04\x0c\x1a\x80\x01\x20The\x20API\x20imp\
    lementation\x20may\x20cache\x20and\x20aggregate\x20the\x20data.\n\x20The\
    \x20data\x20may\x20be\x20lost\x20when\x20rare\x20and\x20unexpected\x20sy\
    stem\x20failures\x20occur.\n\n\x0e\n\x07\x04\0\x04\0\x02\0\x01\x12\x03#\
    \x04\x07\n\x0e\n\x07\x04\0\x04\0\x02\0\x02\x12\x03#\n\x0b\n\xb9\x01\n\
    \x06\x04\0\x04\0\x02\x01\x12\x03(\x04\r\x1a\xa9\x01\x20The\x20API\x20imp\
    lementation\x20doesn't\x20cache\x20and\x20aggregate\x20the\x20data.\n\
    \x20If\x20the\x20method\x20returns\x20successfully,\x20it's\x20guarantee\
    d\x20that\x20the\x20data\x20has\n\x20been\x20persisted\x20in\x20durable\
    \x20storage.\n\n\x0e\n\x07\x04\0\x04\0\x02\x01\x01\x12\x03(\x04\x08\n\
    \x0e\n\x07\x04\0\x04\0\x02\x01\x02\x12\x03(\x0b\x0c\n\xdd\x03\n\x04\x04\
    \0\x02\0\x12\x034\x02\x1a\x1a\xcf\x03\x20Identity\x20of\x20the\x20operat\
    ion.\x20This\x20must\x20be\x20unique\x20within\x20the\x20scope\x20of\x20\
    the\n\x20service\x20that\x20generated\x20the\x20operation.\x20If\x20the\
    \x20service\x20calls\n\x20Check()\x20and\x20Report()\x20on\x20the\x20sam\
    e\x20operation,\x20the\x20two\x20calls\x20should\x20carry\n\x20the\x20sa\
    me\x20id.\n\n\x20UUID\x20version\x204\x20is\x20recommended,\x20though\
    \x20not\x20required.\n\x20In\x20scenarios\x20where\x20an\x20operation\
    \x20is\x20computed\x20from\x20existing\x20information\n\x20and\x20an\x20\
    idempotent\x20id\x20is\x20desirable\x20for\x20deduplication\x20purpose,\
    \x20UUID\x20version\x205\n\x20is\x20recommended.\x20See\x20RFC\x204122\
    \x20for\x20details.\n\n\x0c\n\x05\x04\0\x02\0\x05\x12\x034\x02\x08\n\x0c\
    \n\x05\x04\0\x02\0\x01\x12\x034\t\x15\n\x0c\n\x05\x04\0\x02\0\x03\x12\
    \x034\x18\x19\nN\n\x04\x04\0\x02\x01\x12\x037\x02\x1c\x1aA\x20Fully\x20q\
    ualified\x20name\x20of\x20the\x20operation.\x20Reserved\x20for\x20future\
    \x20use.\n\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x037\x02\x08\n\x0c\n\x05\
    \x04\0\x02\x01\x01\x12\x037\t\x17\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x037\
    \x1a\x1b\n\xe8\x02\n\x04\x04\0\x02\x02\x12\x03B\x02\x19\x1a\xda\x02\x20I\
    dentity\x20of\x20the\x20consumer\x20who\x20is\x20using\x20the\x20service\
    .\n\x20This\x20field\x20should\x20be\x20filled\x20in\x20for\x20the\x20op\
    erations\x20initiated\x20by\x20a\n\x20consumer,\x20but\x20not\x20for\x20\
    service-initiated\x20operations\x20that\x20are\n\x20not\x20related\x20to\
    \x20a\x20specific\x20consumer.\n\n\x20This\x20can\x20be\x20in\x20one\x20\
    of\x20the\x20following\x20formats:\n\x20\x20\x20project:<project_id>,\n\
    \x20\x20\x20project_number:<project_number>,\n\x20\x20\x20api_key:<api_k\
    ey>.\n\n\x0c\n\x05\x04\0\x02\x02\x05\x12\x03B\x02\x08\n\x0c\n\x05\x04\0\
    \x02\x02\x01\x12\x03B\t\x14\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03B\x17\
    \x18\n5\n\x04\x04\0\x02\x03\x12\x03E\x02+\x1a(\x20Required.\x20Start\x20\
    time\x20of\x20the\x20operation.\n\n\x0c\n\x05\x04\0\x02\x03\x06\x12\x03E\
    \x02\x1b\n\x0c\n\x05\x04\0\x02\x03\x01\x12\x03E\x1c&\n\x0c\n\x05\x04\0\
    \x02\x03\x03\x12\x03E)*\n\xa4\x02\n\x04\x04\0\x02\x04\x12\x03L\x02)\x1a\
    \x96\x02\x20End\x20time\x20of\x20the\x20operation.\n\x20Required\x20when\
    \x20the\x20operation\x20is\x20used\x20in\n\x20[ServiceController.Report]\
    [google.api.servicecontrol.v1.ServiceController.Report],\n\x20but\x20opt\
    ional\x20when\x20the\x20operation\x20is\x20used\x20in\n\x20[ServiceContr\
    oller.Check][google.api.servicecontrol.v1.ServiceController.Check].\n\n\
    \x0c\n\x05\x04\0\x02\x04\x06\x12\x03L\x02\x1b\n\x0c\n\x05\x04\0\x02\x04\
    \x01\x12\x03L\x1c$\n\x0c\n\x05\x04\0\x02\x04\x03\x12\x03L'(\n\xa3\x06\n\
    \x04\x04\0\x02\x05\x12\x03]\x02!\x1a\x95\x06\x20Labels\x20describing\x20\
    the\x20operation.\x20Only\x20the\x20following\x20labels\x20are\x20allowe\
    d:\n\n\x20-\x20Labels\x20describing\x20monitored\x20resources\x20as\x20d\
    efined\x20in\n\x20\x20\x20the\x20service\x20configuration.\n\x20-\x20Def\
    ault\x20labels\x20of\x20metric\x20values.\x20When\x20specified,\x20label\
    s\x20defined\x20in\x20the\n\x20\x20\x20metric\x20value\x20override\x20th\
    ese\x20default.\n\x20-\x20The\x20following\x20labels\x20defined\x20by\
    \x20Google\x20Cloud\x20Platform:\n\x20\x20\x20\x20\x20-\x20`cloud.google\
    apis.com/location`\x20describing\x20the\x20location\x20where\x20the\n\
    \x20\x20\x20\x20\x20\x20\x20\x20operation\x20happened,\n\x20\x20\x20\x20\
    \x20-\x20`servicecontrol.googleapis.com/user_agent`\x20describing\x20the\
    \x20user\x20agent\n\x20\x20\x20\x20\x20\x20\x20\x20of\x20the\x20API\x20r\
    equest,\n\x20\x20\x20\x20\x20-\x20`servicecontrol.googleapis.com/service\
    _agent`\x20describing\x20the\x20service\n\x20\x20\x20\x20\x20\x20\x20\
    \x20used\x20to\x20handle\x20the\x20API\x20request\x20(e.g.\x20ESP),\n\
    \x20\x20\x20\x20\x20-\x20`servicecontrol.googleapis.com/platform`\x20des\
    cribing\x20the\x20platform\n\x20\x20\x20\x20\x20\x20\x20\x20where\x20the\
    \x20API\x20is\x20served\x20(e.g.\x20GAE,\x20GCE,\x20GKE).\n\n\x0c\n\x05\
    \x04\0\x02\x05\x06\x12\x03]\x02\x15\n\x0c\n\x05\x04\0\x02\x05\x01\x12\
    \x03]\x16\x1c\n\x0c\n\x05\x04\0\x02\x05\x03\x12\x03]\x1f\x20\n\x9a\x04\n\
    \x04\x04\0\x02\x06\x12\x03i\x020\x1a\x8c\x04\x20Represents\x20informatio\
    n\x20about\x20this\x20operation.\x20Each\x20MetricValueSet\n\x20correspo\
    nds\x20to\x20a\x20metric\x20defined\x20in\x20the\x20service\x20configura\
    tion.\n\x20The\x20data\x20type\x20used\x20in\x20the\x20MetricValueSet\
    \x20must\x20agree\x20with\n\x20the\x20data\x20type\x20specified\x20in\
    \x20the\x20metric\x20definition.\n\n\x20Within\x20a\x20single\x20operati\
    on,\x20it\x20is\x20not\x20allowed\x20to\x20have\x20more\x20than\x20one\n\
    \x20MetricValue\x20instances\x20that\x20have\x20the\x20same\x20metric\
    \x20names\x20and\x20identical\n\x20label\x20value\x20combinations.\x20If\
    \x20a\x20request\x20has\x20such\x20duplicated\x20MetricValue\n\x20instan\
    ces,\x20the\x20entire\x20request\x20is\x20rejected\x20with\n\x20an\x20in\
    valid\x20argument\x20error.\n\n\x0c\n\x05\x04\0\x02\x06\x04\x12\x03i\x02\
    \n\n\x0c\n\x05\x04\0\x02\x06\x06\x12\x03i\x0b\x19\n\x0c\n\x05\x04\0\x02\
    \x06\x01\x12\x03i\x1a+\n\x0c\n\x05\x04\0\x02\x06\x03\x12\x03i./\n3\n\x04\
    \x04\0\x02\x07\x12\x03l\x02$\x1a&\x20Represents\x20information\x20to\x20\
    be\x20logged.\n\n\x0c\n\x05\x04\0\x02\x07\x04\x12\x03l\x02\n\n\x0c\n\x05\
    \x04\0\x02\x07\x06\x12\x03l\x0b\x13\n\x0c\n\x05\x04\0\x02\x07\x01\x12\
    \x03l\x14\x1f\n\x0c\n\x05\x04\0\x02\x07\x03\x12\x03l\"#\n9\n\x04\x04\0\
    \x02\x08\x12\x03o\x02\x1d\x1a,\x20DO\x20NOT\x20USE.\x20This\x20is\x20an\
    \x20experimental\x20field.\n\n\x0c\n\x05\x04\0\x02\x08\x06\x12\x03o\x02\
    \x0c\n\x0c\n\x05\x04\0\x02\x08\x01\x12\x03o\r\x17\n\x0c\n\x05\x04\0\x02\
    \x08\x03\x12\x03o\x1a\x1cb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(4);
            deps.push(super::annotations::file_descriptor().clone());
            deps.push(super::log_entry::file_descriptor().clone());
            deps.push(super::metric_value::file_descriptor().clone());
            deps.push(::protobuf::well_known_types::timestamp::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(Operation::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(1);
            enums.push(operation::Importance::generated_enum_descriptor_data());
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
