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
//! Generated file from `google/api/usage.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_28_0;

#[derive(PartialEq,Clone,Default)]
pub struct Usage {
    // message fields
    pub requirements: ::protobuf::RepeatedField<::std::string::String>,
    pub rules: ::protobuf::RepeatedField<UsageRule>,
    pub producer_notification_channel: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Usage {
    fn default() -> &'a Usage {
        <Usage as ::protobuf::Message>::default_instance()
    }
}

impl Usage {
    pub fn new() -> Usage {
        ::std::default::Default::default()
    }

    // repeated string requirements = 1;


    pub fn get_requirements(&self) -> &[::std::string::String] {
        &self.requirements
    }
    pub fn clear_requirements(&mut self) {
        self.requirements.clear();
    }

    // Param is passed by value, moved
    pub fn set_requirements(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.requirements = v;
    }

    // Mutable pointer to the field.
    pub fn mut_requirements(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.requirements
    }

    // Take field
    pub fn take_requirements(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.requirements, ::protobuf::RepeatedField::new())
    }

    // repeated .google.api.UsageRule rules = 6;


    pub fn get_rules(&self) -> &[UsageRule] {
        &self.rules
    }
    pub fn clear_rules(&mut self) {
        self.rules.clear();
    }

    // Param is passed by value, moved
    pub fn set_rules(&mut self, v: ::protobuf::RepeatedField<UsageRule>) {
        self.rules = v;
    }

    // Mutable pointer to the field.
    pub fn mut_rules(&mut self) -> &mut ::protobuf::RepeatedField<UsageRule> {
        &mut self.rules
    }

    // Take field
    pub fn take_rules(&mut self) -> ::protobuf::RepeatedField<UsageRule> {
        ::std::mem::replace(&mut self.rules, ::protobuf::RepeatedField::new())
    }

    // string producer_notification_channel = 7;


    pub fn get_producer_notification_channel(&self) -> &str {
        &self.producer_notification_channel
    }
    pub fn clear_producer_notification_channel(&mut self) {
        self.producer_notification_channel.clear();
    }

    // Param is passed by value, moved
    pub fn set_producer_notification_channel(&mut self, v: ::std::string::String) {
        self.producer_notification_channel = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_producer_notification_channel(&mut self) -> &mut ::std::string::String {
        &mut self.producer_notification_channel
    }

    // Take field
    pub fn take_producer_notification_channel(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.producer_notification_channel, ::std::string::String::new())
    }
}

