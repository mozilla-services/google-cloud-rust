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
//! Generated file from `google/api/config_change.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_28_0;

#[derive(PartialEq,Clone,Default)]
pub struct ConfigChange {
    // message fields
    pub element: ::std::string::String,
    pub old_value: ::std::string::String,
    pub new_value: ::std::string::String,
    pub change_type: ChangeType,
    pub advices: ::protobuf::RepeatedField<Advice>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a ConfigChange {
    fn default() -> &'a ConfigChange {
        <ConfigChange as ::protobuf::Message>::default_instance()
    }
}

impl ConfigChange {
    pub fn new() -> ConfigChange {
        ::std::default::Default::default()
    }

    // string element = 1;


    pub fn get_element(&self) -> &str {
        &self.element
    }
    pub fn clear_element(&mut self) {
        self.element.clear();
    }

    // Param is passed by value, moved
    pub fn set_element(&mut self, v: ::std::string::String) {
        self.element = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_element(&mut self) -> &mut ::std::string::String {
        &mut self.element
    }

    // Take field
    pub fn take_element(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.element, ::std::string::String::new())
    }

    // string old_value = 2;


    pub fn get_old_value(&self) -> &str {
        &self.old_value
    }
    pub fn clear_old_value(&mut self) {
        self.old_value.clear();
    }

    // Param is passed by value, moved
    pub fn set_old_value(&mut self, v: ::std::string::String) {
        self.old_value = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_old_value(&mut self) -> &mut ::std::string::String {
        &mut self.old_value
    }

    // Take field
    pub fn take_old_value(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.old_value, ::std::string::String::new())
    }

    // string new_value = 3;


    pub fn get_new_value(&self) -> &str {
        &self.new_value
    }
    pub fn clear_new_value(&mut self) {
        self.new_value.clear();
    }

    // Param is passed by value, moved
    pub fn set_new_value(&mut self, v: ::std::string::String) {
        self.new_value = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_new_value(&mut self) -> &mut ::std::string::String {
        &mut self.new_value
    }

    // Take field
    pub fn take_new_value(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.new_value, ::std::string::String::new())
    }

    // .google.api.ChangeType change_type = 4;


    pub fn get_change_type(&self) -> ChangeType {
        self.change_type
    }
    pub fn clear_change_type(&mut self) {
        self.change_type = ChangeType::CHANGE_TYPE_UNSPECIFIED;
    }

    // Param is passed by value, moved
    pub fn set_change_type(&mut self, v: ChangeType) {
        self.change_type = v;
    }

    // repeated .google.api.Advice advices = 5;


    pub fn get_advices(&self) -> &[Advice] {
        &self.advices
    }
    pub fn clear_advices(&mut self) {
        self.advices.clear();
    }

    // Param is passed by value, moved
    pub fn set_advices(&mut self, v: ::protobuf::RepeatedField<Advice>) {
        self.advices = v;
    }

    // Mutable pointer to the field.
    pub fn mut_advices(&mut self) -> &mut ::protobuf::RepeatedField<Advice> {
        &mut self.advices
    }

