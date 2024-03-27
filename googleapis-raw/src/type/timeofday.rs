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

//! Generated file from `google/type/timeofday.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

///  Represents a time of day. The date and time zone are either not significant
///  or are specified elsewhere. An API may choose to allow leap seconds. Related
///  types are [google.type.Date][google.type.Date] and `google.protobuf.Timestamp`.
// @@protoc_insertion_point(message:google.type.TimeOfDay)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct TimeOfDay {
    // message fields
    ///  Hours of day in 24 hour format. Should be from 0 to 23. An API may choose
    ///  to allow the value "24:00:00" for scenarios like business closing time.
    // @@protoc_insertion_point(field:google.type.TimeOfDay.hours)
    pub hours: i32,
    ///  Minutes of hour of day. Must be from 0 to 59.
    // @@protoc_insertion_point(field:google.type.TimeOfDay.minutes)
    pub minutes: i32,
    ///  Seconds of minutes of the time. Must normally be from 0 to 59. An API may
    ///  allow the value 60 if it allows leap-seconds.
    // @@protoc_insertion_point(field:google.type.TimeOfDay.seconds)
    pub seconds: i32,
    ///  Fractions of seconds in nanoseconds. Must be from 0 to 999,999,999.
    // @@protoc_insertion_point(field:google.type.TimeOfDay.nanos)
    pub nanos: i32,
    // special fields
    // @@protoc_insertion_point(special_field:google.type.TimeOfDay.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a TimeOfDay {
    fn default() -> &'a TimeOfDay {
        <TimeOfDay as ::protobuf::Message>::default_instance()
    }
}

