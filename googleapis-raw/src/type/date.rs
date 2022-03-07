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
//! Generated file from `google/type/date.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_27_1;

#[derive(PartialEq,Clone,Default)]
pub struct Date {
    // message fields
    pub year: i32,
    pub month: i32,
    pub day: i32,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Date {
    fn default() -> &'a Date {
        <Date as ::protobuf::Message>::default_instance()
    }
}

impl Date {
    pub fn new() -> Date {
        ::std::default::Default::default()
    }

    // int32 year = 1;


    pub fn get_year(&self) -> i32 {
        self.year
    }
    pub fn clear_year(&mut self) {
        self.year = 0;
    }

    // Param is passed by value, moved
    pub fn set_year(&mut self, v: i32) {
        self.year = v;
    }

    // int32 month = 2;


    pub fn get_month(&self) -> i32 {
        self.month
    }
    pub fn clear_month(&mut self) {
        self.month = 0;
    }

    // Param is passed by value, moved
    pub fn set_month(&mut self, v: i32) {
        self.month = v;
    }

    // int32 day = 3;


    pub fn get_day(&self) -> i32 {
        self.day
    }
    pub fn clear_day(&mut self) {
        self.day = 0;
    }

    // Param is passed by value, moved
    pub fn set_day(&mut self, v: i32) {
        self.day = v;
    }
}

impl ::protobuf::Message for Date {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.year = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.month = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.day = tmp;
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
        if self.year != 0 {
            my_size += ::protobuf::rt::value_size(1, self.year, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.month != 0 {
            my_size += ::protobuf::rt::value_size(2, self.month, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.day != 0 {
            my_size += ::protobuf::rt::value_size(3, self.day, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.year != 0 {
            os.write_int32(1, self.year)?;
        }
        if self.month != 0 {
            os.write_int32(2, self.month)?;
        }
        if self.day != 0 {
            os.write_int32(3, self.day)?;
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

    fn new() -> Date {
        Date::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                "year",
                |m: &Date| { &m.year },
                |m: &mut Date| { &mut m.year },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                "month",
                |m: &Date| { &m.month },
                |m: &mut Date| { &mut m.month },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                "day",
                |m: &Date| { &m.day },
                |m: &mut Date| { &mut m.day },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Date>(
                "Date",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Date {
        static instance: ::protobuf::rt::LazyV2<Date> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Date::new)
    }
}

impl ::protobuf::Clear for Date {
    fn clear(&mut self) {
        self.year = 0;
        self.month = 0;
        self.day = 0;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Date {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Date {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x16google/type/date.proto\x12\x0bgoogle.type\"B\n\x04Date\x12\x12\n\
    \x04year\x18\x01\x20\x01(\x05R\x04year\x12\x14\n\x05month\x18\x02\x20\
    \x01(\x05R\x05month\x12\x10\n\x03day\x18\x03\x20\x01(\x05R\x03dayB]\n\
    \x0fcom.google.typeB\tDateProtoP\x01Z4google.golang.org/genproto/googlea\
    pis/type/date;date\xf8\x01\x01\xa2\x02\x03GTPJ\xc0\x0e\n\x06\x12\x04\x0e\
    \03\x01\n\xbc\x04\n\x01\x0c\x12\x03\x0e\0\x122\xb1\x04\x20Copyright\x202\
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
    \x01\x08\x12\x03\x13\0K\n\t\n\x02\x08\x0b\x12\x03\x13\0K\n\x08\n\x01\x08\
    \x12\x03\x14\0\"\n\t\n\x02\x08\n\x12\x03\x14\0\"\n\x08\n\x01\x08\x12\x03\
    \x15\0*\n\t\n\x02\x08\x08\x12\x03\x15\0*\n\x08\n\x01\x08\x12\x03\x16\0(\
    \n\t\n\x02\x08\x01\x12\x03\x16\0(\n\x08\n\x01\x08\x12\x03\x17\0!\n\t\n\
    \x02\x08$\x12\x03\x17\0!\n\xe9\x04\n\x02\x04\0\x12\x04&\03\x01\x1a\xdc\
    \x04\x20Represents\x20a\x20whole\x20or\x20partial\x20calendar\x20date,\
    \x20such\x20as\x20a\x20birthday.\x20The\x20time\x20of\n\x20day\x20and\
    \x20time\x20zone\x20are\x20either\x20specified\x20elsewhere\x20or\x20are\
    \x20insignificant.\x20The\n\x20date\x20is\x20relative\x20to\x20the\x20Gr\
    egorian\x20Calendar.\x20This\x20can\x20represent\x20one\x20of\x20the\n\
    \x20following:\n\n\x20*\x20A\x20full\x20date,\x20with\x20non-zero\x20yea\
    r,\x20month,\x20and\x20day\x20values\n\x20*\x20A\x20month\x20and\x20day\
    \x20value,\x20with\x20a\x20zero\x20year,\x20such\x20as\x20an\x20annivers\
    ary\n\x20*\x20A\x20year\x20on\x20its\x20own,\x20with\x20zero\x20month\
    \x20and\x20day\x20values\n\x20*\x20A\x20year\x20and\x20month\x20value,\
    \x20with\x20a\x20zero\x20day,\x20such\x20as\x20a\x20credit\x20card\x20ex\
    piration\n\x20date\n\n\x20Related\x20types\x20are\x20[google.type.TimeOf\
    Day][google.type.TimeOfDay]\x20and\n\x20`google.protobuf.Timestamp`.\n\n\
    \n\n\x03\x04\0\x01\x12\x03&\x08\x0c\n`\n\x04\x04\0\x02\0\x12\x03)\x02\
    \x11\x1aS\x20Year\x20of\x20the\x20date.\x20Must\x20be\x20from\x201\x20to\
    \x209999,\x20or\x200\x20to\x20specify\x20a\x20date\x20without\n\x20a\x20\
    year.\n\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03)\x02\x07\n\x0c\n\x05\x04\0\
    \x02\0\x01\x12\x03)\x08\x0c\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03)\x0f\x10\
    \nf\n\x04\x04\0\x02\x01\x12\x03-\x02\x12\x1aY\x20Month\x20of\x20a\x20yea\
    r.\x20Must\x20be\x20from\x201\x20to\x2012,\x20or\x200\x20to\x20specify\
    \x20a\x20year\x20without\x20a\n\x20month\x20and\x20day.\n\n\x0c\n\x05\
    \x04\0\x02\x01\x05\x12\x03-\x02\x07\n\x0c\n\x05\x04\0\x02\x01\x01\x12\
    \x03-\x08\r\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03-\x10\x11\n\xae\x01\n\
    \x04\x04\0\x02\x02\x12\x032\x02\x10\x1a\xa0\x01\x20Day\x20of\x20a\x20mon\
    th.\x20Must\x20be\x20from\x201\x20to\x2031\x20and\x20valid\x20for\x20the\
    \x20year\x20and\x20month,\x20or\x200\n\x20to\x20specify\x20a\x20year\x20\
    by\x20itself\x20or\x20a\x20year\x20and\x20month\x20where\x20the\x20day\
    \x20isn't\n\x20significant.\n\n\x0c\n\x05\x04\0\x02\x02\x05\x12\x032\x02\
    \x07\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x032\x08\x0b\n\x0c\n\x05\x04\0\
    \x02\x02\x03\x12\x032\x0e\x0fb\x06proto3\
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
