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
//! Generated file from `google/iam/v1/iam_policy.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_28_0;

#[derive(PartialEq,Clone,Default)]
pub struct SetIamPolicyRequest {
    // message fields
    pub resource: ::std::string::String,
    pub policy: ::protobuf::SingularPtrField<super::policy::Policy>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a SetIamPolicyRequest {
    fn default() -> &'a SetIamPolicyRequest {
        <SetIamPolicyRequest as ::protobuf::Message>::default_instance()
    }
}

impl SetIamPolicyRequest {
    pub fn new() -> SetIamPolicyRequest {
        ::std::default::Default::default()
    }

    // string resource = 1;


    pub fn get_resource(&self) -> &str {
        &self.resource
    }
    pub fn clear_resource(&mut self) {
        self.resource.clear();
    }

    // Param is passed by value, moved
    pub fn set_resource(&mut self, v: ::std::string::String) {
        self.resource = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_resource(&mut self) -> &mut ::std::string::String {
        &mut self.resource
    }

    // Take field
    pub fn take_resource(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.resource, ::std::string::String::new())
    }

    // .google.iam.v1.Policy policy = 2;


    pub fn get_policy(&self) -> &super::policy::Policy {
        self.policy.as_ref().unwrap_or_else(|| <super::policy::Policy as ::protobuf::Message>::default_instance())
    }
    pub fn clear_policy(&mut self) {
        self.policy.clear();
    }

