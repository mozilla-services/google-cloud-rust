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

//! Generated file from `google/api/monitoring.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_2_0;

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:google.api.Monitoring)
pub struct Monitoring {
    // message fields
    ///  Monitoring configurations for sending metrics to the producer project.
    ///  There can be multiple producer destinations. A monitored resouce type may
    ///  appear in multiple monitoring destinations if different aggregations are
    ///  needed for different sets of metrics associated with that monitored
    ///  resource type. A monitored resource and metric pair may only be used once
    ///  in the Monitoring configuration.
    // @@protoc_insertion_point(field:google.api.Monitoring.producer_destinations)
    pub producer_destinations: ::std::vec::Vec<monitoring::MonitoringDestination>,
    ///  Monitoring configurations for sending metrics to the consumer project.
    ///  There can be multiple consumer destinations. A monitored resouce type may
    ///  appear in multiple monitoring destinations if different aggregations are
    ///  needed for different sets of metrics associated with that monitored
    ///  resource type. A monitored resource and metric pair may only be used once
    ///  in the Monitoring configuration.
    // @@protoc_insertion_point(field:google.api.Monitoring.consumer_destinations)
    pub consumer_destinations: ::std::vec::Vec<monitoring::MonitoringDestination>,
    // special fields
    // @@protoc_insertion_point(special_field:google.api.Monitoring.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Monitoring {
    fn default() -> &'a Monitoring {
        <Monitoring as ::protobuf::Message>::default_instance()
    }
}

