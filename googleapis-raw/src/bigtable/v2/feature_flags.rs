// This file is generated by rust-protobuf 2.28.0. Do not edit
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
//! Generated file from `google/bigtable/v2/feature_flags.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_28_0;

#[derive(PartialEq,Clone,Default)]
pub struct FeatureFlags {
    // message fields
    pub reverse_scans: bool,
    pub mutate_rows_rate_limit: bool,
    pub mutate_rows_rate_limit2: bool,
    pub last_scanned_row_responses: bool,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a FeatureFlags {
    fn default() -> &'a FeatureFlags {
        <FeatureFlags as ::protobuf::Message>::default_instance()
    }
}

impl FeatureFlags {
    pub fn new() -> FeatureFlags {
        ::std::default::Default::default()
    }

    // bool reverse_scans = 1;


    pub fn get_reverse_scans(&self) -> bool {
        self.reverse_scans
    }
    pub fn clear_reverse_scans(&mut self) {
        self.reverse_scans = false;
    }

    // Param is passed by value, moved
    pub fn set_reverse_scans(&mut self, v: bool) {
        self.reverse_scans = v;
    }

    // bool mutate_rows_rate_limit = 3;


    pub fn get_mutate_rows_rate_limit(&self) -> bool {
        self.mutate_rows_rate_limit
    }
    pub fn clear_mutate_rows_rate_limit(&mut self) {
        self.mutate_rows_rate_limit = false;
    }

    // Param is passed by value, moved
    pub fn set_mutate_rows_rate_limit(&mut self, v: bool) {
        self.mutate_rows_rate_limit = v;
    }

    // bool mutate_rows_rate_limit2 = 5;


    pub fn get_mutate_rows_rate_limit2(&self) -> bool {
        self.mutate_rows_rate_limit2
    }
    pub fn clear_mutate_rows_rate_limit2(&mut self) {
        self.mutate_rows_rate_limit2 = false;
    }

    // Param is passed by value, moved
    pub fn set_mutate_rows_rate_limit2(&mut self, v: bool) {
        self.mutate_rows_rate_limit2 = v;
    }

    // bool last_scanned_row_responses = 4;


    pub fn get_last_scanned_row_responses(&self) -> bool {
        self.last_scanned_row_responses
    }
    pub fn clear_last_scanned_row_responses(&mut self) {
        self.last_scanned_row_responses = false;
    }

    // Param is passed by value, moved
    pub fn set_last_scanned_row_responses(&mut self, v: bool) {
        self.last_scanned_row_responses = v;
    }
}

