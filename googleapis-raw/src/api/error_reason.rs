// This file is generated by rust-protobuf 2.24.1. Do not edit
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
//! Generated file from `google/api/error_reason.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_24_1;

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ErrorReason {
    ERROR_REASON_UNSPECIFIED = 0,
    SERVICE_DISABLED = 1,
    BILLING_DISABLED = 2,
    API_KEY_INVALID = 3,
    API_KEY_SERVICE_BLOCKED = 4,
    API_KEY_HTTP_REFERRER_BLOCKED = 7,
    API_KEY_IP_ADDRESS_BLOCKED = 8,
    API_KEY_ANDROID_APP_BLOCKED = 9,
    API_KEY_IOS_APP_BLOCKED = 13,
    RATE_LIMIT_EXCEEDED = 5,
    RESOURCE_QUOTA_EXCEEDED = 6,
    LOCATION_TAX_POLICY_VIOLATED = 10,
    USER_PROJECT_DENIED = 11,
    CONSUMER_SUSPENDED = 12,
    CONSUMER_INVALID = 14,
    SECURITY_POLICY_VIOLATED = 15,
    ACCESS_TOKEN_EXPIRED = 16,
    ACCESS_TOKEN_SCOPE_INSUFFICIENT = 17,
    ACCOUNT_STATE_INVALID = 18,
    ACCESS_TOKEN_TYPE_UNSUPPORTED = 19,
}

