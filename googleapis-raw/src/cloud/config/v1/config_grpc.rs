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

const METHOD_CONFIG_LIST_DEPLOYMENTS: ::grpcio::Method<super::config::ListDeploymentsRequest, super::config::ListDeploymentsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.config.v1.Config/ListDeployments",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CONFIG_GET_DEPLOYMENT: ::grpcio::Method<super::config::GetDeploymentRequest, super::config::Deployment> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.config.v1.Config/GetDeployment",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CONFIG_CREATE_DEPLOYMENT: ::grpcio::Method<super::config::CreateDeploymentRequest, super::operations::Operation> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.config.v1.Config/CreateDeployment",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CONFIG_UPDATE_DEPLOYMENT: ::grpcio::Method<super::config::UpdateDeploymentRequest, super::operations::Operation> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.config.v1.Config/UpdateDeployment",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CONFIG_DELETE_DEPLOYMENT: ::grpcio::Method<super::config::DeleteDeploymentRequest, super::operations::Operation> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.config.v1.Config/DeleteDeployment",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CONFIG_LIST_REVISIONS: ::grpcio::Method<super::config::ListRevisionsRequest, super::config::ListRevisionsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.config.v1.Config/ListRevisions",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CONFIG_GET_REVISION: ::grpcio::Method<super::config::GetRevisionRequest, super::config::Revision> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.config.v1.Config/GetRevision",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CONFIG_GET_RESOURCE: ::grpcio::Method<super::config::GetResourceRequest, super::config::Resource> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.config.v1.Config/GetResource",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CONFIG_LIST_RESOURCES: ::grpcio::Method<super::config::ListResourcesRequest, super::config::ListResourcesResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.config.v1.Config/ListResources",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CONFIG_EXPORT_DEPLOYMENT_STATEFILE: ::grpcio::Method<super::config::ExportDeploymentStatefileRequest, super::config::Statefile> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.config.v1.Config/ExportDeploymentStatefile",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CONFIG_EXPORT_REVISION_STATEFILE: ::grpcio::Method<super::config::ExportRevisionStatefileRequest, super::config::Statefile> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.config.v1.Config/ExportRevisionStatefile",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CONFIG_IMPORT_STATEFILE: ::grpcio::Method<super::config::ImportStatefileRequest, super::config::Statefile> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.config.v1.Config/ImportStatefile",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CONFIG_DELETE_STATEFILE: ::grpcio::Method<super::config::DeleteStatefileRequest, super::empty::Empty> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.config.v1.Config/DeleteStatefile",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CONFIG_LOCK_DEPLOYMENT: ::grpcio::Method<super::config::LockDeploymentRequest, super::operations::Operation> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.config.v1.Config/LockDeployment",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CONFIG_UNLOCK_DEPLOYMENT: ::grpcio::Method<super::config::UnlockDeploymentRequest, super::operations::Operation> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.config.v1.Config/UnlockDeployment",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CONFIG_EXPORT_LOCK_INFO: ::grpcio::Method<super::config::ExportLockInfoRequest, super::config::LockInfo> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.config.v1.Config/ExportLockInfo",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CONFIG_CREATE_PREVIEW: ::grpcio::Method<super::config::CreatePreviewRequest, super::operations::Operation> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.config.v1.Config/CreatePreview",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CONFIG_GET_PREVIEW: ::grpcio::Method<super::config::GetPreviewRequest, super::config::Preview> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.config.v1.Config/GetPreview",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CONFIG_LIST_PREVIEWS: ::grpcio::Method<super::config::ListPreviewsRequest, super::config::ListPreviewsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.config.v1.Config/ListPreviews",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CONFIG_DELETE_PREVIEW: ::grpcio::Method<super::config::DeletePreviewRequest, super::operations::Operation> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.config.v1.Config/DeletePreview",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CONFIG_EXPORT_PREVIEW_RESULT: ::grpcio::Method<super::config::ExportPreviewResultRequest, super::config::ExportPreviewResultResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.config.v1.Config/ExportPreviewResult",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CONFIG_LIST_TERRAFORM_VERSIONS: ::grpcio::Method<super::config::ListTerraformVersionsRequest, super::config::ListTerraformVersionsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.config.v1.Config/ListTerraformVersions",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CONFIG_GET_TERRAFORM_VERSION: ::grpcio::Method<super::config::GetTerraformVersionRequest, super::config::TerraformVersion> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.config.v1.Config/GetTerraformVersion",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct ConfigClient {
    pub client: ::grpcio::Client,
}