impl TimeOfDay {
    pub fn new() -> TimeOfDay {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "hours",
            |m: &TimeOfDay| { &m.hours },
            |m: &mut TimeOfDay| { &mut m.hours },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "minutes",
            |m: &TimeOfDay| { &m.minutes },
            |m: &mut TimeOfDay| { &mut m.minutes },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "seconds",
            |m: &TimeOfDay| { &m.seconds },
            |m: &mut TimeOfDay| { &mut m.seconds },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "nanos",
            |m: &TimeOfDay| { &m.nanos },
            |m: &mut TimeOfDay| { &mut m.nanos },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<TimeOfDay>(
            "TimeOfDay",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for TimeOfDay {
    const NAME: &'static str = "TimeOfDay";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.hours = is.read_int32()?;
                },
                16 => {
                    self.minutes = is.read_int32()?;
                },
                24 => {
                    self.seconds = is.read_int32()?;
                },
                32 => {
                    self.nanos = is.read_int32()?;
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
        if self.hours != 0 {
            my_size += ::protobuf::rt::int32_size(1, self.hours);
        }
        if self.minutes != 0 {
            my_size += ::protobuf::rt::int32_size(2, self.minutes);
        }
        if self.seconds != 0 {
            my_size += ::protobuf::rt::int32_size(3, self.seconds);
        }
        if self.nanos != 0 {
            my_size += ::protobuf::rt::int32_size(4, self.nanos);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.hours != 0 {
            os.write_int32(1, self.hours)?;
        }
        if self.minutes != 0 {
            os.write_int32(2, self.minutes)?;
        }
        if self.seconds != 0 {
            os.write_int32(3, self.seconds)?;
        }
        if self.nanos != 0 {
            os.write_int32(4, self.nanos)?;
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

    fn new() -> TimeOfDay {
        TimeOfDay::new()
    }

    fn clear(&mut self) {
        self.hours = 0;
        self.minutes = 0;
        self.seconds = 0;
        self.nanos = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static TimeOfDay {
        static instance: TimeOfDay = TimeOfDay {
            hours: 0,
            minutes: 0,
            seconds: 0,
            nanos: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for TimeOfDay {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("TimeOfDay").unwrap()).clone()
    }
}

impl ::std::fmt::Display for TimeOfDay {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TimeOfDay {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1bgoogle/type/timeofday.proto\x12\x0bgoogle.type\"k\n\tTimeOfDay\x12\
    \x14\n\x05hours\x18\x01\x20\x01(\x05R\x05hours\x12\x18\n\x07minutes\x18\
    \x02\x20\x01(\x05R\x07minutes\x12\x18\n\x07seconds\x18\x03\x20\x01(\x05R\
    \x07seconds\x12\x14\n\x05nanos\x18\x04\x20\x01(\x05R\x05nanosBl\n\x0fcom\
    .google.typeB\x0eTimeOfDayProtoP\x01Z>google.golang.org/genproto/googlea\
    pis/type/timeofday;timeofday\xf8\x01\x01\xa2\x02\x03GTPJ\xc2\x0c\n\x06\
    \x12\x04\x0f\0+\x01\n\xbe\x04\n\x01\x0c\x12\x03\x0f\0\x122\xb3\x04\x20Co\
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
    \x11\0\x14\n\x08\n\x01\x08\x12\x03\x13\0\x1f\n\t\n\x02\x08\x1f\x12\x03\
    \x13\0\x1f\n\x08\n\x01\x08\x12\x03\x14\0U\n\t\n\x02\x08\x0b\x12\x03\x14\
    \0U\n\x08\n\x01\x08\x12\x03\x15\0\"\n\t\n\x02\x08\n\x12\x03\x15\0\"\n\
    \x08\n\x01\x08\x12\x03\x16\0/\n\t\n\x02\x08\x08\x12\x03\x16\0/\n\x08\n\
    \x01\x08\x12\x03\x17\0(\n\t\n\x02\x08\x01\x12\x03\x17\0(\n\x08\n\x01\x08\
    \x12\x03\x18\0!\n\t\n\x02\x08$\x12\x03\x18\0!\n\xf9\x01\n\x02\x04\0\x12\
    \x04\x1d\0+\x01\x1a\xec\x01\x20Represents\x20a\x20time\x20of\x20day.\x20\
    The\x20date\x20and\x20time\x20zone\x20are\x20either\x20not\x20significan\
    t\n\x20or\x20are\x20specified\x20elsewhere.\x20An\x20API\x20may\x20choos\
    e\x20to\x20allow\x20leap\x20seconds.\x20Related\n\x20types\x20are\x20[go\
    ogle.type.Date][google.type.Date]\x20and\x20`google.protobuf.Timestamp`.\
    \n\n\n\n\x03\x04\0\x01\x12\x03\x1d\x08\x11\n\xa2\x01\n\x04\x04\0\x02\0\
    \x12\x03\x20\x02\x12\x1a\x94\x01\x20Hours\x20of\x20day\x20in\x2024\x20ho\
    ur\x20format.\x20Should\x20be\x20from\x200\x20to\x2023.\x20An\x20API\x20\
    may\x20choose\n\x20to\x20allow\x20the\x20value\x20\"24:00:00\"\x20for\
    \x20scenarios\x20like\x20business\x20closing\x20time.\n\n\x0c\n\x05\x04\
    \0\x02\0\x05\x12\x03\x20\x02\x07\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x20\
    \x08\r\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x20\x10\x11\n<\n\x04\x04\0\
    \x02\x01\x12\x03#\x02\x14\x1a/\x20Minutes\x20of\x20hour\x20of\x20day.\
    \x20Must\x20be\x20from\x200\x20to\x2059.\n\n\x0c\n\x05\x04\0\x02\x01\x05\
    \x12\x03#\x02\x07\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03#\x08\x0f\n\x0c\n\
    \x05\x04\0\x02\x01\x03\x12\x03#\x12\x13\n\x87\x01\n\x04\x04\0\x02\x02\
    \x12\x03'\x02\x14\x1az\x20Seconds\x20of\x20minutes\x20of\x20the\x20time.\
    \x20Must\x20normally\x20be\x20from\x200\x20to\x2059.\x20An\x20API\x20may\
    \n\x20allow\x20the\x20value\x2060\x20if\x20it\x20allows\x20leap-seconds.\
    \n\n\x0c\n\x05\x04\0\x02\x02\x05\x12\x03'\x02\x07\n\x0c\n\x05\x04\0\x02\
    \x02\x01\x12\x03'\x08\x0f\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03'\x12\x13\
    \nR\n\x04\x04\0\x02\x03\x12\x03*\x02\x12\x1aE\x20Fractions\x20of\x20seco\
    nds\x20in\x20nanoseconds.\x20Must\x20be\x20from\x200\x20to\x20999,999,99\
    9.\n\n\x0c\n\x05\x04\0\x02\x03\x05\x12\x03*\x02\x07\n\x0c\n\x05\x04\0\
    \x02\x03\x01\x12\x03*\x08\r\n\x0c\n\x05\x04\0\x02\x03\x03\x12\x03*\x10\
    \x11b\x06proto3\
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
            messages.push(TimeOfDay::generated_message_descriptor_data());
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
