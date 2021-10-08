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

const METHOD_OS_CONFIG_SERVICE_EXECUTE_PATCH_JOB: ::grpcio::Method<super::patch_jobs::ExecutePatchJobRequest, super::patch_jobs::PatchJob> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.osconfig.v1.OsConfigService/ExecutePatchJob",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_OS_CONFIG_SERVICE_GET_PATCH_JOB: ::grpcio::Method<super::patch_jobs::GetPatchJobRequest, super::patch_jobs::PatchJob> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.osconfig.v1.OsConfigService/GetPatchJob",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_OS_CONFIG_SERVICE_CANCEL_PATCH_JOB: ::grpcio::Method<super::patch_jobs::CancelPatchJobRequest, super::patch_jobs::PatchJob> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.osconfig.v1.OsConfigService/CancelPatchJob",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_OS_CONFIG_SERVICE_LIST_PATCH_JOBS: ::grpcio::Method<super::patch_jobs::ListPatchJobsRequest, super::patch_jobs::ListPatchJobsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.osconfig.v1.OsConfigService/ListPatchJobs",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_OS_CONFIG_SERVICE_LIST_PATCH_JOB_INSTANCE_DETAILS: ::grpcio::Method<super::patch_jobs::ListPatchJobInstanceDetailsRequest, super::patch_jobs::ListPatchJobInstanceDetailsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.osconfig.v1.OsConfigService/ListPatchJobInstanceDetails",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_OS_CONFIG_SERVICE_CREATE_PATCH_DEPLOYMENT: ::grpcio::Method<super::patch_deployments::CreatePatchDeploymentRequest, super::patch_deployments::PatchDeployment> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.osconfig.v1.OsConfigService/CreatePatchDeployment",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_OS_CONFIG_SERVICE_GET_PATCH_DEPLOYMENT: ::grpcio::Method<super::patch_deployments::GetPatchDeploymentRequest, super::patch_deployments::PatchDeployment> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.osconfig.v1.OsConfigService/GetPatchDeployment",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_OS_CONFIG_SERVICE_LIST_PATCH_DEPLOYMENTS: ::grpcio::Method<super::patch_deployments::ListPatchDeploymentsRequest, super::patch_deployments::ListPatchDeploymentsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.osconfig.v1.OsConfigService/ListPatchDeployments",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_OS_CONFIG_SERVICE_DELETE_PATCH_DEPLOYMENT: ::grpcio::Method<super::patch_deployments::DeletePatchDeploymentRequest, super::empty::Empty> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.osconfig.v1.OsConfigService/DeletePatchDeployment",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct OsConfigServiceClient {
    client: ::grpcio::Client,
}

