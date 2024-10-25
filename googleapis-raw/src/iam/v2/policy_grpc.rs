// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]


#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_POLICIES_LIST_POLICIES: ::grpcio::Method<super::policy::ListPoliciesRequest, super::policy::ListPoliciesResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.iam.v2.Policies/ListPolicies",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_POLICIES_GET_POLICY: ::grpcio::Method<super::policy::GetPolicyRequest, super::policy::Policy> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.iam.v2.Policies/GetPolicy",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_POLICIES_CREATE_POLICY: ::grpcio::Method<super::policy::CreatePolicyRequest, super::operations::Operation> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.iam.v2.Policies/CreatePolicy",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_POLICIES_UPDATE_POLICY: ::grpcio::Method<super::policy::UpdatePolicyRequest, super::operations::Operation> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.iam.v2.Policies/UpdatePolicy",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_POLICIES_DELETE_POLICY: ::grpcio::Method<super::policy::DeletePolicyRequest, super::operations::Operation> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.iam.v2.Policies/DeletePolicy",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct PoliciesClient {
    pub client: ::grpcio::Client,
}

impl PoliciesClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        PoliciesClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn list_policies_opt(&self, req: &super::policy::ListPoliciesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::policy::ListPoliciesResponse> {
        self.client.unary_call(&METHOD_POLICIES_LIST_POLICIES, req, opt)
    }

    pub fn list_policies(&self, req: &super::policy::ListPoliciesRequest) -> ::grpcio::Result<super::policy::ListPoliciesResponse> {
        self.list_policies_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_policies_async_opt(&self, req: &super::policy::ListPoliciesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::policy::ListPoliciesResponse>> {
        self.client.unary_call_async(&METHOD_POLICIES_LIST_POLICIES, req, opt)
    }

    pub fn list_policies_async(&self, req: &super::policy::ListPoliciesRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::policy::ListPoliciesResponse>> {
        self.list_policies_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_policy_opt(&self, req: &super::policy::GetPolicyRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::policy::Policy> {
        self.client.unary_call(&METHOD_POLICIES_GET_POLICY, req, opt)
    }

    pub fn get_policy(&self, req: &super::policy::GetPolicyRequest) -> ::grpcio::Result<super::policy::Policy> {
        self.get_policy_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_policy_async_opt(&self, req: &super::policy::GetPolicyRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::policy::Policy>> {
        self.client.unary_call_async(&METHOD_POLICIES_GET_POLICY, req, opt)
    }

    pub fn get_policy_async(&self, req: &super::policy::GetPolicyRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::policy::Policy>> {
        self.get_policy_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_policy_opt(&self, req: &super::policy::CreatePolicyRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::operations::Operation> {
        self.client.unary_call(&METHOD_POLICIES_CREATE_POLICY, req, opt)
    }

    pub fn create_policy(&self, req: &super::policy::CreatePolicyRequest) -> ::grpcio::Result<super::operations::Operation> {
        self.create_policy_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_policy_async_opt(&self, req: &super::policy::CreatePolicyRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operations::Operation>> {
        self.client.unary_call_async(&METHOD_POLICIES_CREATE_POLICY, req, opt)
    }

    pub fn create_policy_async(&self, req: &super::policy::CreatePolicyRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operations::Operation>> {
        self.create_policy_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_policy_opt(&self, req: &super::policy::UpdatePolicyRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::operations::Operation> {
        self.client.unary_call(&METHOD_POLICIES_UPDATE_POLICY, req, opt)
    }

    pub fn update_policy(&self, req: &super::policy::UpdatePolicyRequest) -> ::grpcio::Result<super::operations::Operation> {
        self.update_policy_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_policy_async_opt(&self, req: &super::policy::UpdatePolicyRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operations::Operation>> {
        self.client.unary_call_async(&METHOD_POLICIES_UPDATE_POLICY, req, opt)
    }

    pub fn update_policy_async(&self, req: &super::policy::UpdatePolicyRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operations::Operation>> {
        self.update_policy_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_policy_opt(&self, req: &super::policy::DeletePolicyRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::operations::Operation> {
        self.client.unary_call(&METHOD_POLICIES_DELETE_POLICY, req, opt)
    }

    pub fn delete_policy(&self, req: &super::policy::DeletePolicyRequest) -> ::grpcio::Result<super::operations::Operation> {
        self.delete_policy_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_policy_async_opt(&self, req: &super::policy::DeletePolicyRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operations::Operation>> {
        self.client.unary_call_async(&METHOD_POLICIES_DELETE_POLICY, req, opt)
    }

    pub fn delete_policy_async(&self, req: &super::policy::DeletePolicyRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operations::Operation>> {
        self.delete_policy_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::std::future::Future<Output = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Policies {
    fn list_policies(&mut self, ctx: ::grpcio::RpcContext, _req: super::policy::ListPoliciesRequest, sink: ::grpcio::UnarySink<super::policy::ListPoliciesResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_policy(&mut self, ctx: ::grpcio::RpcContext, _req: super::policy::GetPolicyRequest, sink: ::grpcio::UnarySink<super::policy::Policy>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn create_policy(&mut self, ctx: ::grpcio::RpcContext, _req: super::policy::CreatePolicyRequest, sink: ::grpcio::UnarySink<super::operations::Operation>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn update_policy(&mut self, ctx: ::grpcio::RpcContext, _req: super::policy::UpdatePolicyRequest, sink: ::grpcio::UnarySink<super::operations::Operation>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn delete_policy(&mut self, ctx: ::grpcio::RpcContext, _req: super::policy::DeletePolicyRequest, sink: ::grpcio::UnarySink<super::operations::Operation>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
}

pub fn create_policies<S: Policies + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_POLICIES_LIST_POLICIES, move |ctx, req, resp| {
        instance.list_policies(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_POLICIES_GET_POLICY, move |ctx, req, resp| {
        instance.get_policy(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_POLICIES_CREATE_POLICY, move |ctx, req, resp| {
        instance.create_policy(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_POLICIES_UPDATE_POLICY, move |ctx, req, resp| {
        instance.update_policy(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_POLICIES_DELETE_POLICY, move |ctx, req, resp| {
        instance.delete_policy(ctx, req, resp)
    });
    builder.build()
}
