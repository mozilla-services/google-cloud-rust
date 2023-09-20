// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_ACCESS_CONTEXT_MANAGER_LIST_ACCESS_POLICIES: ::grpcio::Method<super::access_context_manager::ListAccessPoliciesRequest, super::access_context_manager::ListAccessPoliciesResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.identity.accesscontextmanager.v1.AccessContextManager/ListAccessPolicies",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ACCESS_CONTEXT_MANAGER_GET_ACCESS_POLICY: ::grpcio::Method<super::access_context_manager::GetAccessPolicyRequest, super::access_policy::AccessPolicy> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.identity.accesscontextmanager.v1.AccessContextManager/GetAccessPolicy",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ACCESS_CONTEXT_MANAGER_CREATE_ACCESS_POLICY: ::grpcio::Method<super::access_policy::AccessPolicy, super::operations::Operation> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.identity.accesscontextmanager.v1.AccessContextManager/CreateAccessPolicy",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ACCESS_CONTEXT_MANAGER_UPDATE_ACCESS_POLICY: ::grpcio::Method<super::access_context_manager::UpdateAccessPolicyRequest, super::operations::Operation> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.identity.accesscontextmanager.v1.AccessContextManager/UpdateAccessPolicy",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ACCESS_CONTEXT_MANAGER_DELETE_ACCESS_POLICY: ::grpcio::Method<super::access_context_manager::DeleteAccessPolicyRequest, super::operations::Operation> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.identity.accesscontextmanager.v1.AccessContextManager/DeleteAccessPolicy",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ACCESS_CONTEXT_MANAGER_LIST_ACCESS_LEVELS: ::grpcio::Method<super::access_context_manager::ListAccessLevelsRequest, super::access_context_manager::ListAccessLevelsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.identity.accesscontextmanager.v1.AccessContextManager/ListAccessLevels",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ACCESS_CONTEXT_MANAGER_GET_ACCESS_LEVEL: ::grpcio::Method<super::access_context_manager::GetAccessLevelRequest, super::access_level::AccessLevel> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.identity.accesscontextmanager.v1.AccessContextManager/GetAccessLevel",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ACCESS_CONTEXT_MANAGER_CREATE_ACCESS_LEVEL: ::grpcio::Method<super::access_context_manager::CreateAccessLevelRequest, super::operations::Operation> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.identity.accesscontextmanager.v1.AccessContextManager/CreateAccessLevel",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ACCESS_CONTEXT_MANAGER_UPDATE_ACCESS_LEVEL: ::grpcio::Method<super::access_context_manager::UpdateAccessLevelRequest, super::operations::Operation> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.identity.accesscontextmanager.v1.AccessContextManager/UpdateAccessLevel",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ACCESS_CONTEXT_MANAGER_DELETE_ACCESS_LEVEL: ::grpcio::Method<super::access_context_manager::DeleteAccessLevelRequest, super::operations::Operation> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.identity.accesscontextmanager.v1.AccessContextManager/DeleteAccessLevel",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ACCESS_CONTEXT_MANAGER_REPLACE_ACCESS_LEVELS: ::grpcio::Method<super::access_context_manager::ReplaceAccessLevelsRequest, super::operations::Operation> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.identity.accesscontextmanager.v1.AccessContextManager/ReplaceAccessLevels",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ACCESS_CONTEXT_MANAGER_LIST_SERVICE_PERIMETERS: ::grpcio::Method<super::access_context_manager::ListServicePerimetersRequest, super::access_context_manager::ListServicePerimetersResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.identity.accesscontextmanager.v1.AccessContextManager/ListServicePerimeters",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ACCESS_CONTEXT_MANAGER_GET_SERVICE_PERIMETER: ::grpcio::Method<super::access_context_manager::GetServicePerimeterRequest, super::service_perimeter::ServicePerimeter> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.identity.accesscontextmanager.v1.AccessContextManager/GetServicePerimeter",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ACCESS_CONTEXT_MANAGER_CREATE_SERVICE_PERIMETER: ::grpcio::Method<super::access_context_manager::CreateServicePerimeterRequest, super::operations::Operation> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.identity.accesscontextmanager.v1.AccessContextManager/CreateServicePerimeter",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ACCESS_CONTEXT_MANAGER_UPDATE_SERVICE_PERIMETER: ::grpcio::Method<super::access_context_manager::UpdateServicePerimeterRequest, super::operations::Operation> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.identity.accesscontextmanager.v1.AccessContextManager/UpdateServicePerimeter",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ACCESS_CONTEXT_MANAGER_DELETE_SERVICE_PERIMETER: ::grpcio::Method<super::access_context_manager::DeleteServicePerimeterRequest, super::operations::Operation> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.identity.accesscontextmanager.v1.AccessContextManager/DeleteServicePerimeter",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ACCESS_CONTEXT_MANAGER_REPLACE_SERVICE_PERIMETERS: ::grpcio::Method<super::access_context_manager::ReplaceServicePerimetersRequest, super::operations::Operation> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.identity.accesscontextmanager.v1.AccessContextManager/ReplaceServicePerimeters",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ACCESS_CONTEXT_MANAGER_COMMIT_SERVICE_PERIMETERS: ::grpcio::Method<super::access_context_manager::CommitServicePerimetersRequest, super::operations::Operation> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.identity.accesscontextmanager.v1.AccessContextManager/CommitServicePerimeters",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ACCESS_CONTEXT_MANAGER_LIST_GCP_USER_ACCESS_BINDINGS: ::grpcio::Method<super::access_context_manager::ListGcpUserAccessBindingsRequest, super::access_context_manager::ListGcpUserAccessBindingsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.identity.accesscontextmanager.v1.AccessContextManager/ListGcpUserAccessBindings",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ACCESS_CONTEXT_MANAGER_GET_GCP_USER_ACCESS_BINDING: ::grpcio::Method<super::access_context_manager::GetGcpUserAccessBindingRequest, super::gcp_user_access_binding::GcpUserAccessBinding> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.identity.accesscontextmanager.v1.AccessContextManager/GetGcpUserAccessBinding",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ACCESS_CONTEXT_MANAGER_CREATE_GCP_USER_ACCESS_BINDING: ::grpcio::Method<super::access_context_manager::CreateGcpUserAccessBindingRequest, super::operations::Operation> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.identity.accesscontextmanager.v1.AccessContextManager/CreateGcpUserAccessBinding",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ACCESS_CONTEXT_MANAGER_UPDATE_GCP_USER_ACCESS_BINDING: ::grpcio::Method<super::access_context_manager::UpdateGcpUserAccessBindingRequest, super::operations::Operation> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.identity.accesscontextmanager.v1.AccessContextManager/UpdateGcpUserAccessBinding",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ACCESS_CONTEXT_MANAGER_DELETE_GCP_USER_ACCESS_BINDING: ::grpcio::Method<super::access_context_manager::DeleteGcpUserAccessBindingRequest, super::operations::Operation> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.identity.accesscontextmanager.v1.AccessContextManager/DeleteGcpUserAccessBinding",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ACCESS_CONTEXT_MANAGER_SET_IAM_POLICY: ::grpcio::Method<super::iam_policy::SetIamPolicyRequest, super::policy::Policy> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.identity.accesscontextmanager.v1.AccessContextManager/SetIamPolicy",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ACCESS_CONTEXT_MANAGER_GET_IAM_POLICY: ::grpcio::Method<super::iam_policy::GetIamPolicyRequest, super::policy::Policy> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.identity.accesscontextmanager.v1.AccessContextManager/GetIamPolicy",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ACCESS_CONTEXT_MANAGER_TEST_IAM_PERMISSIONS: ::grpcio::Method<super::iam_policy::TestIamPermissionsRequest, super::iam_policy::TestIamPermissionsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.identity.accesscontextmanager.v1.AccessContextManager/TestIamPermissions",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct AccessContextManagerClient {
    client: ::grpcio::Client,
}