impl ::protobuf::Message for Usage {
    fn is_initialized(&self) -> bool {
        for v in &self.rules {
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
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.requirements)?;
                },
                6 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.rules)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.producer_notification_channel)?;
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
        for value in &self.requirements {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.rules {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if !self.producer_notification_channel.is_empty() {
            my_size += ::protobuf::rt::string_size(7, &self.producer_notification_channel);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        for v in &self.requirements {
            os.write_string(1, &v)?;
        };
        for v in &self.rules {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if !self.producer_notification_channel.is_empty() {
            os.write_string(7, &self.producer_notification_channel)?;
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

    fn new() -> Usage {
        Usage::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "requirements",
                |m: &Usage| { &m.requirements },
                |m: &mut Usage| { &mut m.requirements },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<UsageRule>>(
                "rules",
                |m: &Usage| { &m.rules },
                |m: &mut Usage| { &mut m.rules },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "producer_notification_channel",
                |m: &Usage| { &m.producer_notification_channel },
                |m: &mut Usage| { &mut m.producer_notification_channel },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Usage>(
                "Usage",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Usage {
        static instance: ::protobuf::rt::LazyV2<Usage> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Usage::new)
    }
}

impl ::protobuf::Clear for Usage {
    fn clear(&mut self) {
        self.requirements.clear();
        self.rules.clear();
        self.producer_notification_channel.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Usage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Usage {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct UsageRule {
    // message fields
    pub selector: ::std::string::String,
    pub allow_unregistered_calls: bool,
    pub skip_service_control: bool,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a UsageRule {
    fn default() -> &'a UsageRule {
        <UsageRule as ::protobuf::Message>::default_instance()
    }
}

impl UsageRule {
    pub fn new() -> UsageRule {
        ::std::default::Default::default()
    }

    // string selector = 1;


    pub fn get_selector(&self) -> &str {
        &self.selector
    }
    pub fn clear_selector(&mut self) {
        self.selector.clear();
    }

    // Param is passed by value, moved
    pub fn set_selector(&mut self, v: ::std::string::String) {
        self.selector = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_selector(&mut self) -> &mut ::std::string::String {
        &mut self.selector
    }

    // Take field
    pub fn take_selector(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.selector, ::std::string::String::new())
    }

    // bool allow_unregistered_calls = 2;


    pub fn get_allow_unregistered_calls(&self) -> bool {
        self.allow_unregistered_calls
    }
    pub fn clear_allow_unregistered_calls(&mut self) {
        self.allow_unregistered_calls = false;
    }

    // Param is passed by value, moved
    pub fn set_allow_unregistered_calls(&mut self, v: bool) {
        self.allow_unregistered_calls = v;
    }

    // bool skip_service_control = 3;


    pub fn get_skip_service_control(&self) -> bool {
        self.skip_service_control
    }
    pub fn clear_skip_service_control(&mut self) {
        self.skip_service_control = false;
    }

    // Param is passed by value, moved
    pub fn set_skip_service_control(&mut self, v: bool) {
        self.skip_service_control = v;
    }
}

impl ::protobuf::Message for UsageRule {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.selector)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.allow_unregistered_calls = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.skip_service_control = tmp;
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
        if !self.selector.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.selector);
        }
        if self.allow_unregistered_calls != false {
            my_size += 2;
        }
        if self.skip_service_control != false {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.selector.is_empty() {
            os.write_string(1, &self.selector)?;
        }
        if self.allow_unregistered_calls != false {
            os.write_bool(2, self.allow_unregistered_calls)?;
        }
        if self.skip_service_control != false {
            os.write_bool(3, self.skip_service_control)?;
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

    fn new() -> UsageRule {
        UsageRule::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "selector",
                |m: &UsageRule| { &m.selector },
                |m: &mut UsageRule| { &mut m.selector },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                "allow_unregistered_calls",
                |m: &UsageRule| { &m.allow_unregistered_calls },
                |m: &mut UsageRule| { &mut m.allow_unregistered_calls },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                "skip_service_control",
                |m: &UsageRule| { &m.skip_service_control },
                |m: &mut UsageRule| { &mut m.skip_service_control },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<UsageRule>(
                "UsageRule",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static UsageRule {
        static instance: ::protobuf::rt::LazyV2<UsageRule> = ::protobuf::rt::LazyV2::INIT;
        instance.get(UsageRule::new)
    }
}

impl ::protobuf::Clear for UsageRule {
    fn clear(&mut self) {
        self.selector.clear();
        self.allow_unregistered_calls = false;
        self.skip_service_control = false;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UsageRule {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UsageRule {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x16google/api/usage.proto\x12\ngoogle.api\"\x9c\x01\n\x05Usage\x12\"\
    \n\x0crequirements\x18\x01\x20\x03(\tR\x0crequirements\x12+\n\x05rules\
    \x18\x06\x20\x03(\x0b2\x15.google.api.UsageRuleR\x05rules\x12B\n\x1dprod\
    ucer_notification_channel\x18\x07\x20\x01(\tR\x1bproducerNotificationCha\
    nnel\"\x93\x01\n\tUsageRule\x12\x1a\n\x08selector\x18\x01\x20\x01(\tR\
    \x08selector\x128\n\x18allow_unregistered_calls\x18\x02\x20\x01(\x08R\
    \x16allowUnregisteredCalls\x120\n\x14skip_service_control\x18\x03\x20\
    \x01(\x08R\x12skipServiceControlBl\n\x0ecom.google.apiB\nUsageProtoP\x01\
    ZEgoogle.golang.org/genproto/googleapis/api/serviceconfig;serviceconfig\
    \xa2\x02\x04GAPIJ\xa0\x1c\n\x06\x12\x04\x0e\0_\x01\n\xbc\x04\n\x01\x0c\
    \x12\x03\x0e\0\x122\xb1\x04\x20Copyright\x202023\x20Google\x20LLC\n\n\
    \x20Licensed\x20under\x20the\x20Apache\x20License,\x20Version\x202.0\x20\
    (the\x20\"License\");\n\x20you\x20may\x20not\x20use\x20this\x20file\x20e\
    xcept\x20in\x20compliance\x20with\x20the\x20License.\n\x20You\x20may\x20\
    obtain\x20a\x20copy\x20of\x20the\x20License\x20at\n\n\x20\x20\x20\x20\
    \x20http://www.apache.org/licenses/LICENSE-2.0\n\n\x20Unless\x20required\
    \x20by\x20applicable\x20law\x20or\x20agreed\x20to\x20in\x20writing,\x20s\
    oftware\n\x20distributed\x20under\x20the\x20License\x20is\x20distributed\
    \x20on\x20an\x20\"AS\x20IS\"\x20BASIS,\n\x20WITHOUT\x20WARRANTIES\x20OR\
    \x20CONDITIONS\x20OF\x20ANY\x20KIND,\x20either\x20express\x20or\x20impli\
    ed.\n\x20See\x20the\x20License\x20for\x20the\x20specific\x20language\x20\
    governing\x20permissions\x20and\n\x20limitations\x20under\x20the\x20Lice\
    nse.\n\n\x08\n\x01\x02\x12\x03\x10\0\x13\n\x08\n\x01\x08\x12\x03\x12\0\\\
    \n\t\n\x02\x08\x0b\x12\x03\x12\0\\\n\x08\n\x01\x08\x12\x03\x13\0\"\n\t\n\
    \x02\x08\n\x12\x03\x13\0\"\n\x08\n\x01\x08\x12\x03\x14\0+\n\t\n\x02\x08\
    \x08\x12\x03\x14\0+\n\x08\n\x01\x08\x12\x03\x15\0'\n\t\n\x02\x08\x01\x12\
    \x03\x15\0'\n\x08\n\x01\x08\x12\x03\x16\0\"\n\t\n\x02\x08$\x12\x03\x16\0\
    \"\n;\n\x02\x04\0\x12\x04\x19\03\x01\x1a/\x20Configuration\x20controllin\
    g\x20usage\x20of\x20a\x20service.\n\n\n\n\x03\x04\0\x01\x12\x03\x19\x08\
    \r\n\xff\x03\n\x04\x04\0\x02\0\x12\x03#\x02#\x1a\xf1\x03\x20Requirements\
    \x20that\x20must\x20be\x20satisfied\x20before\x20a\x20consumer\x20projec\
    t\x20can\x20use\x20the\n\x20service.\x20Each\x20requirement\x20is\x20of\
    \x20the\x20form\x20<service.name>/<requirement-id>;\n\x20for\x20example\
    \x20'serviceusage.googleapis.com/billing-enabled'.\n\n\x20For\x20Google\
    \x20APIs,\x20a\x20Terms\x20of\x20Service\x20requirement\x20must\x20be\
    \x20included\x20here.\n\x20Google\x20Cloud\x20APIs\x20must\x20include\
    \x20\"serviceusage.googleapis.com/tos/cloud\".\n\x20Other\x20Google\x20A\
    PIs\x20should\x20include\n\x20\"serviceusage.googleapis.com/tos/universa\
    l\".\x20Additional\x20ToS\x20can\x20be\n\x20included\x20based\x20on\x20t\
    he\x20business\x20needs.\n\n\x0c\n\x05\x04\0\x02\0\x04\x12\x03#\x02\n\n\
    \x0c\n\x05\x04\0\x02\0\x05\x12\x03#\x0b\x11\n\x0c\n\x05\x04\0\x02\0\x01\
    \x12\x03#\x12\x1e\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03#!\"\n\x95\x01\n\
    \x04\x04\0\x02\x01\x12\x03(\x02\x1f\x1a\x87\x01\x20A\x20list\x20of\x20us\
    age\x20rules\x20that\x20apply\x20to\x20individual\x20API\x20methods.\n\n\
    \x20**NOTE:**\x20All\x20service\x20configuration\x20rules\x20follow\x20\
    \"last\x20one\x20wins\"\x20order.\n\n\x0c\n\x05\x04\0\x02\x01\x04\x12\
    \x03(\x02\n\n\x0c\n\x05\x04\0\x02\x01\x06\x12\x03(\x0b\x14\n\x0c\n\x05\
    \x04\0\x02\x01\x01\x12\x03(\x15\x1a\n\x0c\n\x05\x04\0\x02\x01\x03\x12\
    \x03(\x1d\x1e\n\xbd\x03\n\x04\x04\0\x02\x02\x12\x032\x02+\x1a\xaf\x03\
    \x20The\x20full\x20resource\x20name\x20of\x20a\x20channel\x20used\x20for\
    \x20sending\x20notifications\x20to\x20the\n\x20service\x20producer.\n\n\
    \x20Google\x20Service\x20Management\x20currently\x20only\x20supports\n\
    \x20[Google\x20Cloud\x20Pub/Sub](https://cloud.google.com/pubsub)\x20as\
    \x20a\x20notification\n\x20channel.\x20To\x20use\x20Google\x20Cloud\x20P\
    ub/Sub\x20as\x20the\x20channel,\x20this\x20must\x20be\x20the\x20name\n\
    \x20of\x20a\x20Cloud\x20Pub/Sub\x20topic\x20that\x20uses\x20the\x20Cloud\
    \x20Pub/Sub\x20topic\x20name\x20format\n\x20documented\x20in\x20https://\
    cloud.google.com/pubsub/docs/overview.\n\n\x0c\n\x05\x04\0\x02\x02\x05\
    \x12\x032\x02\x08\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x032\t&\n\x0c\n\x05\
    \x04\0\x02\x02\x03\x12\x032)*\n\xc0\x06\n\x02\x04\x01\x12\x04N\0_\x01\
    \x1a\xb3\x06\x20Usage\x20configuration\x20rules\x20for\x20the\x20service\
    .\n\n\x20NOTE:\x20Under\x20development.\n\n\n\x20Use\x20this\x20rule\x20\
    to\x20configure\x20unregistered\x20calls\x20for\x20the\x20service.\x20Un\
    registered\n\x20calls\x20are\x20calls\x20that\x20do\x20not\x20contain\
    \x20consumer\x20project\x20identity.\n\x20(Example:\x20calls\x20that\x20\
    do\x20not\x20contain\x20an\x20API\x20key).\n\x20By\x20default,\x20API\
    \x20methods\x20do\x20not\x20allow\x20unregistered\x20calls,\x20and\x20ea\
    ch\x20method\x20call\n\x20must\x20be\x20identified\x20by\x20a\x20consume\
    r\x20project\x20identity.\x20Use\x20this\x20rule\x20to\n\x20allow/disall\
    ow\x20unregistered\x20calls.\n\n\x20Example\x20of\x20an\x20API\x20that\
    \x20wants\x20to\x20allow\x20unregistered\x20calls\x20for\x20entire\x20se\
    rvice.\n\n\x20\x20\x20\x20\x20usage:\n\x20\x20\x20\x20\x20\x20\x20rules:\
    \n\x20\x20\x20\x20\x20\x20\x20-\x20selector:\x20\"*\"\n\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20allow_unregistered_calls:\x20true\n\n\x20Example\x20\
    of\x20a\x20method\x20that\x20wants\x20to\x20allow\x20unregistered\x20cal\
    ls.\n\n\x20\x20\x20\x20\x20usage:\n\x20\x20\x20\x20\x20\x20\x20rules:\n\
    \x20\x20\x20\x20\x20\x20\x20-\x20selector:\x20\"google.example.library.v\
    1.LibraryService.CreateBook\"\n\x20\x20\x20\x20\x20\x20\x20\x20\x20allow\
    _unregistered_calls:\x20true\n\n\n\n\x03\x04\x01\x01\x12\x03N\x08\x11\n\
    \xbf\x01\n\x04\x04\x01\x02\0\x12\x03T\x02\x16\x1a\xb1\x01\x20Selects\x20\
    the\x20methods\x20to\x20which\x20this\x20rule\x20applies.\x20Use\x20'*'\
    \x20to\x20indicate\x20all\n\x20methods\x20in\x20all\x20APIs.\n\n\x20Refe\
    r\x20to\x20[selector][google.api.DocumentationRule.selector]\x20for\x20s\
    yntax\n\x20details.\n\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03T\x02\x08\n\
    \x0c\n\x05\x04\x01\x02\0\x01\x12\x03T\t\x11\n\x0c\n\x05\x04\x01\x02\0\
    \x03\x12\x03T\x14\x15\n\x7f\n\x04\x04\x01\x02\x01\x12\x03X\x02$\x1ar\x20\
    If\x20true,\x20the\x20selected\x20method\x20allows\x20unregistered\x20ca\
    lls,\x20e.g.\x20calls\n\x20that\x20don't\x20identify\x20any\x20user\x20o\
    r\x20application.\n\n\x0c\n\x05\x04\x01\x02\x01\x05\x12\x03X\x02\x06\n\
    \x0c\n\x05\x04\x01\x02\x01\x01\x12\x03X\x07\x1f\n\x0c\n\x05\x04\x01\x02\
    \x01\x03\x12\x03X\"#\n\x96\x02\n\x04\x04\x01\x02\x02\x12\x03^\x02\x20\
    \x1a\x88\x02\x20If\x20true,\x20the\x20selected\x20method\x20should\x20sk\
    ip\x20service\x20control\x20and\x20the\x20control\n\x20plane\x20features\
    ,\x20such\x20as\x20quota\x20and\x20billing,\x20will\x20not\x20be\x20avai\
    lable.\n\x20This\x20flag\x20is\x20used\x20by\x20Google\x20Cloud\x20Endpo\
    ints\x20to\x20bypass\x20checks\x20for\x20internal\n\x20methods,\x20such\
    \x20as\x20service\x20health\x20check\x20methods.\n\n\x0c\n\x05\x04\x01\
    \x02\x02\x05\x12\x03^\x02\x06\n\x0c\n\x05\x04\x01\x02\x02\x01\x12\x03^\
    \x07\x1b\n\x0c\n\x05\x04\x01\x02\x02\x03\x12\x03^\x1e\x1fb\x06proto3\
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
