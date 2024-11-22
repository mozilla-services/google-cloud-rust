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

const METHOD_SECURITY_CENTER_BATCH_CREATE_RESOURCE_VALUE_CONFIGS: ::grpcio::Method<super::securitycenter_service::BatchCreateResourceValueConfigsRequest, super::securitycenter_service::BatchCreateResourceValueConfigsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v2.SecurityCenter/BatchCreateResourceValueConfigs",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SECURITY_CENTER_BULK_MUTE_FINDINGS: ::grpcio::Method<super::securitycenter_service::BulkMuteFindingsRequest, super::operations::Operation> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v2.SecurityCenter/BulkMuteFindings",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SECURITY_CENTER_CREATE_BIG_QUERY_EXPORT: ::grpcio::Method<super::securitycenter_service::CreateBigQueryExportRequest, super::bigquery_export::BigQueryExport> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v2.SecurityCenter/CreateBigQueryExport",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SECURITY_CENTER_CREATE_FINDING: ::grpcio::Method<super::securitycenter_service::CreateFindingRequest, super::finding::Finding> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v2.SecurityCenter/CreateFinding",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SECURITY_CENTER_CREATE_MUTE_CONFIG: ::grpcio::Method<super::securitycenter_service::CreateMuteConfigRequest, super::mute_config::MuteConfig> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v2.SecurityCenter/CreateMuteConfig",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SECURITY_CENTER_CREATE_NOTIFICATION_CONFIG: ::grpcio::Method<super::securitycenter_service::CreateNotificationConfigRequest, super::notification_config::NotificationConfig> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v2.SecurityCenter/CreateNotificationConfig",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SECURITY_CENTER_CREATE_SOURCE: ::grpcio::Method<super::securitycenter_service::CreateSourceRequest, super::source::Source> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v2.SecurityCenter/CreateSource",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SECURITY_CENTER_DELETE_BIG_QUERY_EXPORT: ::grpcio::Method<super::securitycenter_service::DeleteBigQueryExportRequest, super::empty::Empty> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v2.SecurityCenter/DeleteBigQueryExport",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SECURITY_CENTER_DELETE_MUTE_CONFIG: ::grpcio::Method<super::securitycenter_service::DeleteMuteConfigRequest, super::empty::Empty> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v2.SecurityCenter/DeleteMuteConfig",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SECURITY_CENTER_DELETE_NOTIFICATION_CONFIG: ::grpcio::Method<super::securitycenter_service::DeleteNotificationConfigRequest, super::empty::Empty> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v2.SecurityCenter/DeleteNotificationConfig",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SECURITY_CENTER_DELETE_RESOURCE_VALUE_CONFIG: ::grpcio::Method<super::securitycenter_service::DeleteResourceValueConfigRequest, super::empty::Empty> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v2.SecurityCenter/DeleteResourceValueConfig",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SECURITY_CENTER_GET_BIG_QUERY_EXPORT: ::grpcio::Method<super::securitycenter_service::GetBigQueryExportRequest, super::bigquery_export::BigQueryExport> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v2.SecurityCenter/GetBigQueryExport",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SECURITY_CENTER_GET_SIMULATION: ::grpcio::Method<super::securitycenter_service::GetSimulationRequest, super::simulation::Simulation> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v2.SecurityCenter/GetSimulation",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SECURITY_CENTER_GET_VALUED_RESOURCE: ::grpcio::Method<super::securitycenter_service::GetValuedResourceRequest, super::valued_resource::ValuedResource> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v2.SecurityCenter/GetValuedResource",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SECURITY_CENTER_GET_IAM_POLICY: ::grpcio::Method<super::iam_policy::GetIamPolicyRequest, super::policy::Policy> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v2.SecurityCenter/GetIamPolicy",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SECURITY_CENTER_GET_MUTE_CONFIG: ::grpcio::Method<super::securitycenter_service::GetMuteConfigRequest, super::mute_config::MuteConfig> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v2.SecurityCenter/GetMuteConfig",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SECURITY_CENTER_GET_NOTIFICATION_CONFIG: ::grpcio::Method<super::securitycenter_service::GetNotificationConfigRequest, super::notification_config::NotificationConfig> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v2.SecurityCenter/GetNotificationConfig",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SECURITY_CENTER_GET_RESOURCE_VALUE_CONFIG: ::grpcio::Method<super::securitycenter_service::GetResourceValueConfigRequest, super::resource_value_config::ResourceValueConfig> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v2.SecurityCenter/GetResourceValueConfig",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SECURITY_CENTER_GET_SOURCE: ::grpcio::Method<super::securitycenter_service::GetSourceRequest, super::source::Source> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v2.SecurityCenter/GetSource",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SECURITY_CENTER_GROUP_FINDINGS: ::grpcio::Method<super::securitycenter_service::GroupFindingsRequest, super::securitycenter_service::GroupFindingsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v2.SecurityCenter/GroupFindings",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SECURITY_CENTER_LIST_ATTACK_PATHS: ::grpcio::Method<super::securitycenter_service::ListAttackPathsRequest, super::securitycenter_service::ListAttackPathsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v2.SecurityCenter/ListAttackPaths",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SECURITY_CENTER_LIST_BIG_QUERY_EXPORTS: ::grpcio::Method<super::securitycenter_service::ListBigQueryExportsRequest, super::securitycenter_service::ListBigQueryExportsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v2.SecurityCenter/ListBigQueryExports",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SECURITY_CENTER_LIST_FINDINGS: ::grpcio::Method<super::securitycenter_service::ListFindingsRequest, super::securitycenter_service::ListFindingsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v2.SecurityCenter/ListFindings",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SECURITY_CENTER_LIST_MUTE_CONFIGS: ::grpcio::Method<super::securitycenter_service::ListMuteConfigsRequest, super::securitycenter_service::ListMuteConfigsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v2.SecurityCenter/ListMuteConfigs",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SECURITY_CENTER_LIST_NOTIFICATION_CONFIGS: ::grpcio::Method<super::securitycenter_service::ListNotificationConfigsRequest, super::securitycenter_service::ListNotificationConfigsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v2.SecurityCenter/ListNotificationConfigs",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SECURITY_CENTER_LIST_RESOURCE_VALUE_CONFIGS: ::grpcio::Method<super::securitycenter_service::ListResourceValueConfigsRequest, super::securitycenter_service::ListResourceValueConfigsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v2.SecurityCenter/ListResourceValueConfigs",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SECURITY_CENTER_LIST_SOURCES: ::grpcio::Method<super::securitycenter_service::ListSourcesRequest, super::securitycenter_service::ListSourcesResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v2.SecurityCenter/ListSources",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SECURITY_CENTER_LIST_VALUED_RESOURCES: ::grpcio::Method<super::securitycenter_service::ListValuedResourcesRequest, super::securitycenter_service::ListValuedResourcesResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v2.SecurityCenter/ListValuedResources",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SECURITY_CENTER_SET_FINDING_STATE: ::grpcio::Method<super::securitycenter_service::SetFindingStateRequest, super::finding::Finding> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v2.SecurityCenter/SetFindingState",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SECURITY_CENTER_SET_IAM_POLICY: ::grpcio::Method<super::iam_policy::SetIamPolicyRequest, super::policy::Policy> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v2.SecurityCenter/SetIamPolicy",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SECURITY_CENTER_SET_MUTE: ::grpcio::Method<super::securitycenter_service::SetMuteRequest, super::finding::Finding> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v2.SecurityCenter/SetMute",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SECURITY_CENTER_TEST_IAM_PERMISSIONS: ::grpcio::Method<super::iam_policy::TestIamPermissionsRequest, super::iam_policy::TestIamPermissionsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v2.SecurityCenter/TestIamPermissions",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SECURITY_CENTER_UPDATE_BIG_QUERY_EXPORT: ::grpcio::Method<super::securitycenter_service::UpdateBigQueryExportRequest, super::bigquery_export::BigQueryExport> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v2.SecurityCenter/UpdateBigQueryExport",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SECURITY_CENTER_UPDATE_EXTERNAL_SYSTEM: ::grpcio::Method<super::securitycenter_service::UpdateExternalSystemRequest, super::external_system::ExternalSystem> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v2.SecurityCenter/UpdateExternalSystem",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SECURITY_CENTER_UPDATE_FINDING: ::grpcio::Method<super::securitycenter_service::UpdateFindingRequest, super::finding::Finding> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v2.SecurityCenter/UpdateFinding",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SECURITY_CENTER_UPDATE_MUTE_CONFIG: ::grpcio::Method<super::securitycenter_service::UpdateMuteConfigRequest, super::mute_config::MuteConfig> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v2.SecurityCenter/UpdateMuteConfig",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SECURITY_CENTER_UPDATE_NOTIFICATION_CONFIG: ::grpcio::Method<super::securitycenter_service::UpdateNotificationConfigRequest, super::notification_config::NotificationConfig> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v2.SecurityCenter/UpdateNotificationConfig",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SECURITY_CENTER_UPDATE_RESOURCE_VALUE_CONFIG: ::grpcio::Method<super::securitycenter_service::UpdateResourceValueConfigRequest, super::resource_value_config::ResourceValueConfig> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v2.SecurityCenter/UpdateResourceValueConfig",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SECURITY_CENTER_UPDATE_SECURITY_MARKS: ::grpcio::Method<super::securitycenter_service::UpdateSecurityMarksRequest, super::security_marks::SecurityMarks> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v2.SecurityCenter/UpdateSecurityMarks",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SECURITY_CENTER_UPDATE_SOURCE: ::grpcio::Method<super::securitycenter_service::UpdateSourceRequest, super::source::Source> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v2.SecurityCenter/UpdateSource",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct SecurityCenterClient {
    pub client: ::grpcio::Client,
}