impl AccessContextManagerClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        AccessContextManagerClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn list_access_policies_opt(&self, req: &super::access_context_manager::ListAccessPoliciesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::access_context_manager::ListAccessPoliciesResponse> {
        self.client.unary_call(&METHOD_ACCESS_CONTEXT_MANAGER_LIST_ACCESS_POLICIES, req, opt)
    }

    pub fn list_access_policies(&self, req: &super::access_context_manager::ListAccessPoliciesRequest) -> ::grpcio::Result<super::access_context_manager::ListAccessPoliciesResponse> {
        self.list_access_policies_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_access_policies_async_opt(&self, req: &super::access_context_manager::ListAccessPoliciesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::access_context_manager::ListAccessPoliciesResponse>> {
        self.client.unary_call_async(&METHOD_ACCESS_CONTEXT_MANAGER_LIST_ACCESS_POLICIES, req, opt)
    }

    pub fn list_access_policies_async(&self, req: &super::access_context_manager::ListAccessPoliciesRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::access_context_manager::ListAccessPoliciesResponse>> {
        self.list_access_policies_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_access_policy_opt(&self, req: &super::access_context_manager::GetAccessPolicyRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::access_policy::AccessPolicy> {
        self.client.unary_call(&METHOD_ACCESS_CONTEXT_MANAGER_GET_ACCESS_POLICY, req, opt)
    }

    pub fn get_access_policy(&self, req: &super::access_context_manager::GetAccessPolicyRequest) -> ::grpcio::Result<super::access_policy::AccessPolicy> {
        self.get_access_policy_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_access_policy_async_opt(&self, req: &super::access_context_manager::GetAccessPolicyRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::access_policy::AccessPolicy>> {
        self.client.unary_call_async(&METHOD_ACCESS_CONTEXT_MANAGER_GET_ACCESS_POLICY, req, opt)
    }

    pub fn get_access_policy_async(&self, req: &super::access_context_manager::GetAccessPolicyRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::access_policy::AccessPolicy>> {
        self.get_access_policy_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_access_policy_opt(&self, req: &super::access_policy::AccessPolicy, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::operations::Operation> {
        self.client.unary_call(&METHOD_ACCESS_CONTEXT_MANAGER_CREATE_ACCESS_POLICY, req, opt)
    }

    pub fn create_access_policy(&self, req: &super::access_policy::AccessPolicy) -> ::grpcio::Result<super::operations::Operation> {
        self.create_access_policy_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_access_policy_async_opt(&self, req: &super::access_policy::AccessPolicy, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operations::Operation>> {
        self.client.unary_call_async(&METHOD_ACCESS_CONTEXT_MANAGER_CREATE_ACCESS_POLICY, req, opt)
    }

    pub fn create_access_policy_async(&self, req: &super::access_policy::AccessPolicy) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operations::Operation>> {
        self.create_access_policy_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_access_policy_opt(&self, req: &super::access_context_manager::UpdateAccessPolicyRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::operations::Operation> {
        self.client.unary_call(&METHOD_ACCESS_CONTEXT_MANAGER_UPDATE_ACCESS_POLICY, req, opt)
    }

    pub fn update_access_policy(&self, req: &super::access_context_manager::UpdateAccessPolicyRequest) -> ::grpcio::Result<super::operations::Operation> {
        self.update_access_policy_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_access_policy_async_opt(&self, req: &super::access_context_manager::UpdateAccessPolicyRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operations::Operation>> {
        self.client.unary_call_async(&METHOD_ACCESS_CONTEXT_MANAGER_UPDATE_ACCESS_POLICY, req, opt)
    }

    pub fn update_access_policy_async(&self, req: &super::access_context_manager::UpdateAccessPolicyRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operations::Operation>> {
        self.update_access_policy_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_access_policy_opt(&self, req: &super::access_context_manager::DeleteAccessPolicyRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::operations::Operation> {
        self.client.unary_call(&METHOD_ACCESS_CONTEXT_MANAGER_DELETE_ACCESS_POLICY, req, opt)
    }

    pub fn delete_access_policy(&self, req: &super::access_context_manager::DeleteAccessPolicyRequest) -> ::grpcio::Result<super::operations::Operation> {
        self.delete_access_policy_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_access_policy_async_opt(&self, req: &super::access_context_manager::DeleteAccessPolicyRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operations::Operation>> {
        self.client.unary_call_async(&METHOD_ACCESS_CONTEXT_MANAGER_DELETE_ACCESS_POLICY, req, opt)
    }

    pub fn delete_access_policy_async(&self, req: &super::access_context_manager::DeleteAccessPolicyRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operations::Operation>> {
        self.delete_access_policy_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_access_levels_opt(&self, req: &super::access_context_manager::ListAccessLevelsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::access_context_manager::ListAccessLevelsResponse> {
        self.client.unary_call(&METHOD_ACCESS_CONTEXT_MANAGER_LIST_ACCESS_LEVELS, req, opt)
    }

    pub fn list_access_levels(&self, req: &super::access_context_manager::ListAccessLevelsRequest) -> ::grpcio::Result<super::access_context_manager::ListAccessLevelsResponse> {
        self.list_access_levels_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_access_levels_async_opt(&self, req: &super::access_context_manager::ListAccessLevelsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::access_context_manager::ListAccessLevelsResponse>> {
        self.client.unary_call_async(&METHOD_ACCESS_CONTEXT_MANAGER_LIST_ACCESS_LEVELS, req, opt)
    }

    pub fn list_access_levels_async(&self, req: &super::access_context_manager::ListAccessLevelsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::access_context_manager::ListAccessLevelsResponse>> {
        self.list_access_levels_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_access_level_opt(&self, req: &super::access_context_manager::GetAccessLevelRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::access_level::AccessLevel> {
        self.client.unary_call(&METHOD_ACCESS_CONTEXT_MANAGER_GET_ACCESS_LEVEL, req, opt)
    }

    pub fn get_access_level(&self, req: &super::access_context_manager::GetAccessLevelRequest) -> ::grpcio::Result<super::access_level::AccessLevel> {
        self.get_access_level_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_access_level_async_opt(&self, req: &super::access_context_manager::GetAccessLevelRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::access_level::AccessLevel>> {
        self.client.unary_call_async(&METHOD_ACCESS_CONTEXT_MANAGER_GET_ACCESS_LEVEL, req, opt)
    }

    pub fn get_access_level_async(&self, req: &super::access_context_manager::GetAccessLevelRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::access_level::AccessLevel>> {
        self.get_access_level_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_access_level_opt(&self, req: &super::access_context_manager::CreateAccessLevelRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::operations::Operation> {
        self.client.unary_call(&METHOD_ACCESS_CONTEXT_MANAGER_CREATE_ACCESS_LEVEL, req, opt)
    }

    pub fn create_access_level(&self, req: &super::access_context_manager::CreateAccessLevelRequest) -> ::grpcio::Result<super::operations::Operation> {
        self.create_access_level_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_access_level_async_opt(&self, req: &super::access_context_manager::CreateAccessLevelRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operations::Operation>> {
        self.client.unary_call_async(&METHOD_ACCESS_CONTEXT_MANAGER_CREATE_ACCESS_LEVEL, req, opt)
    }

    pub fn create_access_level_async(&self, req: &super::access_context_manager::CreateAccessLevelRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operations::Operation>> {
        self.create_access_level_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_access_level_opt(&self, req: &super::access_context_manager::UpdateAccessLevelRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::operations::Operation> {
        self.client.unary_call(&METHOD_ACCESS_CONTEXT_MANAGER_UPDATE_ACCESS_LEVEL, req, opt)
    }

    pub fn update_access_level(&self, req: &super::access_context_manager::UpdateAccessLevelRequest) -> ::grpcio::Result<super::operations::Operation> {
        self.update_access_level_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_access_level_async_opt(&self, req: &super::access_context_manager::UpdateAccessLevelRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operations::Operation>> {
        self.client.unary_call_async(&METHOD_ACCESS_CONTEXT_MANAGER_UPDATE_ACCESS_LEVEL, req, opt)
    }

    pub fn update_access_level_async(&self, req: &super::access_context_manager::UpdateAccessLevelRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operations::Operation>> {
        self.update_access_level_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_access_level_opt(&self, req: &super::access_context_manager::DeleteAccessLevelRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::operations::Operation> {
        self.client.unary_call(&METHOD_ACCESS_CONTEXT_MANAGER_DELETE_ACCESS_LEVEL, req, opt)
    }

    pub fn delete_access_level(&self, req: &super::access_context_manager::DeleteAccessLevelRequest) -> ::grpcio::Result<super::operations::Operation> {
        self.delete_access_level_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_access_level_async_opt(&self, req: &super::access_context_manager::DeleteAccessLevelRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operations::Operation>> {
        self.client.unary_call_async(&METHOD_ACCESS_CONTEXT_MANAGER_DELETE_ACCESS_LEVEL, req, opt)
    }

    pub fn delete_access_level_async(&self, req: &super::access_context_manager::DeleteAccessLevelRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operations::Operation>> {
        self.delete_access_level_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn replace_access_levels_opt(&self, req: &super::access_context_manager::ReplaceAccessLevelsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::operations::Operation> {
        self.client.unary_call(&METHOD_ACCESS_CONTEXT_MANAGER_REPLACE_ACCESS_LEVELS, req, opt)
    }

    pub fn replace_access_levels(&self, req: &super::access_context_manager::ReplaceAccessLevelsRequest) -> ::grpcio::Result<super::operations::Operation> {
        self.replace_access_levels_opt(req, ::grpcio::CallOption::default())
    }

    pub fn replace_access_levels_async_opt(&self, req: &super::access_context_manager::ReplaceAccessLevelsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operations::Operation>> {
        self.client.unary_call_async(&METHOD_ACCESS_CONTEXT_MANAGER_REPLACE_ACCESS_LEVELS, req, opt)
    }

    pub fn replace_access_levels_async(&self, req: &super::access_context_manager::ReplaceAccessLevelsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operations::Operation>> {
        self.replace_access_levels_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_service_perimeters_opt(&self, req: &super::access_context_manager::ListServicePerimetersRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::access_context_manager::ListServicePerimetersResponse> {
        self.client.unary_call(&METHOD_ACCESS_CONTEXT_MANAGER_LIST_SERVICE_PERIMETERS, req, opt)
    }

    pub fn list_service_perimeters(&self, req: &super::access_context_manager::ListServicePerimetersRequest) -> ::grpcio::Result<super::access_context_manager::ListServicePerimetersResponse> {
        self.list_service_perimeters_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_service_perimeters_async_opt(&self, req: &super::access_context_manager::ListServicePerimetersRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::access_context_manager::ListServicePerimetersResponse>> {
        self.client.unary_call_async(&METHOD_ACCESS_CONTEXT_MANAGER_LIST_SERVICE_PERIMETERS, req, opt)
    }

    pub fn list_service_perimeters_async(&self, req: &super::access_context_manager::ListServicePerimetersRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::access_context_manager::ListServicePerimetersResponse>> {
        self.list_service_perimeters_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_service_perimeter_opt(&self, req: &super::access_context_manager::GetServicePerimeterRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service_perimeter::ServicePerimeter> {
        self.client.unary_call(&METHOD_ACCESS_CONTEXT_MANAGER_GET_SERVICE_PERIMETER, req, opt)
    }

    pub fn get_service_perimeter(&self, req: &super::access_context_manager::GetServicePerimeterRequest) -> ::grpcio::Result<super::service_perimeter::ServicePerimeter> {
        self.get_service_perimeter_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_service_perimeter_async_opt(&self, req: &super::access_context_manager::GetServicePerimeterRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service_perimeter::ServicePerimeter>> {
        self.client.unary_call_async(&METHOD_ACCESS_CONTEXT_MANAGER_GET_SERVICE_PERIMETER, req, opt)
    }

    pub fn get_service_perimeter_async(&self, req: &super::access_context_manager::GetServicePerimeterRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service_perimeter::ServicePerimeter>> {
        self.get_service_perimeter_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_service_perimeter_opt(&self, req: &super::access_context_manager::CreateServicePerimeterRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::operations::Operation> {
        self.client.unary_call(&METHOD_ACCESS_CONTEXT_MANAGER_CREATE_SERVICE_PERIMETER, req, opt)
    }

    pub fn create_service_perimeter(&self, req: &super::access_context_manager::CreateServicePerimeterRequest) -> ::grpcio::Result<super::operations::Operation> {
        self.create_service_perimeter_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_service_perimeter_async_opt(&self, req: &super::access_context_manager::CreateServicePerimeterRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operations::Operation>> {
        self.client.unary_call_async(&METHOD_ACCESS_CONTEXT_MANAGER_CREATE_SERVICE_PERIMETER, req, opt)
    }

    pub fn create_service_perimeter_async(&self, req: &super::access_context_manager::CreateServicePerimeterRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operations::Operation>> {
        self.create_service_perimeter_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_service_perimeter_opt(&self, req: &super::access_context_manager::UpdateServicePerimeterRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::operations::Operation> {
        self.client.unary_call(&METHOD_ACCESS_CONTEXT_MANAGER_UPDATE_SERVICE_PERIMETER, req, opt)
    }

    pub fn update_service_perimeter(&self, req: &super::access_context_manager::UpdateServicePerimeterRequest) -> ::grpcio::Result<super::operations::Operation> {
        self.update_service_perimeter_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_service_perimeter_async_opt(&self, req: &super::access_context_manager::UpdateServicePerimeterRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operations::Operation>> {
        self.client.unary_call_async(&METHOD_ACCESS_CONTEXT_MANAGER_UPDATE_SERVICE_PERIMETER, req, opt)
    }

    pub fn update_service_perimeter_async(&self, req: &super::access_context_manager::UpdateServicePerimeterRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operations::Operation>> {
        self.update_service_perimeter_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_service_perimeter_opt(&self, req: &super::access_context_manager::DeleteServicePerimeterRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::operations::Operation> {
        self.client.unary_call(&METHOD_ACCESS_CONTEXT_MANAGER_DELETE_SERVICE_PERIMETER, req, opt)
    }

    pub fn delete_service_perimeter(&self, req: &super::access_context_manager::DeleteServicePerimeterRequest) -> ::grpcio::Result<super::operations::Operation> {
        self.delete_service_perimeter_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_service_perimeter_async_opt(&self, req: &super::access_context_manager::DeleteServicePerimeterRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operations::Operation>> {
        self.client.unary_call_async(&METHOD_ACCESS_CONTEXT_MANAGER_DELETE_SERVICE_PERIMETER, req, opt)
    }

    pub fn delete_service_perimeter_async(&self, req: &super::access_context_manager::DeleteServicePerimeterRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operations::Operation>> {
        self.delete_service_perimeter_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn replace_service_perimeters_opt(&self, req: &super::access_context_manager::ReplaceServicePerimetersRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::operations::Operation> {
        self.client.unary_call(&METHOD_ACCESS_CONTEXT_MANAGER_REPLACE_SERVICE_PERIMETERS, req, opt)
    }

    pub fn replace_service_perimeters(&self, req: &super::access_context_manager::ReplaceServicePerimetersRequest) -> ::grpcio::Result<super::operations::Operation> {
        self.replace_service_perimeters_opt(req, ::grpcio::CallOption::default())
    }

    pub fn replace_service_perimeters_async_opt(&self, req: &super::access_context_manager::ReplaceServicePerimetersRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operations::Operation>> {
        self.client.unary_call_async(&METHOD_ACCESS_CONTEXT_MANAGER_REPLACE_SERVICE_PERIMETERS, req, opt)
    }

    pub fn replace_service_perimeters_async(&self, req: &super::access_context_manager::ReplaceServicePerimetersRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operations::Operation>> {
        self.replace_service_perimeters_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn commit_service_perimeters_opt(&self, req: &super::access_context_manager::CommitServicePerimetersRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::operations::Operation> {
        self.client.unary_call(&METHOD_ACCESS_CONTEXT_MANAGER_COMMIT_SERVICE_PERIMETERS, req, opt)
    }

    pub fn commit_service_perimeters(&self, req: &super::access_context_manager::CommitServicePerimetersRequest) -> ::grpcio::Result<super::operations::Operation> {
        self.commit_service_perimeters_opt(req, ::grpcio::CallOption::default())
    }

    pub fn commit_service_perimeters_async_opt(&self, req: &super::access_context_manager::CommitServicePerimetersRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operations::Operation>> {
        self.client.unary_call_async(&METHOD_ACCESS_CONTEXT_MANAGER_COMMIT_SERVICE_PERIMETERS, req, opt)
    }

    pub fn commit_service_perimeters_async(&self, req: &super::access_context_manager::CommitServicePerimetersRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operations::Operation>> {
        self.commit_service_perimeters_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_gcp_user_access_bindings_opt(&self, req: &super::access_context_manager::ListGcpUserAccessBindingsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::access_context_manager::ListGcpUserAccessBindingsResponse> {
        self.client.unary_call(&METHOD_ACCESS_CONTEXT_MANAGER_LIST_GCP_USER_ACCESS_BINDINGS, req, opt)
    }

    pub fn list_gcp_user_access_bindings(&self, req: &super::access_context_manager::ListGcpUserAccessBindingsRequest) -> ::grpcio::Result<super::access_context_manager::ListGcpUserAccessBindingsResponse> {
        self.list_gcp_user_access_bindings_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_gcp_user_access_bindings_async_opt(&self, req: &super::access_context_manager::ListGcpUserAccessBindingsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::access_context_manager::ListGcpUserAccessBindingsResponse>> {
        self.client.unary_call_async(&METHOD_ACCESS_CONTEXT_MANAGER_LIST_GCP_USER_ACCESS_BINDINGS, req, opt)
    }

    pub fn list_gcp_user_access_bindings_async(&self, req: &super::access_context_manager::ListGcpUserAccessBindingsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::access_context_manager::ListGcpUserAccessBindingsResponse>> {
        self.list_gcp_user_access_bindings_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_gcp_user_access_binding_opt(&self, req: &super::access_context_manager::GetGcpUserAccessBindingRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::gcp_user_access_binding::GcpUserAccessBinding> {
        self.client.unary_call(&METHOD_ACCESS_CONTEXT_MANAGER_GET_GCP_USER_ACCESS_BINDING, req, opt)
    }

    pub fn get_gcp_user_access_binding(&self, req: &super::access_context_manager::GetGcpUserAccessBindingRequest) -> ::grpcio::Result<super::gcp_user_access_binding::GcpUserAccessBinding> {
        self.get_gcp_user_access_binding_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_gcp_user_access_binding_async_opt(&self, req: &super::access_context_manager::GetGcpUserAccessBindingRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::gcp_user_access_binding::GcpUserAccessBinding>> {
        self.client.unary_call_async(&METHOD_ACCESS_CONTEXT_MANAGER_GET_GCP_USER_ACCESS_BINDING, req, opt)
    }

    pub fn get_gcp_user_access_binding_async(&self, req: &super::access_context_manager::GetGcpUserAccessBindingRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::gcp_user_access_binding::GcpUserAccessBinding>> {
        self.get_gcp_user_access_binding_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_gcp_user_access_binding_opt(&self, req: &super::access_context_manager::CreateGcpUserAccessBindingRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::operations::Operation> {
        self.client.unary_call(&METHOD_ACCESS_CONTEXT_MANAGER_CREATE_GCP_USER_ACCESS_BINDING, req, opt)
    }

    pub fn create_gcp_user_access_binding(&self, req: &super::access_context_manager::CreateGcpUserAccessBindingRequest) -> ::grpcio::Result<super::operations::Operation> {
        self.create_gcp_user_access_binding_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_gcp_user_access_binding_async_opt(&self, req: &super::access_context_manager::CreateGcpUserAccessBindingRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operations::Operation>> {
        self.client.unary_call_async(&METHOD_ACCESS_CONTEXT_MANAGER_CREATE_GCP_USER_ACCESS_BINDING, req, opt)
    }

    pub fn create_gcp_user_access_binding_async(&self, req: &super::access_context_manager::CreateGcpUserAccessBindingRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operations::Operation>> {
        self.create_gcp_user_access_binding_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_gcp_user_access_binding_opt(&self, req: &super::access_context_manager::UpdateGcpUserAccessBindingRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::operations::Operation> {
        self.client.unary_call(&METHOD_ACCESS_CONTEXT_MANAGER_UPDATE_GCP_USER_ACCESS_BINDING, req, opt)
    }

    pub fn update_gcp_user_access_binding(&self, req: &super::access_context_manager::UpdateGcpUserAccessBindingRequest) -> ::grpcio::Result<super::operations::Operation> {
        self.update_gcp_user_access_binding_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_gcp_user_access_binding_async_opt(&self, req: &super::access_context_manager::UpdateGcpUserAccessBindingRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operations::Operation>> {
        self.client.unary_call_async(&METHOD_ACCESS_CONTEXT_MANAGER_UPDATE_GCP_USER_ACCESS_BINDING, req, opt)
    }

    pub fn update_gcp_user_access_binding_async(&self, req: &super::access_context_manager::UpdateGcpUserAccessBindingRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operations::Operation>> {
        self.update_gcp_user_access_binding_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_gcp_user_access_binding_opt(&self, req: &super::access_context_manager::DeleteGcpUserAccessBindingRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::operations::Operation> {
        self.client.unary_call(&METHOD_ACCESS_CONTEXT_MANAGER_DELETE_GCP_USER_ACCESS_BINDING, req, opt)
    }

    pub fn delete_gcp_user_access_binding(&self, req: &super::access_context_manager::DeleteGcpUserAccessBindingRequest) -> ::grpcio::Result<super::operations::Operation> {
        self.delete_gcp_user_access_binding_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_gcp_user_access_binding_async_opt(&self, req: &super::access_context_manager::DeleteGcpUserAccessBindingRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operations::Operation>> {
        self.client.unary_call_async(&METHOD_ACCESS_CONTEXT_MANAGER_DELETE_GCP_USER_ACCESS_BINDING, req, opt)
    }

    pub fn delete_gcp_user_access_binding_async(&self, req: &super::access_context_manager::DeleteGcpUserAccessBindingRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operations::Operation>> {
        self.delete_gcp_user_access_binding_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_iam_policy_opt(&self, req: &super::iam_policy::SetIamPolicyRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::policy::Policy> {
        self.client.unary_call(&METHOD_ACCESS_CONTEXT_MANAGER_SET_IAM_POLICY, req, opt)
    }

    pub fn set_iam_policy(&self, req: &super::iam_policy::SetIamPolicyRequest) -> ::grpcio::Result<super::policy::Policy> {
        self.set_iam_policy_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_iam_policy_async_opt(&self, req: &super::iam_policy::SetIamPolicyRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::policy::Policy>> {
        self.client.unary_call_async(&METHOD_ACCESS_CONTEXT_MANAGER_SET_IAM_POLICY, req, opt)
    }

    pub fn set_iam_policy_async(&self, req: &super::iam_policy::SetIamPolicyRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::policy::Policy>> {
        self.set_iam_policy_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_iam_policy_opt(&self, req: &super::iam_policy::GetIamPolicyRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::policy::Policy> {
        self.client.unary_call(&METHOD_ACCESS_CONTEXT_MANAGER_GET_IAM_POLICY, req, opt)
    }

    pub fn get_iam_policy(&self, req: &super::iam_policy::GetIamPolicyRequest) -> ::grpcio::Result<super::policy::Policy> {
        self.get_iam_policy_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_iam_policy_async_opt(&self, req: &super::iam_policy::GetIamPolicyRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::policy::Policy>> {
        self.client.unary_call_async(&METHOD_ACCESS_CONTEXT_MANAGER_GET_IAM_POLICY, req, opt)
    }

    pub fn get_iam_policy_async(&self, req: &super::iam_policy::GetIamPolicyRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::policy::Policy>> {
        self.get_iam_policy_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn test_iam_permissions_opt(&self, req: &super::iam_policy::TestIamPermissionsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::iam_policy::TestIamPermissionsResponse> {
        self.client.unary_call(&METHOD_ACCESS_CONTEXT_MANAGER_TEST_IAM_PERMISSIONS, req, opt)
    }

    pub fn test_iam_permissions(&self, req: &super::iam_policy::TestIamPermissionsRequest) -> ::grpcio::Result<super::iam_policy::TestIamPermissionsResponse> {
        self.test_iam_permissions_opt(req, ::grpcio::CallOption::default())
    }

    pub fn test_iam_permissions_async_opt(&self, req: &super::iam_policy::TestIamPermissionsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::iam_policy::TestIamPermissionsResponse>> {
        self.client.unary_call_async(&METHOD_ACCESS_CONTEXT_MANAGER_TEST_IAM_PERMISSIONS, req, opt)
    }

    pub fn test_iam_permissions_async(&self, req: &super::iam_policy::TestIamPermissionsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::iam_policy::TestIamPermissionsResponse>> {
        self.test_iam_permissions_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::std::future::Future<Output = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait AccessContextManager {
    fn list_access_policies(&mut self, ctx: ::grpcio::RpcContext, _req: super::access_context_manager::ListAccessPoliciesRequest, sink: ::grpcio::UnarySink<super::access_context_manager::ListAccessPoliciesResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_access_policy(&mut self, ctx: ::grpcio::RpcContext, _req: super::access_context_manager::GetAccessPolicyRequest, sink: ::grpcio::UnarySink<super::access_policy::AccessPolicy>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn create_access_policy(&mut self, ctx: ::grpcio::RpcContext, _req: super::access_policy::AccessPolicy, sink: ::grpcio::UnarySink<super::operations::Operation>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn update_access_policy(&mut self, ctx: ::grpcio::RpcContext, _req: super::access_context_manager::UpdateAccessPolicyRequest, sink: ::grpcio::UnarySink<super::operations::Operation>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn delete_access_policy(&mut self, ctx: ::grpcio::RpcContext, _req: super::access_context_manager::DeleteAccessPolicyRequest, sink: ::grpcio::UnarySink<super::operations::Operation>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_access_levels(&mut self, ctx: ::grpcio::RpcContext, _req: super::access_context_manager::ListAccessLevelsRequest, sink: ::grpcio::UnarySink<super::access_context_manager::ListAccessLevelsResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_access_level(&mut self, ctx: ::grpcio::RpcContext, _req: super::access_context_manager::GetAccessLevelRequest, sink: ::grpcio::UnarySink<super::access_level::AccessLevel>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn create_access_level(&mut self, ctx: ::grpcio::RpcContext, _req: super::access_context_manager::CreateAccessLevelRequest, sink: ::grpcio::UnarySink<super::operations::Operation>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn update_access_level(&mut self, ctx: ::grpcio::RpcContext, _req: super::access_context_manager::UpdateAccessLevelRequest, sink: ::grpcio::UnarySink<super::operations::Operation>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn delete_access_level(&mut self, ctx: ::grpcio::RpcContext, _req: super::access_context_manager::DeleteAccessLevelRequest, sink: ::grpcio::UnarySink<super::operations::Operation>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn replace_access_levels(&mut self, ctx: ::grpcio::RpcContext, _req: super::access_context_manager::ReplaceAccessLevelsRequest, sink: ::grpcio::UnarySink<super::operations::Operation>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_service_perimeters(&mut self, ctx: ::grpcio::RpcContext, _req: super::access_context_manager::ListServicePerimetersRequest, sink: ::grpcio::UnarySink<super::access_context_manager::ListServicePerimetersResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_service_perimeter(&mut self, ctx: ::grpcio::RpcContext, _req: super::access_context_manager::GetServicePerimeterRequest, sink: ::grpcio::UnarySink<super::service_perimeter::ServicePerimeter>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn create_service_perimeter(&mut self, ctx: ::grpcio::RpcContext, _req: super::access_context_manager::CreateServicePerimeterRequest, sink: ::grpcio::UnarySink<super::operations::Operation>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn update_service_perimeter(&mut self, ctx: ::grpcio::RpcContext, _req: super::access_context_manager::UpdateServicePerimeterRequest, sink: ::grpcio::UnarySink<super::operations::Operation>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn delete_service_perimeter(&mut self, ctx: ::grpcio::RpcContext, _req: super::access_context_manager::DeleteServicePerimeterRequest, sink: ::grpcio::UnarySink<super::operations::Operation>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn replace_service_perimeters(&mut self, ctx: ::grpcio::RpcContext, _req: super::access_context_manager::ReplaceServicePerimetersRequest, sink: ::grpcio::UnarySink<super::operations::Operation>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn commit_service_perimeters(&mut self, ctx: ::grpcio::RpcContext, _req: super::access_context_manager::CommitServicePerimetersRequest, sink: ::grpcio::UnarySink<super::operations::Operation>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_gcp_user_access_bindings(&mut self, ctx: ::grpcio::RpcContext, _req: super::access_context_manager::ListGcpUserAccessBindingsRequest, sink: ::grpcio::UnarySink<super::access_context_manager::ListGcpUserAccessBindingsResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_gcp_user_access_binding(&mut self, ctx: ::grpcio::RpcContext, _req: super::access_context_manager::GetGcpUserAccessBindingRequest, sink: ::grpcio::UnarySink<super::gcp_user_access_binding::GcpUserAccessBinding>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn create_gcp_user_access_binding(&mut self, ctx: ::grpcio::RpcContext, _req: super::access_context_manager::CreateGcpUserAccessBindingRequest, sink: ::grpcio::UnarySink<super::operations::Operation>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn update_gcp_user_access_binding(&mut self, ctx: ::grpcio::RpcContext, _req: super::access_context_manager::UpdateGcpUserAccessBindingRequest, sink: ::grpcio::UnarySink<super::operations::Operation>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn delete_gcp_user_access_binding(&mut self, ctx: ::grpcio::RpcContext, _req: super::access_context_manager::DeleteGcpUserAccessBindingRequest, sink: ::grpcio::UnarySink<super::operations::Operation>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn set_iam_policy(&mut self, ctx: ::grpcio::RpcContext, _req: super::iam_policy::SetIamPolicyRequest, sink: ::grpcio::UnarySink<super::policy::Policy>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_iam_policy(&mut self, ctx: ::grpcio::RpcContext, _req: super::iam_policy::GetIamPolicyRequest, sink: ::grpcio::UnarySink<super::policy::Policy>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn test_iam_permissions(&mut self, ctx: ::grpcio::RpcContext, _req: super::iam_policy::TestIamPermissionsRequest, sink: ::grpcio::UnarySink<super::iam_policy::TestIamPermissionsResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
}

pub fn create_access_context_manager<S: AccessContextManager + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ACCESS_CONTEXT_MANAGER_LIST_ACCESS_POLICIES, move |ctx, req, resp| {
        instance.list_access_policies(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ACCESS_CONTEXT_MANAGER_GET_ACCESS_POLICY, move |ctx, req, resp| {
        instance.get_access_policy(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ACCESS_CONTEXT_MANAGER_CREATE_ACCESS_POLICY, move |ctx, req, resp| {
        instance.create_access_policy(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ACCESS_CONTEXT_MANAGER_UPDATE_ACCESS_POLICY, move |ctx, req, resp| {
        instance.update_access_policy(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ACCESS_CONTEXT_MANAGER_DELETE_ACCESS_POLICY, move |ctx, req, resp| {
        instance.delete_access_policy(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ACCESS_CONTEXT_MANAGER_LIST_ACCESS_LEVELS, move |ctx, req, resp| {
        instance.list_access_levels(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ACCESS_CONTEXT_MANAGER_GET_ACCESS_LEVEL, move |ctx, req, resp| {
        instance.get_access_level(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ACCESS_CONTEXT_MANAGER_CREATE_ACCESS_LEVEL, move |ctx, req, resp| {
        instance.create_access_level(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ACCESS_CONTEXT_MANAGER_UPDATE_ACCESS_LEVEL, move |ctx, req, resp| {
        instance.update_access_level(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ACCESS_CONTEXT_MANAGER_DELETE_ACCESS_LEVEL, move |ctx, req, resp| {
        instance.delete_access_level(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ACCESS_CONTEXT_MANAGER_REPLACE_ACCESS_LEVELS, move |ctx, req, resp| {
        instance.replace_access_levels(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ACCESS_CONTEXT_MANAGER_LIST_SERVICE_PERIMETERS, move |ctx, req, resp| {
        instance.list_service_perimeters(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ACCESS_CONTEXT_MANAGER_GET_SERVICE_PERIMETER, move |ctx, req, resp| {
        instance.get_service_perimeter(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ACCESS_CONTEXT_MANAGER_CREATE_SERVICE_PERIMETER, move |ctx, req, resp| {
        instance.create_service_perimeter(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ACCESS_CONTEXT_MANAGER_UPDATE_SERVICE_PERIMETER, move |ctx, req, resp| {
        instance.update_service_perimeter(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ACCESS_CONTEXT_MANAGER_DELETE_SERVICE_PERIMETER, move |ctx, req, resp| {
        instance.delete_service_perimeter(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ACCESS_CONTEXT_MANAGER_REPLACE_SERVICE_PERIMETERS, move |ctx, req, resp| {
        instance.replace_service_perimeters(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ACCESS_CONTEXT_MANAGER_COMMIT_SERVICE_PERIMETERS, move |ctx, req, resp| {
        instance.commit_service_perimeters(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ACCESS_CONTEXT_MANAGER_LIST_GCP_USER_ACCESS_BINDINGS, move |ctx, req, resp| {
        instance.list_gcp_user_access_bindings(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ACCESS_CONTEXT_MANAGER_GET_GCP_USER_ACCESS_BINDING, move |ctx, req, resp| {
        instance.get_gcp_user_access_binding(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ACCESS_CONTEXT_MANAGER_CREATE_GCP_USER_ACCESS_BINDING, move |ctx, req, resp| {
        instance.create_gcp_user_access_binding(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ACCESS_CONTEXT_MANAGER_UPDATE_GCP_USER_ACCESS_BINDING, move |ctx, req, resp| {
        instance.update_gcp_user_access_binding(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ACCESS_CONTEXT_MANAGER_DELETE_GCP_USER_ACCESS_BINDING, move |ctx, req, resp| {
        instance.delete_gcp_user_access_binding(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ACCESS_CONTEXT_MANAGER_SET_IAM_POLICY, move |ctx, req, resp| {
        instance.set_iam_policy(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ACCESS_CONTEXT_MANAGER_GET_IAM_POLICY, move |ctx, req, resp| {
        instance.get_iam_policy(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_ACCESS_CONTEXT_MANAGER_TEST_IAM_PERMISSIONS, move |ctx, req, resp| {
        instance.test_iam_permissions(ctx, req, resp)
    });
    builder.build()
}