impl ::protobuf::Message for FeatureFlags {
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
                    let tmp = is.read_bool()?;
                    self.reverse_scans = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.mutate_rows_rate_limit = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.mutate_rows_rate_limit2 = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.last_scanned_row_responses = tmp;
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
        if self.reverse_scans != false {
            my_size += 2;
        }
        if self.mutate_rows_rate_limit != false {
            my_size += 2;
        }
        if self.mutate_rows_rate_limit2 != false {
            my_size += 2;
        }
        if self.last_scanned_row_responses != false {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.reverse_scans != false {
            os.write_bool(1, self.reverse_scans)?;
        }
        if self.mutate_rows_rate_limit != false {
            os.write_bool(3, self.mutate_rows_rate_limit)?;
        }
        if self.mutate_rows_rate_limit2 != false {
            os.write_bool(5, self.mutate_rows_rate_limit2)?;
        }
        if self.last_scanned_row_responses != false {
            os.write_bool(4, self.last_scanned_row_responses)?;
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

    fn new() -> FeatureFlags {
        FeatureFlags::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                "reverse_scans",
                |m: &FeatureFlags| { &m.reverse_scans },
                |m: &mut FeatureFlags| { &mut m.reverse_scans },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                "mutate_rows_rate_limit",
                |m: &FeatureFlags| { &m.mutate_rows_rate_limit },
                |m: &mut FeatureFlags| { &mut m.mutate_rows_rate_limit },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                "mutate_rows_rate_limit2",
                |m: &FeatureFlags| { &m.mutate_rows_rate_limit2 },
                |m: &mut FeatureFlags| { &mut m.mutate_rows_rate_limit2 },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                "last_scanned_row_responses",
                |m: &FeatureFlags| { &m.last_scanned_row_responses },
                |m: &mut FeatureFlags| { &mut m.last_scanned_row_responses },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<FeatureFlags>(
                "FeatureFlags",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static FeatureFlags {
        static instance: ::protobuf::rt::LazyV2<FeatureFlags> = ::protobuf::rt::LazyV2::INIT;
        instance.get(FeatureFlags::new)
    }
}

impl ::protobuf::Clear for FeatureFlags {
    fn clear(&mut self) {
        self.reverse_scans = false;
        self.mutate_rows_rate_limit = false;
        self.mutate_rows_rate_limit2 = false;
        self.last_scanned_row_responses = false;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for FeatureFlags {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FeatureFlags {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n&google/bigtable/v2/feature_flags.proto\x12\x12google.bigtable.v2\"\
    \xdc\x01\n\x0cFeatureFlags\x12#\n\rreverse_scans\x18\x01\x20\x01(\x08R\
    \x0creverseScans\x123\n\x16mutate_rows_rate_limit\x18\x03\x20\x01(\x08R\
    \x13mutateRowsRateLimit\x125\n\x17mutate_rows_rate_limit2\x18\x05\x20\
    \x01(\x08R\x14mutateRowsRateLimit2\x12;\n\x1alast_scanned_row_responses\
    \x18\x04\x20\x01(\x08R\x17lastScannedRowResponsesB\xbd\x01\n\x16com.goog\
    le.bigtable.v2B\x11FeatureFlagsProtoP\x01Z:google.golang.org/genproto/go\
    ogleapis/bigtable/v2;bigtable\xaa\x02\x18Google.Cloud.Bigtable.V2\xca\
    \x02\x18Google\\Cloud\\Bigtable\\V2\xea\x02\x1bGoogle::Cloud::Bigtable::\
    V2J\xb7\x10\n\x06\x12\x04\x0e\04\x01\n\xbc\x04\n\x01\x0c\x12\x03\x0e\0\
    \x122\xb1\x04\x20Copyright\x202023\x20Google\x20LLC\n\n\x20Licensed\x20u\
    nder\x20the\x20Apache\x20License,\x20Version\x202.0\x20(the\x20\"License\
    \");\n\x20you\x20may\x20not\x20use\x20this\x20file\x20except\x20in\x20co\
    mpliance\x20with\x20the\x20License.\n\x20You\x20may\x20obtain\x20a\x20co\
    py\x20of\x20the\x20License\x20at\n\n\x20\x20\x20\x20\x20http://www.apach\
    e.org/licenses/LICENSE-2.0\n\n\x20Unless\x20required\x20by\x20applicable\
    \x20law\x20or\x20agreed\x20to\x20in\x20writing,\x20software\n\x20distrib\
    uted\x20under\x20the\x20License\x20is\x20distributed\x20on\x20an\x20\"AS\
    \x20IS\"\x20BASIS,\n\x20WITHOUT\x20WARRANTIES\x20OR\x20CONDITIONS\x20OF\
    \x20ANY\x20KIND,\x20either\x20express\x20or\x20implied.\n\x20See\x20the\
    \x20License\x20for\x20the\x20specific\x20language\x20governing\x20permis\
    sions\x20and\n\x20limitations\x20under\x20the\x20License.\n\n\x08\n\x01\
    \x02\x12\x03\x10\0\x1b\n\x08\n\x01\x08\x12\x03\x12\05\n\t\n\x02\x08%\x12\
    \x03\x12\05\n\x08\n\x01\x08\x12\x03\x13\0Q\n\t\n\x02\x08\x0b\x12\x03\x13\
    \0Q\n\x08\n\x01\x08\x12\x03\x14\0\"\n\t\n\x02\x08\n\x12\x03\x14\0\"\n\
    \x08\n\x01\x08\x12\x03\x15\02\n\t\n\x02\x08\x08\x12\x03\x15\02\n\x08\n\
    \x01\x08\x12\x03\x16\0/\n\t\n\x02\x08\x01\x12\x03\x16\0/\n\x08\n\x01\x08\
    \x12\x03\x17\05\n\t\n\x02\x08)\x12\x03\x17\05\n\x08\n\x01\x08\x12\x03\
    \x18\04\n\t\n\x02\x08-\x12\x03\x18\04\n\x86\x04\n\x02\x04\0\x12\x04\"\04\
    \x01\x1a\xf9\x03\x20Feature\x20flags\x20supported\x20or\x20enabled\x20by\
    \x20a\x20client.\n\x20This\x20is\x20intended\x20to\x20be\x20sent\x20as\
    \x20part\x20of\x20request\x20metadata\x20to\x20assure\x20the\x20server\n\
    \x20that\x20certain\x20behaviors\x20are\x20safe\x20to\x20enable.\x20This\
    \x20proto\x20is\x20meant\x20to\x20be\n\x20serialized\x20and\x20websafe-b\
    ase64\x20encoded\x20under\x20the\x20`bigtable-features`\x20metadata\n\
    \x20key.\x20The\x20value\x20will\x20remain\x20constant\x20for\x20the\x20\
    lifetime\x20of\x20a\x20client\x20and\x20due\x20to\n\x20HTTP2's\x20HPACK\
    \x20compression,\x20the\x20request\x20overhead\x20will\x20be\x20tiny.\n\
    \x20This\x20is\x20an\x20internal\x20implementation\x20detail\x20and\x20s\
    hould\x20not\x20be\x20used\x20by\x20end\x20users\n\x20directly.\n\n\n\n\
    \x03\x04\0\x01\x12\x03\"\x08\x14\n\xa0\x01\n\x04\x04\0\x02\0\x12\x03%\
    \x02\x19\x1a\x92\x01\x20Notify\x20the\x20server\x20that\x20the\x20client\
    \x20supports\x20reverse\x20scans.\x20The\x20server\x20will\n\x20reject\
    \x20ReadRowsRequests\x20with\x20the\x20reverse\x20bit\x20set\x20when\x20\
    this\x20is\x20absent.\n\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03%\x02\x06\n\
    \x0c\n\x05\x04\0\x02\0\x01\x12\x03%\x07\x14\n\x0c\n\x05\x04\0\x02\0\x03\
    \x12\x03%\x17\x18\n\xc2\x01\n\x04\x04\0\x02\x01\x12\x03*\x02\"\x1a\xb4\
    \x01\x20Notify\x20the\x20server\x20that\x20the\x20client\x20enables\x20b\
    atch\x20write\x20flow\x20control\x20by\n\x20requesting\x20RateLimitInfo\
    \x20from\x20MutateRowsResponse.\x20Due\x20to\x20technical\x20reasons,\n\
    \x20this\x20disables\x20partial\x20retries.\n\n\x0c\n\x05\x04\0\x02\x01\
    \x05\x12\x03*\x02\x06\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03*\x07\x1d\n\
    \x0c\n\x05\x04\0\x02\x01\x03\x12\x03*\x20!\n\xa7\x01\n\x04\x04\0\x02\x02\
    \x12\x03/\x02#\x1a\x99\x01\x20Notify\x20the\x20server\x20that\x20the\x20\
    client\x20enables\x20batch\x20write\x20flow\x20control\x20by\n\x20reques\
    ting\x20RateLimitInfo\x20from\x20MutateRowsResponse.\x20With\x20partial\
    \x20retries\n\x20enabled.\n\n\x0c\n\x05\x04\0\x02\x02\x05\x12\x03/\x02\
    \x06\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03/\x07\x1e\n\x0c\n\x05\x04\0\
    \x02\x02\x03\x12\x03/!\"\n\x81\x01\n\x04\x04\0\x02\x03\x12\x033\x02&\x1a\
    t\x20Notify\x20the\x20server\x20that\x20the\x20client\x20supports\x20the\
    \x20last_scanned_row\x20field\n\x20in\x20ReadRowsResponse\x20for\x20long\
    -running\x20scans.\n\n\x0c\n\x05\x04\0\x02\x03\x05\x12\x033\x02\x06\n\
    \x0c\n\x05\x04\0\x02\x03\x01\x12\x033\x07!\n\x0c\n\x05\x04\0\x02\x03\x03\
    \x12\x033$%b\x06proto3\
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