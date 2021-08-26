// This file is generated by rust-protobuf 2.25.1. Do not edit
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
//! Generated file from `google/api/servicecontrol/v1/check_error.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_25_1;

#[derive(PartialEq,Clone,Default)]
pub struct CheckError {
    // message fields
    pub code: CheckError_Code,
    pub subject: ::std::string::String,
    pub detail: ::std::string::String,
    pub status: ::protobuf::SingularPtrField<super::status::Status>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a CheckError {
    fn default() -> &'a CheckError {
        <CheckError as ::protobuf::Message>::default_instance()
    }
}

impl CheckError {
    pub fn new() -> CheckError {
        ::std::default::Default::default()
    }

    // .google.api.servicecontrol.v1.CheckError.Code code = 1;


    pub fn get_code(&self) -> CheckError_Code {
        self.code
    }
    pub fn clear_code(&mut self) {
        self.code = CheckError_Code::ERROR_CODE_UNSPECIFIED;
    }

    // Param is passed by value, moved
    pub fn set_code(&mut self, v: CheckError_Code) {
        self.code = v;
    }

    // string subject = 4;


    pub fn get_subject(&self) -> &str {
        &self.subject
    }
    pub fn clear_subject(&mut self) {
        self.subject.clear();
    }

    // Param is passed by value, moved
    pub fn set_subject(&mut self, v: ::std::string::String) {
        self.subject = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_subject(&mut self) -> &mut ::std::string::String {
        &mut self.subject
    }

    // Take field
    pub fn take_subject(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.subject, ::std::string::String::new())
    }

    // string detail = 2;


    pub fn get_detail(&self) -> &str {
        &self.detail
    }
    pub fn clear_detail(&mut self) {
        self.detail.clear();
    }

    // Param is passed by value, moved
    pub fn set_detail(&mut self, v: ::std::string::String) {
        self.detail = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_detail(&mut self) -> &mut ::std::string::String {
        &mut self.detail
    }

    // Take field
    pub fn take_detail(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.detail, ::std::string::String::new())
    }

    // .google.rpc.Status status = 3;


