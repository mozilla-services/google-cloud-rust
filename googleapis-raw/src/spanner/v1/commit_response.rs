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
//! Generated file from `google/spanner/v1/commit_response.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_28_0;

#[derive(PartialEq,Clone,Default)]
pub struct CommitResponse {
    // message fields
    pub commit_timestamp: ::protobuf::SingularPtrField<::protobuf::well_known_types::Timestamp>,
    pub commit_stats: ::protobuf::SingularPtrField<CommitResponse_CommitStats>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a CommitResponse {
    fn default() -> &'a CommitResponse {
        <CommitResponse as ::protobuf::Message>::default_instance()
    }
}

impl CommitResponse {
    pub fn new() -> CommitResponse {
        ::std::default::Default::default()
    }

    // .google.protobuf.Timestamp commit_timestamp = 1;


    pub fn get_commit_timestamp(&self) -> &::protobuf::well_known_types::Timestamp {
        self.commit_timestamp.as_ref().unwrap_or_else(|| <::protobuf::well_known_types::Timestamp as ::protobuf::Message>::default_instance())
    }
    pub fn clear_commit_timestamp(&mut self) {
        self.commit_timestamp.clear();
    }

    pub fn has_commit_timestamp(&self) -> bool {
        self.commit_timestamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_commit_timestamp(&mut self, v: ::protobuf::well_known_types::Timestamp) {
        self.commit_timestamp = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_commit_timestamp(&mut self) -> &mut ::protobuf::well_known_types::Timestamp {
        if self.commit_timestamp.is_none() {
            self.commit_timestamp.set_default();
        }
        self.commit_timestamp.as_mut().unwrap()
    }

    // Take field
    pub fn take_commit_timestamp(&mut self) -> ::protobuf::well_known_types::Timestamp {
        self.commit_timestamp.take().unwrap_or_else(|| ::protobuf::well_known_types::Timestamp::new())
    }

    // .google.spanner.v1.CommitResponse.CommitStats commit_stats = 2;


    pub fn get_commit_stats(&self) -> &CommitResponse_CommitStats {
        self.commit_stats.as_ref().unwrap_or_else(|| <CommitResponse_CommitStats as ::protobuf::Message>::default_instance())
    }
    pub fn clear_commit_stats(&mut self) {
        self.commit_stats.clear();
    }

    pub fn has_commit_stats(&self) -> bool {
        self.commit_stats.is_some()
    }

    // Param is passed by value, moved
    pub fn set_commit_stats(&mut self, v: CommitResponse_CommitStats) {
        self.commit_stats = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_commit_stats(&mut self) -> &mut CommitResponse_CommitStats {
        if self.commit_stats.is_none() {
            self.commit_stats.set_default();
        }
        self.commit_stats.as_mut().unwrap()
    }

    // Take field
    pub fn take_commit_stats(&mut self) -> CommitResponse_CommitStats {
        self.commit_stats.take().unwrap_or_else(|| CommitResponse_CommitStats::new())
    }
}

impl ::protobuf::Message for CommitResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.commit_timestamp {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.commit_stats {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.commit_timestamp)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.commit_stats)?;
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
        if let Some(ref v) = self.commit_timestamp.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.commit_stats.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.commit_timestamp.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.commit_stats.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

    fn new() -> CommitResponse {
        CommitResponse::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Timestamp>>(
                "commit_timestamp",
                |m: &CommitResponse| { &m.commit_timestamp },
                |m: &mut CommitResponse| { &mut m.commit_timestamp },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CommitResponse_CommitStats>>(
                "commit_stats",
                |m: &CommitResponse| { &m.commit_stats },
                |m: &mut CommitResponse| { &mut m.commit_stats },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<CommitResponse>(
                "CommitResponse",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static CommitResponse {
        static instance: ::protobuf::rt::LazyV2<CommitResponse> = ::protobuf::rt::LazyV2::INIT;
        instance.get(CommitResponse::new)
    }
}

impl ::protobuf::Clear for CommitResponse {
    fn clear(&mut self) {
        self.commit_timestamp.clear();
        self.commit_stats.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CommitResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CommitResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CommitResponse_CommitStats {
    // message fields
    pub mutation_count: i64,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a CommitResponse_CommitStats {
    fn default() -> &'a CommitResponse_CommitStats {
        <CommitResponse_CommitStats as ::protobuf::Message>::default_instance()
    }
}

impl CommitResponse_CommitStats {
    pub fn new() -> CommitResponse_CommitStats {
        ::std::default::Default::default()
    }

    // int64 mutation_count = 1;


    pub fn get_mutation_count(&self) -> i64 {
        self.mutation_count
    }
    pub fn clear_mutation_count(&mut self) {
        self.mutation_count = 0;
    }

    // Param is passed by value, moved
    pub fn set_mutation_count(&mut self, v: i64) {
        self.mutation_count = v;
    }
}

impl ::protobuf::Message for CommitResponse_CommitStats {
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
                    let tmp = is.read_int64()?;
                    self.mutation_count = tmp;
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
        if self.mutation_count != 0 {
            my_size += ::protobuf::rt::value_size(1, self.mutation_count, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.mutation_count != 0 {
            os.write_int64(1, self.mutation_count)?;
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

    fn new() -> CommitResponse_CommitStats {
        CommitResponse_CommitStats::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "mutation_count",
                |m: &CommitResponse_CommitStats| { &m.mutation_count },
                |m: &mut CommitResponse_CommitStats| { &mut m.mutation_count },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<CommitResponse_CommitStats>(
                "CommitResponse.CommitStats",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static CommitResponse_CommitStats {
        static instance: ::protobuf::rt::LazyV2<CommitResponse_CommitStats> = ::protobuf::rt::LazyV2::INIT;
        instance.get(CommitResponse_CommitStats::new)
    }
}

impl ::protobuf::Clear for CommitResponse_CommitStats {
    fn clear(&mut self) {
        self.mutation_count = 0;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CommitResponse_CommitStats {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CommitResponse_CommitStats {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n'google/spanner/v1/commit_response.proto\x12\x11google.spanner.v1\x1a\
    \x1fgoogle/protobuf/timestamp.proto\"\xdf\x01\n\x0eCommitResponse\x12E\n\
    \x10commit_timestamp\x18\x01\x20\x01(\x0b2\x1a.google.protobuf.Timestamp\
    R\x0fcommitTimestamp\x12P\n\x0ccommit_stats\x18\x02\x20\x01(\x0b2-.googl\
    e.spanner.v1.CommitResponse.CommitStatsR\x0bcommitStats\x1a4\n\x0bCommit\
    Stats\x12%\n\x0emutation_count\x18\x01\x20\x01(\x03R\rmutationCountB\xb6\
    \x01\n\x15com.google.spanner.v1B\x13CommitResponseProtoP\x01Z5cloud.goog\
    le.com/go/spanner/apiv1/spannerpb;spannerpb\xaa\x02\x17Google.Cloud.Span\
    ner.V1\xca\x02\x17Google\\Cloud\\Spanner\\V1\xea\x02\x1aGoogle::Cloud::S\
    panner::V1J\xe9\x0e\n\x06\x12\x04\x0e\01\x01\n\xbc\x04\n\x01\x0c\x12\x03\
    \x0e\0\x122\xb1\x04\x20Copyright\x202022\x20Google\x20LLC\n\n\x20License\
    d\x20under\x20the\x20Apache\x20License,\x20Version\x202.0\x20(the\x20\"L\
    icense\");\n\x20you\x20may\x20not\x20use\x20this\x20file\x20except\x20in\
    \x20compliance\x20with\x20the\x20License.\n\x20You\x20may\x20obtain\x20a\
    \x20copy\x20of\x20the\x20License\x20at\n\n\x20\x20\x20\x20\x20http://www\
    .apache.org/licenses/LICENSE-2.0\n\n\x20Unless\x20required\x20by\x20appl\
    icable\x20law\x20or\x20agreed\x20to\x20in\x20writing,\x20software\n\x20d\
    istributed\x20under\x20the\x20License\x20is\x20distributed\x20on\x20an\
    \x20\"AS\x20IS\"\x20BASIS,\n\x20WITHOUT\x20WARRANTIES\x20OR\x20CONDITION\
    S\x20OF\x20ANY\x20KIND,\x20either\x20express\x20or\x20implied.\n\x20See\
    \x20the\x20License\x20for\x20the\x20specific\x20language\x20governing\
    \x20permissions\x20and\n\x20limitations\x20under\x20the\x20License.\n\n\
    \x08\n\x01\x02\x12\x03\x10\0\x1a\n\t\n\x02\x03\0\x12\x03\x12\0)\n\x08\n\
    \x01\x08\x12\x03\x14\04\n\t\n\x02\x08%\x12\x03\x14\04\n\x08\n\x01\x08\
    \x12\x03\x15\0L\n\t\n\x02\x08\x0b\x12\x03\x15\0L\n\x08\n\x01\x08\x12\x03\
    \x16\0\"\n\t\n\x02\x08\n\x12\x03\x16\0\"\n\x08\n\x01\x08\x12\x03\x17\04\
    \n\t\n\x02\x08\x08\x12\x03\x17\04\n\x08\n\x01\x08\x12\x03\x18\0.\n\t\n\
    \x02\x08\x01\x12\x03\x18\0.\n\x08\n\x01\x08\x12\x03\x19\04\n\t\n\x02\x08\
    )\x12\x03\x19\04\n\x08\n\x01\x08\x12\x03\x1a\03\n\t\n\x02\x08-\x12\x03\
    \x1a\03\nJ\n\x02\x04\0\x12\x04\x1d\01\x01\x1a>\x20The\x20response\x20for\
    \x20[Commit][google.spanner.v1.Spanner.Commit].\n\n\n\n\x03\x04\0\x01\
    \x12\x03\x1d\x08\x16\n5\n\x04\x04\0\x03\0\x12\x04\x1f\x02(\x03\x1a'\x20A\
    dditional\x20statistics\x20about\x20a\x20commit.\n\n\x0c\n\x05\x04\0\x03\
    \0\x01\x12\x03\x1f\n\x15\n\xc3\x04\n\x06\x04\0\x03\0\x02\0\x12\x03'\x04\
    \x1d\x1a\xb3\x04\x20The\x20total\x20number\x20of\x20mutations\x20for\x20\
    the\x20transaction.\x20Knowing\x20the\n\x20`mutation_count`\x20value\x20\
    can\x20help\x20you\x20maximize\x20the\x20number\x20of\x20mutations\n\x20\
    in\x20a\x20transaction\x20and\x20minimize\x20the\x20number\x20of\x20API\
    \x20round\x20trips.\x20You\x20can\n\x20also\x20monitor\x20this\x20value\
    \x20to\x20prevent\x20transactions\x20from\x20exceeding\x20the\x20system\
    \n\x20[limit](https://cloud.google.com/spanner/quotas#limits_for_creatin\
    g_reading_updating_and_deleting_data).\n\x20If\x20the\x20number\x20of\
    \x20mutations\x20exceeds\x20the\x20limit,\x20the\x20server\x20returns\n\
    \x20[INVALID_ARGUMENT](https://cloud.google.com/spanner/docs/reference/r\
    est/v1/Code#ENUM_VALUES.INVALID_ARGUMENT).\n\n\x0e\n\x07\x04\0\x03\0\x02\
    \0\x05\x12\x03'\x04\t\n\x0e\n\x07\x04\0\x03\0\x02\0\x01\x12\x03'\n\x18\n\
    \x0e\n\x07\x04\0\x03\0\x02\0\x03\x12\x03'\x1b\x1c\nN\n\x04\x04\0\x02\0\
    \x12\x03+\x021\x1aA\x20The\x20Cloud\x20Spanner\x20timestamp\x20at\x20whi\
    ch\x20the\x20transaction\x20committed.\n\n\x0c\n\x05\x04\0\x02\0\x06\x12\
    \x03+\x02\x1b\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03+\x1c,\n\x0c\n\x05\x04\
    \0\x02\0\x03\x12\x03+/0\n\xc0\x01\n\x04\x04\0\x02\x01\x12\x030\x02\x1f\
    \x1a\xb2\x01\x20The\x20statistics\x20about\x20this\x20Commit.\x20Not\x20\
    returned\x20by\x20default.\n\x20For\x20more\x20information,\x20see\n\x20\
    [CommitRequest.return_commit_stats][google.spanner.v1.CommitRequest.retu\
    rn_commit_stats].\n\n\x0c\n\x05\x04\0\x02\x01\x06\x12\x030\x02\r\n\x0c\n\
    \x05\x04\0\x02\x01\x01\x12\x030\x0e\x1a\n\x0c\n\x05\x04\0\x02\x01\x03\
    \x12\x030\x1d\x1eb\x06proto3\
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
