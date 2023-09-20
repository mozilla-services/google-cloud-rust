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

const METHOD_ORG_POLICY_LIST_CONSTRAINTS: ::grpcio::Method<super::orgpolicy::ListConstraintsRequest, super::orgpolicy::ListConstraintsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.orgpolicy.v2.OrgPolicy/ListConstraints",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ORG_POLICY_LIST_POLICIES: ::grpcio::Method<super::orgpolicy::ListPoliciesRequest, super::orgpolicy::ListPoliciesResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.orgpolicy.v2.OrgPolicy/ListPolicies",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ORG_POLICY_GET_POLICY: ::grpcio::Method<super::orgpolicy::GetPolicyRequest, super::orgpolicy::Policy> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.orgpolicy.v2.OrgPolicy/GetPolicy",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ORG_POLICY_GET_EFFECTIVE_POLICY: ::grpcio::Method<super::orgpolicy::GetEffectivePolicyRequest, super::orgpolicy::Policy> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.orgpolicy.v2.OrgPolicy/GetEffectivePolicy",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ORG_POLICY_CREATE_POLICY: ::grpcio::Method<super::orgpolicy::CreatePolicyRequest, super::orgpolicy::Policy> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.orgpolicy.v2.OrgPolicy/CreatePolicy",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ORG_POLICY_UPDATE_POLICY: ::grpcio::Method<super::orgpolicy::UpdatePolicyRequest, super::orgpolicy::Policy> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.orgpolicy.v2.OrgPolicy/UpdatePolicy",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ORG_POLICY_DELETE_POLICY: ::grpcio::Method<super::orgpolicy::DeletePolicyRequest, super::empty::Empty> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.orgpolicy.v2.OrgPolicy/DeletePolicy",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct OrgPolicyClient {
    client: ::grpcio::Client,
}