    pub fn get_status(&self) -> &super::status::Status {
        self.status.as_ref().unwrap_or_else(|| <super::status::Status as ::protobuf::Message>::default_instance())
    }
    pub fn clear_status(&mut self) {
        self.status.clear();
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: super::status::Status) {
        self.status = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_status(&mut self) -> &mut super::status::Status {
        if self.status.is_none() {
            self.status.set_default();
        }
        self.status.as_mut().unwrap()
    }

    // Take field
    pub fn take_status(&mut self) -> super::status::Status {
        self.status.take().unwrap_or_else(|| super::status::Status::new())
    }
}

impl ::protobuf::Message for CheckError {
    fn is_initialized(&self) -> bool {
        for v in &self.status {
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
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.code, 1, &mut self.unknown_fields)?
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.subject)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.detail)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.status)?;
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
        if self.code != CheckError_Code::ERROR_CODE_UNSPECIFIED {
            my_size += ::protobuf::rt::enum_size(1, self.code);
        }
        if !self.subject.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.subject);
        }
        if !self.detail.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.detail);
        }
        if let Some(ref v) = self.status.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.code != CheckError_Code::ERROR_CODE_UNSPECIFIED {
            os.write_enum(1, ::protobuf::ProtobufEnum::value(&self.code))?;
        }
        if !self.subject.is_empty() {
            os.write_string(4, &self.subject)?;
        }
        if !self.detail.is_empty() {
            os.write_string(2, &self.detail)?;
        }
        if let Some(ref v) = self.status.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

    fn new() -> CheckError {
        CheckError::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<CheckError_Code>>(
                "code",
                |m: &CheckError| { &m.code },
                |m: &mut CheckError| { &mut m.code },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "subject",
                |m: &CheckError| { &m.subject },
                |m: &mut CheckError| { &mut m.subject },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "detail",
                |m: &CheckError| { &m.detail },
                |m: &mut CheckError| { &mut m.detail },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::status::Status>>(
                "status",
                |m: &CheckError| { &m.status },
                |m: &mut CheckError| { &mut m.status },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<CheckError>(
                "CheckError",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static CheckError {
        static instance: ::protobuf::rt::LazyV2<CheckError> = ::protobuf::rt::LazyV2::INIT;
        instance.get(CheckError::new)
    }
}

impl ::protobuf::Clear for CheckError {
    fn clear(&mut self) {
        self.code = CheckError_Code::ERROR_CODE_UNSPECIFIED;
        self.subject.clear();
        self.detail.clear();
        self.status.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CheckError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CheckError {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum CheckError_Code {
    ERROR_CODE_UNSPECIFIED = 0,
    NOT_FOUND = 5,
    PERMISSION_DENIED = 7,
    RESOURCE_EXHAUSTED = 8,
    SERVICE_NOT_ACTIVATED = 104,
    BILLING_DISABLED = 107,
    PROJECT_DELETED = 108,
    PROJECT_INVALID = 114,
    CONSUMER_INVALID = 125,
    IP_ADDRESS_BLOCKED = 109,
    REFERER_BLOCKED = 110,
    CLIENT_APP_BLOCKED = 111,
    API_TARGET_BLOCKED = 122,
    API_KEY_INVALID = 105,
    API_KEY_EXPIRED = 112,
    API_KEY_NOT_FOUND = 113,
    INVALID_CREDENTIAL = 123,
    NAMESPACE_LOOKUP_UNAVAILABLE = 300,
    SERVICE_STATUS_UNAVAILABLE = 301,
    BILLING_STATUS_UNAVAILABLE = 302,
    CLOUD_RESOURCE_MANAGER_BACKEND_UNAVAILABLE = 305,
}

impl ::protobuf::ProtobufEnum for CheckError_Code {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CheckError_Code> {
        match value {
            0 => ::std::option::Option::Some(CheckError_Code::ERROR_CODE_UNSPECIFIED),
            5 => ::std::option::Option::Some(CheckError_Code::NOT_FOUND),
            7 => ::std::option::Option::Some(CheckError_Code::PERMISSION_DENIED),
            8 => ::std::option::Option::Some(CheckError_Code::RESOURCE_EXHAUSTED),
            104 => ::std::option::Option::Some(CheckError_Code::SERVICE_NOT_ACTIVATED),
            107 => ::std::option::Option::Some(CheckError_Code::BILLING_DISABLED),
            108 => ::std::option::Option::Some(CheckError_Code::PROJECT_DELETED),
            114 => ::std::option::Option::Some(CheckError_Code::PROJECT_INVALID),
            125 => ::std::option::Option::Some(CheckError_Code::CONSUMER_INVALID),
            109 => ::std::option::Option::Some(CheckError_Code::IP_ADDRESS_BLOCKED),
            110 => ::std::option::Option::Some(CheckError_Code::REFERER_BLOCKED),
            111 => ::std::option::Option::Some(CheckError_Code::CLIENT_APP_BLOCKED),
            122 => ::std::option::Option::Some(CheckError_Code::API_TARGET_BLOCKED),
            105 => ::std::option::Option::Some(CheckError_Code::API_KEY_INVALID),
            112 => ::std::option::Option::Some(CheckError_Code::API_KEY_EXPIRED),
            113 => ::std::option::Option::Some(CheckError_Code::API_KEY_NOT_FOUND),
            123 => ::std::option::Option::Some(CheckError_Code::INVALID_CREDENTIAL),
            300 => ::std::option::Option::Some(CheckError_Code::NAMESPACE_LOOKUP_UNAVAILABLE),
            301 => ::std::option::Option::Some(CheckError_Code::SERVICE_STATUS_UNAVAILABLE),
            302 => ::std::option::Option::Some(CheckError_Code::BILLING_STATUS_UNAVAILABLE),
            305 => ::std::option::Option::Some(CheckError_Code::CLOUD_RESOURCE_MANAGER_BACKEND_UNAVAILABLE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [CheckError_Code] = &[
            CheckError_Code::ERROR_CODE_UNSPECIFIED,
            CheckError_Code::NOT_FOUND,
            CheckError_Code::PERMISSION_DENIED,
            CheckError_Code::RESOURCE_EXHAUSTED,
            CheckError_Code::SERVICE_NOT_ACTIVATED,
            CheckError_Code::BILLING_DISABLED,
            CheckError_Code::PROJECT_DELETED,
            CheckError_Code::PROJECT_INVALID,
            CheckError_Code::CONSUMER_INVALID,
            CheckError_Code::IP_ADDRESS_BLOCKED,
            CheckError_Code::REFERER_BLOCKED,
            CheckError_Code::CLIENT_APP_BLOCKED,
            CheckError_Code::API_TARGET_BLOCKED,
            CheckError_Code::API_KEY_INVALID,
            CheckError_Code::API_KEY_EXPIRED,
            CheckError_Code::API_KEY_NOT_FOUND,
            CheckError_Code::INVALID_CREDENTIAL,
            CheckError_Code::NAMESPACE_LOOKUP_UNAVAILABLE,
            CheckError_Code::SERVICE_STATUS_UNAVAILABLE,
            CheckError_Code::BILLING_STATUS_UNAVAILABLE,
            CheckError_Code::CLOUD_RESOURCE_MANAGER_BACKEND_UNAVAILABLE,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<CheckError_Code>("CheckError.Code", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for CheckError_Code {
}

impl ::std::default::Default for CheckError_Code {
    fn default() -> Self {
        CheckError_Code::ERROR_CODE_UNSPECIFIED
    }
}

impl ::protobuf::reflect::ProtobufValue for CheckError_Code {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n.google/api/servicecontrol/v1/check_error.proto\x12\x1cgoogle.api.serv\
    icecontrol.v1\x1a\x17google/rpc/status.proto\"\xcd\x05\n\nCheckError\x12\
    A\n\x04code\x18\x01\x20\x01(\x0e2-.google.api.servicecontrol.v1.CheckErr\
    or.CodeR\x04code\x12\x18\n\x07subject\x18\x04\x20\x01(\tR\x07subject\x12\
    \x16\n\x06detail\x18\x02\x20\x01(\tR\x06detail\x12*\n\x06status\x18\x03\
    \x20\x01(\x0b2\x12.google.rpc.StatusR\x06status\"\x9d\x04\n\x04Code\x12\
    \x1a\n\x16ERROR_CODE_UNSPECIFIED\x10\0\x12\r\n\tNOT_FOUND\x10\x05\x12\
    \x15\n\x11PERMISSION_DENIED\x10\x07\x12\x16\n\x12RESOURCE_EXHAUSTED\x10\
    \x08\x12\x19\n\x15SERVICE_NOT_ACTIVATED\x10h\x12\x14\n\x10BILLING_DISABL\
    ED\x10k\x12\x13\n\x0fPROJECT_DELETED\x10l\x12\x13\n\x0fPROJECT_INVALID\
    \x10r\x12\x14\n\x10CONSUMER_INVALID\x10}\x12\x16\n\x12IP_ADDRESS_BLOCKED\
    \x10m\x12\x13\n\x0fREFERER_BLOCKED\x10n\x12\x16\n\x12CLIENT_APP_BLOCKED\
    \x10o\x12\x16\n\x12API_TARGET_BLOCKED\x10z\x12\x13\n\x0fAPI_KEY_INVALID\
    \x10i\x12\x13\n\x0fAPI_KEY_EXPIRED\x10p\x12\x15\n\x11API_KEY_NOT_FOUND\
    \x10q\x12\x16\n\x12INVALID_CREDENTIAL\x10{\x12!\n\x1cNAMESPACE_LOOKUP_UN\
    AVAILABLE\x10\xac\x02\x12\x1f\n\x1aSERVICE_STATUS_UNAVAILABLE\x10\xad\
    \x02\x12\x1f\n\x1aBILLING_STATUS_UNAVAILABLE\x10\xae\x02\x12/\n*CLOUD_RE\
    SOURCE_MANAGER_BACKEND_UNAVAILABLE\x10\xb1\x02B\xea\x01\n\x20com.google.\
    api.servicecontrol.v1B\x0fCheckErrorProtoP\x01ZJgoogle.golang.org/genpro\
    to/googleapis/api/servicecontrol/v1;servicecontrol\xf8\x01\x01\xaa\x02\
    \x1eGoogle.Cloud.ServiceControl.V1\xca\x02\x1eGoogle\\Cloud\\ServiceCont\
    rol\\V1\xea\x02!Google::Cloud::ServiceControl::V1J\xbe!\n\x06\x12\x04\
    \x0e\0{\x01\n\xbc\x04\n\x01\x0c\x12\x03\x0e\0\x122\xb1\x04\x20Copyright\
    \x202021\x20Google\x20LLC\n\n\x20Licensed\x20under\x20the\x20Apache\x20L\
    icense,\x20Version\x202.0\x20(the\x20\"License\");\n\x20you\x20may\x20no\
    t\x20use\x20this\x20file\x20except\x20in\x20compliance\x20with\x20the\
    \x20License.\n\x20You\x20may\x20obtain\x20a\x20copy\x20of\x20the\x20Lice\
    nse\x20at\n\n\x20\x20\x20\x20\x20http://www.apache.org/licenses/LICENSE-\
    2.0\n\n\x20Unless\x20required\x20by\x20applicable\x20law\x20or\x20agreed\
    \x20to\x20in\x20writing,\x20software\n\x20distributed\x20under\x20the\
    \x20License\x20is\x20distributed\x20on\x20an\x20\"AS\x20IS\"\x20BASIS,\n\
    \x20WITHOUT\x20WARRANTIES\x20OR\x20CONDITIONS\x20OF\x20ANY\x20KIND,\x20e\
    ither\x20express\x20or\x20implied.\n\x20See\x20the\x20License\x20for\x20\
    the\x20specific\x20language\x20governing\x20permissions\x20and\n\x20limi\
    tations\x20under\x20the\x20License.\n\n\x08\n\x01\x02\x12\x03\x10\0%\n\t\
    \n\x02\x03\0\x12\x03\x12\0!\n\x08\n\x01\x08\x12\x03\x14\0\x1f\n\t\n\x02\
    \x08\x1f\x12\x03\x14\0\x1f\n\x08\n\x01\x08\x12\x03\x15\0;\n\t\n\x02\x08%\
    \x12\x03\x15\0;\n\x08\n\x01\x08\x12\x03\x16\0a\n\t\n\x02\x08\x0b\x12\x03\
    \x16\0a\n\x08\n\x01\x08\x12\x03\x17\0\"\n\t\n\x02\x08\n\x12\x03\x17\0\"\
    \n\x08\n\x01\x08\x12\x03\x18\00\n\t\n\x02\x08\x08\x12\x03\x18\00\n\x08\n\
    \x01\x08\x12\x03\x19\09\n\t\n\x02\x08\x01\x12\x03\x19\09\n\x08\n\x01\x08\
    \x12\x03\x1a\0;\n\t\n\x02\x08)\x12\x03\x1a\0;\n\x08\n\x01\x08\x12\x03\
    \x1b\0:\n\t\n\x02\x08-\x12\x03\x1b\0:\n\xa8\x01\n\x02\x04\0\x12\x04\x1f\
    \0{\x01\x1a\x9b\x01\x20Defines\x20the\x20errors\x20to\x20be\x20returned\
    \x20in\n\x20[google.api.servicecontrol.v1.CheckResponse.check_errors][go\
    ogle.api.servicecontrol.v1.CheckResponse.check_errors].\n\n\n\n\x03\x04\
    \0\x01\x12\x03\x1f\x08\x12\n0\n\x04\x04\0\x04\0\x12\x04!\x02g\x03\x1a\"\
    \x20Error\x20codes\x20for\x20Check\x20responses.\n\n\x0c\n\x05\x04\0\x04\
    \0\x01\x12\x03!\x07\x0b\n7\n\x06\x04\0\x04\0\x02\0\x12\x03#\x04\x1f\x1a(\
    \x20This\x20is\x20never\x20used\x20in\x20`CheckResponse`.\n\n\x0e\n\x07\
    \x04\0\x04\0\x02\0\x01\x12\x03#\x04\x1a\n\x0e\n\x07\x04\0\x04\0\x02\0\
    \x02\x12\x03#\x1d\x1e\n\xa5\x01\n\x06\x04\0\x04\0\x02\x01\x12\x03'\x04\
    \x12\x1a\x95\x01\x20The\x20consumer's\x20project\x20id,\x20network\x20co\
    ntainer,\x20or\x20resource\x20container\x20was\n\x20not\x20found.\x20Sam\
    e\x20as\x20[google.rpc.Code.NOT_FOUND][google.rpc.Code.NOT_FOUND].\n\n\
    \x0e\n\x07\x04\0\x04\0\x02\x01\x01\x12\x03'\x04\r\n\x0e\n\x07\x04\0\x04\
    \0\x02\x01\x02\x12\x03'\x10\x11\n\x9e\x01\n\x06\x04\0\x04\0\x02\x02\x12\
    \x03+\x04\x1a\x1a\x8e\x01\x20The\x20consumer\x20doesn't\x20have\x20acces\
    s\x20to\x20the\x20specified\x20resource.\n\x20Same\x20as\x20[google.rpc.\
    Code.PERMISSION_DENIED][google.rpc.Code.PERMISSION_DENIED].\n\n\x0e\n\
    \x07\x04\0\x04\0\x02\x02\x01\x12\x03+\x04\x15\n\x0e\n\x07\x04\0\x04\0\
    \x02\x02\x02\x12\x03+\x18\x19\nv\n\x06\x04\0\x04\0\x02\x03\x12\x03.\x04\
    \x1b\x1ag\x20Quota\x20check\x20failed.\x20Same\x20as\x20[google.rpc.Code\
    .RESOURCE_EXHAUSTED][google.rpc.Code.RESOURCE_EXHAUSTED].\n\n\x0e\n\x07\
    \x04\0\x04\0\x02\x03\x01\x12\x03.\x04\x16\n\x0e\n\x07\x04\0\x04\0\x02\
    \x03\x02\x12\x03.\x19\x1a\n;\n\x06\x04\0\x04\0\x02\x04\x12\x031\x04\x20\
    \x1a,\x20The\x20consumer\x20hasn't\x20activated\x20the\x20service.\n\n\
    \x0e\n\x07\x04\0\x04\0\x02\x04\x01\x12\x031\x04\x19\n\x0e\n\x07\x04\0\
    \x04\0\x02\x04\x02\x12\x031\x1c\x1f\nT\n\x06\x04\0\x04\0\x02\x05\x12\x03\
    4\x04\x1b\x1aE\x20The\x20consumer\x20cannot\x20access\x20the\x20service\
    \x20because\x20billing\x20is\x20disabled.\n\n\x0e\n\x07\x04\0\x04\0\x02\
    \x05\x01\x12\x034\x04\x14\n\x0e\n\x07\x04\0\x04\0\x02\x05\x02\x12\x034\
    \x17\x1a\nS\n\x06\x04\0\x04\0\x02\x06\x12\x037\x04\x1a\x1aD\x20The\x20co\
    nsumer's\x20project\x20has\x20been\x20marked\x20as\x20deleted\x20(soft\
    \x20deletion).\n\n\x0e\n\x07\x04\0\x04\0\x02\x06\x01\x12\x037\x04\x13\n\
    \x0e\n\x07\x04\0\x04\0\x02\x06\x02\x12\x037\x16\x19\nX\n\x06\x04\0\x04\0\
    \x02\x07\x12\x03:\x04\x1a\x1aI\x20The\x20consumer's\x20project\x20number\
    \x20or\x20id\x20does\x20not\x20represent\x20a\x20valid\x20project.\n\n\
    \x0e\n\x07\x04\0\x04\0\x02\x07\x01\x12\x03:\x04\x13\n\x0e\n\x07\x04\0\
    \x04\0\x02\x07\x02\x12\x03:\x16\x19\ne\n\x06\x04\0\x04\0\x02\x08\x12\x03\
    >\x04\x1b\x1aV\x20The\x20input\x20consumer\x20info\x20does\x20not\x20rep\
    resent\x20a\x20valid\x20consumer\x20folder\x20or\n\x20organization.\n\n\
    \x0e\n\x07\x04\0\x04\0\x02\x08\x01\x12\x03>\x04\x14\n\x0e\n\x07\x04\0\
    \x04\0\x02\x08\x02\x12\x03>\x17\x1a\n^\n\x06\x04\0\x04\0\x02\t\x12\x03B\
    \x04\x1d\x1aO\x20The\x20IP\x20address\x20of\x20the\x20consumer\x20is\x20\
    invalid\x20for\x20the\x20specific\x20consumer\n\x20project.\n\n\x0e\n\
    \x07\x04\0\x04\0\x02\t\x01\x12\x03B\x04\x16\n\x0e\n\x07\x04\0\x04\0\x02\
    \t\x02\x12\x03B\x19\x1c\nk\n\x06\x04\0\x04\0\x02\n\x12\x03F\x04\x1a\x1a\
    \\\x20The\x20referer\x20address\x20of\x20the\x20consumer\x20request\x20i\
    s\x20invalid\x20for\x20the\x20specific\n\x20consumer\x20project.\n\n\x0e\
    \n\x07\x04\0\x04\0\x02\n\x01\x12\x03F\x04\x13\n\x0e\n\x07\x04\0\x04\0\
    \x02\n\x02\x12\x03F\x16\x19\nn\n\x06\x04\0\x04\0\x02\x0b\x12\x03J\x04\
    \x1d\x1a_\x20The\x20client\x20application\x20of\x20the\x20consumer\x20re\
    quest\x20is\x20invalid\x20for\x20the\n\x20specific\x20consumer\x20projec\
    t.\n\n\x0e\n\x07\x04\0\x04\0\x02\x0b\x01\x12\x03J\x04\x16\n\x0e\n\x07\
    \x04\0\x04\0\x02\x0b\x02\x12\x03J\x19\x1c\na\n\x06\x04\0\x04\0\x02\x0c\
    \x12\x03N\x04\x1d\x1aR\x20The\x20API\x20targeted\x20by\x20this\x20reques\
    t\x20is\x20invalid\x20for\x20the\x20specified\x20consumer\n\x20project.\
    \n\n\x0e\n\x07\x04\0\x04\0\x02\x0c\x01\x12\x03N\x04\x16\n\x0e\n\x07\x04\
    \0\x04\0\x02\x0c\x02\x12\x03N\x19\x1c\n3\n\x06\x04\0\x04\0\x02\r\x12\x03\
    Q\x04\x1a\x1a$\x20The\x20consumer's\x20API\x20key\x20is\x20invalid.\n\n\
    \x0e\n\x07\x04\0\x04\0\x02\r\x01\x12\x03Q\x04\x13\n\x0e\n\x07\x04\0\x04\
    \0\x02\r\x02\x12\x03Q\x16\x19\n4\n\x06\x04\0\x04\0\x02\x0e\x12\x03T\x04\
    \x1a\x1a%\x20The\x20consumer's\x20API\x20Key\x20has\x20expired.\n\n\x0e\
    \n\x07\x04\0\x04\0\x02\x0e\x01\x12\x03T\x04\x13\n\x0e\n\x07\x04\0\x04\0\
    \x02\x0e\x02\x12\x03T\x16\x19\nG\n\x06\x04\0\x04\0\x02\x0f\x12\x03W\x04\
    \x1c\x1a8\x20The\x20consumer's\x20API\x20Key\x20was\x20not\x20found\x20i\
    n\x20config\x20record.\n\n\x0e\n\x07\x04\0\x04\0\x02\x0f\x01\x12\x03W\
    \x04\x15\n\x0e\n\x07\x04\0\x04\0\x02\x0f\x02\x12\x03W\x18\x1b\nC\n\x06\
    \x04\0\x04\0\x02\x10\x12\x03Z\x04\x1d\x1a4\x20The\x20credential\x20in\
    \x20the\x20request\x20can\x20not\x20be\x20verified.\n\n\x0e\n\x07\x04\0\
    \x04\0\x02\x10\x01\x12\x03Z\x04\x16\n\x0e\n\x07\x04\0\x04\0\x02\x10\x02\
    \x12\x03Z\x19\x1c\nT\n\x06\x04\0\x04\0\x02\x11\x12\x03]\x04'\x1aE\x20The\
    \x20backend\x20server\x20for\x20looking\x20up\x20project\x20id/number\
    \x20is\x20unavailable.\n\n\x0e\n\x07\x04\0\x04\0\x02\x11\x01\x12\x03]\
    \x04\x20\n\x0e\n\x07\x04\0\x04\0\x02\x11\x02\x12\x03]#&\nO\n\x06\x04\0\
    \x04\0\x02\x12\x12\x03`\x04%\x1a@\x20The\x20backend\x20server\x20for\x20\
    checking\x20service\x20status\x20is\x20unavailable.\n\n\x0e\n\x07\x04\0\
    \x04\0\x02\x12\x01\x12\x03`\x04\x1e\n\x0e\n\x07\x04\0\x04\0\x02\x12\x02\
    \x12\x03`!$\nO\n\x06\x04\0\x04\0\x02\x13\x12\x03c\x04%\x1a@\x20The\x20ba\
    ckend\x20server\x20for\x20checking\x20billing\x20status\x20is\x20unavail\
    able.\n\n\x0e\n\x07\x04\0\x04\0\x02\x13\x01\x12\x03c\x04\x1e\n\x0e\n\x07\
    \x04\0\x04\0\x02\x13\x02\x12\x03c!$\nF\n\x06\x04\0\x04\0\x02\x14\x12\x03\
    f\x045\x1a7\x20Cloud\x20Resource\x20Manager\x20backend\x20server\x20is\
    \x20unavailable.\n\n\x0e\n\x07\x04\0\x04\0\x02\x14\x01\x12\x03f\x04.\n\
    \x0e\n\x07\x04\0\x04\0\x02\x14\x02\x12\x03f14\n\x1e\n\x04\x04\0\x02\0\
    \x12\x03j\x02\x10\x1a\x11\x20The\x20error\x20code.\n\n\x0c\n\x05\x04\0\
    \x02\0\x06\x12\x03j\x02\x06\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03j\x07\x0b\
    \n\x0c\n\x05\x04\0\x02\0\x03\x12\x03j\x0e\x0f\n\xe5\x01\n\x04\x04\0\x02\
    \x01\x12\x03r\x02\x15\x1a\xd7\x01\x20Subject\x20to\x20whom\x20this\x20er\
    ror\x20applies.\x20See\x20the\x20specific\x20code\x20enum\x20for\x20more\
    \n\x20details\x20on\x20this\x20field.\x20For\x20example:\n\n\x20-\x20\"p\
    roject:<project-id\x20or\x20project-number>\"\n\x20-\x20\"folder:<folder\
    -id>\"\n\x20-\x20\"organization:<organization-id>\"\n\n\x0c\n\x05\x04\0\
    \x02\x01\x05\x12\x03r\x02\x08\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03r\t\
    \x10\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03r\x13\x14\nP\n\x04\x04\0\x02\
    \x02\x12\x03u\x02\x14\x1aC\x20Free-form\x20text\x20providing\x20details\
    \x20on\x20the\x20error\x20cause\x20of\x20the\x20error.\n\n\x0c\n\x05\x04\
    \0\x02\x02\x05\x12\x03u\x02\x08\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03u\t\
    \x0f\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03u\x12\x13\n\xa2\x01\n\x04\x04\
    \0\x02\x03\x12\x03z\x02\x1f\x1a\x94\x01\x20Contains\x20public\x20informa\
    tion\x20about\x20the\x20check\x20error.\x20If\x20available,\n\x20`status\
    .code`\x20will\x20be\x20non\x20zero\x20and\x20client\x20can\x20propagate\
    \x20it\x20out\x20as\x20public\n\x20error.\n\n\x0c\n\x05\x04\0\x02\x03\
    \x06\x12\x03z\x02\x13\n\x0c\n\x05\x04\0\x02\x03\x01\x12\x03z\x14\x1a\n\
    \x0c\n\x05\x04\0\x02\x03\x03\x12\x03z\x1d\x1eb\x06proto3\
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