    // Take field
    pub fn take_advices(&mut self) -> ::protobuf::RepeatedField<Advice> {
        ::std::mem::replace(&mut self.advices, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for ConfigChange {
    fn is_initialized(&self) -> bool {
        for v in &self.advices {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.element)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.old_value)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.new_value)?;
                },
                4 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.change_type, 4, &mut self.unknown_fields)?
                },
                5 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.advices)?;
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
        if !self.element.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.element);
        }
        if !self.old_value.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.old_value);
        }
        if !self.new_value.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.new_value);
        }
        if self.change_type != ChangeType::CHANGE_TYPE_UNSPECIFIED {
            my_size += ::protobuf::rt::enum_size(4, self.change_type);
        }
        for value in &self.advices {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.element.is_empty() {
            os.write_string(1, &self.element)?;
        }
        if !self.old_value.is_empty() {
            os.write_string(2, &self.old_value)?;
        }
        if !self.new_value.is_empty() {
            os.write_string(3, &self.new_value)?;
        }
        if self.change_type != ChangeType::CHANGE_TYPE_UNSPECIFIED {
            os.write_enum(4, ::protobuf::ProtobufEnum::value(&self.change_type))?;
        }
        for v in &self.advices {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
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

    fn new() -> ConfigChange {
        ConfigChange::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "element",
                |m: &ConfigChange| { &m.element },
                |m: &mut ConfigChange| { &mut m.element },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "old_value",
                |m: &ConfigChange| { &m.old_value },
                |m: &mut ConfigChange| { &mut m.old_value },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "new_value",
                |m: &ConfigChange| { &m.new_value },
                |m: &mut ConfigChange| { &mut m.new_value },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ChangeType>>(
                "change_type",
                |m: &ConfigChange| { &m.change_type },
                |m: &mut ConfigChange| { &mut m.change_type },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Advice>>(
                "advices",
                |m: &ConfigChange| { &m.advices },
                |m: &mut ConfigChange| { &mut m.advices },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<ConfigChange>(
                "ConfigChange",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static ConfigChange {
        static instance: ::protobuf::rt::LazyV2<ConfigChange> = ::protobuf::rt::LazyV2::INIT;
        instance.get(ConfigChange::new)
    }
}

impl ::protobuf::Clear for ConfigChange {
    fn clear(&mut self) {
        self.element.clear();
        self.old_value.clear();
        self.new_value.clear();
        self.change_type = ChangeType::CHANGE_TYPE_UNSPECIFIED;
        self.advices.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ConfigChange {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ConfigChange {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Advice {
    // message fields
    pub description: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Advice {
    fn default() -> &'a Advice {
        <Advice as ::protobuf::Message>::default_instance()
    }
}

impl Advice {
    pub fn new() -> Advice {
        ::std::default::Default::default()
    }

    // string description = 2;


    pub fn get_description(&self) -> &str {
        &self.description
    }
    pub fn clear_description(&mut self) {
        self.description.clear();
    }

    // Param is passed by value, moved
    pub fn set_description(&mut self, v: ::std::string::String) {
        self.description = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_description(&mut self) -> &mut ::std::string::String {
        &mut self.description
    }

    // Take field
    pub fn take_description(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.description, ::std::string::String::new())
    }
}

impl ::protobuf::Message for Advice {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.description)?;
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
        if !self.description.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.description);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.description.is_empty() {
            os.write_string(2, &self.description)?;
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

    fn new() -> Advice {
        Advice::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "description",
                |m: &Advice| { &m.description },
                |m: &mut Advice| { &mut m.description },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Advice>(
                "Advice",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Advice {
        static instance: ::protobuf::rt::LazyV2<Advice> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Advice::new)
    }
}

impl ::protobuf::Clear for Advice {
    fn clear(&mut self) {
        self.description.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Advice {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Advice {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ChangeType {
    CHANGE_TYPE_UNSPECIFIED = 0,
    ADDED = 1,
    REMOVED = 2,
    MODIFIED = 3,
}

impl ::protobuf::ProtobufEnum for ChangeType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ChangeType> {
        match value {
            0 => ::std::option::Option::Some(ChangeType::CHANGE_TYPE_UNSPECIFIED),
            1 => ::std::option::Option::Some(ChangeType::ADDED),
            2 => ::std::option::Option::Some(ChangeType::REMOVED),
            3 => ::std::option::Option::Some(ChangeType::MODIFIED),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ChangeType] = &[
            ChangeType::CHANGE_TYPE_UNSPECIFIED,
            ChangeType::ADDED,
            ChangeType::REMOVED,
            ChangeType::MODIFIED,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<ChangeType>("ChangeType", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for ChangeType {
}

impl ::std::default::Default for ChangeType {
    fn default() -> Self {
        ChangeType::CHANGE_TYPE_UNSPECIFIED
    }
}

impl ::protobuf::reflect::ProtobufValue for ChangeType {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1egoogle/api/config_change.proto\x12\ngoogle.api\"\xc9\x01\n\x0cConf\
    igChange\x12\x18\n\x07element\x18\x01\x20\x01(\tR\x07element\x12\x1b\n\t\
    old_value\x18\x02\x20\x01(\tR\x08oldValue\x12\x1b\n\tnew_value\x18\x03\
    \x20\x01(\tR\x08newValue\x127\n\x0bchange_type\x18\x04\x20\x01(\x0e2\x16\
    .google.api.ChangeTypeR\nchangeType\x12,\n\x07advices\x18\x05\x20\x03(\
    \x0b2\x12.google.api.AdviceR\x07advices\"*\n\x06Advice\x12\x20\n\x0bdesc\
    ription\x18\x02\x20\x01(\tR\x0bdescription*O\n\nChangeType\x12\x1b\n\x17\
    CHANGE_TYPE_UNSPECIFIED\x10\0\x12\t\n\x05ADDED\x10\x01\x12\x0b\n\x07REMO\
    VED\x10\x02\x12\x0c\n\x08MODIFIED\x10\x03Bq\n\x0ecom.google.apiB\x11Conf\
    igChangeProtoP\x01ZCgoogle.golang.org/genproto/googleapis/api/configchan\
    ge;configchange\xa2\x02\x04GAPIJ\xe5\x18\n\x06\x12\x04\x0e\0S\x01\n\xbc\
    \x04\n\x01\x0c\x12\x03\x0e\0\x122\xb1\x04\x20Copyright\x202023\x20Google\
    \x20LLC\n\n\x20Licensed\x20under\x20the\x20Apache\x20License,\x20Version\
    \x202.0\x20(the\x20\"License\");\n\x20you\x20may\x20not\x20use\x20this\
    \x20file\x20except\x20in\x20compliance\x20with\x20the\x20License.\n\x20Y\
    ou\x20may\x20obtain\x20a\x20copy\x20of\x20the\x20License\x20at\n\n\x20\
    \x20\x20\x20\x20http://www.apache.org/licenses/LICENSE-2.0\n\n\x20Unless\
    \x20required\x20by\x20applicable\x20law\x20or\x20agreed\x20to\x20in\x20w\
    riting,\x20software\n\x20distributed\x20under\x20the\x20License\x20is\
    \x20distributed\x20on\x20an\x20\"AS\x20IS\"\x20BASIS,\n\x20WITHOUT\x20WA\
    RRANTIES\x20OR\x20CONDITIONS\x20OF\x20ANY\x20KIND,\x20either\x20express\
    \x20or\x20implied.\n\x20See\x20the\x20License\x20for\x20the\x20specific\
    \x20language\x20governing\x20permissions\x20and\n\x20limitations\x20unde\
    r\x20the\x20License.\n\n\x08\n\x01\x02\x12\x03\x10\0\x13\n\x08\n\x01\x08\
    \x12\x03\x12\0Z\n\t\n\x02\x08\x0b\x12\x03\x12\0Z\n\x08\n\x01\x08\x12\x03\
    \x13\0\"\n\t\n\x02\x08\n\x12\x03\x13\0\"\n\x08\n\x01\x08\x12\x03\x14\02\
    \n\t\n\x02\x08\x08\x12\x03\x14\02\n\x08\n\x01\x08\x12\x03\x15\0'\n\t\n\
    \x02\x08\x01\x12\x03\x15\0'\n\x08\n\x01\x08\x12\x03\x16\0\"\n\t\n\x02\
    \x08$\x12\x03\x16\0\"\n\x8e\x02\n\x02\x04\0\x12\x04\x1e\08\x01\x1a\x81\
    \x02\x20Output\x20generated\x20from\x20semantically\x20comparing\x20two\
    \x20versions\x20of\x20a\x20service\n\x20configuration.\n\n\x20Includes\
    \x20detailed\x20information\x20about\x20a\x20field\x20that\x20have\x20ch\
    anged\x20with\n\x20applicable\x20advice\x20about\x20potential\x20consequ\
    ences\x20for\x20the\x20change,\x20such\x20as\n\x20backwards-incompatibil\
    ity.\n\n\n\n\x03\x04\0\x01\x12\x03\x1e\x08\x14\n\xff\x03\n\x04\x04\0\x02\
    \0\x12\x03(\x02\x15\x1a\xf1\x03\x20Object\x20hierarchy\x20path\x20to\x20\
    the\x20change,\x20with\x20levels\x20separated\x20by\x20a\x20'.'\n\x20cha\
    racter.\x20For\x20repeated\x20fields,\x20an\x20applicable\x20unique\x20i\
    dentifier\x20field\x20is\n\x20used\x20for\x20the\x20index\x20(usually\
    \x20selector,\x20name,\x20or\x20id).\x20For\x20maps,\x20the\x20term\n\
    \x20'key'\x20is\x20used.\x20If\x20the\x20field\x20has\x20no\x20unique\
    \x20identifier,\x20the\x20numeric\x20index\n\x20is\x20used.\n\x20Example\
    s:\n\x20-\x20visibility.rules[selector==\"google.LibraryService.ListBook\
    s\"].restriction\n\x20-\x20quota.metric_rules[selector==\"google\"].metr\
    ic_costs[key==\"reads\"].value\n\x20-\x20logging.producer_destinations[0\
    ]\n\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03(\x02\x08\n\x0c\n\x05\x04\0\x02\0\
    \x01\x12\x03(\t\x10\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03(\x13\x14\n\x97\
    \x01\n\x04\x04\0\x02\x01\x12\x03,\x02\x17\x1a\x89\x01\x20Value\x20of\x20\
    the\x20changed\x20object\x20in\x20the\x20old\x20Service\x20configuration\
    ,\n\x20in\x20JSON\x20format.\x20This\x20field\x20will\x20not\x20be\x20po\
    pulated\x20if\x20ChangeType\x20==\x20ADDED.\n\n\x0c\n\x05\x04\0\x02\x01\
    \x05\x12\x03,\x02\x08\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03,\t\x12\n\x0c\
    \n\x05\x04\0\x02\x01\x03\x12\x03,\x15\x16\n\x99\x01\n\x04\x04\0\x02\x02\
    \x12\x030\x02\x17\x1a\x8b\x01\x20Value\x20of\x20the\x20changed\x20object\
    \x20in\x20the\x20new\x20Service\x20configuration,\n\x20in\x20JSON\x20for\
    mat.\x20This\x20field\x20will\x20not\x20be\x20populated\x20if\x20ChangeT\
    ype\x20==\x20REMOVED.\n\n\x0c\n\x05\x04\0\x02\x02\x05\x12\x030\x02\x08\n\
    \x0c\n\x05\x04\0\x02\x02\x01\x12\x030\t\x12\n\x0c\n\x05\x04\0\x02\x02\
    \x03\x12\x030\x15\x16\nL\n\x04\x04\0\x02\x03\x12\x033\x02\x1d\x1a?\x20Th\
    e\x20type\x20for\x20this\x20change,\x20either\x20ADDED,\x20REMOVED,\x20o\
    r\x20MODIFIED.\n\n\x0c\n\x05\x04\0\x02\x03\x06\x12\x033\x02\x0c\n\x0c\n\
    \x05\x04\0\x02\x03\x01\x12\x033\r\x18\n\x0c\n\x05\x04\0\x02\x03\x03\x12\
    \x033\x1b\x1c\ny\n\x04\x04\0\x02\x04\x12\x037\x02\x1e\x1al\x20Collection\
    \x20of\x20advice\x20provided\x20for\x20this\x20change,\x20useful\x20for\
    \x20determining\x20the\n\x20possible\x20impact\x20of\x20this\x20change.\
    \n\n\x0c\n\x05\x04\0\x02\x04\x04\x12\x037\x02\n\n\x0c\n\x05\x04\0\x02\
    \x04\x06\x12\x037\x0b\x11\n\x0c\n\x05\x04\0\x02\x04\x01\x12\x037\x12\x19\
    \n\x0c\n\x05\x04\0\x02\x04\x03\x12\x037\x1c\x1d\n\x8b\x01\n\x02\x04\x01\
    \x12\x04<\0@\x01\x1a\x7f\x20Generated\x20advice\x20about\x20this\x20chan\
    ge,\x20used\x20for\x20providing\x20more\n\x20information\x20about\x20how\
    \x20a\x20change\x20will\x20affect\x20the\x20existing\x20service.\n\n\n\n\
    \x03\x04\x01\x01\x12\x03<\x08\x0e\n\x82\x01\n\x04\x04\x01\x02\0\x12\x03?\
    \x02\x19\x1au\x20Useful\x20description\x20for\x20why\x20this\x20advice\
    \x20was\x20applied\x20and\x20what\x20actions\x20should\n\x20be\x20taken\
    \x20to\x20mitigate\x20any\x20implied\x20risks.\n\n\x0c\n\x05\x04\x01\x02\
    \0\x05\x12\x03?\x02\x08\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03?\t\x14\n\
    \x0c\n\x05\x04\x01\x02\0\x03\x12\x03?\x17\x18\nb\n\x02\x05\0\x12\x04D\0S\
    \x01\x1aV\x20Classifies\x20set\x20of\x20possible\x20modifications\x20to\
    \x20an\x20object\x20in\x20the\x20service\n\x20configuration.\n\n\n\n\x03\
    \x05\0\x01\x12\x03D\x05\x0f\n%\n\x04\x05\0\x02\0\x12\x03F\x02\x1e\x1a\
    \x18\x20No\x20value\x20was\x20provided.\n\n\x0c\n\x05\x05\0\x02\0\x01\
    \x12\x03F\x02\x19\n\x0c\n\x05\x05\0\x02\0\x02\x12\x03F\x1c\x1d\ny\n\x04\
    \x05\0\x02\x01\x12\x03J\x02\x0c\x1al\x20The\x20changed\x20object\x20exis\
    ts\x20in\x20the\x20'new'\x20service\x20configuration,\x20but\x20not\n\
    \x20in\x20the\x20'old'\x20service\x20configuration.\n\n\x0c\n\x05\x05\0\
    \x02\x01\x01\x12\x03J\x02\x07\n\x0c\n\x05\x05\0\x02\x01\x02\x12\x03J\n\
    \x0b\ny\n\x04\x05\0\x02\x02\x12\x03N\x02\x0e\x1al\x20The\x20changed\x20o\
    bject\x20exists\x20in\x20the\x20'old'\x20service\x20configuration,\x20bu\
    t\x20not\n\x20in\x20the\x20'new'\x20service\x20configuration.\n\n\x0c\n\
    \x05\x05\0\x02\x02\x01\x12\x03N\x02\t\n\x0c\n\x05\x05\0\x02\x02\x02\x12\
    \x03N\x0c\r\ne\n\x04\x05\0\x02\x03\x12\x03R\x02\x0f\x1aX\x20The\x20chang\
    ed\x20object\x20exists\x20in\x20both\x20service\x20configurations,\x20bu\
    t\x20its\x20value\n\x20is\x20different.\n\n\x0c\n\x05\x05\0\x02\x03\x01\
    \x12\x03R\x02\n\n\x0c\n\x05\x05\0\x02\x03\x02\x12\x03R\r\x0eb\x06proto3\
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
