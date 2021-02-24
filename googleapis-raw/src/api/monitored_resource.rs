// This file is generated by rust-protobuf 2.22.0. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![rustfmt::skip]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `google/api/monitored_resource.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_22_0;

#[derive(PartialEq,Clone,Default)]
pub struct MonitoredResourceDescriptor {
    // message fields
    pub name: ::std::string::String,
    pub field_type: ::std::string::String,
    pub display_name: ::std::string::String,
    pub description: ::std::string::String,
    pub labels: ::protobuf::RepeatedField<super::label::LabelDescriptor>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a MonitoredResourceDescriptor {
    fn default() -> &'a MonitoredResourceDescriptor {
        <MonitoredResourceDescriptor as ::protobuf::Message>::default_instance()
    }
}

impl MonitoredResourceDescriptor {
    pub fn new() -> MonitoredResourceDescriptor {
        ::std::default::Default::default()
    }

    // string name = 5;


    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    // string type = 1;


    pub fn get_field_type(&self) -> &str {
        &self.field_type
    }
    pub fn clear_field_type(&mut self) {
        self.field_type.clear();
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: ::std::string::String) {
        self.field_type = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_field_type(&mut self) -> &mut ::std::string::String {
        &mut self.field_type
    }

    // Take field
    pub fn take_field_type(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.field_type, ::std::string::String::new())
    }

    // string display_name = 2;


    pub fn get_display_name(&self) -> &str {
        &self.display_name
    }
    pub fn clear_display_name(&mut self) {
        self.display_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_display_name(&mut self, v: ::std::string::String) {
        self.display_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_display_name(&mut self) -> &mut ::std::string::String {
        &mut self.display_name
    }

    // Take field
    pub fn take_display_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.display_name, ::std::string::String::new())
    }

    // string description = 3;


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

    // repeated .google.api.LabelDescriptor labels = 4;


    pub fn get_labels(&self) -> &[super::label::LabelDescriptor] {
        &self.labels
    }
    pub fn clear_labels(&mut self) {
        self.labels.clear();
    }

    // Param is passed by value, moved
    pub fn set_labels(&mut self, v: ::protobuf::RepeatedField<super::label::LabelDescriptor>) {
        self.labels = v;
    }

    // Mutable pointer to the field.
    pub fn mut_labels(&mut self) -> &mut ::protobuf::RepeatedField<super::label::LabelDescriptor> {
        &mut self.labels
    }