impl ::protobuf::ProtobufEnum for ErrorReason {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ErrorReason> {
        match value {
            0 => ::std::option::Option::Some(ErrorReason::ERROR_REASON_UNSPECIFIED),
            1 => ::std::option::Option::Some(ErrorReason::SERVICE_DISABLED),
            2 => ::std::option::Option::Some(ErrorReason::BILLING_DISABLED),
            3 => ::std::option::Option::Some(ErrorReason::API_KEY_INVALID),
            4 => ::std::option::Option::Some(ErrorReason::API_KEY_SERVICE_BLOCKED),
            7 => ::std::option::Option::Some(ErrorReason::API_KEY_HTTP_REFERRER_BLOCKED),
            8 => ::std::option::Option::Some(ErrorReason::API_KEY_IP_ADDRESS_BLOCKED),
            9 => ::std::option::Option::Some(ErrorReason::API_KEY_ANDROID_APP_BLOCKED),
            13 => ::std::option::Option::Some(ErrorReason::API_KEY_IOS_APP_BLOCKED),
            5 => ::std::option::Option::Some(ErrorReason::RATE_LIMIT_EXCEEDED),
            6 => ::std::option::Option::Some(ErrorReason::RESOURCE_QUOTA_EXCEEDED),
            10 => ::std::option::Option::Some(ErrorReason::LOCATION_TAX_POLICY_VIOLATED),
            11 => ::std::option::Option::Some(ErrorReason::USER_PROJECT_DENIED),
            12 => ::std::option::Option::Some(ErrorReason::CONSUMER_SUSPENDED),
            14 => ::std::option::Option::Some(ErrorReason::CONSUMER_INVALID),
            15 => ::std::option::Option::Some(ErrorReason::SECURITY_POLICY_VIOLATED),
            16 => ::std::option::Option::Some(ErrorReason::ACCESS_TOKEN_EXPIRED),
            17 => ::std::option::Option::Some(ErrorReason::ACCESS_TOKEN_SCOPE_INSUFFICIENT),
            18 => ::std::option::Option::Some(ErrorReason::ACCOUNT_STATE_INVALID),
            19 => ::std::option::Option::Some(ErrorReason::ACCESS_TOKEN_TYPE_UNSUPPORTED),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ErrorReason] = &[
            ErrorReason::ERROR_REASON_UNSPECIFIED,
            ErrorReason::SERVICE_DISABLED,
            ErrorReason::BILLING_DISABLED,
            ErrorReason::API_KEY_INVALID,
            ErrorReason::API_KEY_SERVICE_BLOCKED,
            ErrorReason::API_KEY_HTTP_REFERRER_BLOCKED,
            ErrorReason::API_KEY_IP_ADDRESS_BLOCKED,
            ErrorReason::API_KEY_ANDROID_APP_BLOCKED,
            ErrorReason::API_KEY_IOS_APP_BLOCKED,
            ErrorReason::RATE_LIMIT_EXCEEDED,
            ErrorReason::RESOURCE_QUOTA_EXCEEDED,
            ErrorReason::LOCATION_TAX_POLICY_VIOLATED,
            ErrorReason::USER_PROJECT_DENIED,
            ErrorReason::CONSUMER_SUSPENDED,
            ErrorReason::CONSUMER_INVALID,
            ErrorReason::SECURITY_POLICY_VIOLATED,
            ErrorReason::ACCESS_TOKEN_EXPIRED,
            ErrorReason::ACCESS_TOKEN_SCOPE_INSUFFICIENT,
            ErrorReason::ACCOUNT_STATE_INVALID,
            ErrorReason::ACCESS_TOKEN_TYPE_UNSUPPORTED,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<ErrorReason>("ErrorReason", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for ErrorReason {
}

impl ::std::default::Default for ErrorReason {
    fn default() -> Self {
        ErrorReason::ERROR_REASON_UNSPECIFIED
    }
}

impl ::protobuf::reflect::ProtobufValue for ErrorReason {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1dgoogle/api/error_reason.proto\x12\ngoogle.api*\xc4\x04\n\x0bErrorR\
    eason\x12\x1c\n\x18ERROR_REASON_UNSPECIFIED\x10\0\x12\x14\n\x10SERVICE_D\
    ISABLED\x10\x01\x12\x14\n\x10BILLING_DISABLED\x10\x02\x12\x13\n\x0fAPI_K\
    EY_INVALID\x10\x03\x12\x1b\n\x17API_KEY_SERVICE_BLOCKED\x10\x04\x12!\n\
    \x1dAPI_KEY_HTTP_REFERRER_BLOCKED\x10\x07\x12\x1e\n\x1aAPI_KEY_IP_ADDRES\
    S_BLOCKED\x10\x08\x12\x1f\n\x1bAPI_KEY_ANDROID_APP_BLOCKED\x10\t\x12\x1b\
    \n\x17API_KEY_IOS_APP_BLOCKED\x10\r\x12\x17\n\x13RATE_LIMIT_EXCEEDED\x10\
    \x05\x12\x1b\n\x17RESOURCE_QUOTA_EXCEEDED\x10\x06\x12\x20\n\x1cLOCATION_\
    TAX_POLICY_VIOLATED\x10\n\x12\x17\n\x13USER_PROJECT_DENIED\x10\x0b\x12\
    \x16\n\x12CONSUMER_SUSPENDED\x10\x0c\x12\x14\n\x10CONSUMER_INVALID\x10\
    \x0e\x12\x1c\n\x18SECURITY_POLICY_VIOLATED\x10\x0f\x12\x18\n\x14ACCESS_T\
    OKEN_EXPIRED\x10\x10\x12#\n\x1fACCESS_TOKEN_SCOPE_INSUFFICIENT\x10\x11\
    \x12\x19\n\x15ACCOUNT_STATE_INVALID\x10\x12\x12!\n\x1dACCESS_TOKEN_TYPE_\
    UNSUPPORTED\x10\x13Bp\n\x0ecom.google.apiB\x10ErrorReasonProtoP\x01ZCgoo\
    gle.golang.org/genproto/googleapis/api/error_reason;error_reason\xa2\x02\
    \x04GAPIJ\xcbq\n\x07\x12\x05\x0e\0\x8c\x03\x01\n\xbc\x04\n\x01\x0c\x12\
    \x03\x0e\0\x122\xb1\x04\x20Copyright\x202020\x20Google\x20LLC\n\n\x20Lic\
    ensed\x20under\x20the\x20Apache\x20License,\x20Version\x202.0\x20(the\
    \x20\"License\");\n\x20you\x20may\x20not\x20use\x20this\x20file\x20excep\
    t\x20in\x20compliance\x20with\x20the\x20License.\n\x20You\x20may\x20obta\
    in\x20a\x20copy\x20of\x20the\x20License\x20at\n\n\x20\x20\x20\x20\x20htt\
    p://www.apache.org/licenses/LICENSE-2.0\n\n\x20Unless\x20required\x20by\
    \x20applicable\x20law\x20or\x20agreed\x20to\x20in\x20writing,\x20softwar\
    e\n\x20distributed\x20under\x20the\x20License\x20is\x20distributed\x20on\
    \x20an\x20\"AS\x20IS\"\x20BASIS,\n\x20WITHOUT\x20WARRANTIES\x20OR\x20CON\
    DITIONS\x20OF\x20ANY\x20KIND,\x20either\x20express\x20or\x20implied.\n\
    \x20See\x20the\x20License\x20for\x20the\x20specific\x20language\x20gover\
    ning\x20permissions\x20and\n\x20limitations\x20under\x20the\x20License.\
    \n\n\x08\n\x01\x02\x12\x03\x10\0\x13\n\x08\n\x01\x08\x12\x03\x12\0Z\n\t\
    \n\x02\x08\x0b\x12\x03\x12\0Z\n\x08\n\x01\x08\x12\x03\x13\0\"\n\t\n\x02\
    \x08\n\x12\x03\x13\0\"\n\x08\n\x01\x08\x12\x03\x14\01\n\t\n\x02\x08\x08\
    \x12\x03\x14\01\n\x08\n\x01\x08\x12\x03\x15\0'\n\t\n\x02\x08\x01\x12\x03\
    \x15\0'\n\x08\n\x01\x08\x12\x03\x16\0\"\n\t\n\x02\x08$\x12\x03\x16\0\"\n\
    \xbf\x05\n\x02\x05\0\x12\x05\"\0\x8c\x03\x01\x1a\xb1\x05\x20Defines\x20t\
    he\x20supported\x20values\x20for\x20`google.rpc.ErrorInfo.reason`\x20for\
    \x20the\n\x20`googleapis.com`\x20error\x20domain.\x20This\x20error\x20do\
    main\x20is\x20reserved\x20for\x20[Service\n\x20Infrastructure](https://c\
    loud.google.com/service-infrastructure/docs/overview).\n\x20For\x20each\
    \x20error\x20info\x20of\x20this\x20domain,\x20the\x20metadata\x20key\x20\
    \"service\"\x20refers\x20to\x20the\n\x20logical\x20identifier\x20of\x20a\
    n\x20API\x20service,\x20such\x20as\x20\"pubsub.googleapis.com\".\x20The\
    \n\x20\"consumer\"\x20refers\x20to\x20the\x20entity\x20that\x20consumes\
    \x20an\x20API\x20Service.\x20It\x20typically\x20is\n\x20a\x20Google\x20p\
    roject\x20that\x20owns\x20the\x20client\x20application\x20or\x20the\x20s\
    erver\x20resource,\n\x20such\x20as\x20\"projects/123\".\x20Other\x20meta\
    data\x20keys\x20are\x20specific\x20to\x20each\x20error\n\x20reason.\x20F\
    or\x20more\x20information,\x20see\x20the\x20definition\x20of\x20the\x20s\
    pecific\x20error\n\x20reason.\n\n\n\n\x03\x05\0\x01\x12\x03\"\x05\x10\n-\
    \n\x04\x05\0\x02\0\x12\x03$\x02\x1f\x1a\x20\x20Do\x20not\x20use\x20this\
    \x20default\x20value.\n\n\x0c\n\x05\x05\0\x02\0\x01\x12\x03$\x02\x1a\n\
    \x0c\n\x05\x05\0\x02\0\x02\x12\x03$\x1d\x1e\n\xde\x03\n\x04\x05\0\x02\
    \x01\x12\x035\x02\x17\x1a\xd0\x03\x20The\x20request\x20is\x20calling\x20\
    a\x20disabled\x20service\x20for\x20a\x20consumer.\n\n\x20Example\x20of\
    \x20an\x20ErrorInfo\x20when\x20the\x20consumer\x20\"projects/123\"\x20co\
    ntacting\n\x20\"pubsub.googleapis.com\"\x20service\x20which\x20is\x20dis\
    abled:\n\n\x20\x20\x20\x20\x20{\x20\"reason\":\x20\"SERVICE_DISABLED\",\
    \n\x20\x20\x20\x20\x20\x20\x20\"domain\":\x20\"googleapis.com\",\n\x20\
    \x20\x20\x20\x20\x20\x20\"metadata\":\x20{\n\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\"consumer\":\x20\"projects/123\",\n\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\"service\":\x20\"pubsub.googleapis.com\"\n\x20\x20\x20\x20\x20\
    \x20\x20}\n\x20\x20\x20\x20\x20}\n\n\x20This\x20response\x20indicates\
    \x20the\x20\"pubsub.googleapis.com\"\x20has\x20been\x20disabled\x20in\n\
    \x20\"projects/123\".\n\n\x0c\n\x05\x05\0\x02\x01\x01\x12\x035\x02\x12\n\
    \x0c\n\x05\x05\0\x02\x01\x02\x12\x035\x15\x16\n\xf6\x03\n\x04\x05\0\x02\
    \x02\x12\x03F\x02\x17\x1a\xe8\x03\x20The\x20request\x20whose\x20associat\
    ed\x20billing\x20account\x20is\x20disabled.\n\n\x20Example\x20of\x20an\
    \x20ErrorInfo\x20when\x20the\x20consumer\x20\"projects/123\"\x20fails\
    \x20to\x20contact\n\x20\"pubsub.googleapis.com\"\x20service\x20because\
    \x20the\x20associated\x20billing\x20account\x20is\n\x20disabled:\n\n\x20\
    \x20\x20\x20\x20{\x20\"reason\":\x20\"BILLING_DISABLED\",\n\x20\x20\x20\
    \x20\x20\x20\x20\"domain\":\x20\"googleapis.com\",\n\x20\x20\x20\x20\x20\
    \x20\x20\"metadata\":\x20{\n\x20\x20\x20\x20\x20\x20\x20\x20\x20\"consum\
    er\":\x20\"projects/123\",\n\x20\x20\x20\x20\x20\x20\x20\x20\x20\"servic\
    e\":\x20\"pubsub.googleapis.com\"\n\x20\x20\x20\x20\x20\x20\x20}\n\x20\
    \x20\x20\x20\x20}\n\n\x20This\x20response\x20indicates\x20the\x20billing\
    \x20account\x20associated\x20has\x20been\x20disabled.\n\n\x0c\n\x05\x05\
    \0\x02\x02\x01\x12\x03F\x02\x12\n\x0c\n\x05\x05\0\x02\x02\x02\x12\x03F\
    \x15\x16\n\xda\x03\n\x04\x05\0\x02\x03\x12\x03U\x02\x16\x1a\xcc\x03\x20T\
    he\x20request\x20is\x20denied\x20because\x20the\x20provided\x20[API\n\
    \x20key](https://cloud.google.com/docs/authentication/api-keys)\x20is\
    \x20invalid.\x20It\n\x20may\x20be\x20in\x20a\x20bad\x20format,\x20cannot\
    \x20be\x20found,\x20or\x20has\x20been\x20expired).\n\n\x20Example\x20of\
    \x20an\x20ErrorInfo\x20when\x20the\x20request\x20is\x20contacting\n\x20\
    \"storage.googleapis.com\"\x20service\x20with\x20an\x20invalid\x20API\
    \x20key:\n\n\x20\x20\x20\x20\x20{\x20\"reason\":\x20\"API_KEY_INVALID\",\
    \n\x20\x20\x20\x20\x20\x20\x20\"domain\":\x20\"googleapis.com\",\n\x20\
    \x20\x20\x20\x20\x20\x20\"metadata\":\x20{\n\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\"service\":\x20\"storage.googleapis.com\",\n\x20\x20\x20\x20\
    \x20\x20\x20}\n\x20\x20\x20\x20\x20}\n\n\x0c\n\x05\x05\0\x02\x03\x01\x12\
    \x03U\x02\x11\n\x0c\n\x05\x05\0\x02\x03\x02\x12\x03U\x14\x15\n\x8f\x04\n\
    \x04\x05\0\x02\x04\x12\x03e\x02\x1e\x1a\x81\x04\x20The\x20request\x20is\
    \x20denied\x20because\x20it\x20violates\x20[API\x20key\x20API\n\x20restr\
    ictions](https://cloud.google.com/docs/authentication/api-keys#adding_ap\
    i_restrictions).\n\n\x20Example\x20of\x20an\x20ErrorInfo\x20when\x20the\
    \x20consumer\x20\"projects/123\"\x20fails\x20to\x20call\x20the\n\x20\"st\
    orage.googleapis.com\"\x20service\x20because\x20this\x20service\x20is\
    \x20restricted\x20in\x20the\n\x20API\x20key:\n\n\x20\x20\x20\x20\x20{\
    \x20\"reason\":\x20\"API_KEY_SERVICE_BLOCKED\",\n\x20\x20\x20\x20\x20\
    \x20\x20\"domain\":\x20\"googleapis.com\",\n\x20\x20\x20\x20\x20\x20\x20\
    \"metadata\":\x20{\n\x20\x20\x20\x20\x20\x20\x20\x20\x20\"consumer\":\
    \x20\"projects/123\",\n\x20\x20\x20\x20\x20\x20\x20\x20\x20\"service\":\
    \x20\"storage.googleapis.com\"\n\x20\x20\x20\x20\x20\x20\x20}\n\x20\x20\
    \x20\x20\x20}\n\n\x0c\n\x05\x05\0\x02\x04\x01\x12\x03e\x02\x19\n\x0c\n\
    \x05\x05\0\x02\x04\x02\x12\x03e\x1c\x1d\n\xae\x04\n\x04\x05\0\x02\x05\
    \x12\x03u\x02$\x1a\xa0\x04\x20The\x20request\x20is\x20denied\x20because\
    \x20it\x20violates\x20[API\x20key\x20HTTP\n\x20restrictions](https://clo\
    ud.google.com/docs/authentication/api-keys#adding_http_restrictions).\n\
    \n\x20Example\x20of\x20an\x20ErrorInfo\x20when\x20the\x20consumer\x20\"p\
    rojects/123\"\x20fails\x20to\x20call\n\x20\"storage.googleapis.com\"\x20\
    service\x20because\x20the\x20http\x20referrer\x20of\x20the\x20request\n\
    \x20violates\x20API\x20key\x20HTTP\x20restrictions:\n\n\x20\x20\x20\x20\
    \x20{\x20\"reason\":\x20\"API_KEY_HTTP_REFERRER_BLOCKED\",\n\x20\x20\x20\
    \x20\x20\x20\x20\"domain\":\x20\"googleapis.com\",\n\x20\x20\x20\x20\x20\
    \x20\x20\"metadata\":\x20{\n\x20\x20\x20\x20\x20\x20\x20\x20\x20\"consum\
    er\":\x20\"projects/123\",\n\x20\x20\x20\x20\x20\x20\x20\x20\x20\"servic\
    e\":\x20\"storage.googleapis.com\",\n\x20\x20\x20\x20\x20\x20\x20}\n\x20\
    \x20\x20\x20\x20}\n\n\x0c\n\x05\x05\0\x02\x05\x01\x12\x03u\x02\x1f\n\x0c\
    \n\x05\x05\0\x02\x05\x02\x12\x03u\"#\n\xbb\x04\n\x04\x05\0\x02\x06\x12\
    \x04\x85\x01\x02!\x1a\xac\x04\x20The\x20request\x20is\x20denied\x20becau\
    se\x20it\x20violates\x20[API\x20key\x20IP\x20address\n\x20restrictions](\
    https://cloud.google.com/docs/authentication/api-keys#adding_application\
    _restrictions).\n\n\x20Example\x20of\x20an\x20ErrorInfo\x20when\x20the\
    \x20consumer\x20\"projects/123\"\x20fails\x20to\x20call\n\x20\"storage.g\
    oogleapis.com\"\x20service\x20because\x20the\x20caller\x20IP\x20of\x20th\
    e\x20request\n\x20violates\x20API\x20key\x20IP\x20address\x20restriction\
    s:\n\n\x20\x20\x20\x20\x20{\x20\"reason\":\x20\"API_KEY_IP_ADDRESS_BLOCK\
    ED\",\n\x20\x20\x20\x20\x20\x20\x20\"domain\":\x20\"googleapis.com\",\n\
    \x20\x20\x20\x20\x20\x20\x20\"metadata\":\x20{\n\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\"consumer\":\x20\"projects/123\",\n\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\"service\":\x20\"storage.googleapis.com\",\n\x20\x20\x20\
    \x20\x20\x20\x20}\n\x20\x20\x20\x20\x20}\n\n\r\n\x05\x05\0\x02\x06\x01\
    \x12\x04\x85\x01\x02\x1c\n\r\n\x05\x05\0\x02\x06\x02\x12\x04\x85\x01\x1f\
    \x20\n\xd6\x04\n\x04\x05\0\x02\x07\x12\x04\x95\x01\x02\"\x1a\xc7\x04\x20\
    The\x20request\x20is\x20denied\x20because\x20it\x20violates\x20[API\x20k\
    ey\x20Android\x20application\n\x20restrictions](https://cloud.google.com\
    /docs/authentication/api-keys#adding_application_restrictions).\n\n\x20E\
    xample\x20of\x20an\x20ErrorInfo\x20when\x20the\x20consumer\x20\"projects\
    /123\"\x20fails\x20to\x20call\n\x20\"storage.googleapis.com\"\x20service\
    \x20because\x20the\x20request\x20from\x20the\x20Android\x20apps\n\x20vio\
    lates\x20the\x20API\x20key\x20Android\x20application\x20restrictions:\n\
    \n\x20\x20\x20\x20\x20{\x20\"reason\":\x20\"API_KEY_ANDROID_APP_BLOCKED\
    \",\n\x20\x20\x20\x20\x20\x20\x20\"domain\":\x20\"googleapis.com\",\n\
    \x20\x20\x20\x20\x20\x20\x20\"metadata\":\x20{\n\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\"consumer\":\x20\"projects/123\",\n\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\"service\":\x20\"storage.googleapis.com\"\n\x20\x20\x20\x20\
    \x20\x20\x20}\n\x20\x20\x20\x20\x20}\n\n\r\n\x05\x05\0\x02\x07\x01\x12\
    \x04\x95\x01\x02\x1d\n\r\n\x05\x05\0\x02\x07\x02\x12\x04\x95\x01\x20!\n\
    \xc6\x04\n\x04\x05\0\x02\x08\x12\x04\xa5\x01\x02\x1f\x1a\xb7\x04\x20The\
    \x20request\x20is\x20denied\x20because\x20it\x20violates\x20[API\x20key\
    \x20iOS\x20application\n\x20restrictions](https://cloud.google.com/docs/\
    authentication/api-keys#adding_application_restrictions).\n\n\x20Example\
    \x20of\x20an\x20ErrorInfo\x20when\x20the\x20consumer\x20\"projects/123\"\
    \x20fails\x20to\x20call\n\x20\"storage.googleapis.com\"\x20service\x20be\
    cause\x20the\x20request\x20from\x20the\x20iOS\x20apps\n\x20violates\x20t\
    he\x20API\x20key\x20iOS\x20application\x20restrictions:\n\n\x20\x20\x20\
    \x20\x20{\x20\"reason\":\x20\"API_KEY_IOS_APP_BLOCKED\",\n\x20\x20\x20\
    \x20\x20\x20\x20\"domain\":\x20\"googleapis.com\",\n\x20\x20\x20\x20\x20\
    \x20\x20\"metadata\":\x20{\n\x20\x20\x20\x20\x20\x20\x20\x20\x20\"consum\
    er\":\x20\"projects/123\",\n\x20\x20\x20\x20\x20\x20\x20\x20\x20\"servic\
    e\":\x20\"storage.googleapis.com\"\n\x20\x20\x20\x20\x20\x20\x20}\n\x20\
    \x20\x20\x20\x20}\n\n\r\n\x05\x05\0\x02\x08\x01\x12\x04\xa5\x01\x02\x19\
    \n\r\n\x05\x05\0\x02\x08\x02\x12\x04\xa5\x01\x1c\x1e\n\x80\n\n\x04\x05\0\
    \x02\t\x12\x04\xc8\x01\x02\x1a\x1a\xf1\t\x20The\x20request\x20is\x20deni\
    ed\x20because\x20there\x20is\x20not\x20enough\x20rate\x20quota\x20for\
    \x20the\n\x20consumer.\n\n\x20Example\x20of\x20an\x20ErrorInfo\x20when\
    \x20the\x20consumer\x20\"projects/123\"\x20fails\x20to\x20contact\n\x20\
    \"pubsub.googleapis.com\"\x20service\x20because\x20consumer's\x20rate\
    \x20quota\x20usage\x20has\n\x20reached\x20the\x20maximum\x20value\x20set\
    \x20for\x20the\x20quota\x20limit\n\x20\"ReadsPerMinutePerProject\"\x20on\
    \x20the\x20quota\x20metric\n\x20\"pubsub.googleapis.com/read_requests\":\
    \n\n\x20\x20\x20\x20\x20{\x20\"reason\":\x20\"RATE_LIMIT_EXCEEDED\",\n\
    \x20\x20\x20\x20\x20\x20\x20\"domain\":\x20\"googleapis.com\",\n\x20\x20\
    \x20\x20\x20\x20\x20\"metadata\":\x20{\n\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\"consumer\":\x20\"projects/123\",\n\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\"service\":\x20\"pubsub.googleapis.com\",\n\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\"quota_metric\":\x20\"pubsub.googleapis.com/read_requests\"\
    ,\n\x20\x20\x20\x20\x20\x20\x20\x20\x20\"quota_limit\":\x20\"ReadsPerMin\
    utePerProject\"\n\x20\x20\x20\x20\x20\x20\x20}\n\x20\x20\x20\x20\x20}\n\
    \n\x20Example\x20of\x20an\x20ErrorInfo\x20when\x20the\x20consumer\x20\"p\
    rojects/123\"\x20checks\x20quota\x20on\n\x20the\x20service\x20\"dataflow\
    .googleapis.com\"\x20and\x20hits\x20the\x20organization\x20quota\n\x20li\
    mit\x20\"DefaultRequestsPerMinutePerOrganization\"\x20on\x20the\x20metri\
    c\n\x20\"dataflow.googleapis.com/default_requests\".\n\n\x20\x20\x20\x20\
    \x20{\x20\"reason\":\x20\"RATE_LIMIT_EXCEEDED\",\n\x20\x20\x20\x20\x20\
    \x20\x20\"domain\":\x20\"googleapis.com\",\n\x20\x20\x20\x20\x20\x20\x20\
    \"metadata\":\x20{\n\x20\x20\x20\x20\x20\x20\x20\x20\x20\"consumer\":\
    \x20\"projects/123\",\n\x20\x20\x20\x20\x20\x20\x20\x20\x20\"service\":\
    \x20\"dataflow.googleapis.com\",\n\x20\x20\x20\x20\x20\x20\x20\x20\x20\"\
    quota_metric\":\x20\"dataflow.googleapis.com/default_requests\",\n\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\"quota_limit\":\x20\"DefaultRequestsPer\
    MinutePerOrganization\"\n\x20\x20\x20\x20\x20\x20\x20}\n\x20\x20\x20\x20\
    \x20}\n\n\r\n\x05\x05\0\x02\t\x01\x12\x04\xc8\x01\x02\x15\n\r\n\x05\x05\
    \0\x02\t\x02\x12\x04\xc8\x01\x18\x19\n\xc0\t\n\x04\x05\0\x02\n\x12\x04\
    \xea\x01\x02\x1e\x1a\xb1\t\x20The\x20request\x20is\x20denied\x20because\
    \x20there\x20is\x20not\x20enough\x20resource\x20quota\x20for\x20the\n\
    \x20consumer.\n\n\x20Example\x20of\x20an\x20ErrorInfo\x20when\x20the\x20\
    consumer\x20\"projects/123\"\x20fails\x20to\x20contact\n\x20\"compute.go\
    ogleapis.com\"\x20service\x20because\x20consumer's\x20resource\x20quota\
    \x20usage\n\x20has\x20reached\x20the\x20maximum\x20value\x20set\x20for\
    \x20the\x20quota\x20limit\x20\"VMsPerProject\"\n\x20on\x20the\x20quota\
    \x20metric\x20\"compute.googleapis.com/vms\":\n\n\x20\x20\x20\x20\x20{\
    \x20\"reason\":\x20\"RESOURCE_QUOTA_EXCEEDED\",\n\x20\x20\x20\x20\x20\
    \x20\x20\"domain\":\x20\"googleapis.com\",\n\x20\x20\x20\x20\x20\x20\x20\
    \"metadata\":\x20{\n\x20\x20\x20\x20\x20\x20\x20\x20\x20\"consumer\":\
    \x20\"projects/123\",\n\x20\x20\x20\x20\x20\x20\x20\x20\x20\"service\":\
    \x20\"compute.googleapis.com\",\n\x20\x20\x20\x20\x20\x20\x20\x20\x20\"q\
    uota_metric\":\x20\"compute.googleapis.com/vms\",\n\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\"quota_limit\":\x20\"VMsPerProject\"\n\x20\x20\x20\x20\
    \x20\x20\x20}\n\x20\x20\x20\x20\x20}\n\n\x20Example\x20of\x20an\x20Error\
    Info\x20when\x20the\x20consumer\x20\"projects/123\"\x20checks\x20resourc\
    e\n\x20quota\x20on\x20the\x20service\x20\"dataflow.googleapis.com\"\x20a\
    nd\x20hits\x20the\x20organization\n\x20quota\x20limit\x20\"jobs-per-orga\
    nization\"\x20on\x20the\x20metric\n\x20\"dataflow.googleapis.com/job_cou\
    nt\".\n\n\x20\x20\x20\x20\x20{\x20\"reason\":\x20\"RESOURCE_QUOTA_EXCEED\
    ED\",\n\x20\x20\x20\x20\x20\x20\x20\"domain\":\x20\"googleapis.com\",\n\
    \x20\x20\x20\x20\x20\x20\x20\"metadata\":\x20{\n\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\"consumer\":\x20\"projects/123\",\n\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\"service\":\x20\"dataflow.googleapis.com\",\n\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\"quota_metric\":\x20\"dataflow.googleapis.com/j\
    ob_count\",\n\x20\x20\x20\x20\x20\x20\x20\x20\x20\"quota_limit\":\x20\"j\
    obs-per-organization\"\n\x20\x20\x20\x20\x20\x20\x20}\n\x20\x20\x20\x20\
    \x20}\n\n\r\n\x05\x05\0\x02\n\x01\x12\x04\xea\x01\x02\x19\n\r\n\x05\x05\
    \0\x02\n\x02\x12\x04\xea\x01\x1c\x1d\n\xdd\x05\n\x04\x05\0\x02\x0b\x12\
    \x04\xff\x01\x02$\x1a\xce\x05\x20The\x20request\x20whose\x20associated\
    \x20billing\x20account\x20address\x20is\x20in\x20a\x20tax\x20restricted\
    \n\x20location,\x20violates\x20the\x20local\x20tax\x20restrictions\x20wh\
    en\x20creating\x20resources\x20in\n\x20the\x20restricted\x20region.\n\n\
    \x20Example\x20of\x20an\x20ErrorInfo\x20when\x20creating\x20the\x20Cloud\
    \x20Storage\x20Bucket\x20in\x20the\n\x20container\x20\"projects/123\"\
    \x20under\x20a\x20tax\x20restricted\x20region\n\x20\"locations/asia-nort\
    heast3\":\n\n\x20\x20\x20\x20\x20{\x20\"reason\":\x20\"LOCATION_TAX_POLI\
    CY_VIOLATED\",\n\x20\x20\x20\x20\x20\x20\x20\"domain\":\x20\"googleapis.\
    com\",\n\x20\x20\x20\x20\x20\x20\x20\"metadata\":\x20{\n\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\"consumer\":\x20\"projects/123\",\n\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\"service\":\x20\"storage.googleapis.com\",\n\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\"location\":\x20\"locations/asia-northe\
    ast3\"\n\x20\x20\x20\x20\x20\x20\x20}\n\x20\x20\x20\x20\x20}\n\n\x20This\
    \x20response\x20indicates\x20creating\x20the\x20Cloud\x20Storage\x20Buck\
    et\x20in\n\x20\"locations/asia-northeast3\"\x20violates\x20the\x20locati\
    on\x20tax\x20restriction.\n\n\r\n\x05\x05\0\x02\x0b\x01\x12\x04\xff\x01\
    \x02\x1e\n\r\n\x05\x05\0\x02\x0b\x02\x12\x04\xff\x01!#\n\xda\x04\n\x04\
    \x05\0\x02\x0c\x12\x04\x90\x02\x02\x1b\x1a\xcb\x04\x20The\x20request\x20\
    is\x20denied\x20because\x20the\x20caller\x20does\x20not\x20have\x20requi\
    red\x20permission\n\x20on\x20the\x20user\x20project\x20\"projects/123\"\
    \x20or\x20the\x20user\x20project\x20is\x20invalid.\x20For\x20more\n\x20i\
    nformation,\x20check\x20the\x20[userProject\x20System\n\x20Parameters](h\
    ttps://cloud.google.com/apis/docs/system-parameters).\n\n\x20Example\x20\
    of\x20an\x20ErrorInfo\x20when\x20the\x20caller\x20is\x20calling\x20Cloud\
    \x20Storage\x20service\n\x20with\x20insufficient\x20permissions\x20on\
    \x20the\x20user\x20project:\n\n\x20\x20\x20\x20\x20{\x20\"reason\":\x20\
    \"USER_PROJECT_DENIED\",\n\x20\x20\x20\x20\x20\x20\x20\"domain\":\x20\"g\
    oogleapis.com\",\n\x20\x20\x20\x20\x20\x20\x20\"metadata\":\x20{\n\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\"consumer\":\x20\"projects/123\",\n\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\"service\":\x20\"storage.googleapis.com\
    \"\n\x20\x20\x20\x20\x20\x20\x20}\n\x20\x20\x20\x20\x20}\n\n\r\n\x05\x05\
    \0\x02\x0c\x01\x12\x04\x90\x02\x02\x15\n\r\n\x05\x05\0\x02\x0c\x02\x12\
    \x04\x90\x02\x18\x1a\n\xb9\x04\n\x04\x05\0\x02\r\x12\x04\xa1\x02\x02\x1a\
    \x1a\xaa\x04\x20The\x20request\x20is\x20denied\x20because\x20the\x20cons\
    umer\x20\"projects/123\"\x20is\x20suspended\x20due\n\x20to\x20Terms\x20o\
    f\x20Service(Tos)\x20violations.\x20Check\x20[Project\x20suspension\n\
    \x20guidelines](https://cloud.google.com/resource-manager/docs/project-s\
    uspension-guidelines)\n\x20for\x20more\x20information.\n\n\x20Example\
    \x20of\x20an\x20ErrorInfo\x20when\x20calling\x20Cloud\x20Storage\x20serv\
    ice\x20with\x20the\n\x20suspended\x20consumer\x20\"projects/123\":\n\n\
    \x20\x20\x20\x20\x20{\x20\"reason\":\x20\"CONSUMER_SUSPENDED\",\n\x20\
    \x20\x20\x20\x20\x20\x20\"domain\":\x20\"googleapis.com\",\n\x20\x20\x20\
    \x20\x20\x20\x20\"metadata\":\x20{\n\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \"consumer\":\x20\"projects/123\",\n\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \"service\":\x20\"storage.googleapis.com\"\n\x20\x20\x20\x20\x20\x20\x20\
    }\n\x20\x20\x20\x20\x20}\n\n\r\n\x05\x05\0\x02\r\x01\x12\x04\xa1\x02\x02\
    \x14\n\r\n\x05\x05\0\x02\r\x02\x12\x04\xa1\x02\x17\x19\n\xbd\x03\n\x04\
    \x05\0\x02\x0e\x12\x04\xb0\x02\x02\x18\x1a\xae\x03\x20The\x20request\x20\
    is\x20denied\x20because\x20the\x20associated\x20consumer\x20is\x20invali\
    d.\x20It\x20may\x20be\n\x20in\x20a\x20bad\x20format,\x20cannot\x20be\x20\
    found,\x20or\x20have\x20been\x20deleted.\n\n\x20Example\x20of\x20an\x20E\
    rrorInfo\x20when\x20calling\x20Cloud\x20Storage\x20service\x20with\x20th\
    e\n\x20invalid\x20consumer\x20\"projects/123\":\n\n\x20\x20\x20\x20\x20{\
    \x20\"reason\":\x20\"CONSUMER_INVALID\",\n\x20\x20\x20\x20\x20\x20\x20\"\
    domain\":\x20\"googleapis.com\",\n\x20\x20\x20\x20\x20\x20\x20\"metadata\
    \":\x20{\n\x20\x20\x20\x20\x20\x20\x20\x20\x20\"consumer\":\x20\"project\
    s/123\",\n\x20\x20\x20\x20\x20\x20\x20\x20\x20\"service\":\x20\"storage.\
    googleapis.com\"\n\x20\x20\x20\x20\x20\x20\x20}\n\x20\x20\x20\x20\x20}\n\
    \n\r\n\x05\x05\0\x02\x0e\x01\x12\x04\xb0\x02\x02\x12\n\r\n\x05\x05\0\x02\
    \x0e\x02\x12\x04\xb0\x02\x15\x17\n\xc8\x06\n\x04\x05\0\x02\x0f\x12\x04\
    \xc5\x02\x02\x20\x1a\xb9\x06\x20The\x20request\x20is\x20denied\x20becaus\
    e\x20it\x20violates\x20[VPC\x20Service\n\x20Controls](https://cloud.goog\
    le.com/vpc-service-controls/docs/overview).\n\x20The\x20'uid'\x20field\
    \x20is\x20a\x20random\x20generated\x20identifier\x20that\x20customer\x20\
    can\x20use\x20it\n\x20to\x20search\x20the\x20audit\x20log\x20for\x20a\
    \x20request\x20rejected\x20by\x20VPC\x20Service\x20Controls.\x20For\n\
    \x20more\x20information,\x20please\x20refer\x20[VPC\x20Service\x20Contro\
    ls\n\x20Troubleshooting](https://cloud.google.com/vpc-service-controls/d\
    ocs/troubleshooting#unique-id)\n\n\x20Example\x20of\x20an\x20ErrorInfo\
    \x20when\x20the\x20consumer\x20\"projects/123\"\x20fails\x20to\x20call\n\
    \x20Cloud\x20Storage\x20service\x20because\x20the\x20request\x20is\x20pr\
    ohibited\x20by\x20the\x20VPC\x20Service\n\x20Controls.\n\n\x20\x20\x20\
    \x20\x20{\x20\"reason\":\x20\"SECURITY_POLICY_VIOLATED\",\n\x20\x20\x20\
    \x20\x20\x20\x20\"domain\":\x20\"googleapis.com\",\n\x20\x20\x20\x20\x20\
    \x20\x20\"metadata\":\x20{\n\x20\x20\x20\x20\x20\x20\x20\x20\x20\"uid\":\
    \x20\"123456789abcde\",\n\x20\x20\x20\x20\x20\x20\x20\x20\x20\"consumer\
    \":\x20\"projects/123\",\n\x20\x20\x20\x20\x20\x20\x20\x20\x20\"service\
    \":\x20\"storage.googleapis.com\"\n\x20\x20\x20\x20\x20\x20\x20}\n\x20\
    \x20\x20\x20\x20}\n\n\r\n\x05\x05\0\x02\x0f\x01\x12\x04\xc5\x02\x02\x1a\
    \n\r\n\x05\x05\0\x02\x0f\x02\x12\x04\xc5\x02\x1d\x1f\n\x99\x03\n\x04\x05\
    \0\x02\x10\x12\x04\xd3\x02\x02\x1c\x1a\x8a\x03\x20The\x20request\x20is\
    \x20denied\x20because\x20the\x20provided\x20access\x20token\x20has\x20ex\
    pired.\n\n\x20Example\x20of\x20an\x20ErrorInfo\x20when\x20the\x20request\
    \x20is\x20calling\x20Cloud\x20Storage\x20service\n\x20with\x20an\x20expi\
    red\x20access\x20token:\n\n\x20\x20\x20\x20\x20{\x20\"reason\":\x20\"ACC\
    ESS_TOKEN_EXPIRED\",\n\x20\x20\x20\x20\x20\x20\x20\"domain\":\x20\"googl\
    eapis.com\",\n\x20\x20\x20\x20\x20\x20\x20\"metadata\":\x20{\n\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\"service\":\x20\"storage.googleapis.com\",\
    \n\x20\x20\x20\x20\x20\x20\x20\x20\x20\"method\":\x20\"google.storage.v1\
    .Storage.GetObject\"\n\x20\x20\x20\x20\x20\x20\x20}\n\x20\x20\x20\x20\
    \x20}\n\n\r\n\x05\x05\0\x02\x10\x01\x12\x04\xd3\x02\x02\x16\n\r\n\x05\
    \x05\0\x02\x10\x02\x12\x04\xd3\x02\x19\x1b\n\xc4\x05\n\x04\x05\0\x02\x11\
    \x12\x04\xe6\x02\x02'\x1a\xb5\x05\x20The\x20request\x20is\x20denied\x20b\
    ecause\x20the\x20provided\x20access\x20token\x20doesn't\x20have\x20at\n\
    \x20least\x20one\x20of\x20the\x20acceptable\x20scopes\x20required\x20for\
    \x20the\x20API.\x20Please\x20check\n\x20[OAuth\x202.0\x20Scopes\x20for\
    \x20Google\n\x20APIs](https://developers.google.com/identity/protocols/o\
    auth2/scopes)\x20for\n\x20the\x20list\x20of\x20the\x20OAuth\x202.0\x20sc\
    opes\x20that\x20you\x20might\x20need\x20to\x20request\x20to\x20access\n\
    \x20the\x20API.\n\n\x20Example\x20of\x20an\x20ErrorInfo\x20when\x20the\
    \x20request\x20is\x20calling\x20Cloud\x20Storage\x20service\n\x20with\
    \x20an\x20access\x20token\x20that\x20is\x20missing\x20required\x20scopes\
    :\n\n\x20\x20\x20\x20\x20{\x20\"reason\":\x20\"ACCESS_TOKEN_SCOPE_INSUFF\
    ICIENT\",\n\x20\x20\x20\x20\x20\x20\x20\"domain\":\x20\"googleapis.com\"\
    ,\n\x20\x20\x20\x20\x20\x20\x20\"metadata\":\x20{\n\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\"service\":\x20\"storage.googleapis.com\",\n\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\"method\":\x20\"google.storage.v1.Storage.G\
    etObject\"\n\x20\x20\x20\x20\x20\x20\x20}\n\x20\x20\x20\x20\x20}\n\n\r\n\
    \x05\x05\0\x02\x11\x01\x12\x04\xe6\x02\x02!\n\r\n\x05\x05\0\x02\x11\x02\
    \x12\x04\xe6\x02$&\n\xf1\x06\n\x04\x05\0\x02\x12\x12\x04\xfc\x02\x02\x1d\
    \x1a\xe2\x06\x20The\x20request\x20is\x20denied\x20because\x20the\x20acco\
    unt\x20associated\x20with\x20the\x20provided\n\x20access\x20token\x20is\
    \x20in\x20an\x20invalid\x20state,\x20such\x20as\x20disabled\x20or\x20del\
    eted.\n\x20For\x20more\x20information,\x20see\x20https://cloud.google.co\
    m/docs/authentication.\n\n\x20Warning:\x20For\x20privacy\x20reasons,\x20\
    the\x20server\x20may\x20not\x20be\x20able\x20to\x20disclose\x20the\n\x20\
    email\x20address\x20for\x20some\x20accounts.\x20The\x20client\x20MUST\
    \x20NOT\x20depend\x20on\x20the\n\x20availability\x20of\x20the\x20`email`\
    \x20attribute.\n\n\x20Example\x20of\x20an\x20ErrorInfo\x20when\x20the\
    \x20request\x20is\x20to\x20the\x20Cloud\x20Storage\x20API\x20with\n\x20a\
    n\x20access\x20token\x20that\x20is\x20associated\x20with\x20a\x20disable\
    d\x20or\x20deleted\x20[service\n\x20account](http://cloud/iam/docs/servi\
    ce-accounts):\n\n\x20\x20\x20\x20\x20{\x20\"reason\":\x20\"ACCOUNT_STATE\
    _INVALID\",\n\x20\x20\x20\x20\x20\x20\x20\"domain\":\x20\"googleapis.com\
    \",\n\x20\x20\x20\x20\x20\x20\x20\"metadata\":\x20{\n\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\"service\":\x20\"storage.googleapis.com\",\n\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\"method\":\x20\"google.storage.v1.Stora\
    ge.GetObject\",\n\x20\x20\x20\x20\x20\x20\x20\x20\x20\"email\":\x20\"use\
    r@123.iam.gserviceaccount.com\"\n\x20\x20\x20\x20\x20\x20\x20}\n\x20\x20\
    \x20\x20\x20}\n\n\r\n\x05\x05\0\x02\x12\x01\x12\x04\xfc\x02\x02\x17\n\r\
    \n\x05\x05\0\x02\x12\x02\x12\x04\xfc\x02\x1a\x1c\n\xc9\x03\n\x04\x05\0\
    \x02\x13\x12\x04\x8b\x03\x02%\x1a\xba\x03\x20The\x20request\x20is\x20den\
    ied\x20because\x20the\x20type\x20of\x20the\x20provided\x20access\x20toke\
    n\x20is\x20not\n\x20supported\x20by\x20the\x20API\x20being\x20called.\n\
    \n\x20Example\x20of\x20an\x20ErrorInfo\x20when\x20the\x20request\x20is\
    \x20to\x20the\x20Cloud\x20Storage\x20API\x20with\n\x20an\x20unsupported\
    \x20token\x20type.\n\n\x20\x20\x20\x20\x20{\x20\"reason\":\x20\"ACCESS_T\
    OKEN_TYPE_UNSUPPORTED\",\n\x20\x20\x20\x20\x20\x20\x20\"domain\":\x20\"g\
    oogleapis.com\",\n\x20\x20\x20\x20\x20\x20\x20\"metadata\":\x20{\n\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\"service\":\x20\"storage.googleapis.com\
    \",\n\x20\x20\x20\x20\x20\x20\x20\x20\x20\"method\":\x20\"google.storage\
    .v1.Storage.GetObject\"\n\x20\x20\x20\x20\x20\x20\x20}\n\x20\x20\x20\x20\
    \x20}\n\n\r\n\x05\x05\0\x02\x13\x01\x12\x04\x8b\x03\x02\x1f\n\r\n\x05\
    \x05\0\x02\x13\x02\x12\x04\x8b\x03\"$b\x06proto3\
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