    pub fn has_policy(&self) -> bool {
        self.policy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_policy(&mut self, v: super::policy::Policy) {
        self.policy = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_policy(&mut self) -> &mut super::policy::Policy {
        if self.policy.is_none() {
            self.policy.set_default();
        }
        self.policy.as_mut().unwrap()
    }

    // Take field
    pub fn take_policy(&mut self) -> super::policy::Policy {
        self.policy.take().unwrap_or_else(|| super::policy::Policy::new())
    }
}

impl ::protobuf::Message for SetIamPolicyRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.policy {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.resource)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.policy)?;
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
        if !self.resource.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.resource);
        }
        if let Some(ref v) = self.policy.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.resource.is_empty() {
            os.write_string(1, &self.resource)?;
        }
        if let Some(ref v) = self.policy.as_ref() {
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

    fn new() -> SetIamPolicyRequest {
        SetIamPolicyRequest::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "resource",
                |m: &SetIamPolicyRequest| { &m.resource },
                |m: &mut SetIamPolicyRequest| { &mut m.resource },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::policy::Policy>>(
                "policy",
                |m: &SetIamPolicyRequest| { &m.policy },
                |m: &mut SetIamPolicyRequest| { &mut m.policy },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<SetIamPolicyRequest>(
                "SetIamPolicyRequest",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static SetIamPolicyRequest {
        static instance: ::protobuf::rt::LazyV2<SetIamPolicyRequest> = ::protobuf::rt::LazyV2::INIT;
        instance.get(SetIamPolicyRequest::new)
    }
}

impl ::protobuf::Clear for SetIamPolicyRequest {
    fn clear(&mut self) {
        self.resource.clear();
        self.policy.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SetIamPolicyRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SetIamPolicyRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetIamPolicyRequest {
    // message fields
    pub resource: ::std::string::String,
    pub options: ::protobuf::SingularPtrField<super::options::GetPolicyOptions>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a GetIamPolicyRequest {
    fn default() -> &'a GetIamPolicyRequest {
        <GetIamPolicyRequest as ::protobuf::Message>::default_instance()
    }
}

impl GetIamPolicyRequest {
    pub fn new() -> GetIamPolicyRequest {
        ::std::default::Default::default()
    }

    // string resource = 1;


    pub fn get_resource(&self) -> &str {
        &self.resource
    }
    pub fn clear_resource(&mut self) {
        self.resource.clear();
    }

    // Param is passed by value, moved
    pub fn set_resource(&mut self, v: ::std::string::String) {
        self.resource = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_resource(&mut self) -> &mut ::std::string::String {
        &mut self.resource
    }

    // Take field
    pub fn take_resource(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.resource, ::std::string::String::new())
    }

    // .google.iam.v1.GetPolicyOptions options = 2;


    pub fn get_options(&self) -> &super::options::GetPolicyOptions {
        self.options.as_ref().unwrap_or_else(|| <super::options::GetPolicyOptions as ::protobuf::Message>::default_instance())
    }
    pub fn clear_options(&mut self) {
        self.options.clear();
    }

    pub fn has_options(&self) -> bool {
        self.options.is_some()
    }

    // Param is passed by value, moved
    pub fn set_options(&mut self, v: super::options::GetPolicyOptions) {
        self.options = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_options(&mut self) -> &mut super::options::GetPolicyOptions {
        if self.options.is_none() {
            self.options.set_default();
        }
        self.options.as_mut().unwrap()
    }

    // Take field
    pub fn take_options(&mut self) -> super::options::GetPolicyOptions {
        self.options.take().unwrap_or_else(|| super::options::GetPolicyOptions::new())
    }
}

impl ::protobuf::Message for GetIamPolicyRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.options {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.resource)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.options)?;
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
        if !self.resource.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.resource);
        }
        if let Some(ref v) = self.options.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.resource.is_empty() {
            os.write_string(1, &self.resource)?;
        }
        if let Some(ref v) = self.options.as_ref() {
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

    fn new() -> GetIamPolicyRequest {
        GetIamPolicyRequest::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "resource",
                |m: &GetIamPolicyRequest| { &m.resource },
                |m: &mut GetIamPolicyRequest| { &mut m.resource },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::options::GetPolicyOptions>>(
                "options",
                |m: &GetIamPolicyRequest| { &m.options },
                |m: &mut GetIamPolicyRequest| { &mut m.options },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<GetIamPolicyRequest>(
                "GetIamPolicyRequest",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static GetIamPolicyRequest {
        static instance: ::protobuf::rt::LazyV2<GetIamPolicyRequest> = ::protobuf::rt::LazyV2::INIT;
        instance.get(GetIamPolicyRequest::new)
    }
}

impl ::protobuf::Clear for GetIamPolicyRequest {
    fn clear(&mut self) {
        self.resource.clear();
        self.options.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetIamPolicyRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetIamPolicyRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TestIamPermissionsRequest {
    // message fields
    pub resource: ::std::string::String,
    pub permissions: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a TestIamPermissionsRequest {
    fn default() -> &'a TestIamPermissionsRequest {
        <TestIamPermissionsRequest as ::protobuf::Message>::default_instance()
    }
}

impl TestIamPermissionsRequest {
    pub fn new() -> TestIamPermissionsRequest {
        ::std::default::Default::default()
    }

    // string resource = 1;


    pub fn get_resource(&self) -> &str {
        &self.resource
    }
    pub fn clear_resource(&mut self) {
        self.resource.clear();
    }

    // Param is passed by value, moved
    pub fn set_resource(&mut self, v: ::std::string::String) {
        self.resource = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_resource(&mut self) -> &mut ::std::string::String {
        &mut self.resource
    }

    // Take field
    pub fn take_resource(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.resource, ::std::string::String::new())
    }

    // repeated string permissions = 2;


    pub fn get_permissions(&self) -> &[::std::string::String] {
        &self.permissions
    }
    pub fn clear_permissions(&mut self) {
        self.permissions.clear();
    }

    // Param is passed by value, moved
    pub fn set_permissions(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.permissions = v;
    }

    // Mutable pointer to the field.
    pub fn mut_permissions(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.permissions
    }

    // Take field
    pub fn take_permissions(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.permissions, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for TestIamPermissionsRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.resource)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.permissions)?;
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
        if !self.resource.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.resource);
        }
        for value in &self.permissions {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.resource.is_empty() {
            os.write_string(1, &self.resource)?;
        }
        for v in &self.permissions {
            os.write_string(2, &v)?;
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

    fn new() -> TestIamPermissionsRequest {
        TestIamPermissionsRequest::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "resource",
                |m: &TestIamPermissionsRequest| { &m.resource },
                |m: &mut TestIamPermissionsRequest| { &mut m.resource },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "permissions",
                |m: &TestIamPermissionsRequest| { &m.permissions },
                |m: &mut TestIamPermissionsRequest| { &mut m.permissions },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<TestIamPermissionsRequest>(
                "TestIamPermissionsRequest",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static TestIamPermissionsRequest {
        static instance: ::protobuf::rt::LazyV2<TestIamPermissionsRequest> = ::protobuf::rt::LazyV2::INIT;
        instance.get(TestIamPermissionsRequest::new)
    }
}

impl ::protobuf::Clear for TestIamPermissionsRequest {
    fn clear(&mut self) {
        self.resource.clear();
        self.permissions.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TestIamPermissionsRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TestIamPermissionsRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TestIamPermissionsResponse {
    // message fields
    pub permissions: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a TestIamPermissionsResponse {
    fn default() -> &'a TestIamPermissionsResponse {
        <TestIamPermissionsResponse as ::protobuf::Message>::default_instance()
    }
}

impl TestIamPermissionsResponse {
    pub fn new() -> TestIamPermissionsResponse {
        ::std::default::Default::default()
    }

    // repeated string permissions = 1;


    pub fn get_permissions(&self) -> &[::std::string::String] {
        &self.permissions
    }
    pub fn clear_permissions(&mut self) {
        self.permissions.clear();
    }

    // Param is passed by value, moved
    pub fn set_permissions(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.permissions = v;
    }

    // Mutable pointer to the field.
    pub fn mut_permissions(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.permissions
    }

    // Take field
    pub fn take_permissions(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.permissions, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for TestIamPermissionsResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.permissions)?;
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
        for value in &self.permissions {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        for v in &self.permissions {
            os.write_string(1, &v)?;
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

    fn new() -> TestIamPermissionsResponse {
        TestIamPermissionsResponse::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "permissions",
                |m: &TestIamPermissionsResponse| { &m.permissions },
                |m: &mut TestIamPermissionsResponse| { &mut m.permissions },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<TestIamPermissionsResponse>(
                "TestIamPermissionsResponse",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static TestIamPermissionsResponse {
        static instance: ::protobuf::rt::LazyV2<TestIamPermissionsResponse> = ::protobuf::rt::LazyV2::INIT;
        instance.get(TestIamPermissionsResponse::new)
    }
}

impl ::protobuf::Clear for TestIamPermissionsResponse {
    fn clear(&mut self) {
        self.permissions.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TestIamPermissionsResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TestIamPermissionsResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1egoogle/iam/v1/iam_policy.proto\x12\rgoogle.iam.v1\x1a\x1bgoogle/ia\
    m/v1/options.proto\x1a\x1agoogle/iam/v1/policy.proto\x1a\x1cgoogle/api/a\
    nnotations.proto\x1a\x17google/api/client.proto\x1a\x1fgoogle/api/field_\
    behavior.proto\x1a\x19google/api/resource.proto\"p\n\x13SetIamPolicyRequ\
    est\x12%\n\x08resource\x18\x01\x20\x01(\tR\x08resourceB\t\xfaA\x03\n\x01\
    *\xe0A\x02\x122\n\x06policy\x18\x02\x20\x01(\x0b2\x15.google.iam.v1.Poli\
    cyR\x06policyB\x03\xe0A\x02\"w\n\x13GetIamPolicyRequest\x12%\n\x08resour\
    ce\x18\x01\x20\x01(\tR\x08resourceB\t\xfaA\x03\n\x01*\xe0A\x02\x129\n\
    \x07options\x18\x02\x20\x01(\x0b2\x1f.google.iam.v1.GetPolicyOptionsR\
    \x07options\"i\n\x19TestIamPermissionsRequest\x12%\n\x08resource\x18\x01\
    \x20\x01(\tR\x08resourceB\t\xfaA\x03\n\x01*\xe0A\x02\x12%\n\x0bpermissio\
    ns\x18\x02\x20\x03(\tR\x0bpermissionsB\x03\xe0A\x02\">\n\x1aTestIamPermi\
    ssionsResponse\x12\x20\n\x0bpermissions\x18\x01\x20\x03(\tR\x0bpermissio\
    ns2\xb4\x03\n\tIAMPolicy\x12t\n\x0cSetIamPolicy\x12\".google.iam.v1.SetI\
    amPolicyRequest\x1a\x15.google.iam.v1.Policy\")\x82\xd3\xe4\x93\x02#\"\
    \x1e/v1/{resource=**}:setIamPolicy:\x01*\x12t\n\x0cGetIamPolicy\x12\".go\
    ogle.iam.v1.GetIamPolicyRequest\x1a\x15.google.iam.v1.Policy\")\x82\xd3\
    \xe4\x93\x02#\"\x1e/v1/{resource=**}:getIamPolicy:\x01*\x12\x9a\x01\n\
    \x12TestIamPermissions\x12(.google.iam.v1.TestIamPermissionsRequest\x1a)\
    .google.iam.v1.TestIamPermissionsResponse\"/\x82\xd3\xe4\x93\x02)\"$/v1/\
    {resource=**}:testIamPermissions:\x01*\x1a\x1e\xcaA\x1biam-meta-api.goog\
    leapis.comB\x86\x01\n\x11com.google.iam.v1B\x0eIamPolicyProtoP\x01Z0goog\
    le.golang.org/genproto/googleapis/iam/v1;iam\xf8\x01\x01\xaa\x02\x13Goog\
    le.Cloud.Iam.V1\xca\x02\x13Google\\Cloud\\Iam\\V1J\xe4$\n\x07\x12\x05\
    \x0f\0\x90\x01\x01\n\xbe\x04\n\x01\x0c\x12\x03\x0f\0\x122\xb3\x04\x20Cop\
    yright\x202019\x20Google\x20LLC.\n\n\x20Licensed\x20under\x20the\x20Apac\
    he\x20License,\x20Version\x202.0\x20(the\x20\"License\");\n\x20you\x20ma\
    y\x20not\x20use\x20this\x20file\x20except\x20in\x20compliance\x20with\
    \x20the\x20License.\n\x20You\x20may\x20obtain\x20a\x20copy\x20of\x20the\
    \x20License\x20at\n\n\x20\x20\x20\x20\x20http://www.apache.org/licenses/\
    LICENSE-2.0\n\n\x20Unless\x20required\x20by\x20applicable\x20law\x20or\
    \x20agreed\x20to\x20in\x20writing,\x20software\n\x20distributed\x20under\
    \x20the\x20License\x20is\x20distributed\x20on\x20an\x20\"AS\x20IS\"\x20B\
    ASIS,\n\x20WITHOUT\x20WARRANTIES\x20OR\x20CONDITIONS\x20OF\x20ANY\x20KIN\
    D,\x20either\x20express\x20or\x20implied.\n\x20See\x20the\x20License\x20\
    for\x20the\x20specific\x20language\x20governing\x20permissions\x20and\n\
    \x20limitations\x20under\x20the\x20License.\n\n\n\x08\n\x01\x02\x12\x03\
    \x11\0\x16\n\t\n\x02\x03\0\x12\x03\x13\0%\n\t\n\x02\x03\x01\x12\x03\x14\
    \0$\n\t\n\x02\x03\x02\x12\x03\x15\0&\n\t\n\x02\x03\x03\x12\x03\x16\0!\n\
    \t\n\x02\x03\x04\x12\x03\x17\0)\n\t\n\x02\x03\x05\x12\x03\x18\0#\n\x08\n\
    \x01\x08\x12\x03\x1a\0\x1f\n\t\n\x02\x08\x1f\x12\x03\x1a\0\x1f\n\x08\n\
    \x01\x08\x12\x03\x1b\00\n\t\n\x02\x08%\x12\x03\x1b\00\n\x08\n\x01\x08\
    \x12\x03\x1c\0G\n\t\n\x02\x08\x0b\x12\x03\x1c\0G\n\x08\n\x01\x08\x12\x03\
    \x1d\0\"\n\t\n\x02\x08\n\x12\x03\x1d\0\"\n\x08\n\x01\x08\x12\x03\x1e\0/\
    \n\t\n\x02\x08\x08\x12\x03\x1e\0/\n\x08\n\x01\x08\x12\x03\x1f\0*\n\t\n\
    \x02\x08\x01\x12\x03\x1f\0*\n\x08\n\x01\x08\x12\x03\x20\00\n\t\n\x02\x08\
    )\x12\x03\x20\00\n\xbb\x07\n\x02\x06\0\x12\x04;\0^\x01\x1a\xae\x07\x20##\
    \x20API\x20Overview\n\n\x20Manages\x20Identity\x20and\x20Access\x20Manag\
    ement\x20(IAM)\x20policies.\n\n\x20Any\x20implementation\x20of\x20an\x20\
    API\x20that\x20offers\x20access\x20control\x20features\n\x20implements\
    \x20the\x20google.iam.v1.IAMPolicy\x20interface.\n\n\x20##\x20Data\x20mo\
    del\n\n\x20Access\x20control\x20is\x20applied\x20when\x20a\x20principal\
    \x20(user\x20or\x20service\x20account),\x20takes\n\x20some\x20action\x20\
    on\x20a\x20resource\x20exposed\x20by\x20a\x20service.\x20Resources,\x20i\
    dentified\x20by\n\x20URI-like\x20names,\x20are\x20the\x20unit\x20of\x20a\
    ccess\x20control\x20specification.\x20Service\n\x20implementations\x20ca\
    n\x20choose\x20the\x20granularity\x20of\x20access\x20control\x20and\x20t\
    he\n\x20supported\x20permissions\x20for\x20their\x20resources.\n\x20For\
    \x20example\x20one\x20database\x20service\x20may\x20allow\x20access\x20c\
    ontrol\x20to\x20be\n\x20specified\x20only\x20at\x20the\x20Table\x20level\
    ,\x20whereas\x20another\x20might\x20allow\x20access\x20control\n\x20to\
    \x20also\x20be\x20specified\x20at\x20the\x20Column\x20level.\n\n\x20##\
    \x20Policy\x20Structure\n\n\x20See\x20google.iam.v1.Policy\n\n\x20This\
    \x20is\x20intentionally\x20not\x20a\x20CRUD\x20style\x20API\x20because\
    \x20access\x20control\x20policies\n\x20are\x20created\x20and\x20deleted\
    \x20implicitly\x20with\x20the\x20resources\x20to\x20which\x20they\x20are\
    \n\x20attached.\n\n\n\n\x03\x06\0\x01\x12\x03;\x08\x11\n\n\n\x03\x06\0\
    \x03\x12\x03<\x02C\n\x0c\n\x05\x06\0\x03\x99\x08\x12\x03<\x02C\nh\n\x04\
    \x06\0\x02\0\x12\x04@\x02E\x03\x1aZ\x20Sets\x20the\x20access\x20control\
    \x20policy\x20on\x20the\x20specified\x20resource.\x20Replaces\x20any\n\
    \x20existing\x20policy.\n\n\x0c\n\x05\x06\0\x02\0\x01\x12\x03@\x06\x12\n\
    \x0c\n\x05\x06\0\x02\0\x02\x12\x03@\x13&\n\x0c\n\x05\x06\0\x02\0\x03\x12\
    \x03@17\n\r\n\x05\x06\0\x02\0\x04\x12\x04A\x04D\x06\n\x11\n\t\x06\0\x02\
    \0\x04\xb0\xca\xbc\"\x12\x04A\x04D\x06\n\x90\x01\n\x04\x06\0\x02\x01\x12\
    \x04J\x02O\x03\x1a\x81\x01\x20Gets\x20the\x20access\x20control\x20policy\
    \x20for\x20a\x20resource.\n\x20Returns\x20an\x20empty\x20policy\x20if\
    \x20the\x20resource\x20exists\x20and\x20does\x20not\x20have\x20a\x20poli\
    cy\n\x20set.\n\n\x0c\n\x05\x06\0\x02\x01\x01\x12\x03J\x06\x12\n\x0c\n\
    \x05\x06\0\x02\x01\x02\x12\x03J\x13&\n\x0c\n\x05\x06\0\x02\x01\x03\x12\
    \x03J17\n\r\n\x05\x06\0\x02\x01\x04\x12\x04K\x04N\x06\n\x11\n\t\x06\0\
    \x02\x01\x04\xb0\xca\xbc\"\x12\x04K\x04N\x06\n\xf2\x02\n\x04\x06\0\x02\
    \x02\x12\x04X\x02]\x03\x1a\xe3\x02\x20Returns\x20permissions\x20that\x20\
    a\x20caller\x20has\x20on\x20the\x20specified\x20resource.\n\x20If\x20the\
    \x20resource\x20does\x20not\x20exist,\x20this\x20will\x20return\x20an\
    \x20empty\x20set\x20of\n\x20permissions,\x20not\x20a\x20NOT_FOUND\x20err\
    or.\n\n\x20Note:\x20This\x20operation\x20is\x20designed\x20to\x20be\x20u\
    sed\x20for\x20building\x20permission-aware\n\x20UIs\x20and\x20command-li\
    ne\x20tools,\x20not\x20for\x20authorization\x20checking.\x20This\x20oper\
    ation\n\x20may\x20\"fail\x20open\"\x20without\x20warning.\n\n\x0c\n\x05\
    \x06\0\x02\x02\x01\x12\x03X\x06\x18\n\x0c\n\x05\x06\0\x02\x02\x02\x12\
    \x03X\x192\n\x0c\n\x05\x06\0\x02\x02\x03\x12\x03X=W\n\r\n\x05\x06\0\x02\
    \x02\x04\x12\x04Y\x04\\\x06\n\x11\n\t\x06\0\x02\x02\x04\xb0\xca\xbc\"\
    \x12\x04Y\x04\\\x06\n8\n\x02\x04\0\x12\x04a\0m\x01\x1a,\x20Request\x20me\
    ssage\x20for\x20`SetIamPolicy`\x20method.\n\n\n\n\x03\x04\0\x01\x12\x03a\
    \x08\x1b\n\x9b\x01\n\x04\x04\0\x02\0\x12\x04d\x02f0\x1a\x8c\x01\x20REQUI\
    RED:\x20The\x20resource\x20for\x20which\x20the\x20policy\x20is\x20being\
    \x20specified.\n\x20See\x20the\x20operation\x20documentation\x20for\x20t\
    he\x20appropriate\x20value\x20for\x20this\x20field.\n\n\x0c\n\x05\x04\0\
    \x02\0\x05\x12\x03d\x02\x08\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03d\t\x11\n\
    \x0c\n\x05\x04\0\x02\0\x03\x12\x03d\x14\x15\n\r\n\x05\x04\0\x02\0\x08\
    \x12\x04d\x16f/\n\x0f\n\x08\x04\0\x02\0\x08\x9c\x08\0\x12\x03e\x04*\n\
    \x0f\n\x08\x04\0\x02\0\x08\x9f\x08\x01\x12\x03f\x04.\n\xf3\x01\n\x04\x04\
    \0\x02\x01\x12\x03l\x02=\x1a\xe5\x01\x20REQUIRED:\x20The\x20complete\x20\
    policy\x20to\x20be\x20applied\x20to\x20the\x20`resource`.\x20The\x20size\
    \x20of\n\x20the\x20policy\x20is\x20limited\x20to\x20a\x20few\x2010s\x20o\
    f\x20KB.\x20An\x20empty\x20policy\x20is\x20a\n\x20valid\x20policy\x20but\
    \x20certain\x20Cloud\x20Platform\x20services\x20(such\x20as\x20Projects)\
    \n\x20might\x20reject\x20them.\n\n\x0c\n\x05\x04\0\x02\x01\x06\x12\x03l\
    \x02\x08\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03l\t\x0f\n\x0c\n\x05\x04\0\
    \x02\x01\x03\x12\x03l\x12\x13\n\x0c\n\x05\x04\0\x02\x01\x08\x12\x03l\x14\
    <\n\x0f\n\x08\x04\0\x02\x01\x08\x9c\x08\0\x12\x03l\x15;\n8\n\x02\x04\x01\
    \x12\x04p\0z\x01\x1a,\x20Request\x20message\x20for\x20`GetIamPolicy`\x20\
    method.\n\n\n\n\x03\x04\x01\x01\x12\x03p\x08\x1b\n\x9b\x01\n\x04\x04\x01\
    \x02\0\x12\x04s\x02u0\x1a\x8c\x01\x20REQUIRED:\x20The\x20resource\x20for\
    \x20which\x20the\x20policy\x20is\x20being\x20requested.\n\x20See\x20the\
    \x20operation\x20documentation\x20for\x20the\x20appropriate\x20value\x20\
    for\x20this\x20field.\n\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03s\x02\x08\n\
    \x0c\n\x05\x04\x01\x02\0\x01\x12\x03s\t\x11\n\x0c\n\x05\x04\x01\x02\0\
    \x03\x12\x03s\x14\x15\n\r\n\x05\x04\x01\x02\0\x08\x12\x04s\x16u/\n\x0f\n\
    \x08\x04\x01\x02\0\x08\x9c\x08\0\x12\x03t\x04*\n\x0f\n\x08\x04\x01\x02\0\
    \x08\x9f\x08\x01\x12\x03u\x04.\n\x85\x01\n\x04\x04\x01\x02\x01\x12\x03y\
    \x02\x1f\x1ax\x20OPTIONAL:\x20A\x20`GetPolicyOptions`\x20object\x20for\
    \x20specifying\x20options\x20to\n\x20`GetIamPolicy`.\x20This\x20field\
    \x20is\x20only\x20used\x20by\x20Cloud\x20IAM.\n\n\x0c\n\x05\x04\x01\x02\
    \x01\x06\x12\x03y\x02\x12\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\x03y\x13\
    \x1a\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03y\x1d\x1e\n?\n\x02\x04\x02\
    \x12\x05}\0\x89\x01\x01\x1a2\x20Request\x20message\x20for\x20`TestIamPer\
    missions`\x20method.\n\n\n\n\x03\x04\x02\x01\x12\x03}\x08!\n\xa4\x01\n\
    \x04\x04\x02\x02\0\x12\x06\x80\x01\x02\x82\x010\x1a\x93\x01\x20REQUIRED:\
    \x20The\x20resource\x20for\x20which\x20the\x20policy\x20detail\x20is\x20\
    being\x20requested.\n\x20See\x20the\x20operation\x20documentation\x20for\
    \x20the\x20appropriate\x20value\x20for\x20this\x20field.\n\n\r\n\x05\x04\
    \x02\x02\0\x05\x12\x04\x80\x01\x02\x08\n\r\n\x05\x04\x02\x02\0\x01\x12\
    \x04\x80\x01\t\x11\n\r\n\x05\x04\x02\x02\0\x03\x12\x04\x80\x01\x14\x15\n\
    \x0f\n\x05\x04\x02\x02\0\x08\x12\x06\x80\x01\x15\x82\x01/\n\x10\n\x08\
    \x04\x02\x02\0\x08\x9c\x08\0\x12\x04\x81\x01\x04*\n\x10\n\x08\x04\x02\
    \x02\0\x08\x9f\x08\x01\x12\x04\x82\x01\x04.\n\xf1\x01\n\x04\x04\x02\x02\
    \x01\x12\x04\x88\x01\x02K\x1a\xe2\x01\x20The\x20set\x20of\x20permissions\
    \x20to\x20check\x20for\x20the\x20`resource`.\x20Permissions\x20with\n\
    \x20wildcards\x20(such\x20as\x20'*'\x20or\x20'storage.*')\x20are\x20not\
    \x20allowed.\x20For\x20more\n\x20information\x20see\n\x20[IAM\x20Overvie\
    w](https://cloud.google.com/iam/docs/overview#permissions).\n\n\r\n\x05\
    \x04\x02\x02\x01\x04\x12\x04\x88\x01\x02\n\n\r\n\x05\x04\x02\x02\x01\x05\
    \x12\x04\x88\x01\x0b\x11\n\r\n\x05\x04\x02\x02\x01\x01\x12\x04\x88\x01\
    \x12\x1d\n\r\n\x05\x04\x02\x02\x01\x03\x12\x04\x88\x01\x20!\n\r\n\x05\
    \x04\x02\x02\x01\x08\x12\x04\x88\x01\"J\n\x10\n\x08\x04\x02\x02\x01\x08\
    \x9c\x08\0\x12\x04\x88\x01#I\nA\n\x02\x04\x03\x12\x06\x8c\x01\0\x90\x01\
    \x01\x1a3\x20Response\x20message\x20for\x20`TestIamPermissions`\x20metho\
    d.\n\n\x0b\n\x03\x04\x03\x01\x12\x04\x8c\x01\x08\"\n]\n\x04\x04\x03\x02\
    \0\x12\x04\x8f\x01\x02\"\x1aO\x20A\x20subset\x20of\x20`TestPermissionsRe\
    quest.permissions`\x20that\x20the\x20caller\x20is\n\x20allowed.\n\n\r\n\
    \x05\x04\x03\x02\0\x04\x12\x04\x8f\x01\x02\n\n\r\n\x05\x04\x03\x02\0\x05\
    \x12\x04\x8f\x01\x0b\x11\n\r\n\x05\x04\x03\x02\0\x01\x12\x04\x8f\x01\x12\
    \x1d\n\r\n\x05\x04\x03\x02\0\x03\x12\x04\x8f\x01\x20!b\x06proto3\
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