    // Take field
    pub fn take_labels(&mut self) -> ::protobuf::RepeatedField<super::label::LabelDescriptor> {
        ::std::mem::replace(&mut self.labels, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for MonitoredResourceDescriptor {
    fn is_initialized(&self) -> bool {
        for v in &self.labels {
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
                5 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.field_type)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.display_name)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.description)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.labels)?;
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
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.name);
        }
        if !self.field_type.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.field_type);
        }
        if !self.display_name.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.display_name);
        }
        if !self.description.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.description);
        }
        for value in &self.labels {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(5, &self.name)?;
        }
        if !self.field_type.is_empty() {
            os.write_string(1, &self.field_type)?;
        }
        if !self.display_name.is_empty() {
            os.write_string(2, &self.display_name)?;
        }
        if !self.description.is_empty() {
            os.write_string(3, &self.description)?;
        }
        for v in &self.labels {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

    fn new() -> MonitoredResourceDescriptor {
        MonitoredResourceDescriptor::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "name",
                |m: &MonitoredResourceDescriptor| { &m.name },
                |m: &mut MonitoredResourceDescriptor| { &mut m.name },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "type",
                |m: &MonitoredResourceDescriptor| { &m.field_type },
                |m: &mut MonitoredResourceDescriptor| { &mut m.field_type },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "display_name",
                |m: &MonitoredResourceDescriptor| { &m.display_name },
                |m: &mut MonitoredResourceDescriptor| { &mut m.display_name },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "description",
                |m: &MonitoredResourceDescriptor| { &m.description },
                |m: &mut MonitoredResourceDescriptor| { &mut m.description },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::label::LabelDescriptor>>(
                "labels",
                |m: &MonitoredResourceDescriptor| { &m.labels },
                |m: &mut MonitoredResourceDescriptor| { &mut m.labels },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<MonitoredResourceDescriptor>(
                "MonitoredResourceDescriptor",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static MonitoredResourceDescriptor {
        static instance: ::protobuf::rt::LazyV2<MonitoredResourceDescriptor> = ::protobuf::rt::LazyV2::INIT;
        instance.get(MonitoredResourceDescriptor::new)
    }
}

impl ::protobuf::Clear for MonitoredResourceDescriptor {
    fn clear(&mut self) {
        self.name.clear();
        self.field_type.clear();
        self.display_name.clear();
        self.description.clear();
        self.labels.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MonitoredResourceDescriptor {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MonitoredResourceDescriptor {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct MonitoredResource {
    // message fields
    pub field_type: ::std::string::String,
    pub labels: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a MonitoredResource {
    fn default() -> &'a MonitoredResource {
        <MonitoredResource as ::protobuf::Message>::default_instance()
    }
}

impl MonitoredResource {
    pub fn new() -> MonitoredResource {
        ::std::default::Default::default()
    }

    // string type = 1;


    pub fn get_field_type(&self) -> &str {
        &self.field_type
    }
    pub fn clear_field_type(&mut self) {
        self.field_type.clear();
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: ::std::string::String) {
        self.field_type = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_field_type(&mut self) -> &mut ::std::string::String {
        &mut self.field_type
    }

    // Take field
    pub fn take_field_type(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.field_type, ::std::string::String::new())
    }

    // repeated .google.api.MonitoredResource.LabelsEntry labels = 2;


    pub fn get_labels(&self) -> &::std::collections::HashMap<::std::string::String, ::std::string::String> {
        &self.labels
    }
    pub fn clear_labels(&mut self) {
        self.labels.clear();
    }

    // Param is passed by value, moved
    pub fn set_labels(&mut self, v: ::std::collections::HashMap<::std::string::String, ::std::string::String>) {
        self.labels = v;
    }

    // Mutable pointer to the field.
    pub fn mut_labels(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, ::std::string::String> {
        &mut self.labels
    }

    // Take field
    pub fn take_labels(&mut self) -> ::std::collections::HashMap<::std::string::String, ::std::string::String> {
        ::std::mem::replace(&mut self.labels, ::std::collections::HashMap::new())
    }
}

impl ::protobuf::Message for MonitoredResource {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.field_type)?;
                },
                2 => {
                    ::protobuf::rt::read_map_into::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(wire_type, is, &mut self.labels)?;
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
        if !self.field_type.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.field_type);
        }
        my_size += ::protobuf::rt::compute_map_size::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(2, &self.labels);
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.field_type.is_empty() {
            os.write_string(1, &self.field_type)?;
        }
        ::protobuf::rt::write_map_with_cached_sizes::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(2, &self.labels, os)?;
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

    fn new() -> MonitoredResource {
        MonitoredResource::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "type",
                |m: &MonitoredResource| { &m.field_type },
                |m: &mut MonitoredResource| { &mut m.field_type },
            ));
            fields.push(::protobuf::reflect::accessor::make_map_accessor::<_, ::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(
                "labels",
                |m: &MonitoredResource| { &m.labels },
                |m: &mut MonitoredResource| { &mut m.labels },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<MonitoredResource>(
                "MonitoredResource",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static MonitoredResource {
        static instance: ::protobuf::rt::LazyV2<MonitoredResource> = ::protobuf::rt::LazyV2::INIT;
        instance.get(MonitoredResource::new)
    }
}

impl ::protobuf::Clear for MonitoredResource {
    fn clear(&mut self) {
        self.field_type.clear();
        self.labels.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MonitoredResource {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MonitoredResource {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct MonitoredResourceMetadata {
    // message fields
    pub system_labels: ::protobuf::SingularPtrField<::protobuf::well_known_types::Struct>,
    pub user_labels: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a MonitoredResourceMetadata {
    fn default() -> &'a MonitoredResourceMetadata {
        <MonitoredResourceMetadata as ::protobuf::Message>::default_instance()
    }
}

impl MonitoredResourceMetadata {
    pub fn new() -> MonitoredResourceMetadata {
        ::std::default::Default::default()
    }

    // .google.protobuf.Struct system_labels = 1;


    pub fn get_system_labels(&self) -> &::protobuf::well_known_types::Struct {
        self.system_labels.as_ref().unwrap_or_else(|| <::protobuf::well_known_types::Struct as ::protobuf::Message>::default_instance())
    }
    pub fn clear_system_labels(&mut self) {
        self.system_labels.clear();
    }

    pub fn has_system_labels(&self) -> bool {
        self.system_labels.is_some()
    }

    // Param is passed by value, moved
    pub fn set_system_labels(&mut self, v: ::protobuf::well_known_types::Struct) {
        self.system_labels = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_system_labels(&mut self) -> &mut ::protobuf::well_known_types::Struct {
        if self.system_labels.is_none() {
            self.system_labels.set_default();
        }
        self.system_labels.as_mut().unwrap()
    }

    // Take field
    pub fn take_system_labels(&mut self) -> ::protobuf::well_known_types::Struct {
        self.system_labels.take().unwrap_or_else(|| ::protobuf::well_known_types::Struct::new())
    }

    // repeated .google.api.MonitoredResourceMetadata.UserLabelsEntry user_labels = 2;


    pub fn get_user_labels(&self) -> &::std::collections::HashMap<::std::string::String, ::std::string::String> {
        &self.user_labels
    }
    pub fn clear_user_labels(&mut self) {
        self.user_labels.clear();
    }

    // Param is passed by value, moved
    pub fn set_user_labels(&mut self, v: ::std::collections::HashMap<::std::string::String, ::std::string::String>) {
        self.user_labels = v;
    }

    // Mutable pointer to the field.
    pub fn mut_user_labels(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, ::std::string::String> {
        &mut self.user_labels
    }

    // Take field
    pub fn take_user_labels(&mut self) -> ::std::collections::HashMap<::std::string::String, ::std::string::String> {
        ::std::mem::replace(&mut self.user_labels, ::std::collections::HashMap::new())
    }
}

impl ::protobuf::Message for MonitoredResourceMetadata {
    fn is_initialized(&self) -> bool {
        for v in &self.system_labels {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.system_labels)?;
                },
                2 => {
                    ::protobuf::rt::read_map_into::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(wire_type, is, &mut self.user_labels)?;
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
        if let Some(ref v) = self.system_labels.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::compute_map_size::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(2, &self.user_labels);
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.system_labels.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        ::protobuf::rt::write_map_with_cached_sizes::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(2, &self.user_labels, os)?;
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

    fn new() -> MonitoredResourceMetadata {
        MonitoredResourceMetadata::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Struct>>(
                "system_labels",
                |m: &MonitoredResourceMetadata| { &m.system_labels },
                |m: &mut MonitoredResourceMetadata| { &mut m.system_labels },
            ));
            fields.push(::protobuf::reflect::accessor::make_map_accessor::<_, ::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(
                "user_labels",
                |m: &MonitoredResourceMetadata| { &m.user_labels },
                |m: &mut MonitoredResourceMetadata| { &mut m.user_labels },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<MonitoredResourceMetadata>(
                "MonitoredResourceMetadata",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static MonitoredResourceMetadata {
        static instance: ::protobuf::rt::LazyV2<MonitoredResourceMetadata> = ::protobuf::rt::LazyV2::INIT;
        instance.get(MonitoredResourceMetadata::new)
    }
}

impl ::protobuf::Clear for MonitoredResourceMetadata {
    fn clear(&mut self) {
        self.system_labels.clear();
        self.user_labels.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MonitoredResourceMetadata {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MonitoredResourceMetadata {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n#google/api/monitored_resource.proto\x12\ngoogle.api\x1a\x16google/api\
    /label.proto\x1a\x1cgoogle/protobuf/struct.proto\"\xbf\x01\n\x1bMonitore\
    dResourceDescriptor\x12\x12\n\x04name\x18\x05\x20\x01(\tR\x04name\x12\
    \x12\n\x04type\x18\x01\x20\x01(\tR\x04type\x12!\n\x0cdisplay_name\x18\
    \x02\x20\x01(\tR\x0bdisplayName\x12\x20\n\x0bdescription\x18\x03\x20\x01\
    (\tR\x0bdescription\x123\n\x06labels\x18\x04\x20\x03(\x0b2\x1b.google.ap\
    i.LabelDescriptorR\x06labels\"\xa5\x01\n\x11MonitoredResource\x12\x12\n\
    \x04type\x18\x01\x20\x01(\tR\x04type\x12A\n\x06labels\x18\x02\x20\x03(\
    \x0b2).google.api.MonitoredResource.LabelsEntryR\x06labels\x1a9\n\x0bLab\
    elsEntry\x12\x10\n\x03key\x18\x01\x20\x01(\tR\x03key\x12\x14\n\x05value\
    \x18\x02\x20\x01(\tR\x05value:\x028\x01\"\xf0\x01\n\x19MonitoredResource\
    Metadata\x12<\n\rsystem_labels\x18\x01\x20\x01(\x0b2\x17.google.protobuf\
    .StructR\x0csystemLabels\x12V\n\x0buser_labels\x18\x02\x20\x03(\x0b25.go\
    ogle.api.MonitoredResourceMetadata.UserLabelsEntryR\nuserLabels\x1a=\n\
    \x0fUserLabelsEntry\x12\x10\n\x03key\x18\x01\x20\x01(\tR\x03key\x12\x14\
    \n\x05value\x18\x02\x20\x01(\tR\x05value:\x028\x01By\n\x0ecom.google.api\
    B\x16MonitoredResourceProtoP\x01ZCgoogle.golang.org/genproto/googleapis/\
    api/monitoredres;monitoredres\xf8\x01\x01\xa2\x02\x04GAPIJ\xb5)\n\x06\
    \x12\x04\x0f\0s\x01\n\xbe\x04\n\x01\x0c\x12\x03\x0f\0\x122\xb3\x04\x20Co\
    pyright\x202018\x20Google\x20LLC.\n\n\x20Licensed\x20under\x20the\x20Apa\
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
    \x11\0\x13\n\t\n\x02\x03\0\x12\x03\x13\0\x20\n\t\n\x02\x03\x01\x12\x03\
    \x14\0&\n\x08\n\x01\x08\x12\x03\x16\0\x1f\n\t\n\x02\x08\x1f\x12\x03\x16\
    \0\x1f\n\x08\n\x01\x08\x12\x03\x17\0Z\n\t\n\x02\x08\x0b\x12\x03\x17\0Z\n\
    \x08\n\x01\x08\x12\x03\x18\0\"\n\t\n\x02\x08\n\x12\x03\x18\0\"\n\x08\n\
    \x01\x08\x12\x03\x19\07\n\t\n\x02\x08\x08\x12\x03\x19\07\n\x08\n\x01\x08\
    \x12\x03\x1a\0'\n\t\n\x02\x08\x01\x12\x03\x1a\0'\n\x08\n\x01\x08\x12\x03\
    \x1b\0\"\n\t\n\x02\x08$\x12\x03\x1b\0\"\n\xa2\x04\n\x02\x04\0\x12\x04'\0\
    C\x01\x1a\x95\x04\x20An\x20object\x20that\x20describes\x20the\x20schema\
    \x20of\x20a\x20[MonitoredResource][google.api.MonitoredResource]\x20obje\
    ct\x20using\x20a\n\x20type\x20name\x20and\x20a\x20set\x20of\x20labels.\
    \x20\x20For\x20example,\x20the\x20monitored\x20resource\n\x20descriptor\
    \x20for\x20Google\x20Compute\x20Engine\x20VM\x20instances\x20has\x20a\
    \x20type\x20of\n\x20`\"gce_instance\"`\x20and\x20specifies\x20the\x20use\
    \x20of\x20the\x20labels\x20`\"instance_id\"`\x20and\n\x20`\"zone\"`\x20t\
    o\x20identify\x20particular\x20VM\x20instances.\n\n\x20Different\x20APIs\
    \x20can\x20support\x20different\x20monitored\x20resource\x20types.\x20AP\
    Is\x20generally\n\x20provide\x20a\x20`list`\x20method\x20that\x20returns\
    \x20the\x20monitored\x20resource\x20descriptors\x20used\n\x20by\x20the\
    \x20API.\n\n\n\n\x03\x04\0\x01\x12\x03'\x08#\n\xa1\x03\n\x04\x04\0\x02\0\
    \x12\x03.\x02\x12\x1a\x93\x03\x20Optional.\x20The\x20resource\x20name\
    \x20of\x20the\x20monitored\x20resource\x20descriptor:\n\x20`\"projects/{\
    project_id}/monitoredResourceDescriptors/{type}\"`\x20where\n\x20{type}\
    \x20is\x20the\x20value\x20of\x20the\x20`type`\x20field\x20in\x20this\x20\
    object\x20and\n\x20{project_id}\x20is\x20a\x20project\x20ID\x20that\x20p\
    rovides\x20API-specific\x20context\x20for\n\x20accessing\x20the\x20type.\
    \x20\x20APIs\x20that\x20do\x20not\x20use\x20project\x20information\x20ca\
    n\x20use\x20the\n\x20resource\x20name\x20format\x20`\"monitoredResourceD\
    escriptors/{type}\"`.\n\n\r\n\x05\x04\0\x02\0\x04\x12\x04.\x02'%\n\x0c\n\
    \x05\x04\0\x02\0\x05\x12\x03.\x02\x08\n\x0c\n\x05\x04\0\x02\0\x01\x12\
    \x03.\t\r\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03.\x10\x11\n\xc2\x01\n\x04\
    \x04\0\x02\x01\x12\x033\x02\x12\x1a\xb4\x01\x20Required.\x20The\x20monit\
    ored\x20resource\x20type.\x20For\x20example,\x20the\x20type\n\x20`\"clou\
    dsql_database\"`\x20represents\x20databases\x20in\x20Google\x20Cloud\x20\
    SQL.\n\x20The\x20maximum\x20length\x20of\x20this\x20value\x20is\x20256\
    \x20characters.\n\n\r\n\x05\x04\0\x02\x01\x04\x12\x043\x02.\x12\n\x0c\n\
    \x05\x04\0\x02\x01\x05\x12\x033\x02\x08\n\x0c\n\x05\x04\0\x02\x01\x01\
    \x12\x033\t\r\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x033\x10\x11\n\xf5\x01\n\
    \x04\x04\0\x02\x02\x12\x039\x02\x1a\x1a\xe7\x01\x20Optional.\x20A\x20con\
    cise\x20name\x20for\x20the\x20monitored\x20resource\x20type\x20that\x20m\
    ight\x20be\n\x20displayed\x20in\x20user\x20interfaces.\x20It\x20should\
    \x20be\x20a\x20Title\x20Cased\x20Noun\x20Phrase,\n\x20without\x20any\x20\
    article\x20or\x20other\x20determiners.\x20For\x20example,\n\x20`\"Google\
    \x20Cloud\x20SQL\x20Database\"`.\n\n\r\n\x05\x04\0\x02\x02\x04\x12\x049\
    \x023\x12\n\x0c\n\x05\x04\0\x02\x02\x05\x12\x039\x02\x08\n\x0c\n\x05\x04\
    \0\x02\x02\x01\x12\x039\t\x15\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x039\x18\
    \x19\nt\n\x04\x04\0\x02\x03\x12\x03=\x02\x19\x1ag\x20Optional.\x20A\x20d\
    etailed\x20description\x20of\x20the\x20monitored\x20resource\x20type\x20\
    that\x20might\n\x20be\x20used\x20in\x20documentation.\n\n\r\n\x05\x04\0\
    \x02\x03\x04\x12\x04=\x029\x1a\n\x0c\n\x05\x04\0\x02\x03\x05\x12\x03=\
    \x02\x08\n\x0c\n\x05\x04\0\x02\x03\x01\x12\x03=\t\x14\n\x0c\n\x05\x04\0\
    \x02\x03\x03\x12\x03=\x17\x18\n\xe1\x01\n\x04\x04\0\x02\x04\x12\x03B\x02\
    &\x1a\xd3\x01\x20Required.\x20A\x20set\x20of\x20labels\x20used\x20to\x20\
    describe\x20instances\x20of\x20this\x20monitored\n\x20resource\x20type.\
    \x20For\x20example,\x20an\x20individual\x20Google\x20Cloud\x20SQL\x20dat\
    abase\x20is\n\x20identified\x20by\x20values\x20for\x20the\x20labels\x20`\
    \"database_id\"`\x20and\x20`\"zone\"`.\n\n\x0c\n\x05\x04\0\x02\x04\x04\
    \x12\x03B\x02\n\n\x0c\n\x05\x04\0\x02\x04\x06\x12\x03B\x0b\x1a\n\x0c\n\
    \x05\x04\0\x02\x04\x01\x12\x03B\x1b!\n\x0c\n\x05\x04\0\x02\x04\x03\x12\
    \x03B$%\n\xcb\x06\n\x02\x04\x01\x12\x04R\0\\\x01\x1a\xbe\x06\x20An\x20ob\
    ject\x20representing\x20a\x20resource\x20that\x20can\x20be\x20used\x20fo\
    r\x20monitoring,\x20logging,\n\x20billing,\x20or\x20other\x20purposes.\
    \x20Examples\x20include\x20virtual\x20machine\x20instances,\n\x20databas\
    es,\x20and\x20storage\x20devices\x20such\x20as\x20disks.\x20The\x20`type\
    `\x20field\x20identifies\x20a\n\x20[MonitoredResourceDescriptor][google.\
    api.MonitoredResourceDescriptor]\x20object\x20that\x20describes\x20the\
    \x20resource's\n\x20schema.\x20Information\x20in\x20the\x20`labels`\x20f\
    ield\x20identifies\x20the\x20actual\x20resource\x20and\n\x20its\x20attri\
    butes\x20according\x20to\x20the\x20schema.\x20For\x20example,\x20a\x20pa\
    rticular\x20Compute\n\x20Engine\x20VM\x20instance\x20could\x20be\x20repr\
    esented\x20by\x20the\x20following\x20object,\x20because\x20the\n\x20[Mon\
    itoredResourceDescriptor][google.api.MonitoredResourceDescriptor]\x20for\
    \x20`\"gce_instance\"`\x20has\x20labels\n\x20`\"instance_id\"`\x20and\
    \x20`\"zone\"`:\n\n\x20\x20\x20\x20\x20{\x20\"type\":\x20\"gce_instance\
    \",\n\x20\x20\x20\x20\x20\x20\x20\"labels\":\x20{\x20\"instance_id\":\
    \x20\"12345678901234\",\n\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\"zone\":\x20\"us-central1-a\"\x20}}\n\n\
    \n\n\x03\x04\x01\x01\x12\x03R\x08\x19\n\xfb\x01\n\x04\x04\x01\x02\0\x12\
    \x03V\x02\x12\x1a\xed\x01\x20Required.\x20The\x20monitored\x20resource\
    \x20type.\x20This\x20field\x20must\x20match\n\x20the\x20`type`\x20field\
    \x20of\x20a\x20[MonitoredResourceDescriptor][google.api.MonitoredResourc\
    eDescriptor]\x20object.\x20For\n\x20example,\x20the\x20type\x20of\x20a\
    \x20Compute\x20Engine\x20VM\x20instance\x20is\x20`gce_instance`.\n\n\r\n\
    \x05\x04\x01\x02\0\x04\x12\x04V\x02R\x1b\n\x0c\n\x05\x04\x01\x02\0\x05\
    \x12\x03V\x02\x08\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03V\t\r\n\x0c\n\x05\
    \x04\x01\x02\0\x03\x12\x03V\x10\x11\n\xd7\x01\n\x04\x04\x01\x02\x01\x12\
    \x03[\x02!\x1a\xc9\x01\x20Required.\x20Values\x20for\x20all\x20of\x20the\
    \x20labels\x20listed\x20in\x20the\x20associated\x20monitored\n\x20resour\
    ce\x20descriptor.\x20For\x20example,\x20Compute\x20Engine\x20VM\x20insta\
    nces\x20use\x20the\n\x20labels\x20`\"project_id\"`,\x20`\"instance_id\"`\
    ,\x20and\x20`\"zone\"`.\n\n\r\n\x05\x04\x01\x02\x01\x04\x12\x04[\x02V\
    \x12\n\x0c\n\x05\x04\x01\x02\x01\x06\x12\x03[\x02\x15\n\x0c\n\x05\x04\
    \x01\x02\x01\x01\x12\x03[\x16\x1c\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\
    \x03[\x1f\x20\n\xb9\x03\n\x02\x04\x02\x12\x04d\0s\x01\x1a\xac\x03\x20Aux\
    iliary\x20metadata\x20for\x20a\x20[MonitoredResource][google.api.Monitor\
    edResource]\x20object.\n\x20[MonitoredResource][google.api.MonitoredReso\
    urce]\x20objects\x20contain\x20the\x20minimum\x20set\x20of\x20informatio\
    n\x20to\n\x20uniquely\x20identify\x20a\x20monitored\x20resource\x20insta\
    nce.\x20There\x20is\x20some\x20other\x20useful\n\x20auxiliary\x20metadat\
    a.\x20Monitoring\x20and\x20Logging\x20use\x20an\x20ingestion\n\x20pipeli\
    ne\x20to\x20extract\x20metadata\x20for\x20cloud\x20resources\x20of\x20al\
    l\x20types,\x20and\x20store\n\x20the\x20metadata\x20in\x20this\x20messag\
    e.\n\n\n\n\x03\x04\x02\x01\x12\x03d\x08!\n\xa1\x03\n\x04\x04\x02\x02\0\
    \x12\x03o\x02+\x1a\x93\x03\x20Output\x20only.\x20Values\x20for\x20predef\
    ined\x20system\x20metadata\x20labels.\n\x20System\x20labels\x20are\x20a\
    \x20kind\x20of\x20metadata\x20extracted\x20by\x20Google,\x20including\n\
    \x20\"machine_image\",\x20\"vpc\",\x20\"subnet_id\",\n\x20\"security_gro\
    up\",\x20\"name\",\x20etc.\n\x20System\x20label\x20values\x20can\x20be\
    \x20only\x20strings,\x20Boolean\x20values,\x20or\x20a\x20list\x20of\n\
    \x20strings.\x20For\x20example:\n\n\x20\x20\x20\x20\x20{\x20\"name\":\
    \x20\"my-test-instance\",\n\x20\x20\x20\x20\x20\x20\x20\"security_group\
    \":\x20[\"a\",\x20\"b\",\x20\"c\"],\n\x20\x20\x20\x20\x20\x20\x20\"spot_\
    instance\":\x20false\x20}\n\n\r\n\x05\x04\x02\x02\0\x04\x12\x04o\x02d#\n\
    \x0c\n\x05\x04\x02\x02\0\x06\x12\x03o\x02\x18\n\x0c\n\x05\x04\x02\x02\0\
    \x01\x12\x03o\x19&\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03o)*\nB\n\x04\x04\
    \x02\x02\x01\x12\x03r\x02&\x1a5\x20Output\x20only.\x20A\x20map\x20of\x20\
    user-defined\x20metadata\x20labels.\n\n\r\n\x05\x04\x02\x02\x01\x04\x12\
    \x04r\x02o+\n\x0c\n\x05\x04\x02\x02\x01\x06\x12\x03r\x02\x15\n\x0c\n\x05\
    \x04\x02\x02\x01\x01\x12\x03r\x16!\n\x0c\n\x05\x04\x02\x02\x01\x03\x12\
    \x03r$%b\x06proto3\
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