impl ConfigClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        ConfigClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn list_deployments_opt(&self, req: &super::config::ListDeploymentsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::config::ListDeploymentsResponse> {
        self.client.unary_call(&METHOD_CONFIG_LIST_DEPLOYMENTS, req, opt)
    }

    pub fn list_deployments(&self, req: &super::config::ListDeploymentsRequest) -> ::grpcio::Result<super::config::ListDeploymentsResponse> {
        self.list_deployments_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_deployments_async_opt(&self, req: &super::config::ListDeploymentsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::config::ListDeploymentsResponse>> {
        self.client.unary_call_async(&METHOD_CONFIG_LIST_DEPLOYMENTS, req, opt)
    }

    pub fn list_deployments_async(&self, req: &super::config::ListDeploymentsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::config::ListDeploymentsResponse>> {
        self.list_deployments_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_deployment_opt(&self, req: &super::config::GetDeploymentRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::config::Deployment> {
        self.client.unary_call(&METHOD_CONFIG_GET_DEPLOYMENT, req, opt)
    }

    pub fn get_deployment(&self, req: &super::config::GetDeploymentRequest) -> ::grpcio::Result<super::config::Deployment> {
        self.get_deployment_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_deployment_async_opt(&self, req: &super::config::GetDeploymentRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::config::Deployment>> {
        self.client.unary_call_async(&METHOD_CONFIG_GET_DEPLOYMENT, req, opt)
    }

    pub fn get_deployment_async(&self, req: &super::config::GetDeploymentRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::config::Deployment>> {
        self.get_deployment_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_deployment_opt(&self, req: &super::config::CreateDeploymentRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::operations::Operation> {
        self.client.unary_call(&METHOD_CONFIG_CREATE_DEPLOYMENT, req, opt)
    }

    pub fn create_deployment(&self, req: &super::config::CreateDeploymentRequest) -> ::grpcio::Result<super::operations::Operation> {
        self.create_deployment_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_deployment_async_opt(&self, req: &super::config::CreateDeploymentRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operations::Operation>> {
        self.client.unary_call_async(&METHOD_CONFIG_CREATE_DEPLOYMENT, req, opt)
    }

    pub fn create_deployment_async(&self, req: &super::config::CreateDeploymentRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operations::Operation>> {
        self.create_deployment_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_deployment_opt(&self, req: &super::config::UpdateDeploymentRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::operations::Operation> {
        self.client.unary_call(&METHOD_CONFIG_UPDATE_DEPLOYMENT, req, opt)
    }

    pub fn update_deployment(&self, req: &super::config::UpdateDeploymentRequest) -> ::grpcio::Result<super::operations::Operation> {
        self.update_deployment_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_deployment_async_opt(&self, req: &super::config::UpdateDeploymentRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operations::Operation>> {
        self.client.unary_call_async(&METHOD_CONFIG_UPDATE_DEPLOYMENT, req, opt)
    }

    pub fn update_deployment_async(&self, req: &super::config::UpdateDeploymentRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operations::Operation>> {
        self.update_deployment_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_deployment_opt(&self, req: &super::config::DeleteDeploymentRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::operations::Operation> {
        self.client.unary_call(&METHOD_CONFIG_DELETE_DEPLOYMENT, req, opt)
    }

    pub fn delete_deployment(&self, req: &super::config::DeleteDeploymentRequest) -> ::grpcio::Result<super::operations::Operation> {
        self.delete_deployment_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_deployment_async_opt(&self, req: &super::config::DeleteDeploymentRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operations::Operation>> {
        self.client.unary_call_async(&METHOD_CONFIG_DELETE_DEPLOYMENT, req, opt)
    }

    pub fn delete_deployment_async(&self, req: &super::config::DeleteDeploymentRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operations::Operation>> {
        self.delete_deployment_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_revisions_opt(&self, req: &super::config::ListRevisionsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::config::ListRevisionsResponse> {
        self.client.unary_call(&METHOD_CONFIG_LIST_REVISIONS, req, opt)
    }

    pub fn list_revisions(&self, req: &super::config::ListRevisionsRequest) -> ::grpcio::Result<super::config::ListRevisionsResponse> {
        self.list_revisions_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_revisions_async_opt(&self, req: &super::config::ListRevisionsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::config::ListRevisionsResponse>> {
        self.client.unary_call_async(&METHOD_CONFIG_LIST_REVISIONS, req, opt)
    }

    pub fn list_revisions_async(&self, req: &super::config::ListRevisionsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::config::ListRevisionsResponse>> {
        self.list_revisions_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_revision_opt(&self, req: &super::config::GetRevisionRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::config::Revision> {
        self.client.unary_call(&METHOD_CONFIG_GET_REVISION, req, opt)
    }

    pub fn get_revision(&self, req: &super::config::GetRevisionRequest) -> ::grpcio::Result<super::config::Revision> {
        self.get_revision_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_revision_async_opt(&self, req: &super::config::GetRevisionRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::config::Revision>> {
        self.client.unary_call_async(&METHOD_CONFIG_GET_REVISION, req, opt)
    }

    pub fn get_revision_async(&self, req: &super::config::GetRevisionRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::config::Revision>> {
        self.get_revision_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_resource_opt(&self, req: &super::config::GetResourceRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::config::Resource> {
        self.client.unary_call(&METHOD_CONFIG_GET_RESOURCE, req, opt)
    }

    pub fn get_resource(&self, req: &super::config::GetResourceRequest) -> ::grpcio::Result<super::config::Resource> {
        self.get_resource_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_resource_async_opt(&self, req: &super::config::GetResourceRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::config::Resource>> {
        self.client.unary_call_async(&METHOD_CONFIG_GET_RESOURCE, req, opt)
    }

    pub fn get_resource_async(&self, req: &super::config::GetResourceRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::config::Resource>> {
        self.get_resource_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_resources_opt(&self, req: &super::config::ListResourcesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::config::ListResourcesResponse> {
        self.client.unary_call(&METHOD_CONFIG_LIST_RESOURCES, req, opt)
    }

    pub fn list_resources(&self, req: &super::config::ListResourcesRequest) -> ::grpcio::Result<super::config::ListResourcesResponse> {
        self.list_resources_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_resources_async_opt(&self, req: &super::config::ListResourcesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::config::ListResourcesResponse>> {
        self.client.unary_call_async(&METHOD_CONFIG_LIST_RESOURCES, req, opt)
    }

    pub fn list_resources_async(&self, req: &super::config::ListResourcesRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::config::ListResourcesResponse>> {
        self.list_resources_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn export_deployment_statefile_opt(&self, req: &super::config::ExportDeploymentStatefileRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::config::Statefile> {
        self.client.unary_call(&METHOD_CONFIG_EXPORT_DEPLOYMENT_STATEFILE, req, opt)
    }

    pub fn export_deployment_statefile(&self, req: &super::config::ExportDeploymentStatefileRequest) -> ::grpcio::Result<super::config::Statefile> {
        self.export_deployment_statefile_opt(req, ::grpcio::CallOption::default())
    }

    pub fn export_deployment_statefile_async_opt(&self, req: &super::config::ExportDeploymentStatefileRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::config::Statefile>> {
        self.client.unary_call_async(&METHOD_CONFIG_EXPORT_DEPLOYMENT_STATEFILE, req, opt)
    }

    pub fn export_deployment_statefile_async(&self, req: &super::config::ExportDeploymentStatefileRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::config::Statefile>> {
        self.export_deployment_statefile_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn export_revision_statefile_opt(&self, req: &super::config::ExportRevisionStatefileRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::config::Statefile> {
        self.client.unary_call(&METHOD_CONFIG_EXPORT_REVISION_STATEFILE, req, opt)
    }

    pub fn export_revision_statefile(&self, req: &super::config::ExportRevisionStatefileRequest) -> ::grpcio::Result<super::config::Statefile> {
        self.export_revision_statefile_opt(req, ::grpcio::CallOption::default())
    }

    pub fn export_revision_statefile_async_opt(&self, req: &super::config::ExportRevisionStatefileRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::config::Statefile>> {
        self.client.unary_call_async(&METHOD_CONFIG_EXPORT_REVISION_STATEFILE, req, opt)
    }

    pub fn export_revision_statefile_async(&self, req: &super::config::ExportRevisionStatefileRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::config::Statefile>> {
        self.export_revision_statefile_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn import_statefile_opt(&self, req: &super::config::ImportStatefileRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::config::Statefile> {
        self.client.unary_call(&METHOD_CONFIG_IMPORT_STATEFILE, req, opt)
    }

    pub fn import_statefile(&self, req: &super::config::ImportStatefileRequest) -> ::grpcio::Result<super::config::Statefile> {
        self.import_statefile_opt(req, ::grpcio::CallOption::default())
    }

    pub fn import_statefile_async_opt(&self, req: &super::config::ImportStatefileRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::config::Statefile>> {
        self.client.unary_call_async(&METHOD_CONFIG_IMPORT_STATEFILE, req, opt)
    }

    pub fn import_statefile_async(&self, req: &super::config::ImportStatefileRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::config::Statefile>> {
        self.import_statefile_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_statefile_opt(&self, req: &super::config::DeleteStatefileRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::empty::Empty> {
        self.client.unary_call(&METHOD_CONFIG_DELETE_STATEFILE, req, opt)
    }

    pub fn delete_statefile(&self, req: &super::config::DeleteStatefileRequest) -> ::grpcio::Result<super::empty::Empty> {
        self.delete_statefile_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_statefile_async_opt(&self, req: &super::config::DeleteStatefileRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.client.unary_call_async(&METHOD_CONFIG_DELETE_STATEFILE, req, opt)
    }

    pub fn delete_statefile_async(&self, req: &super::config::DeleteStatefileRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.delete_statefile_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn lock_deployment_opt(&self, req: &super::config::LockDeploymentRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::operations::Operation> {
        self.client.unary_call(&METHOD_CONFIG_LOCK_DEPLOYMENT, req, opt)
    }

    pub fn lock_deployment(&self, req: &super::config::LockDeploymentRequest) -> ::grpcio::Result<super::operations::Operation> {
        self.lock_deployment_opt(req, ::grpcio::CallOption::default())
    }

    pub fn lock_deployment_async_opt(&self, req: &super::config::LockDeploymentRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operations::Operation>> {
        self.client.unary_call_async(&METHOD_CONFIG_LOCK_DEPLOYMENT, req, opt)
    }

    pub fn lock_deployment_async(&self, req: &super::config::LockDeploymentRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operations::Operation>> {
        self.lock_deployment_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn unlock_deployment_opt(&self, req: &super::config::UnlockDeploymentRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::operations::Operation> {
        self.client.unary_call(&METHOD_CONFIG_UNLOCK_DEPLOYMENT, req, opt)
    }

    pub fn unlock_deployment(&self, req: &super::config::UnlockDeploymentRequest) -> ::grpcio::Result<super::operations::Operation> {
        self.unlock_deployment_opt(req, ::grpcio::CallOption::default())
    }

    pub fn unlock_deployment_async_opt(&self, req: &super::config::UnlockDeploymentRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operations::Operation>> {
        self.client.unary_call_async(&METHOD_CONFIG_UNLOCK_DEPLOYMENT, req, opt)
    }

    pub fn unlock_deployment_async(&self, req: &super::config::UnlockDeploymentRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operations::Operation>> {
        self.unlock_deployment_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn export_lock_info_opt(&self, req: &super::config::ExportLockInfoRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::config::LockInfo> {
        self.client.unary_call(&METHOD_CONFIG_EXPORT_LOCK_INFO, req, opt)
    }

    pub fn export_lock_info(&self, req: &super::config::ExportLockInfoRequest) -> ::grpcio::Result<super::config::LockInfo> {
        self.export_lock_info_opt(req, ::grpcio::CallOption::default())
    }

    pub fn export_lock_info_async_opt(&self, req: &super::config::ExportLockInfoRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::config::LockInfo>> {
        self.client.unary_call_async(&METHOD_CONFIG_EXPORT_LOCK_INFO, req, opt)
    }

    pub fn export_lock_info_async(&self, req: &super::config::ExportLockInfoRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::config::LockInfo>> {
        self.export_lock_info_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_preview_opt(&self, req: &super::config::CreatePreviewRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::operations::Operation> {
        self.client.unary_call(&METHOD_CONFIG_CREATE_PREVIEW, req, opt)
    }

    pub fn create_preview(&self, req: &super::config::CreatePreviewRequest) -> ::grpcio::Result<super::operations::Operation> {
        self.create_preview_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_preview_async_opt(&self, req: &super::config::CreatePreviewRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operations::Operation>> {
        self.client.unary_call_async(&METHOD_CONFIG_CREATE_PREVIEW, req, opt)
    }

    pub fn create_preview_async(&self, req: &super::config::CreatePreviewRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operations::Operation>> {
        self.create_preview_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_preview_opt(&self, req: &super::config::GetPreviewRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::config::Preview> {
        self.client.unary_call(&METHOD_CONFIG_GET_PREVIEW, req, opt)
    }

    pub fn get_preview(&self, req: &super::config::GetPreviewRequest) -> ::grpcio::Result<super::config::Preview> {
        self.get_preview_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_preview_async_opt(&self, req: &super::config::GetPreviewRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::config::Preview>> {
        self.client.unary_call_async(&METHOD_CONFIG_GET_PREVIEW, req, opt)
    }

    pub fn get_preview_async(&self, req: &super::config::GetPreviewRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::config::Preview>> {
        self.get_preview_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_previews_opt(&self, req: &super::config::ListPreviewsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::config::ListPreviewsResponse> {
        self.client.unary_call(&METHOD_CONFIG_LIST_PREVIEWS, req, opt)
    }

    pub fn list_previews(&self, req: &super::config::ListPreviewsRequest) -> ::grpcio::Result<super::config::ListPreviewsResponse> {
        self.list_previews_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_previews_async_opt(&self, req: &super::config::ListPreviewsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::config::ListPreviewsResponse>> {
        self.client.unary_call_async(&METHOD_CONFIG_LIST_PREVIEWS, req, opt)
    }

    pub fn list_previews_async(&self, req: &super::config::ListPreviewsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::config::ListPreviewsResponse>> {
        self.list_previews_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_preview_opt(&self, req: &super::config::DeletePreviewRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::operations::Operation> {
        self.client.unary_call(&METHOD_CONFIG_DELETE_PREVIEW, req, opt)
    }

    pub fn delete_preview(&self, req: &super::config::DeletePreviewRequest) -> ::grpcio::Result<super::operations::Operation> {
        self.delete_preview_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_preview_async_opt(&self, req: &super::config::DeletePreviewRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operations::Operation>> {
        self.client.unary_call_async(&METHOD_CONFIG_DELETE_PREVIEW, req, opt)
    }

    pub fn delete_preview_async(&self, req: &super::config::DeletePreviewRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operations::Operation>> {
        self.delete_preview_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn export_preview_result_opt(&self, req: &super::config::ExportPreviewResultRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::config::ExportPreviewResultResponse> {
        self.client.unary_call(&METHOD_CONFIG_EXPORT_PREVIEW_RESULT, req, opt)
    }

    pub fn export_preview_result(&self, req: &super::config::ExportPreviewResultRequest) -> ::grpcio::Result<super::config::ExportPreviewResultResponse> {
        self.export_preview_result_opt(req, ::grpcio::CallOption::default())
    }

    pub fn export_preview_result_async_opt(&self, req: &super::config::ExportPreviewResultRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::config::ExportPreviewResultResponse>> {
        self.client.unary_call_async(&METHOD_CONFIG_EXPORT_PREVIEW_RESULT, req, opt)
    }

    pub fn export_preview_result_async(&self, req: &super::config::ExportPreviewResultRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::config::ExportPreviewResultResponse>> {
        self.export_preview_result_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_terraform_versions_opt(&self, req: &super::config::ListTerraformVersionsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::config::ListTerraformVersionsResponse> {
        self.client.unary_call(&METHOD_CONFIG_LIST_TERRAFORM_VERSIONS, req, opt)
    }

    pub fn list_terraform_versions(&self, req: &super::config::ListTerraformVersionsRequest) -> ::grpcio::Result<super::config::ListTerraformVersionsResponse> {
        self.list_terraform_versions_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_terraform_versions_async_opt(&self, req: &super::config::ListTerraformVersionsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::config::ListTerraformVersionsResponse>> {
        self.client.unary_call_async(&METHOD_CONFIG_LIST_TERRAFORM_VERSIONS, req, opt)
    }

    pub fn list_terraform_versions_async(&self, req: &super::config::ListTerraformVersionsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::config::ListTerraformVersionsResponse>> {
        self.list_terraform_versions_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_terraform_version_opt(&self, req: &super::config::GetTerraformVersionRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::config::TerraformVersion> {
        self.client.unary_call(&METHOD_CONFIG_GET_TERRAFORM_VERSION, req, opt)
    }

    pub fn get_terraform_version(&self, req: &super::config::GetTerraformVersionRequest) -> ::grpcio::Result<super::config::TerraformVersion> {
        self.get_terraform_version_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_terraform_version_async_opt(&self, req: &super::config::GetTerraformVersionRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::config::TerraformVersion>> {
        self.client.unary_call_async(&METHOD_CONFIG_GET_TERRAFORM_VERSION, req, opt)
    }

    pub fn get_terraform_version_async(&self, req: &super::config::GetTerraformVersionRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::config::TerraformVersion>> {
        self.get_terraform_version_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::std::future::Future<Output = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Config {
    fn list_deployments(&mut self, ctx: ::grpcio::RpcContext, _req: super::config::ListDeploymentsRequest, sink: ::grpcio::UnarySink<super::config::ListDeploymentsResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_deployment(&mut self, ctx: ::grpcio::RpcContext, _req: super::config::GetDeploymentRequest, sink: ::grpcio::UnarySink<super::config::Deployment>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn create_deployment(&mut self, ctx: ::grpcio::RpcContext, _req: super::config::CreateDeploymentRequest, sink: ::grpcio::UnarySink<super::operations::Operation>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn update_deployment(&mut self, ctx: ::grpcio::RpcContext, _req: super::config::UpdateDeploymentRequest, sink: ::grpcio::UnarySink<super::operations::Operation>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn delete_deployment(&mut self, ctx: ::grpcio::RpcContext, _req: super::config::DeleteDeploymentRequest, sink: ::grpcio::UnarySink<super::operations::Operation>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_revisions(&mut self, ctx: ::grpcio::RpcContext, _req: super::config::ListRevisionsRequest, sink: ::grpcio::UnarySink<super::config::ListRevisionsResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_revision(&mut self, ctx: ::grpcio::RpcContext, _req: super::config::GetRevisionRequest, sink: ::grpcio::UnarySink<super::config::Revision>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_resource(&mut self, ctx: ::grpcio::RpcContext, _req: super::config::GetResourceRequest, sink: ::grpcio::UnarySink<super::config::Resource>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_resources(&mut self, ctx: ::grpcio::RpcContext, _req: super::config::ListResourcesRequest, sink: ::grpcio::UnarySink<super::config::ListResourcesResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn export_deployment_statefile(&mut self, ctx: ::grpcio::RpcContext, _req: super::config::ExportDeploymentStatefileRequest, sink: ::grpcio::UnarySink<super::config::Statefile>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn export_revision_statefile(&mut self, ctx: ::grpcio::RpcContext, _req: super::config::ExportRevisionStatefileRequest, sink: ::grpcio::UnarySink<super::config::Statefile>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn import_statefile(&mut self, ctx: ::grpcio::RpcContext, _req: super::config::ImportStatefileRequest, sink: ::grpcio::UnarySink<super::config::Statefile>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn delete_statefile(&mut self, ctx: ::grpcio::RpcContext, _req: super::config::DeleteStatefileRequest, sink: ::grpcio::UnarySink<super::empty::Empty>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn lock_deployment(&mut self, ctx: ::grpcio::RpcContext, _req: super::config::LockDeploymentRequest, sink: ::grpcio::UnarySink<super::operations::Operation>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn unlock_deployment(&mut self, ctx: ::grpcio::RpcContext, _req: super::config::UnlockDeploymentRequest, sink: ::grpcio::UnarySink<super::operations::Operation>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn export_lock_info(&mut self, ctx: ::grpcio::RpcContext, _req: super::config::ExportLockInfoRequest, sink: ::grpcio::UnarySink<super::config::LockInfo>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn create_preview(&mut self, ctx: ::grpcio::RpcContext, _req: super::config::CreatePreviewRequest, sink: ::grpcio::UnarySink<super::operations::Operation>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_preview(&mut self, ctx: ::grpcio::RpcContext, _req: super::config::GetPreviewRequest, sink: ::grpcio::UnarySink<super::config::Preview>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_previews(&mut self, ctx: ::grpcio::RpcContext, _req: super::config::ListPreviewsRequest, sink: ::grpcio::UnarySink<super::config::ListPreviewsResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn delete_preview(&mut self, ctx: ::grpcio::RpcContext, _req: super::config::DeletePreviewRequest, sink: ::grpcio::UnarySink<super::operations::Operation>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn export_preview_result(&mut self, ctx: ::grpcio::RpcContext, _req: super::config::ExportPreviewResultRequest, sink: ::grpcio::UnarySink<super::config::ExportPreviewResultResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_terraform_versions(&mut self, ctx: ::grpcio::RpcContext, _req: super::config::ListTerraformVersionsRequest, sink: ::grpcio::UnarySink<super::config::ListTerraformVersionsResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_terraform_version(&mut self, ctx: ::grpcio::RpcContext, _req: super::config::GetTerraformVersionRequest, sink: ::grpcio::UnarySink<super::config::TerraformVersion>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
}

pub fn create_config<S: Config + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CONFIG_LIST_DEPLOYMENTS, move |ctx, req, resp| {
        instance.list_deployments(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CONFIG_GET_DEPLOYMENT, move |ctx, req, resp| {
        instance.get_deployment(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CONFIG_CREATE_DEPLOYMENT, move |ctx, req, resp| {
        instance.create_deployment(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CONFIG_UPDATE_DEPLOYMENT, move |ctx, req, resp| {
        instance.update_deployment(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CONFIG_DELETE_DEPLOYMENT, move |ctx, req, resp| {
        instance.delete_deployment(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CONFIG_LIST_REVISIONS, move |ctx, req, resp| {
        instance.list_revisions(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CONFIG_GET_REVISION, move |ctx, req, resp| {
        instance.get_revision(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CONFIG_GET_RESOURCE, move |ctx, req, resp| {
        instance.get_resource(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CONFIG_LIST_RESOURCES, move |ctx, req, resp| {
        instance.list_resources(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CONFIG_EXPORT_DEPLOYMENT_STATEFILE, move |ctx, req, resp| {
        instance.export_deployment_statefile(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CONFIG_EXPORT_REVISION_STATEFILE, move |ctx, req, resp| {
        instance.export_revision_statefile(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CONFIG_IMPORT_STATEFILE, move |ctx, req, resp| {
        instance.import_statefile(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CONFIG_DELETE_STATEFILE, move |ctx, req, resp| {
        instance.delete_statefile(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CONFIG_LOCK_DEPLOYMENT, move |ctx, req, resp| {
        instance.lock_deployment(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CONFIG_UNLOCK_DEPLOYMENT, move |ctx, req, resp| {
        instance.unlock_deployment(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CONFIG_EXPORT_LOCK_INFO, move |ctx, req, resp| {
        instance.export_lock_info(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CONFIG_CREATE_PREVIEW, move |ctx, req, resp| {
        instance.create_preview(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CONFIG_GET_PREVIEW, move |ctx, req, resp| {
        instance.get_preview(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CONFIG_LIST_PREVIEWS, move |ctx, req, resp| {
        instance.list_previews(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CONFIG_DELETE_PREVIEW, move |ctx, req, resp| {
        instance.delete_preview(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CONFIG_EXPORT_PREVIEW_RESULT, move |ctx, req, resp| {
        instance.export_preview_result(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CONFIG_LIST_TERRAFORM_VERSIONS, move |ctx, req, resp| {
        instance.list_terraform_versions(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_CONFIG_GET_TERRAFORM_VERSION, move |ctx, req, resp| {
        instance.get_terraform_version(ctx, req, resp)
    });
    builder.build()
}