impl OrgPolicyClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        OrgPolicyClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn list_constraints_opt(&self, req: &super::orgpolicy::ListConstraintsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::orgpolicy::ListConstraintsResponse> {
        self.client.unary_call(&METHOD_ORG_POLICY_LIST_CONSTRAINTS, req, opt)
    }

    pub fn list_constraints(&self, req: &super::orgpolicy::ListConstraintsRequest) -> ::grpcio::Result<super::orgpolicy::ListConstraintsResponse> {
        self.list_constraints_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_constraints_async_opt(&self, req: &super::orgpolicy::ListConstraintsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::orgpolicy::ListConstraintsResponse>> {
        self.client.unary_call_async(&METHOD_ORG_POLICY_LIST_CONSTRAINTS, req, opt)
    }

    pub fn list_constraints_async(&self, req: &super::orgpolicy::ListConstraintsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::orgpolicy::ListConstraintsResponse>> {
        self.list_constraints_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_policies_opt(&self, req: &super::orgpolicy::ListPoliciesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::orgpolicy::ListPoliciesResponse> {
        self.client.unary_call(&METHOD_ORG_POLICY_LIST_POLICIES, req, opt)
    }

    pub fn list_policies(&self, req: &super::orgpolicy::ListPoliciesRequest) -> ::grpcio::Result<super::orgpolicy::ListPoliciesResponse> {
        self.list_policies_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_policies_async_opt(&self, req: &super::orgpolicy::ListPoliciesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::orgpolicy::ListPoliciesResponse>> {
        self.client.unary_call_async(&METHOD_ORG_POLICY_LIST_POLICIES, req, opt)
    }

    pub fn list_policies_async(&self, req: &super::orgpolicy::ListPoliciesRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::orgpolicy::ListPoliciesResponse>> {
        self.list_policies_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_policy_opt(&self, req: &super::orgpolicy::GetPolicyRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::orgpolicy::Policy> {
        self.client.unary_call(&METHOD_ORG_POLICY_GET_POLICY, req, opt)
    }

    pub fn get_policy(&self, req: &super::orgpolicy::GetPolicyRequest) -> ::grpcio::Result<super::orgpolicy::Policy> {
        self.get_policy_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_policy_async_opt(&self, req: &super::orgpolicy::GetPolicyRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::orgpolicy::Policy>> {
        self.client.unary_call_async(&METHOD_ORG_POLICY_GET_POLICY, req, opt)
    }

    pub fn get_policy_async(&self, req: &super::orgpolicy::GetPolicyRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::orgpolicy::Policy>> {
        self.get_policy_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_effective_policy_opt(&self, req: &super::orgpolicy::GetEffectivePolicyRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::orgpolicy::Policy> {
        self.client.unary_call(&METHOD_ORG_POLICY_GET_EFFECTIVE_POLICY, req, opt)
    }

    pub fn get_effective_policy(&self, req: &super::orgpolicy::GetEffectivePolicyRequest) -> ::grpcio::Result<super::orgpolicy::Policy> {
        self.get_effective_policy_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_effective_policy_async_opt(&self, req: &super::orgpolicy::GetEffectivePolicyRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::orgpolicy::Policy>> {
        self.client.unary_call_async(&METHOD_ORG_POLICY_GET_EFFECTIVE_POLICY, req, opt)
    }

    pub fn get_effective_policy_async(&self, req: &super::orgpolicy::GetEffectivePolicyRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::orgpolicy::Policy>> {
        self.get_effective_policy_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_policy_opt(&self, req: &super::orgpolicy::CreatePolicyRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::orgpolicy::Policy> {
        self.client.unary_call(&METHOD_ORG_POLICY_CREATE_POLICY, req, opt)
    }

    pub fn create_policy(&self, req: &super::orgpolicy::CreatePolicyRequest) -> ::grpcio::Result<super::orgpolicy::Policy> {
        self.create_policy_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_policy_async_opt(&self, req: &super::orgpolicy::CreatePolicyRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::orgpolicy::Policy>> {
        self.client.unary_call_async(&METHOD_ORG_POLICY_CREATE_POLICY, req, opt)
    }

    pub fn create_policy_async(&self, req: &super::orgpolicy::CreatePolicyRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::orgpolicy::Policy>> {
        self.create_policy_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_policy_opt(&self, req: &super::orgpolicy::UpdatePolicyRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::orgpolicy::Policy> {
        self.client.unary_call(&METHOD_ORG_POLICY_UPDATE_POLICY, req, opt)
    }

    pub fn update_policy(&self, req: &super::orgpolicy::UpdatePolicyRequest) -> ::grpcio::Result<super::orgpolicy::Policy> {
        self.update_policy_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_policy_async_opt(&self, req: &super::orgpolicy::UpdatePolicyRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::orgpolicy::Policy>> {
        self.client.unary_call_async(&METHOD_ORG_POLICY_UPDATE_POLICY, req, opt)
    }

    pub fn update_policy_async(&self, req: &super::orgpolicy::UpdatePolicyRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::orgpolicy::Policy>> {
        self.update_policy_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_policy_opt(&self, req: &super::orgpolicy::DeletePolicyRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::empty::Empty> {
        self.client.unary_call(&METHOD_ORG_POLICY_DELETE_POLICY, req, opt)
    }

    pub fn delete_policy(&self, req: &super::orgpolicy::DeletePolicyRequest) -> ::grpcio::Result<super::empty::Empty> {
        self.delete_policy_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_policy_async_opt(&self, req: &super::orgpolicy::DeletePolicyRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.client.unary_call_async(&METHOD_ORG_POLICY_DELETE_POLICY, req, opt)
    }

    pub fn delete_policy_async(&self, req: &super::orgpolicy::DeletePolicyRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.delete_policy_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::std::future::Future<Output = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait OrgPolicy {
    fn list_constraints(&mut self, ctx: ::grpcio::RpcContext, _req: super::orgpolicy::ListConstraintsRequest, sink: ::grpcio::UnarySink<super::orgpolicy::ListConstraintsResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_policies(&mut self, ctx: ::grpcio::RpcContext, _req: super::orgpolicy::ListPoliciesRequest, sink: ::grpcio::UnarySink<super::orgpolicy::ListPoliciesResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_policy(&mut self, ctx: ::grpcio::RpcContext, _req: super::orgpolicy::GetPolicyRequest, sink: ::grpcio::UnarySink<super::orgpolicy::Policy>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_effective_policy(&mut self, ctx: ::grpcio::RpcContext, _req: super::orgpolicy::GetEffectivePolicyRequest, sink: ::grpcio::UnarySink<super::orgpolicy::Policy>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn create_policy(&mut self, ctx: ::grpcio::RpcContext, _req: super::orgpolicy::CreatePolicyRequest, sink: ::grpcio::UnarySink<super::orgpolicy::Policy>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn update_policy(&mut self, ctx: ::grpcio::RpcContext, _req: super::orgpolicy::UpdatePolicyRequest, sink: ::grpcio::UnarySink<super::orgpolicy::Policy>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn delete_policy(&mut self, ctx: ::grpcio::RpcContext, _req: super::orgpolicy::DeletePolicyRequest, sink: ::grpcio::UnarySink<super::empty::Empty>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
}

pub fn create_org_policy<S: OrgPolicy + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ORG_POLICY_LIST_CONSTRAINTS, move |ctx, req, resp| {
        instance.list_constraints(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ORG_POLICY_LIST_POLICIES, move |ctx, req, resp| {
        instance.list_policies(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ORG_POLICY_GET_POLICY, move |ctx, req, resp| {
        instance.get_policy(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ORG_POLICY_GET_EFFECTIVE_POLICY, move |ctx, req, resp| {
        instance.get_effective_policy(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ORG_POLICY_CREATE_POLICY, move |ctx, req, resp| {
        instance.create_policy(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ORG_POLICY_UPDATE_POLICY, move |ctx, req, resp| {
        instance.update_policy(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_ORG_POLICY_DELETE_POLICY, move |ctx, req, resp| {
        instance.delete_policy(ctx, req, resp)
    });
    builder.build()
}