impl SecurityCenterClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        SecurityCenterClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn batch_create_resource_value_configs_opt(&self, req: &super::securitycenter_service::BatchCreateResourceValueConfigsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::securitycenter_service::BatchCreateResourceValueConfigsResponse> {
        self.client.unary_call(&METHOD_SECURITY_CENTER_BATCH_CREATE_RESOURCE_VALUE_CONFIGS, req, opt)
    }

    pub fn batch_create_resource_value_configs(&self, req: &super::securitycenter_service::BatchCreateResourceValueConfigsRequest) -> ::grpcio::Result<super::securitycenter_service::BatchCreateResourceValueConfigsResponse> {
        self.batch_create_resource_value_configs_opt(req, ::grpcio::CallOption::default())
    }

    pub fn batch_create_resource_value_configs_async_opt(&self, req: &super::securitycenter_service::BatchCreateResourceValueConfigsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::securitycenter_service::BatchCreateResourceValueConfigsResponse>> {
        self.client.unary_call_async(&METHOD_SECURITY_CENTER_BATCH_CREATE_RESOURCE_VALUE_CONFIGS, req, opt)
    }

    pub fn batch_create_resource_value_configs_async(&self, req: &super::securitycenter_service::BatchCreateResourceValueConfigsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::securitycenter_service::BatchCreateResourceValueConfigsResponse>> {
        self.batch_create_resource_value_configs_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn bulk_mute_findings_opt(&self, req: &super::securitycenter_service::BulkMuteFindingsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::operations::Operation> {
        self.client.unary_call(&METHOD_SECURITY_CENTER_BULK_MUTE_FINDINGS, req, opt)
    }

    pub fn bulk_mute_findings(&self, req: &super::securitycenter_service::BulkMuteFindingsRequest) -> ::grpcio::Result<super::operations::Operation> {
        self.bulk_mute_findings_opt(req, ::grpcio::CallOption::default())
    }

    pub fn bulk_mute_findings_async_opt(&self, req: &super::securitycenter_service::BulkMuteFindingsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operations::Operation>> {
        self.client.unary_call_async(&METHOD_SECURITY_CENTER_BULK_MUTE_FINDINGS, req, opt)
    }

    pub fn bulk_mute_findings_async(&self, req: &super::securitycenter_service::BulkMuteFindingsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operations::Operation>> {
        self.bulk_mute_findings_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_big_query_export_opt(&self, req: &super::securitycenter_service::CreateBigQueryExportRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::bigquery_export::BigQueryExport> {
        self.client.unary_call(&METHOD_SECURITY_CENTER_CREATE_BIG_QUERY_EXPORT, req, opt)
    }

    pub fn create_big_query_export(&self, req: &super::securitycenter_service::CreateBigQueryExportRequest) -> ::grpcio::Result<super::bigquery_export::BigQueryExport> {
        self.create_big_query_export_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_big_query_export_async_opt(&self, req: &super::securitycenter_service::CreateBigQueryExportRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::bigquery_export::BigQueryExport>> {
        self.client.unary_call_async(&METHOD_SECURITY_CENTER_CREATE_BIG_QUERY_EXPORT, req, opt)
    }

    pub fn create_big_query_export_async(&self, req: &super::securitycenter_service::CreateBigQueryExportRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::bigquery_export::BigQueryExport>> {
        self.create_big_query_export_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_finding_opt(&self, req: &super::securitycenter_service::CreateFindingRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::finding::Finding> {
        self.client.unary_call(&METHOD_SECURITY_CENTER_CREATE_FINDING, req, opt)
    }

    pub fn create_finding(&self, req: &super::securitycenter_service::CreateFindingRequest) -> ::grpcio::Result<super::finding::Finding> {
        self.create_finding_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_finding_async_opt(&self, req: &super::securitycenter_service::CreateFindingRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::finding::Finding>> {
        self.client.unary_call_async(&METHOD_SECURITY_CENTER_CREATE_FINDING, req, opt)
    }

    pub fn create_finding_async(&self, req: &super::securitycenter_service::CreateFindingRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::finding::Finding>> {
        self.create_finding_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_mute_config_opt(&self, req: &super::securitycenter_service::CreateMuteConfigRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::mute_config::MuteConfig> {
        self.client.unary_call(&METHOD_SECURITY_CENTER_CREATE_MUTE_CONFIG, req, opt)
    }

    pub fn create_mute_config(&self, req: &super::securitycenter_service::CreateMuteConfigRequest) -> ::grpcio::Result<super::mute_config::MuteConfig> {
        self.create_mute_config_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_mute_config_async_opt(&self, req: &super::securitycenter_service::CreateMuteConfigRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::mute_config::MuteConfig>> {
        self.client.unary_call_async(&METHOD_SECURITY_CENTER_CREATE_MUTE_CONFIG, req, opt)
    }

    pub fn create_mute_config_async(&self, req: &super::securitycenter_service::CreateMuteConfigRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::mute_config::MuteConfig>> {
        self.create_mute_config_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_notification_config_opt(&self, req: &super::securitycenter_service::CreateNotificationConfigRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::notification_config::NotificationConfig> {
        self.client.unary_call(&METHOD_SECURITY_CENTER_CREATE_NOTIFICATION_CONFIG, req, opt)
    }

    pub fn create_notification_config(&self, req: &super::securitycenter_service::CreateNotificationConfigRequest) -> ::grpcio::Result<super::notification_config::NotificationConfig> {
        self.create_notification_config_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_notification_config_async_opt(&self, req: &super::securitycenter_service::CreateNotificationConfigRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::notification_config::NotificationConfig>> {
        self.client.unary_call_async(&METHOD_SECURITY_CENTER_CREATE_NOTIFICATION_CONFIG, req, opt)
    }

    pub fn create_notification_config_async(&self, req: &super::securitycenter_service::CreateNotificationConfigRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::notification_config::NotificationConfig>> {
        self.create_notification_config_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_source_opt(&self, req: &super::securitycenter_service::CreateSourceRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::source::Source> {
        self.client.unary_call(&METHOD_SECURITY_CENTER_CREATE_SOURCE, req, opt)
    }

    pub fn create_source(&self, req: &super::securitycenter_service::CreateSourceRequest) -> ::grpcio::Result<super::source::Source> {
        self.create_source_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_source_async_opt(&self, req: &super::securitycenter_service::CreateSourceRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::source::Source>> {
        self.client.unary_call_async(&METHOD_SECURITY_CENTER_CREATE_SOURCE, req, opt)
    }

    pub fn create_source_async(&self, req: &super::securitycenter_service::CreateSourceRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::source::Source>> {
        self.create_source_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_big_query_export_opt(&self, req: &super::securitycenter_service::DeleteBigQueryExportRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::empty::Empty> {
        self.client.unary_call(&METHOD_SECURITY_CENTER_DELETE_BIG_QUERY_EXPORT, req, opt)
    }

    pub fn delete_big_query_export(&self, req: &super::securitycenter_service::DeleteBigQueryExportRequest) -> ::grpcio::Result<super::empty::Empty> {
        self.delete_big_query_export_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_big_query_export_async_opt(&self, req: &super::securitycenter_service::DeleteBigQueryExportRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.client.unary_call_async(&METHOD_SECURITY_CENTER_DELETE_BIG_QUERY_EXPORT, req, opt)
    }

    pub fn delete_big_query_export_async(&self, req: &super::securitycenter_service::DeleteBigQueryExportRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.delete_big_query_export_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_mute_config_opt(&self, req: &super::securitycenter_service::DeleteMuteConfigRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::empty::Empty> {
        self.client.unary_call(&METHOD_SECURITY_CENTER_DELETE_MUTE_CONFIG, req, opt)
    }

    pub fn delete_mute_config(&self, req: &super::securitycenter_service::DeleteMuteConfigRequest) -> ::grpcio::Result<super::empty::Empty> {
        self.delete_mute_config_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_mute_config_async_opt(&self, req: &super::securitycenter_service::DeleteMuteConfigRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.client.unary_call_async(&METHOD_SECURITY_CENTER_DELETE_MUTE_CONFIG, req, opt)
    }

    pub fn delete_mute_config_async(&self, req: &super::securitycenter_service::DeleteMuteConfigRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.delete_mute_config_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_notification_config_opt(&self, req: &super::securitycenter_service::DeleteNotificationConfigRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::empty::Empty> {
        self.client.unary_call(&METHOD_SECURITY_CENTER_DELETE_NOTIFICATION_CONFIG, req, opt)
    }

    pub fn delete_notification_config(&self, req: &super::securitycenter_service::DeleteNotificationConfigRequest) -> ::grpcio::Result<super::empty::Empty> {
        self.delete_notification_config_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_notification_config_async_opt(&self, req: &super::securitycenter_service::DeleteNotificationConfigRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.client.unary_call_async(&METHOD_SECURITY_CENTER_DELETE_NOTIFICATION_CONFIG, req, opt)
    }

    pub fn delete_notification_config_async(&self, req: &super::securitycenter_service::DeleteNotificationConfigRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.delete_notification_config_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_resource_value_config_opt(&self, req: &super::securitycenter_service::DeleteResourceValueConfigRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::empty::Empty> {
        self.client.unary_call(&METHOD_SECURITY_CENTER_DELETE_RESOURCE_VALUE_CONFIG, req, opt)
    }

    pub fn delete_resource_value_config(&self, req: &super::securitycenter_service::DeleteResourceValueConfigRequest) -> ::grpcio::Result<super::empty::Empty> {
        self.delete_resource_value_config_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_resource_value_config_async_opt(&self, req: &super::securitycenter_service::DeleteResourceValueConfigRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.client.unary_call_async(&METHOD_SECURITY_CENTER_DELETE_RESOURCE_VALUE_CONFIG, req, opt)
    }

    pub fn delete_resource_value_config_async(&self, req: &super::securitycenter_service::DeleteResourceValueConfigRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.delete_resource_value_config_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_big_query_export_opt(&self, req: &super::securitycenter_service::GetBigQueryExportRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::bigquery_export::BigQueryExport> {
        self.client.unary_call(&METHOD_SECURITY_CENTER_GET_BIG_QUERY_EXPORT, req, opt)
    }

    pub fn get_big_query_export(&self, req: &super::securitycenter_service::GetBigQueryExportRequest) -> ::grpcio::Result<super::bigquery_export::BigQueryExport> {
        self.get_big_query_export_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_big_query_export_async_opt(&self, req: &super::securitycenter_service::GetBigQueryExportRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::bigquery_export::BigQueryExport>> {
        self.client.unary_call_async(&METHOD_SECURITY_CENTER_GET_BIG_QUERY_EXPORT, req, opt)
    }

    pub fn get_big_query_export_async(&self, req: &super::securitycenter_service::GetBigQueryExportRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::bigquery_export::BigQueryExport>> {
        self.get_big_query_export_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_simulation_opt(&self, req: &super::securitycenter_service::GetSimulationRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::simulation::Simulation> {
        self.client.unary_call(&METHOD_SECURITY_CENTER_GET_SIMULATION, req, opt)
    }

    pub fn get_simulation(&self, req: &super::securitycenter_service::GetSimulationRequest) -> ::grpcio::Result<super::simulation::Simulation> {
        self.get_simulation_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_simulation_async_opt(&self, req: &super::securitycenter_service::GetSimulationRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::simulation::Simulation>> {
        self.client.unary_call_async(&METHOD_SECURITY_CENTER_GET_SIMULATION, req, opt)
    }

    pub fn get_simulation_async(&self, req: &super::securitycenter_service::GetSimulationRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::simulation::Simulation>> {
        self.get_simulation_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_valued_resource_opt(&self, req: &super::securitycenter_service::GetValuedResourceRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::valued_resource::ValuedResource> {
        self.client.unary_call(&METHOD_SECURITY_CENTER_GET_VALUED_RESOURCE, req, opt)
    }

    pub fn get_valued_resource(&self, req: &super::securitycenter_service::GetValuedResourceRequest) -> ::grpcio::Result<super::valued_resource::ValuedResource> {
        self.get_valued_resource_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_valued_resource_async_opt(&self, req: &super::securitycenter_service::GetValuedResourceRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::valued_resource::ValuedResource>> {
        self.client.unary_call_async(&METHOD_SECURITY_CENTER_GET_VALUED_RESOURCE, req, opt)
    }

    pub fn get_valued_resource_async(&self, req: &super::securitycenter_service::GetValuedResourceRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::valued_resource::ValuedResource>> {
        self.get_valued_resource_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_iam_policy_opt(&self, req: &super::iam_policy::GetIamPolicyRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::policy::Policy> {
        self.client.unary_call(&METHOD_SECURITY_CENTER_GET_IAM_POLICY, req, opt)
    }

    pub fn get_iam_policy(&self, req: &super::iam_policy::GetIamPolicyRequest) -> ::grpcio::Result<super::policy::Policy> {
        self.get_iam_policy_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_iam_policy_async_opt(&self, req: &super::iam_policy::GetIamPolicyRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::policy::Policy>> {
        self.client.unary_call_async(&METHOD_SECURITY_CENTER_GET_IAM_POLICY, req, opt)
    }

    pub fn get_iam_policy_async(&self, req: &super::iam_policy::GetIamPolicyRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::policy::Policy>> {
        self.get_iam_policy_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_mute_config_opt(&self, req: &super::securitycenter_service::GetMuteConfigRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::mute_config::MuteConfig> {
        self.client.unary_call(&METHOD_SECURITY_CENTER_GET_MUTE_CONFIG, req, opt)
    }

    pub fn get_mute_config(&self, req: &super::securitycenter_service::GetMuteConfigRequest) -> ::grpcio::Result<super::mute_config::MuteConfig> {
        self.get_mute_config_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_mute_config_async_opt(&self, req: &super::securitycenter_service::GetMuteConfigRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::mute_config::MuteConfig>> {
        self.client.unary_call_async(&METHOD_SECURITY_CENTER_GET_MUTE_CONFIG, req, opt)
    }

    pub fn get_mute_config_async(&self, req: &super::securitycenter_service::GetMuteConfigRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::mute_config::MuteConfig>> {
        self.get_mute_config_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_notification_config_opt(&self, req: &super::securitycenter_service::GetNotificationConfigRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::notification_config::NotificationConfig> {
        self.client.unary_call(&METHOD_SECURITY_CENTER_GET_NOTIFICATION_CONFIG, req, opt)
    }

    pub fn get_notification_config(&self, req: &super::securitycenter_service::GetNotificationConfigRequest) -> ::grpcio::Result<super::notification_config::NotificationConfig> {
        self.get_notification_config_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_notification_config_async_opt(&self, req: &super::securitycenter_service::GetNotificationConfigRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::notification_config::NotificationConfig>> {
        self.client.unary_call_async(&METHOD_SECURITY_CENTER_GET_NOTIFICATION_CONFIG, req, opt)
    }

    pub fn get_notification_config_async(&self, req: &super::securitycenter_service::GetNotificationConfigRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::notification_config::NotificationConfig>> {
        self.get_notification_config_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_resource_value_config_opt(&self, req: &super::securitycenter_service::GetResourceValueConfigRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::resource_value_config::ResourceValueConfig> {
        self.client.unary_call(&METHOD_SECURITY_CENTER_GET_RESOURCE_VALUE_CONFIG, req, opt)
    }

    pub fn get_resource_value_config(&self, req: &super::securitycenter_service::GetResourceValueConfigRequest) -> ::grpcio::Result<super::resource_value_config::ResourceValueConfig> {
        self.get_resource_value_config_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_resource_value_config_async_opt(&self, req: &super::securitycenter_service::GetResourceValueConfigRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::resource_value_config::ResourceValueConfig>> {
        self.client.unary_call_async(&METHOD_SECURITY_CENTER_GET_RESOURCE_VALUE_CONFIG, req, opt)
    }

    pub fn get_resource_value_config_async(&self, req: &super::securitycenter_service::GetResourceValueConfigRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::resource_value_config::ResourceValueConfig>> {
        self.get_resource_value_config_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_source_opt(&self, req: &super::securitycenter_service::GetSourceRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::source::Source> {
        self.client.unary_call(&METHOD_SECURITY_CENTER_GET_SOURCE, req, opt)
    }

    pub fn get_source(&self, req: &super::securitycenter_service::GetSourceRequest) -> ::grpcio::Result<super::source::Source> {
        self.get_source_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_source_async_opt(&self, req: &super::securitycenter_service::GetSourceRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::source::Source>> {
        self.client.unary_call_async(&METHOD_SECURITY_CENTER_GET_SOURCE, req, opt)
    }

    pub fn get_source_async(&self, req: &super::securitycenter_service::GetSourceRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::source::Source>> {
        self.get_source_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn group_findings_opt(&self, req: &super::securitycenter_service::GroupFindingsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::securitycenter_service::GroupFindingsResponse> {
        self.client.unary_call(&METHOD_SECURITY_CENTER_GROUP_FINDINGS, req, opt)
    }

    pub fn group_findings(&self, req: &super::securitycenter_service::GroupFindingsRequest) -> ::grpcio::Result<super::securitycenter_service::GroupFindingsResponse> {
        self.group_findings_opt(req, ::grpcio::CallOption::default())
    }

    pub fn group_findings_async_opt(&self, req: &super::securitycenter_service::GroupFindingsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::securitycenter_service::GroupFindingsResponse>> {
        self.client.unary_call_async(&METHOD_SECURITY_CENTER_GROUP_FINDINGS, req, opt)
    }

    pub fn group_findings_async(&self, req: &super::securitycenter_service::GroupFindingsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::securitycenter_service::GroupFindingsResponse>> {
        self.group_findings_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_attack_paths_opt(&self, req: &super::securitycenter_service::ListAttackPathsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::securitycenter_service::ListAttackPathsResponse> {
        self.client.unary_call(&METHOD_SECURITY_CENTER_LIST_ATTACK_PATHS, req, opt)
    }

    pub fn list_attack_paths(&self, req: &super::securitycenter_service::ListAttackPathsRequest) -> ::grpcio::Result<super::securitycenter_service::ListAttackPathsResponse> {
        self.list_attack_paths_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_attack_paths_async_opt(&self, req: &super::securitycenter_service::ListAttackPathsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::securitycenter_service::ListAttackPathsResponse>> {
        self.client.unary_call_async(&METHOD_SECURITY_CENTER_LIST_ATTACK_PATHS, req, opt)
    }

    pub fn list_attack_paths_async(&self, req: &super::securitycenter_service::ListAttackPathsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::securitycenter_service::ListAttackPathsResponse>> {
        self.list_attack_paths_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_big_query_exports_opt(&self, req: &super::securitycenter_service::ListBigQueryExportsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::securitycenter_service::ListBigQueryExportsResponse> {
        self.client.unary_call(&METHOD_SECURITY_CENTER_LIST_BIG_QUERY_EXPORTS, req, opt)
    }

    pub fn list_big_query_exports(&self, req: &super::securitycenter_service::ListBigQueryExportsRequest) -> ::grpcio::Result<super::securitycenter_service::ListBigQueryExportsResponse> {
        self.list_big_query_exports_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_big_query_exports_async_opt(&self, req: &super::securitycenter_service::ListBigQueryExportsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::securitycenter_service::ListBigQueryExportsResponse>> {
        self.client.unary_call_async(&METHOD_SECURITY_CENTER_LIST_BIG_QUERY_EXPORTS, req, opt)
    }

    pub fn list_big_query_exports_async(&self, req: &super::securitycenter_service::ListBigQueryExportsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::securitycenter_service::ListBigQueryExportsResponse>> {
        self.list_big_query_exports_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_findings_opt(&self, req: &super::securitycenter_service::ListFindingsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::securitycenter_service::ListFindingsResponse> {
        self.client.unary_call(&METHOD_SECURITY_CENTER_LIST_FINDINGS, req, opt)
    }

    pub fn list_findings(&self, req: &super::securitycenter_service::ListFindingsRequest) -> ::grpcio::Result<super::securitycenter_service::ListFindingsResponse> {
        self.list_findings_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_findings_async_opt(&self, req: &super::securitycenter_service::ListFindingsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::securitycenter_service::ListFindingsResponse>> {
        self.client.unary_call_async(&METHOD_SECURITY_CENTER_LIST_FINDINGS, req, opt)
    }

    pub fn list_findings_async(&self, req: &super::securitycenter_service::ListFindingsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::securitycenter_service::ListFindingsResponse>> {
        self.list_findings_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_mute_configs_opt(&self, req: &super::securitycenter_service::ListMuteConfigsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::securitycenter_service::ListMuteConfigsResponse> {
        self.client.unary_call(&METHOD_SECURITY_CENTER_LIST_MUTE_CONFIGS, req, opt)
    }

    pub fn list_mute_configs(&self, req: &super::securitycenter_service::ListMuteConfigsRequest) -> ::grpcio::Result<super::securitycenter_service::ListMuteConfigsResponse> {
        self.list_mute_configs_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_mute_configs_async_opt(&self, req: &super::securitycenter_service::ListMuteConfigsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::securitycenter_service::ListMuteConfigsResponse>> {
        self.client.unary_call_async(&METHOD_SECURITY_CENTER_LIST_MUTE_CONFIGS, req, opt)
    }

    pub fn list_mute_configs_async(&self, req: &super::securitycenter_service::ListMuteConfigsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::securitycenter_service::ListMuteConfigsResponse>> {
        self.list_mute_configs_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_notification_configs_opt(&self, req: &super::securitycenter_service::ListNotificationConfigsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::securitycenter_service::ListNotificationConfigsResponse> {
        self.client.unary_call(&METHOD_SECURITY_CENTER_LIST_NOTIFICATION_CONFIGS, req, opt)
    }

    pub fn list_notification_configs(&self, req: &super::securitycenter_service::ListNotificationConfigsRequest) -> ::grpcio::Result<super::securitycenter_service::ListNotificationConfigsResponse> {
        self.list_notification_configs_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_notification_configs_async_opt(&self, req: &super::securitycenter_service::ListNotificationConfigsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::securitycenter_service::ListNotificationConfigsResponse>> {
        self.client.unary_call_async(&METHOD_SECURITY_CENTER_LIST_NOTIFICATION_CONFIGS, req, opt)
    }

    pub fn list_notification_configs_async(&self, req: &super::securitycenter_service::ListNotificationConfigsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::securitycenter_service::ListNotificationConfigsResponse>> {
        self.list_notification_configs_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_resource_value_configs_opt(&self, req: &super::securitycenter_service::ListResourceValueConfigsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::securitycenter_service::ListResourceValueConfigsResponse> {
        self.client.unary_call(&METHOD_SECURITY_CENTER_LIST_RESOURCE_VALUE_CONFIGS, req, opt)
    }

    pub fn list_resource_value_configs(&self, req: &super::securitycenter_service::ListResourceValueConfigsRequest) -> ::grpcio::Result<super::securitycenter_service::ListResourceValueConfigsResponse> {
        self.list_resource_value_configs_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_resource_value_configs_async_opt(&self, req: &super::securitycenter_service::ListResourceValueConfigsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::securitycenter_service::ListResourceValueConfigsResponse>> {
        self.client.unary_call_async(&METHOD_SECURITY_CENTER_LIST_RESOURCE_VALUE_CONFIGS, req, opt)
    }

    pub fn list_resource_value_configs_async(&self, req: &super::securitycenter_service::ListResourceValueConfigsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::securitycenter_service::ListResourceValueConfigsResponse>> {
        self.list_resource_value_configs_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_sources_opt(&self, req: &super::securitycenter_service::ListSourcesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::securitycenter_service::ListSourcesResponse> {
        self.client.unary_call(&METHOD_SECURITY_CENTER_LIST_SOURCES, req, opt)
    }

    pub fn list_sources(&self, req: &super::securitycenter_service::ListSourcesRequest) -> ::grpcio::Result<super::securitycenter_service::ListSourcesResponse> {
        self.list_sources_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_sources_async_opt(&self, req: &super::securitycenter_service::ListSourcesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::securitycenter_service::ListSourcesResponse>> {
        self.client.unary_call_async(&METHOD_SECURITY_CENTER_LIST_SOURCES, req, opt)
    }

    pub fn list_sources_async(&self, req: &super::securitycenter_service::ListSourcesRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::securitycenter_service::ListSourcesResponse>> {
        self.list_sources_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_valued_resources_opt(&self, req: &super::securitycenter_service::ListValuedResourcesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::securitycenter_service::ListValuedResourcesResponse> {
        self.client.unary_call(&METHOD_SECURITY_CENTER_LIST_VALUED_RESOURCES, req, opt)
    }

    pub fn list_valued_resources(&self, req: &super::securitycenter_service::ListValuedResourcesRequest) -> ::grpcio::Result<super::securitycenter_service::ListValuedResourcesResponse> {
        self.list_valued_resources_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_valued_resources_async_opt(&self, req: &super::securitycenter_service::ListValuedResourcesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::securitycenter_service::ListValuedResourcesResponse>> {
        self.client.unary_call_async(&METHOD_SECURITY_CENTER_LIST_VALUED_RESOURCES, req, opt)
    }

    pub fn list_valued_resources_async(&self, req: &super::securitycenter_service::ListValuedResourcesRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::securitycenter_service::ListValuedResourcesResponse>> {
        self.list_valued_resources_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_finding_state_opt(&self, req: &super::securitycenter_service::SetFindingStateRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::finding::Finding> {
        self.client.unary_call(&METHOD_SECURITY_CENTER_SET_FINDING_STATE, req, opt)
    }

    pub fn set_finding_state(&self, req: &super::securitycenter_service::SetFindingStateRequest) -> ::grpcio::Result<super::finding::Finding> {
        self.set_finding_state_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_finding_state_async_opt(&self, req: &super::securitycenter_service::SetFindingStateRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::finding::Finding>> {
        self.client.unary_call_async(&METHOD_SECURITY_CENTER_SET_FINDING_STATE, req, opt)
    }

    pub fn set_finding_state_async(&self, req: &super::securitycenter_service::SetFindingStateRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::finding::Finding>> {
        self.set_finding_state_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_iam_policy_opt(&self, req: &super::iam_policy::SetIamPolicyRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::policy::Policy> {
        self.client.unary_call(&METHOD_SECURITY_CENTER_SET_IAM_POLICY, req, opt)
    }

    pub fn set_iam_policy(&self, req: &super::iam_policy::SetIamPolicyRequest) -> ::grpcio::Result<super::policy::Policy> {
        self.set_iam_policy_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_iam_policy_async_opt(&self, req: &super::iam_policy::SetIamPolicyRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::policy::Policy>> {
        self.client.unary_call_async(&METHOD_SECURITY_CENTER_SET_IAM_POLICY, req, opt)
    }

    pub fn set_iam_policy_async(&self, req: &super::iam_policy::SetIamPolicyRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::policy::Policy>> {
        self.set_iam_policy_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_mute_opt(&self, req: &super::securitycenter_service::SetMuteRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::finding::Finding> {
        self.client.unary_call(&METHOD_SECURITY_CENTER_SET_MUTE, req, opt)
    }

    pub fn set_mute(&self, req: &super::securitycenter_service::SetMuteRequest) -> ::grpcio::Result<super::finding::Finding> {
        self.set_mute_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_mute_async_opt(&self, req: &super::securitycenter_service::SetMuteRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::finding::Finding>> {
        self.client.unary_call_async(&METHOD_SECURITY_CENTER_SET_MUTE, req, opt)
    }

    pub fn set_mute_async(&self, req: &super::securitycenter_service::SetMuteRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::finding::Finding>> {
        self.set_mute_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn test_iam_permissions_opt(&self, req: &super::iam_policy::TestIamPermissionsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::iam_policy::TestIamPermissionsResponse> {
        self.client.unary_call(&METHOD_SECURITY_CENTER_TEST_IAM_PERMISSIONS, req, opt)
    }

    pub fn test_iam_permissions(&self, req: &super::iam_policy::TestIamPermissionsRequest) -> ::grpcio::Result<super::iam_policy::TestIamPermissionsResponse> {
        self.test_iam_permissions_opt(req, ::grpcio::CallOption::default())
    }

    pub fn test_iam_permissions_async_opt(&self, req: &super::iam_policy::TestIamPermissionsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::iam_policy::TestIamPermissionsResponse>> {
        self.client.unary_call_async(&METHOD_SECURITY_CENTER_TEST_IAM_PERMISSIONS, req, opt)
    }

    pub fn test_iam_permissions_async(&self, req: &super::iam_policy::TestIamPermissionsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::iam_policy::TestIamPermissionsResponse>> {
        self.test_iam_permissions_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_big_query_export_opt(&self, req: &super::securitycenter_service::UpdateBigQueryExportRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::bigquery_export::BigQueryExport> {
        self.client.unary_call(&METHOD_SECURITY_CENTER_UPDATE_BIG_QUERY_EXPORT, req, opt)
    }

    pub fn update_big_query_export(&self, req: &super::securitycenter_service::UpdateBigQueryExportRequest) -> ::grpcio::Result<super::bigquery_export::BigQueryExport> {
        self.update_big_query_export_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_big_query_export_async_opt(&self, req: &super::securitycenter_service::UpdateBigQueryExportRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::bigquery_export::BigQueryExport>> {
        self.client.unary_call_async(&METHOD_SECURITY_CENTER_UPDATE_BIG_QUERY_EXPORT, req, opt)
    }

    pub fn update_big_query_export_async(&self, req: &super::securitycenter_service::UpdateBigQueryExportRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::bigquery_export::BigQueryExport>> {
        self.update_big_query_export_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_external_system_opt(&self, req: &super::securitycenter_service::UpdateExternalSystemRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::external_system::ExternalSystem> {
        self.client.unary_call(&METHOD_SECURITY_CENTER_UPDATE_EXTERNAL_SYSTEM, req, opt)
    }

    pub fn update_external_system(&self, req: &super::securitycenter_service::UpdateExternalSystemRequest) -> ::grpcio::Result<super::external_system::ExternalSystem> {
        self.update_external_system_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_external_system_async_opt(&self, req: &super::securitycenter_service::UpdateExternalSystemRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::external_system::ExternalSystem>> {
        self.client.unary_call_async(&METHOD_SECURITY_CENTER_UPDATE_EXTERNAL_SYSTEM, req, opt)
    }

    pub fn update_external_system_async(&self, req: &super::securitycenter_service::UpdateExternalSystemRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::external_system::ExternalSystem>> {
        self.update_external_system_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_finding_opt(&self, req: &super::securitycenter_service::UpdateFindingRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::finding::Finding> {
        self.client.unary_call(&METHOD_SECURITY_CENTER_UPDATE_FINDING, req, opt)
    }

    pub fn update_finding(&self, req: &super::securitycenter_service::UpdateFindingRequest) -> ::grpcio::Result<super::finding::Finding> {
        self.update_finding_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_finding_async_opt(&self, req: &super::securitycenter_service::UpdateFindingRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::finding::Finding>> {
        self.client.unary_call_async(&METHOD_SECURITY_CENTER_UPDATE_FINDING, req, opt)
    }

    pub fn update_finding_async(&self, req: &super::securitycenter_service::UpdateFindingRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::finding::Finding>> {
        self.update_finding_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_mute_config_opt(&self, req: &super::securitycenter_service::UpdateMuteConfigRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::mute_config::MuteConfig> {
        self.client.unary_call(&METHOD_SECURITY_CENTER_UPDATE_MUTE_CONFIG, req, opt)
    }

    pub fn update_mute_config(&self, req: &super::securitycenter_service::UpdateMuteConfigRequest) -> ::grpcio::Result<super::mute_config::MuteConfig> {
        self.update_mute_config_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_mute_config_async_opt(&self, req: &super::securitycenter_service::UpdateMuteConfigRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::mute_config::MuteConfig>> {
        self.client.unary_call_async(&METHOD_SECURITY_CENTER_UPDATE_MUTE_CONFIG, req, opt)
    }

    pub fn update_mute_config_async(&self, req: &super::securitycenter_service::UpdateMuteConfigRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::mute_config::MuteConfig>> {
        self.update_mute_config_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_notification_config_opt(&self, req: &super::securitycenter_service::UpdateNotificationConfigRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::notification_config::NotificationConfig> {
        self.client.unary_call(&METHOD_SECURITY_CENTER_UPDATE_NOTIFICATION_CONFIG, req, opt)
    }

    pub fn update_notification_config(&self, req: &super::securitycenter_service::UpdateNotificationConfigRequest) -> ::grpcio::Result<super::notification_config::NotificationConfig> {
        self.update_notification_config_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_notification_config_async_opt(&self, req: &super::securitycenter_service::UpdateNotificationConfigRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::notification_config::NotificationConfig>> {
        self.client.unary_call_async(&METHOD_SECURITY_CENTER_UPDATE_NOTIFICATION_CONFIG, req, opt)
    }

    pub fn update_notification_config_async(&self, req: &super::securitycenter_service::UpdateNotificationConfigRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::notification_config::NotificationConfig>> {
        self.update_notification_config_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_resource_value_config_opt(&self, req: &super::securitycenter_service::UpdateResourceValueConfigRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::resource_value_config::ResourceValueConfig> {
        self.client.unary_call(&METHOD_SECURITY_CENTER_UPDATE_RESOURCE_VALUE_CONFIG, req, opt)
    }

    pub fn update_resource_value_config(&self, req: &super::securitycenter_service::UpdateResourceValueConfigRequest) -> ::grpcio::Result<super::resource_value_config::ResourceValueConfig> {
        self.update_resource_value_config_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_resource_value_config_async_opt(&self, req: &super::securitycenter_service::UpdateResourceValueConfigRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::resource_value_config::ResourceValueConfig>> {
        self.client.unary_call_async(&METHOD_SECURITY_CENTER_UPDATE_RESOURCE_VALUE_CONFIG, req, opt)
    }

    pub fn update_resource_value_config_async(&self, req: &super::securitycenter_service::UpdateResourceValueConfigRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::resource_value_config::ResourceValueConfig>> {
        self.update_resource_value_config_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_security_marks_opt(&self, req: &super::securitycenter_service::UpdateSecurityMarksRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::security_marks::SecurityMarks> {
        self.client.unary_call(&METHOD_SECURITY_CENTER_UPDATE_SECURITY_MARKS, req, opt)
    }

    pub fn update_security_marks(&self, req: &super::securitycenter_service::UpdateSecurityMarksRequest) -> ::grpcio::Result<super::security_marks::SecurityMarks> {
        self.update_security_marks_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_security_marks_async_opt(&self, req: &super::securitycenter_service::UpdateSecurityMarksRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::security_marks::SecurityMarks>> {
        self.client.unary_call_async(&METHOD_SECURITY_CENTER_UPDATE_SECURITY_MARKS, req, opt)
    }

    pub fn update_security_marks_async(&self, req: &super::securitycenter_service::UpdateSecurityMarksRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::security_marks::SecurityMarks>> {
        self.update_security_marks_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_source_opt(&self, req: &super::securitycenter_service::UpdateSourceRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::source::Source> {
        self.client.unary_call(&METHOD_SECURITY_CENTER_UPDATE_SOURCE, req, opt)
    }

    pub fn update_source(&self, req: &super::securitycenter_service::UpdateSourceRequest) -> ::grpcio::Result<super::source::Source> {
        self.update_source_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_source_async_opt(&self, req: &super::securitycenter_service::UpdateSourceRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::source::Source>> {
        self.client.unary_call_async(&METHOD_SECURITY_CENTER_UPDATE_SOURCE, req, opt)
    }

    pub fn update_source_async(&self, req: &super::securitycenter_service::UpdateSourceRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::source::Source>> {
        self.update_source_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::std::future::Future<Output = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait SecurityCenter {
    fn batch_create_resource_value_configs(&mut self, ctx: ::grpcio::RpcContext, _req: super::securitycenter_service::BatchCreateResourceValueConfigsRequest, sink: ::grpcio::UnarySink<super::securitycenter_service::BatchCreateResourceValueConfigsResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn bulk_mute_findings(&mut self, ctx: ::grpcio::RpcContext, _req: super::securitycenter_service::BulkMuteFindingsRequest, sink: ::grpcio::UnarySink<super::operations::Operation>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn create_big_query_export(&mut self, ctx: ::grpcio::RpcContext, _req: super::securitycenter_service::CreateBigQueryExportRequest, sink: ::grpcio::UnarySink<super::bigquery_export::BigQueryExport>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn create_finding(&mut self, ctx: ::grpcio::RpcContext, _req: super::securitycenter_service::CreateFindingRequest, sink: ::grpcio::UnarySink<super::finding::Finding>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn create_mute_config(&mut self, ctx: ::grpcio::RpcContext, _req: super::securitycenter_service::CreateMuteConfigRequest, sink: ::grpcio::UnarySink<super::mute_config::MuteConfig>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn create_notification_config(&mut self, ctx: ::grpcio::RpcContext, _req: super::securitycenter_service::CreateNotificationConfigRequest, sink: ::grpcio::UnarySink<super::notification_config::NotificationConfig>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn create_source(&mut self, ctx: ::grpcio::RpcContext, _req: super::securitycenter_service::CreateSourceRequest, sink: ::grpcio::UnarySink<super::source::Source>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn delete_big_query_export(&mut self, ctx: ::grpcio::RpcContext, _req: super::securitycenter_service::DeleteBigQueryExportRequest, sink: ::grpcio::UnarySink<super::empty::Empty>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn delete_mute_config(&mut self, ctx: ::grpcio::RpcContext, _req: super::securitycenter_service::DeleteMuteConfigRequest, sink: ::grpcio::UnarySink<super::empty::Empty>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn delete_notification_config(&mut self, ctx: ::grpcio::RpcContext, _req: super::securitycenter_service::DeleteNotificationConfigRequest, sink: ::grpcio::UnarySink<super::empty::Empty>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn delete_resource_value_config(&mut self, ctx: ::grpcio::RpcContext, _req: super::securitycenter_service::DeleteResourceValueConfigRequest, sink: ::grpcio::UnarySink<super::empty::Empty>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_big_query_export(&mut self, ctx: ::grpcio::RpcContext, _req: super::securitycenter_service::GetBigQueryExportRequest, sink: ::grpcio::UnarySink<super::bigquery_export::BigQueryExport>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_simulation(&mut self, ctx: ::grpcio::RpcContext, _req: super::securitycenter_service::GetSimulationRequest, sink: ::grpcio::UnarySink<super::simulation::Simulation>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_valued_resource(&mut self, ctx: ::grpcio::RpcContext, _req: super::securitycenter_service::GetValuedResourceRequest, sink: ::grpcio::UnarySink<super::valued_resource::ValuedResource>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_iam_policy(&mut self, ctx: ::grpcio::RpcContext, _req: super::iam_policy::GetIamPolicyRequest, sink: ::grpcio::UnarySink<super::policy::Policy>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_mute_config(&mut self, ctx: ::grpcio::RpcContext, _req: super::securitycenter_service::GetMuteConfigRequest, sink: ::grpcio::UnarySink<super::mute_config::MuteConfig>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_notification_config(&mut self, ctx: ::grpcio::RpcContext, _req: super::securitycenter_service::GetNotificationConfigRequest, sink: ::grpcio::UnarySink<super::notification_config::NotificationConfig>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_resource_value_config(&mut self, ctx: ::grpcio::RpcContext, _req: super::securitycenter_service::GetResourceValueConfigRequest, sink: ::grpcio::UnarySink<super::resource_value_config::ResourceValueConfig>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_source(&mut self, ctx: ::grpcio::RpcContext, _req: super::securitycenter_service::GetSourceRequest, sink: ::grpcio::UnarySink<super::source::Source>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn group_findings(&mut self, ctx: ::grpcio::RpcContext, _req: super::securitycenter_service::GroupFindingsRequest, sink: ::grpcio::UnarySink<super::securitycenter_service::GroupFindingsResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_attack_paths(&mut self, ctx: ::grpcio::RpcContext, _req: super::securitycenter_service::ListAttackPathsRequest, sink: ::grpcio::UnarySink<super::securitycenter_service::ListAttackPathsResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_big_query_exports(&mut self, ctx: ::grpcio::RpcContext, _req: super::securitycenter_service::ListBigQueryExportsRequest, sink: ::grpcio::UnarySink<super::securitycenter_service::ListBigQueryExportsResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_findings(&mut self, ctx: ::grpcio::RpcContext, _req: super::securitycenter_service::ListFindingsRequest, sink: ::grpcio::UnarySink<super::securitycenter_service::ListFindingsResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_mute_configs(&mut self, ctx: ::grpcio::RpcContext, _req: super::securitycenter_service::ListMuteConfigsRequest, sink: ::grpcio::UnarySink<super::securitycenter_service::ListMuteConfigsResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_notification_configs(&mut self, ctx: ::grpcio::RpcContext, _req: super::securitycenter_service::ListNotificationConfigsRequest, sink: ::grpcio::UnarySink<super::securitycenter_service::ListNotificationConfigsResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_resource_value_configs(&mut self, ctx: ::grpcio::RpcContext, _req: super::securitycenter_service::ListResourceValueConfigsRequest, sink: ::grpcio::UnarySink<super::securitycenter_service::ListResourceValueConfigsResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_sources(&mut self, ctx: ::grpcio::RpcContext, _req: super::securitycenter_service::ListSourcesRequest, sink: ::grpcio::UnarySink<super::securitycenter_service::ListSourcesResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_valued_resources(&mut self, ctx: ::grpcio::RpcContext, _req: super::securitycenter_service::ListValuedResourcesRequest, sink: ::grpcio::UnarySink<super::securitycenter_service::ListValuedResourcesResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn set_finding_state(&mut self, ctx: ::grpcio::RpcContext, _req: super::securitycenter_service::SetFindingStateRequest, sink: ::grpcio::UnarySink<super::finding::Finding>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn set_iam_policy(&mut self, ctx: ::grpcio::RpcContext, _req: super::iam_policy::SetIamPolicyRequest, sink: ::grpcio::UnarySink<super::policy::Policy>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn set_mute(&mut self, ctx: ::grpcio::RpcContext, _req: super::securitycenter_service::SetMuteRequest, sink: ::grpcio::UnarySink<super::finding::Finding>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn test_iam_permissions(&mut self, ctx: ::grpcio::RpcContext, _req: super::iam_policy::TestIamPermissionsRequest, sink: ::grpcio::UnarySink<super::iam_policy::TestIamPermissionsResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn update_big_query_export(&mut self, ctx: ::grpcio::RpcContext, _req: super::securitycenter_service::UpdateBigQueryExportRequest, sink: ::grpcio::UnarySink<super::bigquery_export::BigQueryExport>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn update_external_system(&mut self, ctx: ::grpcio::RpcContext, _req: super::securitycenter_service::UpdateExternalSystemRequest, sink: ::grpcio::UnarySink<super::external_system::ExternalSystem>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn update_finding(&mut self, ctx: ::grpcio::RpcContext, _req: super::securitycenter_service::UpdateFindingRequest, sink: ::grpcio::UnarySink<super::finding::Finding>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn update_mute_config(&mut self, ctx: ::grpcio::RpcContext, _req: super::securitycenter_service::UpdateMuteConfigRequest, sink: ::grpcio::UnarySink<super::mute_config::MuteConfig>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn update_notification_config(&mut self, ctx: ::grpcio::RpcContext, _req: super::securitycenter_service::UpdateNotificationConfigRequest, sink: ::grpcio::UnarySink<super::notification_config::NotificationConfig>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn update_resource_value_config(&mut self, ctx: ::grpcio::RpcContext, _req: super::securitycenter_service::UpdateResourceValueConfigRequest, sink: ::grpcio::UnarySink<super::resource_value_config::ResourceValueConfig>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn update_security_marks(&mut self, ctx: ::grpcio::RpcContext, _req: super::securitycenter_service::UpdateSecurityMarksRequest, sink: ::grpcio::UnarySink<super::security_marks::SecurityMarks>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn update_source(&mut self, ctx: ::grpcio::RpcContext, _req: super::securitycenter_service::UpdateSourceRequest, sink: ::grpcio::UnarySink<super::source::Source>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
}

pub fn create_security_center<S: SecurityCenter + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SECURITY_CENTER_BATCH_CREATE_RESOURCE_VALUE_CONFIGS, move |ctx, req, resp| {
        instance.batch_create_resource_value_configs(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SECURITY_CENTER_BULK_MUTE_FINDINGS, move |ctx, req, resp| {
        instance.bulk_mute_findings(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SECURITY_CENTER_CREATE_BIG_QUERY_EXPORT, move |ctx, req, resp| {
        instance.create_big_query_export(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SECURITY_CENTER_CREATE_FINDING, move |ctx, req, resp| {
        instance.create_finding(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SECURITY_CENTER_CREATE_MUTE_CONFIG, move |ctx, req, resp| {
        instance.create_mute_config(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SECURITY_CENTER_CREATE_NOTIFICATION_CONFIG, move |ctx, req, resp| {
        instance.create_notification_config(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SECURITY_CENTER_CREATE_SOURCE, move |ctx, req, resp| {
        instance.create_source(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SECURITY_CENTER_DELETE_BIG_QUERY_EXPORT, move |ctx, req, resp| {
        instance.delete_big_query_export(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SECURITY_CENTER_DELETE_MUTE_CONFIG, move |ctx, req, resp| {
        instance.delete_mute_config(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SECURITY_CENTER_DELETE_NOTIFICATION_CONFIG, move |ctx, req, resp| {
        instance.delete_notification_config(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SECURITY_CENTER_DELETE_RESOURCE_VALUE_CONFIG, move |ctx, req, resp| {
        instance.delete_resource_value_config(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SECURITY_CENTER_GET_BIG_QUERY_EXPORT, move |ctx, req, resp| {
        instance.get_big_query_export(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SECURITY_CENTER_GET_SIMULATION, move |ctx, req, resp| {
        instance.get_simulation(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SECURITY_CENTER_GET_VALUED_RESOURCE, move |ctx, req, resp| {
        instance.get_valued_resource(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SECURITY_CENTER_GET_IAM_POLICY, move |ctx, req, resp| {
        instance.get_iam_policy(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SECURITY_CENTER_GET_MUTE_CONFIG, move |ctx, req, resp| {
        instance.get_mute_config(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SECURITY_CENTER_GET_NOTIFICATION_CONFIG, move |ctx, req, resp| {
        instance.get_notification_config(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SECURITY_CENTER_GET_RESOURCE_VALUE_CONFIG, move |ctx, req, resp| {
        instance.get_resource_value_config(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SECURITY_CENTER_GET_SOURCE, move |ctx, req, resp| {
        instance.get_source(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SECURITY_CENTER_GROUP_FINDINGS, move |ctx, req, resp| {
        instance.group_findings(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SECURITY_CENTER_LIST_ATTACK_PATHS, move |ctx, req, resp| {
        instance.list_attack_paths(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SECURITY_CENTER_LIST_BIG_QUERY_EXPORTS, move |ctx, req, resp| {
        instance.list_big_query_exports(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SECURITY_CENTER_LIST_FINDINGS, move |ctx, req, resp| {
        instance.list_findings(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SECURITY_CENTER_LIST_MUTE_CONFIGS, move |ctx, req, resp| {
        instance.list_mute_configs(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SECURITY_CENTER_LIST_NOTIFICATION_CONFIGS, move |ctx, req, resp| {
        instance.list_notification_configs(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SECURITY_CENTER_LIST_RESOURCE_VALUE_CONFIGS, move |ctx, req, resp| {
        instance.list_resource_value_configs(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SECURITY_CENTER_LIST_SOURCES, move |ctx, req, resp| {
        instance.list_sources(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SECURITY_CENTER_LIST_VALUED_RESOURCES, move |ctx, req, resp| {
        instance.list_valued_resources(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SECURITY_CENTER_SET_FINDING_STATE, move |ctx, req, resp| {
        instance.set_finding_state(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SECURITY_CENTER_SET_IAM_POLICY, move |ctx, req, resp| {
        instance.set_iam_policy(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SECURITY_CENTER_SET_MUTE, move |ctx, req, resp| {
        instance.set_mute(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SECURITY_CENTER_TEST_IAM_PERMISSIONS, move |ctx, req, resp| {
        instance.test_iam_permissions(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SECURITY_CENTER_UPDATE_BIG_QUERY_EXPORT, move |ctx, req, resp| {
        instance.update_big_query_export(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SECURITY_CENTER_UPDATE_EXTERNAL_SYSTEM, move |ctx, req, resp| {
        instance.update_external_system(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SECURITY_CENTER_UPDATE_FINDING, move |ctx, req, resp| {
        instance.update_finding(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SECURITY_CENTER_UPDATE_MUTE_CONFIG, move |ctx, req, resp| {
        instance.update_mute_config(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SECURITY_CENTER_UPDATE_NOTIFICATION_CONFIG, move |ctx, req, resp| {
        instance.update_notification_config(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SECURITY_CENTER_UPDATE_RESOURCE_VALUE_CONFIG, move |ctx, req, resp| {
        instance.update_resource_value_config(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SECURITY_CENTER_UPDATE_SECURITY_MARKS, move |ctx, req, resp| {
        instance.update_security_marks(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_SECURITY_CENTER_UPDATE_SOURCE, move |ctx, req, resp| {
        instance.update_source(ctx, req, resp)
    });
    builder.build()
}