impl OsConfigServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        OsConfigServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn execute_patch_job_opt(&self, req: &super::patch_jobs::ExecutePatchJobRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::patch_jobs::PatchJob> {
        self.client.unary_call(&METHOD_OS_CONFIG_SERVICE_EXECUTE_PATCH_JOB, req, opt)
    }

    pub fn execute_patch_job(&self, req: &super::patch_jobs::ExecutePatchJobRequest) -> ::grpcio::Result<super::patch_jobs::PatchJob> {
        self.execute_patch_job_opt(req, ::grpcio::CallOption::default())
    }

    pub fn execute_patch_job_async_opt(&self, req: &super::patch_jobs::ExecutePatchJobRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::patch_jobs::PatchJob>> {
        self.client.unary_call_async(&METHOD_OS_CONFIG_SERVICE_EXECUTE_PATCH_JOB, req, opt)
    }

    pub fn execute_patch_job_async(&self, req: &super::patch_jobs::ExecutePatchJobRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::patch_jobs::PatchJob>> {
        self.execute_patch_job_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_patch_job_opt(&self, req: &super::patch_jobs::GetPatchJobRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::patch_jobs::PatchJob> {
        self.client.unary_call(&METHOD_OS_CONFIG_SERVICE_GET_PATCH_JOB, req, opt)
    }

    pub fn get_patch_job(&self, req: &super::patch_jobs::GetPatchJobRequest) -> ::grpcio::Result<super::patch_jobs::PatchJob> {
        self.get_patch_job_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_patch_job_async_opt(&self, req: &super::patch_jobs::GetPatchJobRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::patch_jobs::PatchJob>> {
        self.client.unary_call_async(&METHOD_OS_CONFIG_SERVICE_GET_PATCH_JOB, req, opt)
    }

    pub fn get_patch_job_async(&self, req: &super::patch_jobs::GetPatchJobRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::patch_jobs::PatchJob>> {
        self.get_patch_job_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn cancel_patch_job_opt(&self, req: &super::patch_jobs::CancelPatchJobRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::patch_jobs::PatchJob> {
        self.client.unary_call(&METHOD_OS_CONFIG_SERVICE_CANCEL_PATCH_JOB, req, opt)
    }

    pub fn cancel_patch_job(&self, req: &super::patch_jobs::CancelPatchJobRequest) -> ::grpcio::Result<super::patch_jobs::PatchJob> {
        self.cancel_patch_job_opt(req, ::grpcio::CallOption::default())
    }

    pub fn cancel_patch_job_async_opt(&self, req: &super::patch_jobs::CancelPatchJobRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::patch_jobs::PatchJob>> {
        self.client.unary_call_async(&METHOD_OS_CONFIG_SERVICE_CANCEL_PATCH_JOB, req, opt)
    }

    pub fn cancel_patch_job_async(&self, req: &super::patch_jobs::CancelPatchJobRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::patch_jobs::PatchJob>> {
        self.cancel_patch_job_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_patch_jobs_opt(&self, req: &super::patch_jobs::ListPatchJobsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::patch_jobs::ListPatchJobsResponse> {
        self.client.unary_call(&METHOD_OS_CONFIG_SERVICE_LIST_PATCH_JOBS, req, opt)
    }

    pub fn list_patch_jobs(&self, req: &super::patch_jobs::ListPatchJobsRequest) -> ::grpcio::Result<super::patch_jobs::ListPatchJobsResponse> {
        self.list_patch_jobs_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_patch_jobs_async_opt(&self, req: &super::patch_jobs::ListPatchJobsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::patch_jobs::ListPatchJobsResponse>> {
        self.client.unary_call_async(&METHOD_OS_CONFIG_SERVICE_LIST_PATCH_JOBS, req, opt)
    }

    pub fn list_patch_jobs_async(&self, req: &super::patch_jobs::ListPatchJobsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::patch_jobs::ListPatchJobsResponse>> {
        self.list_patch_jobs_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_patch_job_instance_details_opt(&self, req: &super::patch_jobs::ListPatchJobInstanceDetailsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::patch_jobs::ListPatchJobInstanceDetailsResponse> {
        self.client.unary_call(&METHOD_OS_CONFIG_SERVICE_LIST_PATCH_JOB_INSTANCE_DETAILS, req, opt)
    }

    pub fn list_patch_job_instance_details(&self, req: &super::patch_jobs::ListPatchJobInstanceDetailsRequest) -> ::grpcio::Result<super::patch_jobs::ListPatchJobInstanceDetailsResponse> {
        self.list_patch_job_instance_details_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_patch_job_instance_details_async_opt(&self, req: &super::patch_jobs::ListPatchJobInstanceDetailsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::patch_jobs::ListPatchJobInstanceDetailsResponse>> {
        self.client.unary_call_async(&METHOD_OS_CONFIG_SERVICE_LIST_PATCH_JOB_INSTANCE_DETAILS, req, opt)
    }

    pub fn list_patch_job_instance_details_async(&self, req: &super::patch_jobs::ListPatchJobInstanceDetailsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::patch_jobs::ListPatchJobInstanceDetailsResponse>> {
        self.list_patch_job_instance_details_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_patch_deployment_opt(&self, req: &super::patch_deployments::CreatePatchDeploymentRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::patch_deployments::PatchDeployment> {
        self.client.unary_call(&METHOD_OS_CONFIG_SERVICE_CREATE_PATCH_DEPLOYMENT, req, opt)
    }

    pub fn create_patch_deployment(&self, req: &super::patch_deployments::CreatePatchDeploymentRequest) -> ::grpcio::Result<super::patch_deployments::PatchDeployment> {
        self.create_patch_deployment_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_patch_deployment_async_opt(&self, req: &super::patch_deployments::CreatePatchDeploymentRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::patch_deployments::PatchDeployment>> {
        self.client.unary_call_async(&METHOD_OS_CONFIG_SERVICE_CREATE_PATCH_DEPLOYMENT, req, opt)
    }

    pub fn create_patch_deployment_async(&self, req: &super::patch_deployments::CreatePatchDeploymentRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::patch_deployments::PatchDeployment>> {
        self.create_patch_deployment_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_patch_deployment_opt(&self, req: &super::patch_deployments::GetPatchDeploymentRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::patch_deployments::PatchDeployment> {
        self.client.unary_call(&METHOD_OS_CONFIG_SERVICE_GET_PATCH_DEPLOYMENT, req, opt)
    }

    pub fn get_patch_deployment(&self, req: &super::patch_deployments::GetPatchDeploymentRequest) -> ::grpcio::Result<super::patch_deployments::PatchDeployment> {
        self.get_patch_deployment_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_patch_deployment_async_opt(&self, req: &super::patch_deployments::GetPatchDeploymentRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::patch_deployments::PatchDeployment>> {
        self.client.unary_call_async(&METHOD_OS_CONFIG_SERVICE_GET_PATCH_DEPLOYMENT, req, opt)
    }

    pub fn get_patch_deployment_async(&self, req: &super::patch_deployments::GetPatchDeploymentRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::patch_deployments::PatchDeployment>> {
        self.get_patch_deployment_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_patch_deployments_opt(&self, req: &super::patch_deployments::ListPatchDeploymentsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::patch_deployments::ListPatchDeploymentsResponse> {
        self.client.unary_call(&METHOD_OS_CONFIG_SERVICE_LIST_PATCH_DEPLOYMENTS, req, opt)
    }

    pub fn list_patch_deployments(&self, req: &super::patch_deployments::ListPatchDeploymentsRequest) -> ::grpcio::Result<super::patch_deployments::ListPatchDeploymentsResponse> {
        self.list_patch_deployments_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_patch_deployments_async_opt(&self, req: &super::patch_deployments::ListPatchDeploymentsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::patch_deployments::ListPatchDeploymentsResponse>> {
        self.client.unary_call_async(&METHOD_OS_CONFIG_SERVICE_LIST_PATCH_DEPLOYMENTS, req, opt)
    }

    pub fn list_patch_deployments_async(&self, req: &super::patch_deployments::ListPatchDeploymentsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::patch_deployments::ListPatchDeploymentsResponse>> {
        self.list_patch_deployments_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_patch_deployment_opt(&self, req: &super::patch_deployments::DeletePatchDeploymentRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::empty::Empty> {
        self.client.unary_call(&METHOD_OS_CONFIG_SERVICE_DELETE_PATCH_DEPLOYMENT, req, opt)
    }

    pub fn delete_patch_deployment(&self, req: &super::patch_deployments::DeletePatchDeploymentRequest) -> ::grpcio::Result<super::empty::Empty> {
        self.delete_patch_deployment_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_patch_deployment_async_opt(&self, req: &super::patch_deployments::DeletePatchDeploymentRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.client.unary_call_async(&METHOD_OS_CONFIG_SERVICE_DELETE_PATCH_DEPLOYMENT, req, opt)
    }

    pub fn delete_patch_deployment_async(&self, req: &super::patch_deployments::DeletePatchDeploymentRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.delete_patch_deployment_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Output = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait OsConfigService {
    fn execute_patch_job(&mut self, ctx: ::grpcio::RpcContext, _req: super::patch_jobs::ExecutePatchJobRequest, sink: ::grpcio::UnarySink<super::patch_jobs::PatchJob>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_patch_job(&mut self, ctx: ::grpcio::RpcContext, _req: super::patch_jobs::GetPatchJobRequest, sink: ::grpcio::UnarySink<super::patch_jobs::PatchJob>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn cancel_patch_job(&mut self, ctx: ::grpcio::RpcContext, _req: super::patch_jobs::CancelPatchJobRequest, sink: ::grpcio::UnarySink<super::patch_jobs::PatchJob>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_patch_jobs(&mut self, ctx: ::grpcio::RpcContext, _req: super::patch_jobs::ListPatchJobsRequest, sink: ::grpcio::UnarySink<super::patch_jobs::ListPatchJobsResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_patch_job_instance_details(&mut self, ctx: ::grpcio::RpcContext, _req: super::patch_jobs::ListPatchJobInstanceDetailsRequest, sink: ::grpcio::UnarySink<super::patch_jobs::ListPatchJobInstanceDetailsResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn create_patch_deployment(&mut self, ctx: ::grpcio::RpcContext, _req: super::patch_deployments::CreatePatchDeploymentRequest, sink: ::grpcio::UnarySink<super::patch_deployments::PatchDeployment>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_patch_deployment(&mut self, ctx: ::grpcio::RpcContext, _req: super::patch_deployments::GetPatchDeploymentRequest, sink: ::grpcio::UnarySink<super::patch_deployments::PatchDeployment>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_patch_deployments(&mut self, ctx: ::grpcio::RpcContext, _req: super::patch_deployments::ListPatchDeploymentsRequest, sink: ::grpcio::UnarySink<super::patch_deployments::ListPatchDeploymentsResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn delete_patch_deployment(&mut self, ctx: ::grpcio::RpcContext, _req: super::patch_deployments::DeletePatchDeploymentRequest, sink: ::grpcio::UnarySink<super::empty::Empty>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
}

pub fn create_os_config_service<S: OsConfigService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_OS_CONFIG_SERVICE_EXECUTE_PATCH_JOB, move |ctx, req, resp| {
        instance.execute_patch_job(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_OS_CONFIG_SERVICE_GET_PATCH_JOB, move |ctx, req, resp| {
        instance.get_patch_job(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_OS_CONFIG_SERVICE_CANCEL_PATCH_JOB, move |ctx, req, resp| {
        instance.cancel_patch_job(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_OS_CONFIG_SERVICE_LIST_PATCH_JOBS, move |ctx, req, resp| {
        instance.list_patch_jobs(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_OS_CONFIG_SERVICE_LIST_PATCH_JOB_INSTANCE_DETAILS, move |ctx, req, resp| {
        instance.list_patch_job_instance_details(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_OS_CONFIG_SERVICE_CREATE_PATCH_DEPLOYMENT, move |ctx, req, resp| {
        instance.create_patch_deployment(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_OS_CONFIG_SERVICE_GET_PATCH_DEPLOYMENT, move |ctx, req, resp| {
        instance.get_patch_deployment(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_OS_CONFIG_SERVICE_LIST_PATCH_DEPLOYMENTS, move |ctx, req, resp| {
        instance.list_patch_deployments(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_OS_CONFIG_SERVICE_DELETE_PATCH_DEPLOYMENT, move |ctx, req, resp| {
        instance.delete_patch_deployment(ctx, req, resp)
    });
    builder.build()
}