impl Monitoring {
    pub fn new() -> Monitoring {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "producer_destinations",
            |m: &Monitoring| { &m.producer_destinations },
            |m: &mut Monitoring| { &mut m.producer_destinations },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "consumer_destinations",
            |m: &Monitoring| { &m.consumer_destinations },
            |m: &mut Monitoring| { &mut m.consumer_destinations },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Monitoring>(
            "Monitoring",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Monitoring {
    const NAME: &'static str = "Monitoring";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.producer_destinations.push(is.read_message()?);
                },
                18 => {
                    self.consumer_destinations.push(is.read_message()?);
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
        for value in &self.producer_destinations {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.consumer_destinations {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.producer_destinations {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        };
        for v in &self.consumer_destinations {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> Monitoring {
        Monitoring::new()
    }

    fn clear(&mut self) {
        self.producer_destinations.clear();
        self.consumer_destinations.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Monitoring {
        static instance: Monitoring = Monitoring {
            producer_destinations: ::std::vec::Vec::new(),
            consumer_destinations: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Monitoring {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Monitoring").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Monitoring {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Monitoring {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `Monitoring`
pub mod monitoring {
    ///  Configuration of a specific monitoring destination (the producer project
    ///  or the consumer project).
    #[derive(PartialEq,Clone,Default,Debug)]
    // @@protoc_insertion_point(message:google.api.Monitoring.MonitoringDestination)
    pub struct MonitoringDestination {
        // message fields
        ///  The monitored resource type. The type must be defined in
        ///  [Service.monitored_resources][google.api.Service.monitored_resources] section.
        // @@protoc_insertion_point(field:google.api.Monitoring.MonitoringDestination.monitored_resource)
        pub monitored_resource: ::std::string::String,
        ///  Types of the metrics to report to this monitoring destination.
        ///  Each type must be defined in [Service.metrics][google.api.Service.metrics] section.
        // @@protoc_insertion_point(field:google.api.Monitoring.MonitoringDestination.metrics)
        pub metrics: ::std::vec::Vec<::std::string::String>,
        // special fields
        // @@protoc_insertion_point(special_field:google.api.Monitoring.MonitoringDestination.special_fields)
        pub special_fields: ::protobuf::SpecialFields,
    }

    impl<'a> ::std::default::Default for &'a MonitoringDestination {
        fn default() -> &'a MonitoringDestination {
            <MonitoringDestination as ::protobuf::Message>::default_instance()
        }
    }

    impl MonitoringDestination {
        pub fn new() -> MonitoringDestination {
            ::std::default::Default::default()
        }

        pub(in super) fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
            let mut fields = ::std::vec::Vec::with_capacity(2);
            let mut oneofs = ::std::vec::Vec::with_capacity(0);
            fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
                "monitored_resource",
                |m: &MonitoringDestination| { &m.monitored_resource },
                |m: &mut MonitoringDestination| { &mut m.monitored_resource },
            ));
            fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
                "metrics",
                |m: &MonitoringDestination| { &m.metrics },
                |m: &mut MonitoringDestination| { &mut m.metrics },
            ));
            ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MonitoringDestination>(
                "Monitoring.MonitoringDestination",
                fields,
                oneofs,
            )
        }
    }

    impl ::protobuf::Message for MonitoringDestination {
        const NAME: &'static str = "MonitoringDestination";

        fn is_initialized(&self) -> bool {
            true
        }

        fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
            while let Some(tag) = is.read_raw_tag_or_eof()? {
                match tag {
                    10 => {
                        self.monitored_resource = is.read_string()?;
                    },
                    18 => {
                        self.metrics.push(is.read_string()?);
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
            if !self.monitored_resource.is_empty() {
                my_size += ::protobuf::rt::string_size(1, &self.monitored_resource);
            }
            for value in &self.metrics {
                my_size += ::protobuf::rt::string_size(2, &value);
            };
            my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
            self.special_fields.cached_size().set(my_size as u32);
            my_size
        }

        fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
            if !self.monitored_resource.is_empty() {
                os.write_string(1, &self.monitored_resource)?;
            }
            for v in &self.metrics {
                os.write_string(2, &v)?;
            };
            os.write_unknown_fields(self.special_fields.unknown_fields())?;
            ::std::result::Result::Ok(())
        }

        fn special_fields(&self) -> &::protobuf::SpecialFields {
            &self.special_fields
        }

        fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
            &mut self.special_fields
        }

        fn new() -> MonitoringDestination {
            MonitoringDestination::new()
        }

        fn clear(&mut self) {
            self.monitored_resource.clear();
            self.metrics.clear();
            self.special_fields.clear();
        }

        fn default_instance() -> &'static MonitoringDestination {
            static instance: MonitoringDestination = MonitoringDestination {
                monitored_resource: ::std::string::String::new(),
                metrics: ::std::vec::Vec::new(),
                special_fields: ::protobuf::SpecialFields::new(),
            };
            &instance
        }
    }

    impl ::protobuf::MessageFull for MonitoringDestination {
        fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| super::file_descriptor().message_by_package_relative_name("Monitoring.MonitoringDestination").unwrap()).clone()
        }
    }

    impl ::std::fmt::Display for MonitoringDestination {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            ::protobuf::text_format::fmt(self, f)
        }
    }

    impl ::protobuf::reflect::ProtobufValue for MonitoringDestination {
        type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1bgoogle/api/monitoring.proto\x12\ngoogle.api\"\xb4\x02\n\nMonitorin\
    g\x12a\n\x15producer_destinations\x18\x01\x20\x03(\x0b2,.google.api.Moni\
    toring.MonitoringDestinationR\x14producerDestinations\x12a\n\x15consumer\
    _destinations\x18\x02\x20\x03(\x0b2,.google.api.Monitoring.MonitoringDes\
    tinationR\x14consumerDestinations\x1a`\n\x15MonitoringDestination\x12-\n\
    \x12monitored_resource\x18\x01\x20\x01(\tR\x11monitoredResource\x12\x18\
    \n\x07metrics\x18\x02\x20\x03(\tR\x07metricsBq\n\x0ecom.google.apiB\x0fM\
    onitoringProtoP\x01ZEgoogle.golang.org/genproto/googleapis/api/serviceco\
    nfig;serviceconfig\xa2\x02\x04GAPIJ\x8a\x1c\n\x06\x12\x04\x0f\0Z\x01\n\
    \xbe\x04\n\x01\x0c\x12\x03\x0f\0\x122\xb3\x04\x20Copyright\x202019\x20Go\
    ogle\x20LLC.\n\n\x20Licensed\x20under\x20the\x20Apache\x20License,\x20Ve\
    rsion\x202.0\x20(the\x20\"License\");\n\x20you\x20may\x20not\x20use\x20t\
    his\x20file\x20except\x20in\x20compliance\x20with\x20the\x20License.\n\
    \x20You\x20may\x20obtain\x20a\x20copy\x20of\x20the\x20License\x20at\n\n\
    \x20\x20\x20\x20\x20http://www.apache.org/licenses/LICENSE-2.0\n\n\x20Un\
    less\x20required\x20by\x20applicable\x20law\x20or\x20agreed\x20to\x20in\
    \x20writing,\x20software\n\x20distributed\x20under\x20the\x20License\x20\
    is\x20distributed\x20on\x20an\x20\"AS\x20IS\"\x20BASIS,\n\x20WITHOUT\x20\
    WARRANTIES\x20OR\x20CONDITIONS\x20OF\x20ANY\x20KIND,\x20either\x20expres\
    s\x20or\x20implied.\n\x20See\x20the\x20License\x20for\x20the\x20specific\
    \x20language\x20governing\x20permissions\x20and\n\x20limitations\x20unde\
    r\x20the\x20License.\n\n\n\x08\n\x01\x02\x12\x03\x11\0\x13\n\x08\n\x01\
    \x08\x12\x03\x13\0\\\n\t\n\x02\x08\x0b\x12\x03\x13\0\\\n\x08\n\x01\x08\
    \x12\x03\x14\0\"\n\t\n\x02\x08\n\x12\x03\x14\0\"\n\x08\n\x01\x08\x12\x03\
    \x15\00\n\t\n\x02\x08\x08\x12\x03\x15\00\n\x08\n\x01\x08\x12\x03\x16\0'\
    \n\t\n\x02\x08\x01\x12\x03\x16\0'\n\x08\n\x01\x08\x12\x03\x17\0\"\n\t\n\
    \x02\x08$\x12\x03\x17\0\"\n\xd7\n\n\x02\x04\0\x12\x04>\0Z\x01\x1a\xca\n\
    \x20Monitoring\x20configuration\x20of\x20the\x20service.\n\n\x20The\x20e\
    xample\x20below\x20shows\x20how\x20to\x20configure\x20monitored\x20resou\
    rces\x20and\x20metrics\n\x20for\x20monitoring.\x20In\x20the\x20example,\
    \x20a\x20monitored\x20resource\x20and\x20two\x20metrics\x20are\n\x20defi\
    ned.\x20The\x20`library.googleapis.com/book/returned_count`\x20metric\
    \x20is\x20sent\n\x20to\x20both\x20producer\x20and\x20consumer\x20project\
    s,\x20whereas\x20the\n\x20`library.googleapis.com/book/overdue_count`\
    \x20metric\x20is\x20only\x20sent\x20to\x20the\n\x20consumer\x20project.\
    \n\n\x20\x20\x20\x20\x20monitored_resources:\n\x20\x20\x20\x20\x20-\x20t\
    ype:\x20library.googleapis.com/branch\n\x20\x20\x20\x20\x20\x20\x20label\
    s:\n\x20\x20\x20\x20\x20\x20\x20-\x20key:\x20/city\n\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20description:\x20The\x20city\x20where\x20the\x20library\
    \x20branch\x20is\x20located\x20in.\n\x20\x20\x20\x20\x20\x20\x20-\x20key\
    :\x20/name\n\x20\x20\x20\x20\x20\x20\x20\x20\x20description:\x20The\x20n\
    ame\x20of\x20the\x20branch.\n\x20\x20\x20\x20\x20metrics:\n\x20\x20\x20\
    \x20\x20-\x20name:\x20library.googleapis.com/book/returned_count\n\x20\
    \x20\x20\x20\x20\x20\x20metric_kind:\x20DELTA\n\x20\x20\x20\x20\x20\x20\
    \x20value_type:\x20INT64\n\x20\x20\x20\x20\x20\x20\x20labels:\n\x20\x20\
    \x20\x20\x20\x20\x20-\x20key:\x20/customer_id\n\x20\x20\x20\x20\x20-\x20\
    name:\x20library.googleapis.com/book/overdue_count\n\x20\x20\x20\x20\x20\
    \x20\x20metric_kind:\x20GAUGE\n\x20\x20\x20\x20\x20\x20\x20value_type:\
    \x20INT64\n\x20\x20\x20\x20\x20\x20\x20labels:\n\x20\x20\x20\x20\x20\x20\
    \x20-\x20key:\x20/customer_id\n\x20\x20\x20\x20\x20monitoring:\n\x20\x20\
    \x20\x20\x20\x20\x20producer_destinations:\n\x20\x20\x20\x20\x20\x20\x20\
    -\x20monitored_resource:\x20library.googleapis.com/branch\n\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20metrics:\n\x20\x20\x20\x20\x20\x20\x20\x20\x20-\
    \x20library.googleapis.com/book/returned_count\n\x20\x20\x20\x20\x20\x20\
    \x20consumer_destinations:\n\x20\x20\x20\x20\x20\x20\x20-\x20monitored_r\
    esource:\x20library.googleapis.com/branch\n\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20metrics:\n\x20\x20\x20\x20\x20\x20\x20\x20\x20-\x20library.googl\
    eapis.com/book/returned_count\n\x20\x20\x20\x20\x20\x20\x20\x20\x20-\x20\
    library.googleapis.com/book/overdue_count\n\n\n\n\x03\x04\0\x01\x12\x03>\
    \x08\x12\ns\n\x04\x04\0\x03\0\x12\x04A\x02I\x03\x1ae\x20Configuration\
    \x20of\x20a\x20specific\x20monitoring\x20destination\x20(the\x20producer\
    \x20project\n\x20or\x20the\x20consumer\x20project).\n\n\x0c\n\x05\x04\0\
    \x03\0\x01\x12\x03A\n\x1f\n\x9a\x01\n\x06\x04\0\x03\0\x02\0\x12\x03D\x04\
    \"\x1a\x8a\x01\x20The\x20monitored\x20resource\x20type.\x20The\x20type\
    \x20must\x20be\x20defined\x20in\n\x20[Service.monitored_resources][googl\
    e.api.Service.monitored_resources]\x20section.\n\n\x0e\n\x07\x04\0\x03\0\
    \x02\0\x05\x12\x03D\x04\n\n\x0e\n\x07\x04\0\x03\0\x02\0\x01\x12\x03D\x0b\
    \x1d\n\x0e\n\x07\x04\0\x03\0\x02\0\x03\x12\x03D\x20!\n\xa5\x01\n\x06\x04\
    \0\x03\0\x02\x01\x12\x03H\x04\x20\x1a\x95\x01\x20Types\x20of\x20the\x20m\
    etrics\x20to\x20report\x20to\x20this\x20monitoring\x20destination.\n\x20\
    Each\x20type\x20must\x20be\x20defined\x20in\x20[Service.metrics][google.\
    api.Service.metrics]\x20section.\n\n\x0e\n\x07\x04\0\x03\0\x02\x01\x04\
    \x12\x03H\x04\x0c\n\x0e\n\x07\x04\0\x03\0\x02\x01\x05\x12\x03H\r\x13\n\
    \x0e\n\x07\x04\0\x03\0\x02\x01\x01\x12\x03H\x14\x1b\n\x0e\n\x07\x04\0\
    \x03\0\x02\x01\x03\x12\x03H\x1e\x1f\n\x9d\x03\n\x04\x04\0\x02\0\x12\x03Q\
    \x02;\x1a\x8f\x03\x20Monitoring\x20configurations\x20for\x20sending\x20m\
    etrics\x20to\x20the\x20producer\x20project.\n\x20There\x20can\x20be\x20m\
    ultiple\x20producer\x20destinations.\x20A\x20monitored\x20resouce\x20typ\
    e\x20may\n\x20appear\x20in\x20multiple\x20monitoring\x20destinations\x20\
    if\x20different\x20aggregations\x20are\n\x20needed\x20for\x20different\
    \x20sets\x20of\x20metrics\x20associated\x20with\x20that\x20monitored\n\
    \x20resource\x20type.\x20A\x20monitored\x20resource\x20and\x20metric\x20\
    pair\x20may\x20only\x20be\x20used\x20once\n\x20in\x20the\x20Monitoring\
    \x20configuration.\n\n\x0c\n\x05\x04\0\x02\0\x04\x12\x03Q\x02\n\n\x0c\n\
    \x05\x04\0\x02\0\x06\x12\x03Q\x0b\x20\n\x0c\n\x05\x04\0\x02\0\x01\x12\
    \x03Q!6\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03Q9:\n\x9d\x03\n\x04\x04\0\x02\
    \x01\x12\x03Y\x02;\x1a\x8f\x03\x20Monitoring\x20configurations\x20for\
    \x20sending\x20metrics\x20to\x20the\x20consumer\x20project.\n\x20There\
    \x20can\x20be\x20multiple\x20consumer\x20destinations.\x20A\x20monitored\
    \x20resouce\x20type\x20may\n\x20appear\x20in\x20multiple\x20monitoring\
    \x20destinations\x20if\x20different\x20aggregations\x20are\n\x20needed\
    \x20for\x20different\x20sets\x20of\x20metrics\x20associated\x20with\x20t\
    hat\x20monitored\n\x20resource\x20type.\x20A\x20monitored\x20resource\
    \x20and\x20metric\x20pair\x20may\x20only\x20be\x20used\x20once\n\x20in\
    \x20the\x20Monitoring\x20configuration.\n\n\x0c\n\x05\x04\0\x02\x01\x04\
    \x12\x03Y\x02\n\n\x0c\n\x05\x04\0\x02\x01\x06\x12\x03Y\x0b\x20\n\x0c\n\
    \x05\x04\0\x02\x01\x01\x12\x03Y!6\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03Y\
    9:b\x06proto3\
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
            let mut messages = ::std::vec::Vec::with_capacity(2);
            messages.push(Monitoring::generated_message_descriptor_data());
            messages.push(monitoring::MonitoringDestination::generated_message_descriptor_data());
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
