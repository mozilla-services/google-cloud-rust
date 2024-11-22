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

const METHOD_SECURITY_CENTER_BULK_MUTE_FINDINGS: ::grpcio::Method<
    super::securitycenter_service::BulkMuteFindingsRequest,
    super::operations::Operation,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v1.SecurityCenter/BulkMuteFindings",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_SECURITY_CENTER_CREATE_SECURITY_HEALTH_ANALYTICS_CUSTOM_MODULE: ::grpcio::Method<
    super::securitycenter_service::CreateSecurityHealthAnalyticsCustomModuleRequest,
    super::security_health_analytics_custom_module::SecurityHealthAnalyticsCustomModule,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name:
        "/google.cloud.securitycenter.v1.SecurityCenter/CreateSecurityHealthAnalyticsCustomModule",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_SECURITY_CENTER_CREATE_SOURCE: ::grpcio::Method<
    super::securitycenter_service::CreateSourceRequest,
    super::source::Source,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v1.SecurityCenter/CreateSource",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_SECURITY_CENTER_CREATE_FINDING: ::grpcio::Method<
    super::securitycenter_service::CreateFindingRequest,
    super::finding::Finding,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v1.SecurityCenter/CreateFinding",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_SECURITY_CENTER_CREATE_MUTE_CONFIG: ::grpcio::Method<
    super::securitycenter_service::CreateMuteConfigRequest,
    super::mute_config::MuteConfig,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v1.SecurityCenter/CreateMuteConfig",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_SECURITY_CENTER_CREATE_NOTIFICATION_CONFIG: ::grpcio::Method<
    super::securitycenter_service::CreateNotificationConfigRequest,
    super::notification_config::NotificationConfig,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v1.SecurityCenter/CreateNotificationConfig",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_SECURITY_CENTER_DELETE_MUTE_CONFIG: ::grpcio::Method<
    super::securitycenter_service::DeleteMuteConfigRequest,
    super::empty::Empty,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v1.SecurityCenter/DeleteMuteConfig",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_SECURITY_CENTER_DELETE_NOTIFICATION_CONFIG: ::grpcio::Method<
    super::securitycenter_service::DeleteNotificationConfigRequest,
    super::empty::Empty,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v1.SecurityCenter/DeleteNotificationConfig",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_SECURITY_CENTER_DELETE_SECURITY_HEALTH_ANALYTICS_CUSTOM_MODULE: ::grpcio::Method<
    super::securitycenter_service::DeleteSecurityHealthAnalyticsCustomModuleRequest,
    super::empty::Empty,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name:
        "/google.cloud.securitycenter.v1.SecurityCenter/DeleteSecurityHealthAnalyticsCustomModule",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_SECURITY_CENTER_GET_SIMULATION: ::grpcio::Method<
    super::securitycenter_service::GetSimulationRequest,
    super::simulation::Simulation,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v1.SecurityCenter/GetSimulation",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_SECURITY_CENTER_GET_VALUED_RESOURCE: ::grpcio::Method<
    super::securitycenter_service::GetValuedResourceRequest,
    super::valued_resource::ValuedResource,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v1.SecurityCenter/GetValuedResource",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_SECURITY_CENTER_GET_BIG_QUERY_EXPORT: ::grpcio::Method<
    super::securitycenter_service::GetBigQueryExportRequest,
    super::bigquery_export::BigQueryExport,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v1.SecurityCenter/GetBigQueryExport",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_SECURITY_CENTER_GET_IAM_POLICY: ::grpcio::Method<
    super::iam_policy::GetIamPolicyRequest,
    super::policy::Policy,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v1.SecurityCenter/GetIamPolicy",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_SECURITY_CENTER_GET_MUTE_CONFIG: ::grpcio::Method<
    super::securitycenter_service::GetMuteConfigRequest,
    super::mute_config::MuteConfig,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v1.SecurityCenter/GetMuteConfig",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_SECURITY_CENTER_GET_NOTIFICATION_CONFIG: ::grpcio::Method<
    super::securitycenter_service::GetNotificationConfigRequest,
    super::notification_config::NotificationConfig,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v1.SecurityCenter/GetNotificationConfig",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_SECURITY_CENTER_GET_ORGANIZATION_SETTINGS: ::grpcio::Method<
    super::securitycenter_service::GetOrganizationSettingsRequest,
    super::organization_settings::OrganizationSettings,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v1.SecurityCenter/GetOrganizationSettings",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_SECURITY_CENTER_GET_EFFECTIVE_SECURITY_HEALTH_ANALYTICS_CUSTOM_MODULE: ::grpcio::Method<super::securitycenter_service::GetEffectiveSecurityHealthAnalyticsCustomModuleRequest, super::effective_security_health_analytics_custom_module::EffectiveSecurityHealthAnalyticsCustomModule> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v1.SecurityCenter/GetEffectiveSecurityHealthAnalyticsCustomModule",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SECURITY_CENTER_GET_SECURITY_HEALTH_ANALYTICS_CUSTOM_MODULE: ::grpcio::Method<
    super::securitycenter_service::GetSecurityHealthAnalyticsCustomModuleRequest,
    super::security_health_analytics_custom_module::SecurityHealthAnalyticsCustomModule,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v1.SecurityCenter/GetSecurityHealthAnalyticsCustomModule",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_SECURITY_CENTER_GET_SOURCE: ::grpcio::Method<
    super::securitycenter_service::GetSourceRequest,
    super::source::Source,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v1.SecurityCenter/GetSource",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_SECURITY_CENTER_GROUP_ASSETS: ::grpcio::Method<
    super::securitycenter_service::GroupAssetsRequest,
    super::securitycenter_service::GroupAssetsResponse,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v1.SecurityCenter/GroupAssets",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_SECURITY_CENTER_GROUP_FINDINGS: ::grpcio::Method<
    super::securitycenter_service::GroupFindingsRequest,
    super::securitycenter_service::GroupFindingsResponse,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v1.SecurityCenter/GroupFindings",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_SECURITY_CENTER_LIST_ASSETS: ::grpcio::Method<
    super::securitycenter_service::ListAssetsRequest,
    super::securitycenter_service::ListAssetsResponse,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v1.SecurityCenter/ListAssets",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_SECURITY_CENTER_LIST_DESCENDANT_SECURITY_HEALTH_ANALYTICS_CUSTOM_MODULES: ::grpcio::Method<super::securitycenter_service::ListDescendantSecurityHealthAnalyticsCustomModulesRequest, super::securitycenter_service::ListDescendantSecurityHealthAnalyticsCustomModulesResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v1.SecurityCenter/ListDescendantSecurityHealthAnalyticsCustomModules",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SECURITY_CENTER_LIST_FINDINGS: ::grpcio::Method<
    super::securitycenter_service::ListFindingsRequest,
    super::securitycenter_service::ListFindingsResponse,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v1.SecurityCenter/ListFindings",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_SECURITY_CENTER_LIST_MUTE_CONFIGS: ::grpcio::Method<
    super::securitycenter_service::ListMuteConfigsRequest,
    super::securitycenter_service::ListMuteConfigsResponse,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v1.SecurityCenter/ListMuteConfigs",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_SECURITY_CENTER_LIST_NOTIFICATION_CONFIGS: ::grpcio::Method<
    super::securitycenter_service::ListNotificationConfigsRequest,
    super::securitycenter_service::ListNotificationConfigsResponse,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v1.SecurityCenter/ListNotificationConfigs",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_SECURITY_CENTER_LIST_EFFECTIVE_SECURITY_HEALTH_ANALYTICS_CUSTOM_MODULES: ::grpcio::Method<super::securitycenter_service::ListEffectiveSecurityHealthAnalyticsCustomModulesRequest, super::securitycenter_service::ListEffectiveSecurityHealthAnalyticsCustomModulesResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v1.SecurityCenter/ListEffectiveSecurityHealthAnalyticsCustomModules",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SECURITY_CENTER_LIST_SECURITY_HEALTH_ANALYTICS_CUSTOM_MODULES: ::grpcio::Method<
    super::securitycenter_service::ListSecurityHealthAnalyticsCustomModulesRequest,
    super::securitycenter_service::ListSecurityHealthAnalyticsCustomModulesResponse,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v1.SecurityCenter/ListSecurityHealthAnalyticsCustomModules",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_SECURITY_CENTER_LIST_SOURCES: ::grpcio::Method<
    super::securitycenter_service::ListSourcesRequest,
    super::securitycenter_service::ListSourcesResponse,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v1.SecurityCenter/ListSources",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_SECURITY_CENTER_RUN_ASSET_DISCOVERY: ::grpcio::Method<
    super::securitycenter_service::RunAssetDiscoveryRequest,
    super::operations::Operation,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v1.SecurityCenter/RunAssetDiscovery",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_SECURITY_CENTER_SET_FINDING_STATE: ::grpcio::Method<
    super::securitycenter_service::SetFindingStateRequest,
    super::finding::Finding,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v1.SecurityCenter/SetFindingState",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_SECURITY_CENTER_SET_MUTE: ::grpcio::Method<
    super::securitycenter_service::SetMuteRequest,
    super::finding::Finding,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v1.SecurityCenter/SetMute",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_SECURITY_CENTER_SET_IAM_POLICY: ::grpcio::Method<
    super::iam_policy::SetIamPolicyRequest,
    super::policy::Policy,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v1.SecurityCenter/SetIamPolicy",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_SECURITY_CENTER_TEST_IAM_PERMISSIONS: ::grpcio::Method<
    super::iam_policy::TestIamPermissionsRequest,
    super::iam_policy::TestIamPermissionsResponse,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v1.SecurityCenter/TestIamPermissions",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_SECURITY_CENTER_SIMULATE_SECURITY_HEALTH_ANALYTICS_CUSTOM_MODULE: ::grpcio::Method<
    super::securitycenter_service::SimulateSecurityHealthAnalyticsCustomModuleRequest,
    super::securitycenter_service::SimulateSecurityHealthAnalyticsCustomModuleResponse,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name:
        "/google.cloud.securitycenter.v1.SecurityCenter/SimulateSecurityHealthAnalyticsCustomModule",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_SECURITY_CENTER_UPDATE_EXTERNAL_SYSTEM: ::grpcio::Method<
    super::securitycenter_service::UpdateExternalSystemRequest,
    super::external_system::ExternalSystem,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v1.SecurityCenter/UpdateExternalSystem",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_SECURITY_CENTER_UPDATE_FINDING: ::grpcio::Method<
    super::securitycenter_service::UpdateFindingRequest,
    super::finding::Finding,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v1.SecurityCenter/UpdateFinding",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_SECURITY_CENTER_UPDATE_MUTE_CONFIG: ::grpcio::Method<
    super::securitycenter_service::UpdateMuteConfigRequest,
    super::mute_config::MuteConfig,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v1.SecurityCenter/UpdateMuteConfig",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_SECURITY_CENTER_UPDATE_NOTIFICATION_CONFIG: ::grpcio::Method<
    super::securitycenter_service::UpdateNotificationConfigRequest,
    super::notification_config::NotificationConfig,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v1.SecurityCenter/UpdateNotificationConfig",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_SECURITY_CENTER_UPDATE_ORGANIZATION_SETTINGS: ::grpcio::Method<
    super::securitycenter_service::UpdateOrganizationSettingsRequest,
    super::organization_settings::OrganizationSettings,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v1.SecurityCenter/UpdateOrganizationSettings",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_SECURITY_CENTER_UPDATE_SECURITY_HEALTH_ANALYTICS_CUSTOM_MODULE: ::grpcio::Method<
    super::securitycenter_service::UpdateSecurityHealthAnalyticsCustomModuleRequest,
    super::security_health_analytics_custom_module::SecurityHealthAnalyticsCustomModule,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name:
        "/google.cloud.securitycenter.v1.SecurityCenter/UpdateSecurityHealthAnalyticsCustomModule",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_SECURITY_CENTER_UPDATE_SOURCE: ::grpcio::Method<
    super::securitycenter_service::UpdateSourceRequest,
    super::source::Source,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v1.SecurityCenter/UpdateSource",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_SECURITY_CENTER_UPDATE_SECURITY_MARKS: ::grpcio::Method<
    super::securitycenter_service::UpdateSecurityMarksRequest,
    super::security_marks::SecurityMarks,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v1.SecurityCenter/UpdateSecurityMarks",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_SECURITY_CENTER_CREATE_BIG_QUERY_EXPORT: ::grpcio::Method<
    super::securitycenter_service::CreateBigQueryExportRequest,
    super::bigquery_export::BigQueryExport,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v1.SecurityCenter/CreateBigQueryExport",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_SECURITY_CENTER_DELETE_BIG_QUERY_EXPORT: ::grpcio::Method<
    super::securitycenter_service::DeleteBigQueryExportRequest,
    super::empty::Empty,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v1.SecurityCenter/DeleteBigQueryExport",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_SECURITY_CENTER_UPDATE_BIG_QUERY_EXPORT: ::grpcio::Method<
    super::securitycenter_service::UpdateBigQueryExportRequest,
    super::bigquery_export::BigQueryExport,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v1.SecurityCenter/UpdateBigQueryExport",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_SECURITY_CENTER_LIST_BIG_QUERY_EXPORTS: ::grpcio::Method<
    super::securitycenter_service::ListBigQueryExportsRequest,
    super::securitycenter_service::ListBigQueryExportsResponse,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v1.SecurityCenter/ListBigQueryExports",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_SECURITY_CENTER_CREATE_EVENT_THREAT_DETECTION_CUSTOM_MODULE: ::grpcio::Method<
    super::securitycenter_service::CreateEventThreatDetectionCustomModuleRequest,
    super::event_threat_detection_custom_module::EventThreatDetectionCustomModule,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v1.SecurityCenter/CreateEventThreatDetectionCustomModule",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_SECURITY_CENTER_DELETE_EVENT_THREAT_DETECTION_CUSTOM_MODULE: ::grpcio::Method<
    super::securitycenter_service::DeleteEventThreatDetectionCustomModuleRequest,
    super::empty::Empty,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v1.SecurityCenter/DeleteEventThreatDetectionCustomModule",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_SECURITY_CENTER_GET_EVENT_THREAT_DETECTION_CUSTOM_MODULE: ::grpcio::Method<
    super::securitycenter_service::GetEventThreatDetectionCustomModuleRequest,
    super::event_threat_detection_custom_module::EventThreatDetectionCustomModule,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v1.SecurityCenter/GetEventThreatDetectionCustomModule",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_SECURITY_CENTER_LIST_DESCENDANT_EVENT_THREAT_DETECTION_CUSTOM_MODULES: ::grpcio::Method<super::securitycenter_service::ListDescendantEventThreatDetectionCustomModulesRequest, super::securitycenter_service::ListDescendantEventThreatDetectionCustomModulesResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v1.SecurityCenter/ListDescendantEventThreatDetectionCustomModules",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SECURITY_CENTER_LIST_EVENT_THREAT_DETECTION_CUSTOM_MODULES: ::grpcio::Method<
    super::securitycenter_service::ListEventThreatDetectionCustomModulesRequest,
    super::securitycenter_service::ListEventThreatDetectionCustomModulesResponse,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v1.SecurityCenter/ListEventThreatDetectionCustomModules",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_SECURITY_CENTER_UPDATE_EVENT_THREAT_DETECTION_CUSTOM_MODULE: ::grpcio::Method<
    super::securitycenter_service::UpdateEventThreatDetectionCustomModuleRequest,
    super::event_threat_detection_custom_module::EventThreatDetectionCustomModule,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v1.SecurityCenter/UpdateEventThreatDetectionCustomModule",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_SECURITY_CENTER_VALIDATE_EVENT_THREAT_DETECTION_CUSTOM_MODULE: ::grpcio::Method<
    super::securitycenter_service::ValidateEventThreatDetectionCustomModuleRequest,
    super::securitycenter_service::ValidateEventThreatDetectionCustomModuleResponse,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v1.SecurityCenter/ValidateEventThreatDetectionCustomModule",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_SECURITY_CENTER_GET_EFFECTIVE_EVENT_THREAT_DETECTION_CUSTOM_MODULE: ::grpcio::Method<super::securitycenter_service::GetEffectiveEventThreatDetectionCustomModuleRequest, super::effective_event_threat_detection_custom_module::EffectiveEventThreatDetectionCustomModule> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v1.SecurityCenter/GetEffectiveEventThreatDetectionCustomModule",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SECURITY_CENTER_LIST_EFFECTIVE_EVENT_THREAT_DETECTION_CUSTOM_MODULES: ::grpcio::Method<super::securitycenter_service::ListEffectiveEventThreatDetectionCustomModulesRequest, super::securitycenter_service::ListEffectiveEventThreatDetectionCustomModulesResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v1.SecurityCenter/ListEffectiveEventThreatDetectionCustomModules",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SECURITY_CENTER_BATCH_CREATE_RESOURCE_VALUE_CONFIGS: ::grpcio::Method<
    super::securitycenter_service::BatchCreateResourceValueConfigsRequest,
    super::securitycenter_service::BatchCreateResourceValueConfigsResponse,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v1.SecurityCenter/BatchCreateResourceValueConfigs",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_SECURITY_CENTER_DELETE_RESOURCE_VALUE_CONFIG: ::grpcio::Method<
    super::securitycenter_service::DeleteResourceValueConfigRequest,
    super::empty::Empty,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v1.SecurityCenter/DeleteResourceValueConfig",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_SECURITY_CENTER_GET_RESOURCE_VALUE_CONFIG: ::grpcio::Method<
    super::securitycenter_service::GetResourceValueConfigRequest,
    super::resource_value_config::ResourceValueConfig,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v1.SecurityCenter/GetResourceValueConfig",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_SECURITY_CENTER_LIST_RESOURCE_VALUE_CONFIGS: ::grpcio::Method<
    super::securitycenter_service::ListResourceValueConfigsRequest,
    super::securitycenter_service::ListResourceValueConfigsResponse,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v1.SecurityCenter/ListResourceValueConfigs",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_SECURITY_CENTER_UPDATE_RESOURCE_VALUE_CONFIG: ::grpcio::Method<
    super::securitycenter_service::UpdateResourceValueConfigRequest,
    super::resource_value_config::ResourceValueConfig,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v1.SecurityCenter/UpdateResourceValueConfig",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_SECURITY_CENTER_LIST_VALUED_RESOURCES: ::grpcio::Method<
    super::securitycenter_service::ListValuedResourcesRequest,
    super::securitycenter_service::ListValuedResourcesResponse,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v1.SecurityCenter/ListValuedResources",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_SECURITY_CENTER_LIST_ATTACK_PATHS: ::grpcio::Method<
    super::securitycenter_service::ListAttackPathsRequest,
    super::securitycenter_service::ListAttackPathsResponse,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.securitycenter.v1.SecurityCenter/ListAttackPaths",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
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

    pub fn bulk_mute_findings_opt(
        &self,
        req: &super::securitycenter_service::BulkMuteFindingsRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::operations::Operation> {
        self.client
            .unary_call(&METHOD_SECURITY_CENTER_BULK_MUTE_FINDINGS, req, opt)
    }

    pub fn bulk_mute_findings(
        &self,
        req: &super::securitycenter_service::BulkMuteFindingsRequest,
    ) -> ::grpcio::Result<super::operations::Operation> {
        self.bulk_mute_findings_opt(req, ::grpcio::CallOption::default())
    }

    pub fn bulk_mute_findings_async_opt(
        &self,
        req: &super::securitycenter_service::BulkMuteFindingsRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operations::Operation>> {
        self.client
            .unary_call_async(&METHOD_SECURITY_CENTER_BULK_MUTE_FINDINGS, req, opt)
    }

    pub fn bulk_mute_findings_async(
        &self,
        req: &super::securitycenter_service::BulkMuteFindingsRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operations::Operation>> {
        self.bulk_mute_findings_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_security_health_analytics_custom_module_opt(
        &self,
        req: &super::securitycenter_service::CreateSecurityHealthAnalyticsCustomModuleRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<
        super::security_health_analytics_custom_module::SecurityHealthAnalyticsCustomModule,
    > {
        self.client.unary_call(
            &METHOD_SECURITY_CENTER_CREATE_SECURITY_HEALTH_ANALYTICS_CUSTOM_MODULE,
            req,
            opt,
        )
    }

    pub fn create_security_health_analytics_custom_module(
        &self,
        req: &super::securitycenter_service::CreateSecurityHealthAnalyticsCustomModuleRequest,
    ) -> ::grpcio::Result<
        super::security_health_analytics_custom_module::SecurityHealthAnalyticsCustomModule,
    > {
        self.create_security_health_analytics_custom_module_opt(
            req,
            ::grpcio::CallOption::default(),
        )
    }

    pub fn create_security_health_analytics_custom_module_async_opt(
        &self,
        req: &super::securitycenter_service::CreateSecurityHealthAnalyticsCustomModuleRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<
        ::grpcio::ClientUnaryReceiver<
            super::security_health_analytics_custom_module::SecurityHealthAnalyticsCustomModule,
        >,
    > {
        self.client.unary_call_async(
            &METHOD_SECURITY_CENTER_CREATE_SECURITY_HEALTH_ANALYTICS_CUSTOM_MODULE,
            req,
            opt,
        )
    }

    pub fn create_security_health_analytics_custom_module_async(
        &self,
        req: &super::securitycenter_service::CreateSecurityHealthAnalyticsCustomModuleRequest,
    ) -> ::grpcio::Result<
        ::grpcio::ClientUnaryReceiver<
            super::security_health_analytics_custom_module::SecurityHealthAnalyticsCustomModule,
        >,
    > {
        self.create_security_health_analytics_custom_module_async_opt(
            req,
            ::grpcio::CallOption::default(),
        )
    }

    pub fn create_source_opt(
        &self,
        req: &super::securitycenter_service::CreateSourceRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::source::Source> {
        self.client
            .unary_call(&METHOD_SECURITY_CENTER_CREATE_SOURCE, req, opt)
    }

    pub fn create_source(
        &self,
        req: &super::securitycenter_service::CreateSourceRequest,
    ) -> ::grpcio::Result<super::source::Source> {
        self.create_source_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_source_async_opt(
        &self,
        req: &super::securitycenter_service::CreateSourceRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::source::Source>> {
        self.client
            .unary_call_async(&METHOD_SECURITY_CENTER_CREATE_SOURCE, req, opt)
    }

    pub fn create_source_async(
        &self,
        req: &super::securitycenter_service::CreateSourceRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::source::Source>> {
        self.create_source_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_finding_opt(
        &self,
        req: &super::securitycenter_service::CreateFindingRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::finding::Finding> {
        self.client
            .unary_call(&METHOD_SECURITY_CENTER_CREATE_FINDING, req, opt)
    }

    pub fn create_finding(
        &self,
        req: &super::securitycenter_service::CreateFindingRequest,
    ) -> ::grpcio::Result<super::finding::Finding> {
        self.create_finding_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_finding_async_opt(
        &self,
        req: &super::securitycenter_service::CreateFindingRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::finding::Finding>> {
        self.client
            .unary_call_async(&METHOD_SECURITY_CENTER_CREATE_FINDING, req, opt)
    }

    pub fn create_finding_async(
        &self,
        req: &super::securitycenter_service::CreateFindingRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::finding::Finding>> {
        self.create_finding_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_mute_config_opt(
        &self,
        req: &super::securitycenter_service::CreateMuteConfigRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::mute_config::MuteConfig> {
        self.client
            .unary_call(&METHOD_SECURITY_CENTER_CREATE_MUTE_CONFIG, req, opt)
    }

    pub fn create_mute_config(
        &self,
        req: &super::securitycenter_service::CreateMuteConfigRequest,
    ) -> ::grpcio::Result<super::mute_config::MuteConfig> {
        self.create_mute_config_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_mute_config_async_opt(
        &self,
        req: &super::securitycenter_service::CreateMuteConfigRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::mute_config::MuteConfig>> {
        self.client
            .unary_call_async(&METHOD_SECURITY_CENTER_CREATE_MUTE_CONFIG, req, opt)
    }

    pub fn create_mute_config_async(
        &self,
        req: &super::securitycenter_service::CreateMuteConfigRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::mute_config::MuteConfig>> {
        self.create_mute_config_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_notification_config_opt(
        &self,
        req: &super::securitycenter_service::CreateNotificationConfigRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::notification_config::NotificationConfig> {
        self.client
            .unary_call(&METHOD_SECURITY_CENTER_CREATE_NOTIFICATION_CONFIG, req, opt)
    }

    pub fn create_notification_config(
        &self,
        req: &super::securitycenter_service::CreateNotificationConfigRequest,
    ) -> ::grpcio::Result<super::notification_config::NotificationConfig> {
        self.create_notification_config_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_notification_config_async_opt(
        &self,
        req: &super::securitycenter_service::CreateNotificationConfigRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<
        ::grpcio::ClientUnaryReceiver<super::notification_config::NotificationConfig>,
    > {
        self.client
            .unary_call_async(&METHOD_SECURITY_CENTER_CREATE_NOTIFICATION_CONFIG, req, opt)
    }

    pub fn create_notification_config_async(
        &self,
        req: &super::securitycenter_service::CreateNotificationConfigRequest,
    ) -> ::grpcio::Result<
        ::grpcio::ClientUnaryReceiver<super::notification_config::NotificationConfig>,
    > {
        self.create_notification_config_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_mute_config_opt(
        &self,
        req: &super::securitycenter_service::DeleteMuteConfigRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::empty::Empty> {
        self.client
            .unary_call(&METHOD_SECURITY_CENTER_DELETE_MUTE_CONFIG, req, opt)
    }

    pub fn delete_mute_config(
        &self,
        req: &super::securitycenter_service::DeleteMuteConfigRequest,
    ) -> ::grpcio::Result<super::empty::Empty> {
        self.delete_mute_config_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_mute_config_async_opt(
        &self,
        req: &super::securitycenter_service::DeleteMuteConfigRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.client
            .unary_call_async(&METHOD_SECURITY_CENTER_DELETE_MUTE_CONFIG, req, opt)
    }

    pub fn delete_mute_config_async(
        &self,
        req: &super::securitycenter_service::DeleteMuteConfigRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.delete_mute_config_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_notification_config_opt(
        &self,
        req: &super::securitycenter_service::DeleteNotificationConfigRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::empty::Empty> {
        self.client
            .unary_call(&METHOD_SECURITY_CENTER_DELETE_NOTIFICATION_CONFIG, req, opt)
    }

    pub fn delete_notification_config(
        &self,
        req: &super::securitycenter_service::DeleteNotificationConfigRequest,
    ) -> ::grpcio::Result<super::empty::Empty> {
        self.delete_notification_config_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_notification_config_async_opt(
        &self,
        req: &super::securitycenter_service::DeleteNotificationConfigRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.client
            .unary_call_async(&METHOD_SECURITY_CENTER_DELETE_NOTIFICATION_CONFIG, req, opt)
    }

    pub fn delete_notification_config_async(
        &self,
        req: &super::securitycenter_service::DeleteNotificationConfigRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.delete_notification_config_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_security_health_analytics_custom_module_opt(
        &self,
        req: &super::securitycenter_service::DeleteSecurityHealthAnalyticsCustomModuleRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::empty::Empty> {
        self.client.unary_call(
            &METHOD_SECURITY_CENTER_DELETE_SECURITY_HEALTH_ANALYTICS_CUSTOM_MODULE,
            req,
            opt,
        )
    }

    pub fn delete_security_health_analytics_custom_module(
        &self,
        req: &super::securitycenter_service::DeleteSecurityHealthAnalyticsCustomModuleRequest,
    ) -> ::grpcio::Result<super::empty::Empty> {
        self.delete_security_health_analytics_custom_module_opt(
            req,
            ::grpcio::CallOption::default(),
        )
    }

    pub fn delete_security_health_analytics_custom_module_async_opt(
        &self,
        req: &super::securitycenter_service::DeleteSecurityHealthAnalyticsCustomModuleRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.client.unary_call_async(
            &METHOD_SECURITY_CENTER_DELETE_SECURITY_HEALTH_ANALYTICS_CUSTOM_MODULE,
            req,
            opt,
        )
    }

    pub fn delete_security_health_analytics_custom_module_async(
        &self,
        req: &super::securitycenter_service::DeleteSecurityHealthAnalyticsCustomModuleRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.delete_security_health_analytics_custom_module_async_opt(
            req,
            ::grpcio::CallOption::default(),
        )
    }

    pub fn get_simulation_opt(
        &self,
        req: &super::securitycenter_service::GetSimulationRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::simulation::Simulation> {
        self.client
            .unary_call(&METHOD_SECURITY_CENTER_GET_SIMULATION, req, opt)
    }

    pub fn get_simulation(
        &self,
        req: &super::securitycenter_service::GetSimulationRequest,
    ) -> ::grpcio::Result<super::simulation::Simulation> {
        self.get_simulation_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_simulation_async_opt(
        &self,
        req: &super::securitycenter_service::GetSimulationRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::simulation::Simulation>> {
        self.client
            .unary_call_async(&METHOD_SECURITY_CENTER_GET_SIMULATION, req, opt)
    }

    pub fn get_simulation_async(
        &self,
        req: &super::securitycenter_service::GetSimulationRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::simulation::Simulation>> {
        self.get_simulation_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_valued_resource_opt(
        &self,
        req: &super::securitycenter_service::GetValuedResourceRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::valued_resource::ValuedResource> {
        self.client
            .unary_call(&METHOD_SECURITY_CENTER_GET_VALUED_RESOURCE, req, opt)
    }

    pub fn get_valued_resource(
        &self,
        req: &super::securitycenter_service::GetValuedResourceRequest,
    ) -> ::grpcio::Result<super::valued_resource::ValuedResource> {
        self.get_valued_resource_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_valued_resource_async_opt(
        &self,
        req: &super::securitycenter_service::GetValuedResourceRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::valued_resource::ValuedResource>>
    {
        self.client
            .unary_call_async(&METHOD_SECURITY_CENTER_GET_VALUED_RESOURCE, req, opt)
    }

    pub fn get_valued_resource_async(
        &self,
        req: &super::securitycenter_service::GetValuedResourceRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::valued_resource::ValuedResource>>
    {
        self.get_valued_resource_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_big_query_export_opt(
        &self,
        req: &super::securitycenter_service::GetBigQueryExportRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::bigquery_export::BigQueryExport> {
        self.client
            .unary_call(&METHOD_SECURITY_CENTER_GET_BIG_QUERY_EXPORT, req, opt)
    }

    pub fn get_big_query_export(
        &self,
        req: &super::securitycenter_service::GetBigQueryExportRequest,
    ) -> ::grpcio::Result<super::bigquery_export::BigQueryExport> {
        self.get_big_query_export_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_big_query_export_async_opt(
        &self,
        req: &super::securitycenter_service::GetBigQueryExportRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::bigquery_export::BigQueryExport>>
    {
        self.client
            .unary_call_async(&METHOD_SECURITY_CENTER_GET_BIG_QUERY_EXPORT, req, opt)
    }

    pub fn get_big_query_export_async(
        &self,
        req: &super::securitycenter_service::GetBigQueryExportRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::bigquery_export::BigQueryExport>>
    {
        self.get_big_query_export_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_iam_policy_opt(
        &self,
        req: &super::iam_policy::GetIamPolicyRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::policy::Policy> {
        self.client
            .unary_call(&METHOD_SECURITY_CENTER_GET_IAM_POLICY, req, opt)
    }

    pub fn get_iam_policy(
        &self,
        req: &super::iam_policy::GetIamPolicyRequest,
    ) -> ::grpcio::Result<super::policy::Policy> {
        self.get_iam_policy_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_iam_policy_async_opt(
        &self,
        req: &super::iam_policy::GetIamPolicyRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::policy::Policy>> {
        self.client
            .unary_call_async(&METHOD_SECURITY_CENTER_GET_IAM_POLICY, req, opt)
    }

    pub fn get_iam_policy_async(
        &self,
        req: &super::iam_policy::GetIamPolicyRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::policy::Policy>> {
        self.get_iam_policy_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_mute_config_opt(
        &self,
        req: &super::securitycenter_service::GetMuteConfigRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::mute_config::MuteConfig> {
        self.client
            .unary_call(&METHOD_SECURITY_CENTER_GET_MUTE_CONFIG, req, opt)
    }

    pub fn get_mute_config(
        &self,
        req: &super::securitycenter_service::GetMuteConfigRequest,
    ) -> ::grpcio::Result<super::mute_config::MuteConfig> {
        self.get_mute_config_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_mute_config_async_opt(
        &self,
        req: &super::securitycenter_service::GetMuteConfigRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::mute_config::MuteConfig>> {
        self.client
            .unary_call_async(&METHOD_SECURITY_CENTER_GET_MUTE_CONFIG, req, opt)
    }

    pub fn get_mute_config_async(
        &self,
        req: &super::securitycenter_service::GetMuteConfigRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::mute_config::MuteConfig>> {
        self.get_mute_config_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_notification_config_opt(
        &self,
        req: &super::securitycenter_service::GetNotificationConfigRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::notification_config::NotificationConfig> {
        self.client
            .unary_call(&METHOD_SECURITY_CENTER_GET_NOTIFICATION_CONFIG, req, opt)
    }

    pub fn get_notification_config(
        &self,
        req: &super::securitycenter_service::GetNotificationConfigRequest,
    ) -> ::grpcio::Result<super::notification_config::NotificationConfig> {
        self.get_notification_config_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_notification_config_async_opt(
        &self,
        req: &super::securitycenter_service::GetNotificationConfigRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<
        ::grpcio::ClientUnaryReceiver<super::notification_config::NotificationConfig>,
    > {
        self.client
            .unary_call_async(&METHOD_SECURITY_CENTER_GET_NOTIFICATION_CONFIG, req, opt)
    }

    pub fn get_notification_config_async(
        &self,
        req: &super::securitycenter_service::GetNotificationConfigRequest,
    ) -> ::grpcio::Result<
        ::grpcio::ClientUnaryReceiver<super::notification_config::NotificationConfig>,
    > {
        self.get_notification_config_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_organization_settings_opt(
        &self,
        req: &super::securitycenter_service::GetOrganizationSettingsRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::organization_settings::OrganizationSettings> {
        self.client
            .unary_call(&METHOD_SECURITY_CENTER_GET_ORGANIZATION_SETTINGS, req, opt)
    }

    pub fn get_organization_settings(
        &self,
        req: &super::securitycenter_service::GetOrganizationSettingsRequest,
    ) -> ::grpcio::Result<super::organization_settings::OrganizationSettings> {
        self.get_organization_settings_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_organization_settings_async_opt(
        &self,
        req: &super::securitycenter_service::GetOrganizationSettingsRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<
        ::grpcio::ClientUnaryReceiver<super::organization_settings::OrganizationSettings>,
    > {
        self.client
            .unary_call_async(&METHOD_SECURITY_CENTER_GET_ORGANIZATION_SETTINGS, req, opt)
    }

    pub fn get_organization_settings_async(
        &self,
        req: &super::securitycenter_service::GetOrganizationSettingsRequest,
    ) -> ::grpcio::Result<
        ::grpcio::ClientUnaryReceiver<super::organization_settings::OrganizationSettings>,
    > {
        self.get_organization_settings_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_effective_security_health_analytics_custom_module_opt(&self, req: &super::securitycenter_service::GetEffectiveSecurityHealthAnalyticsCustomModuleRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::effective_security_health_analytics_custom_module::EffectiveSecurityHealthAnalyticsCustomModule>{
        self.client.unary_call(
            &METHOD_SECURITY_CENTER_GET_EFFECTIVE_SECURITY_HEALTH_ANALYTICS_CUSTOM_MODULE,
            req,
            opt,
        )
    }

    pub fn get_effective_security_health_analytics_custom_module(&self, req: &super::securitycenter_service::GetEffectiveSecurityHealthAnalyticsCustomModuleRequest) -> ::grpcio::Result<super::effective_security_health_analytics_custom_module::EffectiveSecurityHealthAnalyticsCustomModule>{
        self.get_effective_security_health_analytics_custom_module_opt(
            req,
            ::grpcio::CallOption::default(),
        )
    }

    pub fn get_effective_security_health_analytics_custom_module_async_opt(&self, req: &super::securitycenter_service::GetEffectiveSecurityHealthAnalyticsCustomModuleRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::effective_security_health_analytics_custom_module::EffectiveSecurityHealthAnalyticsCustomModule>>{
        self.client.unary_call_async(
            &METHOD_SECURITY_CENTER_GET_EFFECTIVE_SECURITY_HEALTH_ANALYTICS_CUSTOM_MODULE,
            req,
            opt,
        )
    }

    pub fn get_effective_security_health_analytics_custom_module_async(&self, req: &super::securitycenter_service::GetEffectiveSecurityHealthAnalyticsCustomModuleRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::effective_security_health_analytics_custom_module::EffectiveSecurityHealthAnalyticsCustomModule>>{
        self.get_effective_security_health_analytics_custom_module_async_opt(
            req,
            ::grpcio::CallOption::default(),
        )
    }

    pub fn get_security_health_analytics_custom_module_opt(
        &self,
        req: &super::securitycenter_service::GetSecurityHealthAnalyticsCustomModuleRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<
        super::security_health_analytics_custom_module::SecurityHealthAnalyticsCustomModule,
    > {
        self.client.unary_call(
            &METHOD_SECURITY_CENTER_GET_SECURITY_HEALTH_ANALYTICS_CUSTOM_MODULE,
            req,
            opt,
        )
    }

    pub fn get_security_health_analytics_custom_module(
        &self,
        req: &super::securitycenter_service::GetSecurityHealthAnalyticsCustomModuleRequest,
    ) -> ::grpcio::Result<
        super::security_health_analytics_custom_module::SecurityHealthAnalyticsCustomModule,
    > {
        self.get_security_health_analytics_custom_module_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_security_health_analytics_custom_module_async_opt(
        &self,
        req: &super::securitycenter_service::GetSecurityHealthAnalyticsCustomModuleRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<
        ::grpcio::ClientUnaryReceiver<
            super::security_health_analytics_custom_module::SecurityHealthAnalyticsCustomModule,
        >,
    > {
        self.client.unary_call_async(
            &METHOD_SECURITY_CENTER_GET_SECURITY_HEALTH_ANALYTICS_CUSTOM_MODULE,
            req,
            opt,
        )
    }

    pub fn get_security_health_analytics_custom_module_async(
        &self,
        req: &super::securitycenter_service::GetSecurityHealthAnalyticsCustomModuleRequest,
    ) -> ::grpcio::Result<
        ::grpcio::ClientUnaryReceiver<
            super::security_health_analytics_custom_module::SecurityHealthAnalyticsCustomModule,
        >,
    > {
        self.get_security_health_analytics_custom_module_async_opt(
            req,
            ::grpcio::CallOption::default(),
        )
    }

    pub fn get_source_opt(
        &self,
        req: &super::securitycenter_service::GetSourceRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::source::Source> {
        self.client
            .unary_call(&METHOD_SECURITY_CENTER_GET_SOURCE, req, opt)
    }

    pub fn get_source(
        &self,
        req: &super::securitycenter_service::GetSourceRequest,
    ) -> ::grpcio::Result<super::source::Source> {
        self.get_source_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_source_async_opt(
        &self,
        req: &super::securitycenter_service::GetSourceRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::source::Source>> {
        self.client
            .unary_call_async(&METHOD_SECURITY_CENTER_GET_SOURCE, req, opt)
    }

    pub fn get_source_async(
        &self,
        req: &super::securitycenter_service::GetSourceRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::source::Source>> {
        self.get_source_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn group_assets_opt(
        &self,
        req: &super::securitycenter_service::GroupAssetsRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::securitycenter_service::GroupAssetsResponse> {
        self.client
            .unary_call(&METHOD_SECURITY_CENTER_GROUP_ASSETS, req, opt)
    }

    pub fn group_assets(
        &self,
        req: &super::securitycenter_service::GroupAssetsRequest,
    ) -> ::grpcio::Result<super::securitycenter_service::GroupAssetsResponse> {
        self.group_assets_opt(req, ::grpcio::CallOption::default())
    }

    pub fn group_assets_async_opt(
        &self,
        req: &super::securitycenter_service::GroupAssetsRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<
        ::grpcio::ClientUnaryReceiver<super::securitycenter_service::GroupAssetsResponse>,
    > {
        self.client
            .unary_call_async(&METHOD_SECURITY_CENTER_GROUP_ASSETS, req, opt)
    }

    pub fn group_assets_async(
        &self,
        req: &super::securitycenter_service::GroupAssetsRequest,
    ) -> ::grpcio::Result<
        ::grpcio::ClientUnaryReceiver<super::securitycenter_service::GroupAssetsResponse>,
    > {
        self.group_assets_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn group_findings_opt(
        &self,
        req: &super::securitycenter_service::GroupFindingsRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::securitycenter_service::GroupFindingsResponse> {
        self.client
            .unary_call(&METHOD_SECURITY_CENTER_GROUP_FINDINGS, req, opt)
    }

    pub fn group_findings(
        &self,
        req: &super::securitycenter_service::GroupFindingsRequest,
    ) -> ::grpcio::Result<super::securitycenter_service::GroupFindingsResponse> {
        self.group_findings_opt(req, ::grpcio::CallOption::default())
    }

    pub fn group_findings_async_opt(
        &self,
        req: &super::securitycenter_service::GroupFindingsRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<
        ::grpcio::ClientUnaryReceiver<super::securitycenter_service::GroupFindingsResponse>,
    > {
        self.client
            .unary_call_async(&METHOD_SECURITY_CENTER_GROUP_FINDINGS, req, opt)
    }

    pub fn group_findings_async(
        &self,
        req: &super::securitycenter_service::GroupFindingsRequest,
    ) -> ::grpcio::Result<
        ::grpcio::ClientUnaryReceiver<super::securitycenter_service::GroupFindingsResponse>,
    > {
        self.group_findings_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_assets_opt(
        &self,
        req: &super::securitycenter_service::ListAssetsRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::securitycenter_service::ListAssetsResponse> {
        self.client
            .unary_call(&METHOD_SECURITY_CENTER_LIST_ASSETS, req, opt)
    }

    pub fn list_assets(
        &self,
        req: &super::securitycenter_service::ListAssetsRequest,
    ) -> ::grpcio::Result<super::securitycenter_service::ListAssetsResponse> {
        self.list_assets_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_assets_async_opt(
        &self,
        req: &super::securitycenter_service::ListAssetsRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<
        ::grpcio::ClientUnaryReceiver<super::securitycenter_service::ListAssetsResponse>,
    > {
        self.client
            .unary_call_async(&METHOD_SECURITY_CENTER_LIST_ASSETS, req, opt)
    }

    pub fn list_assets_async(
        &self,
        req: &super::securitycenter_service::ListAssetsRequest,
    ) -> ::grpcio::Result<
        ::grpcio::ClientUnaryReceiver<super::securitycenter_service::ListAssetsResponse>,
    > {
        self.list_assets_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_descendant_security_health_analytics_custom_modules_opt(
        &self,
        req: &super::securitycenter_service::ListDescendantSecurityHealthAnalyticsCustomModulesRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<
        super::securitycenter_service::ListDescendantSecurityHealthAnalyticsCustomModulesResponse,
    > {
        self.client.unary_call(
            &METHOD_SECURITY_CENTER_LIST_DESCENDANT_SECURITY_HEALTH_ANALYTICS_CUSTOM_MODULES,
            req,
            opt,
        )
    }

    pub fn list_descendant_security_health_analytics_custom_modules(
        &self,
        req: &super::securitycenter_service::ListDescendantSecurityHealthAnalyticsCustomModulesRequest,
    ) -> ::grpcio::Result<
        super::securitycenter_service::ListDescendantSecurityHealthAnalyticsCustomModulesResponse,
    > {
        self.list_descendant_security_health_analytics_custom_modules_opt(
            req,
            ::grpcio::CallOption::default(),
        )
    }

    pub fn list_descendant_security_health_analytics_custom_modules_async_opt(&self, req: &super::securitycenter_service::ListDescendantSecurityHealthAnalyticsCustomModulesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::securitycenter_service::ListDescendantSecurityHealthAnalyticsCustomModulesResponse>>{
        self.client.unary_call_async(
            &METHOD_SECURITY_CENTER_LIST_DESCENDANT_SECURITY_HEALTH_ANALYTICS_CUSTOM_MODULES,
            req,
            opt,
        )
    }

    pub fn list_descendant_security_health_analytics_custom_modules_async(&self, req: &super::securitycenter_service::ListDescendantSecurityHealthAnalyticsCustomModulesRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::securitycenter_service::ListDescendantSecurityHealthAnalyticsCustomModulesResponse>>{
        self.list_descendant_security_health_analytics_custom_modules_async_opt(
            req,
            ::grpcio::CallOption::default(),
        )
    }

    pub fn list_findings_opt(
        &self,
        req: &super::securitycenter_service::ListFindingsRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::securitycenter_service::ListFindingsResponse> {
        self.client
            .unary_call(&METHOD_SECURITY_CENTER_LIST_FINDINGS, req, opt)
    }

    pub fn list_findings(
        &self,
        req: &super::securitycenter_service::ListFindingsRequest,
    ) -> ::grpcio::Result<super::securitycenter_service::ListFindingsResponse> {
        self.list_findings_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_findings_async_opt(
        &self,
        req: &super::securitycenter_service::ListFindingsRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<
        ::grpcio::ClientUnaryReceiver<super::securitycenter_service::ListFindingsResponse>,
    > {
        self.client
            .unary_call_async(&METHOD_SECURITY_CENTER_LIST_FINDINGS, req, opt)
    }

    pub fn list_findings_async(
        &self,
        req: &super::securitycenter_service::ListFindingsRequest,
    ) -> ::grpcio::Result<
        ::grpcio::ClientUnaryReceiver<super::securitycenter_service::ListFindingsResponse>,
    > {
        self.list_findings_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_mute_configs_opt(
        &self,
        req: &super::securitycenter_service::ListMuteConfigsRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::securitycenter_service::ListMuteConfigsResponse> {
        self.client
            .unary_call(&METHOD_SECURITY_CENTER_LIST_MUTE_CONFIGS, req, opt)
    }

    pub fn list_mute_configs(
        &self,
        req: &super::securitycenter_service::ListMuteConfigsRequest,
    ) -> ::grpcio::Result<super::securitycenter_service::ListMuteConfigsResponse> {
        self.list_mute_configs_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_mute_configs_async_opt(
        &self,
        req: &super::securitycenter_service::ListMuteConfigsRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<
        ::grpcio::ClientUnaryReceiver<super::securitycenter_service::ListMuteConfigsResponse>,
    > {
        self.client
            .unary_call_async(&METHOD_SECURITY_CENTER_LIST_MUTE_CONFIGS, req, opt)
    }

    pub fn list_mute_configs_async(
        &self,
        req: &super::securitycenter_service::ListMuteConfigsRequest,
    ) -> ::grpcio::Result<
        ::grpcio::ClientUnaryReceiver<super::securitycenter_service::ListMuteConfigsResponse>,
    > {
        self.list_mute_configs_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_notification_configs_opt(
        &self,
        req: &super::securitycenter_service::ListNotificationConfigsRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::securitycenter_service::ListNotificationConfigsResponse> {
        self.client
            .unary_call(&METHOD_SECURITY_CENTER_LIST_NOTIFICATION_CONFIGS, req, opt)
    }

    pub fn list_notification_configs(
        &self,
        req: &super::securitycenter_service::ListNotificationConfigsRequest,
    ) -> ::grpcio::Result<super::securitycenter_service::ListNotificationConfigsResponse> {
        self.list_notification_configs_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_notification_configs_async_opt(
        &self,
        req: &super::securitycenter_service::ListNotificationConfigsRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<
        ::grpcio::ClientUnaryReceiver<
            super::securitycenter_service::ListNotificationConfigsResponse,
        >,
    > {
        self.client
            .unary_call_async(&METHOD_SECURITY_CENTER_LIST_NOTIFICATION_CONFIGS, req, opt)
    }

    pub fn list_notification_configs_async(
        &self,
        req: &super::securitycenter_service::ListNotificationConfigsRequest,
    ) -> ::grpcio::Result<
        ::grpcio::ClientUnaryReceiver<
            super::securitycenter_service::ListNotificationConfigsResponse,
        >,
    > {
        self.list_notification_configs_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_effective_security_health_analytics_custom_modules_opt(
        &self,
        req: &super::securitycenter_service::ListEffectiveSecurityHealthAnalyticsCustomModulesRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<
        super::securitycenter_service::ListEffectiveSecurityHealthAnalyticsCustomModulesResponse,
    > {
        self.client.unary_call(
            &METHOD_SECURITY_CENTER_LIST_EFFECTIVE_SECURITY_HEALTH_ANALYTICS_CUSTOM_MODULES,
            req,
            opt,
        )
    }

    pub fn list_effective_security_health_analytics_custom_modules(
        &self,
        req: &super::securitycenter_service::ListEffectiveSecurityHealthAnalyticsCustomModulesRequest,
    ) -> ::grpcio::Result<
        super::securitycenter_service::ListEffectiveSecurityHealthAnalyticsCustomModulesResponse,
    > {
        self.list_effective_security_health_analytics_custom_modules_opt(
            req,
            ::grpcio::CallOption::default(),
        )
    }

    pub fn list_effective_security_health_analytics_custom_modules_async_opt(&self, req: &super::securitycenter_service::ListEffectiveSecurityHealthAnalyticsCustomModulesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::securitycenter_service::ListEffectiveSecurityHealthAnalyticsCustomModulesResponse>>{
        self.client.unary_call_async(
            &METHOD_SECURITY_CENTER_LIST_EFFECTIVE_SECURITY_HEALTH_ANALYTICS_CUSTOM_MODULES,
            req,
            opt,
        )
    }

    pub fn list_effective_security_health_analytics_custom_modules_async(&self, req: &super::securitycenter_service::ListEffectiveSecurityHealthAnalyticsCustomModulesRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::securitycenter_service::ListEffectiveSecurityHealthAnalyticsCustomModulesResponse>>{
        self.list_effective_security_health_analytics_custom_modules_async_opt(
            req,
            ::grpcio::CallOption::default(),
        )
    }

    pub fn list_security_health_analytics_custom_modules_opt(
        &self,
        req: &super::securitycenter_service::ListSecurityHealthAnalyticsCustomModulesRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<
        super::securitycenter_service::ListSecurityHealthAnalyticsCustomModulesResponse,
    > {
        self.client.unary_call(
            &METHOD_SECURITY_CENTER_LIST_SECURITY_HEALTH_ANALYTICS_CUSTOM_MODULES,
            req,
            opt,
        )
    }

    pub fn list_security_health_analytics_custom_modules(
        &self,
        req: &super::securitycenter_service::ListSecurityHealthAnalyticsCustomModulesRequest,
    ) -> ::grpcio::Result<
        super::securitycenter_service::ListSecurityHealthAnalyticsCustomModulesResponse,
    > {
        self.list_security_health_analytics_custom_modules_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_security_health_analytics_custom_modules_async_opt(
        &self,
        req: &super::securitycenter_service::ListSecurityHealthAnalyticsCustomModulesRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<
        ::grpcio::ClientUnaryReceiver<
            super::securitycenter_service::ListSecurityHealthAnalyticsCustomModulesResponse,
        >,
    > {
        self.client.unary_call_async(
            &METHOD_SECURITY_CENTER_LIST_SECURITY_HEALTH_ANALYTICS_CUSTOM_MODULES,
            req,
            opt,
        )
    }

    pub fn list_security_health_analytics_custom_modules_async(
        &self,
        req: &super::securitycenter_service::ListSecurityHealthAnalyticsCustomModulesRequest,
    ) -> ::grpcio::Result<
        ::grpcio::ClientUnaryReceiver<
            super::securitycenter_service::ListSecurityHealthAnalyticsCustomModulesResponse,
        >,
    > {
        self.list_security_health_analytics_custom_modules_async_opt(
            req,
            ::grpcio::CallOption::default(),
        )
    }

    pub fn list_sources_opt(
        &self,
        req: &super::securitycenter_service::ListSourcesRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::securitycenter_service::ListSourcesResponse> {
        self.client
            .unary_call(&METHOD_SECURITY_CENTER_LIST_SOURCES, req, opt)
    }

    pub fn list_sources(
        &self,
        req: &super::securitycenter_service::ListSourcesRequest,
    ) -> ::grpcio::Result<super::securitycenter_service::ListSourcesResponse> {
        self.list_sources_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_sources_async_opt(
        &self,
        req: &super::securitycenter_service::ListSourcesRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<
        ::grpcio::ClientUnaryReceiver<super::securitycenter_service::ListSourcesResponse>,
    > {
        self.client
            .unary_call_async(&METHOD_SECURITY_CENTER_LIST_SOURCES, req, opt)
    }

    pub fn list_sources_async(
        &self,
        req: &super::securitycenter_service::ListSourcesRequest,
    ) -> ::grpcio::Result<
        ::grpcio::ClientUnaryReceiver<super::securitycenter_service::ListSourcesResponse>,
    > {
        self.list_sources_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn run_asset_discovery_opt(
        &self,
        req: &super::securitycenter_service::RunAssetDiscoveryRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::operations::Operation> {
        self.client
            .unary_call(&METHOD_SECURITY_CENTER_RUN_ASSET_DISCOVERY, req, opt)
    }

    pub fn run_asset_discovery(
        &self,
        req: &super::securitycenter_service::RunAssetDiscoveryRequest,
    ) -> ::grpcio::Result<super::operations::Operation> {
        self.run_asset_discovery_opt(req, ::grpcio::CallOption::default())
    }

    pub fn run_asset_discovery_async_opt(
        &self,
        req: &super::securitycenter_service::RunAssetDiscoveryRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operations::Operation>> {
        self.client
            .unary_call_async(&METHOD_SECURITY_CENTER_RUN_ASSET_DISCOVERY, req, opt)
    }

    pub fn run_asset_discovery_async(
        &self,
        req: &super::securitycenter_service::RunAssetDiscoveryRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operations::Operation>> {
        self.run_asset_discovery_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_finding_state_opt(
        &self,
        req: &super::securitycenter_service::SetFindingStateRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::finding::Finding> {
        self.client
            .unary_call(&METHOD_SECURITY_CENTER_SET_FINDING_STATE, req, opt)
    }

    pub fn set_finding_state(
        &self,
        req: &super::securitycenter_service::SetFindingStateRequest,
    ) -> ::grpcio::Result<super::finding::Finding> {
        self.set_finding_state_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_finding_state_async_opt(
        &self,
        req: &super::securitycenter_service::SetFindingStateRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::finding::Finding>> {
        self.client
            .unary_call_async(&METHOD_SECURITY_CENTER_SET_FINDING_STATE, req, opt)
    }

    pub fn set_finding_state_async(
        &self,
        req: &super::securitycenter_service::SetFindingStateRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::finding::Finding>> {
        self.set_finding_state_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_mute_opt(
        &self,
        req: &super::securitycenter_service::SetMuteRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::finding::Finding> {
        self.client
            .unary_call(&METHOD_SECURITY_CENTER_SET_MUTE, req, opt)
    }

    pub fn set_mute(
        &self,
        req: &super::securitycenter_service::SetMuteRequest,
    ) -> ::grpcio::Result<super::finding::Finding> {
        self.set_mute_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_mute_async_opt(
        &self,
        req: &super::securitycenter_service::SetMuteRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::finding::Finding>> {
        self.client
            .unary_call_async(&METHOD_SECURITY_CENTER_SET_MUTE, req, opt)
    }

    pub fn set_mute_async(
        &self,
        req: &super::securitycenter_service::SetMuteRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::finding::Finding>> {
        self.set_mute_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_iam_policy_opt(
        &self,
        req: &super::iam_policy::SetIamPolicyRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::policy::Policy> {
        self.client
            .unary_call(&METHOD_SECURITY_CENTER_SET_IAM_POLICY, req, opt)
    }

    pub fn set_iam_policy(
        &self,
        req: &super::iam_policy::SetIamPolicyRequest,
    ) -> ::grpcio::Result<super::policy::Policy> {
        self.set_iam_policy_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_iam_policy_async_opt(
        &self,
        req: &super::iam_policy::SetIamPolicyRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::policy::Policy>> {
        self.client
            .unary_call_async(&METHOD_SECURITY_CENTER_SET_IAM_POLICY, req, opt)
    }

    pub fn set_iam_policy_async(
        &self,
        req: &super::iam_policy::SetIamPolicyRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::policy::Policy>> {
        self.set_iam_policy_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn test_iam_permissions_opt(
        &self,
        req: &super::iam_policy::TestIamPermissionsRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::iam_policy::TestIamPermissionsResponse> {
        self.client
            .unary_call(&METHOD_SECURITY_CENTER_TEST_IAM_PERMISSIONS, req, opt)
    }

    pub fn test_iam_permissions(
        &self,
        req: &super::iam_policy::TestIamPermissionsRequest,
    ) -> ::grpcio::Result<super::iam_policy::TestIamPermissionsResponse> {
        self.test_iam_permissions_opt(req, ::grpcio::CallOption::default())
    }

    pub fn test_iam_permissions_async_opt(
        &self,
        req: &super::iam_policy::TestIamPermissionsRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<
        ::grpcio::ClientUnaryReceiver<super::iam_policy::TestIamPermissionsResponse>,
    > {
        self.client
            .unary_call_async(&METHOD_SECURITY_CENTER_TEST_IAM_PERMISSIONS, req, opt)
    }

    pub fn test_iam_permissions_async(
        &self,
        req: &super::iam_policy::TestIamPermissionsRequest,
    ) -> ::grpcio::Result<
        ::grpcio::ClientUnaryReceiver<super::iam_policy::TestIamPermissionsResponse>,
    > {
        self.test_iam_permissions_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn simulate_security_health_analytics_custom_module_opt(
        &self,
        req: &super::securitycenter_service::SimulateSecurityHealthAnalyticsCustomModuleRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<
        super::securitycenter_service::SimulateSecurityHealthAnalyticsCustomModuleResponse,
    > {
        self.client.unary_call(
            &METHOD_SECURITY_CENTER_SIMULATE_SECURITY_HEALTH_ANALYTICS_CUSTOM_MODULE,
            req,
            opt,
        )
    }

    pub fn simulate_security_health_analytics_custom_module(
        &self,
        req: &super::securitycenter_service::SimulateSecurityHealthAnalyticsCustomModuleRequest,
    ) -> ::grpcio::Result<
        super::securitycenter_service::SimulateSecurityHealthAnalyticsCustomModuleResponse,
    > {
        self.simulate_security_health_analytics_custom_module_opt(
            req,
            ::grpcio::CallOption::default(),
        )
    }

    pub fn simulate_security_health_analytics_custom_module_async_opt(
        &self,
        req: &super::securitycenter_service::SimulateSecurityHealthAnalyticsCustomModuleRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<
        ::grpcio::ClientUnaryReceiver<
            super::securitycenter_service::SimulateSecurityHealthAnalyticsCustomModuleResponse,
        >,
    > {
        self.client.unary_call_async(
            &METHOD_SECURITY_CENTER_SIMULATE_SECURITY_HEALTH_ANALYTICS_CUSTOM_MODULE,
            req,
            opt,
        )
    }

    pub fn simulate_security_health_analytics_custom_module_async(
        &self,
        req: &super::securitycenter_service::SimulateSecurityHealthAnalyticsCustomModuleRequest,
    ) -> ::grpcio::Result<
        ::grpcio::ClientUnaryReceiver<
            super::securitycenter_service::SimulateSecurityHealthAnalyticsCustomModuleResponse,
        >,
    > {
        self.simulate_security_health_analytics_custom_module_async_opt(
            req,
            ::grpcio::CallOption::default(),
        )
    }

    pub fn update_external_system_opt(
        &self,
        req: &super::securitycenter_service::UpdateExternalSystemRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::external_system::ExternalSystem> {
        self.client
            .unary_call(&METHOD_SECURITY_CENTER_UPDATE_EXTERNAL_SYSTEM, req, opt)
    }

    pub fn update_external_system(
        &self,
        req: &super::securitycenter_service::UpdateExternalSystemRequest,
    ) -> ::grpcio::Result<super::external_system::ExternalSystem> {
        self.update_external_system_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_external_system_async_opt(
        &self,
        req: &super::securitycenter_service::UpdateExternalSystemRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::external_system::ExternalSystem>>
    {
        self.client
            .unary_call_async(&METHOD_SECURITY_CENTER_UPDATE_EXTERNAL_SYSTEM, req, opt)
    }

    pub fn update_external_system_async(
        &self,
        req: &super::securitycenter_service::UpdateExternalSystemRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::external_system::ExternalSystem>>
    {
        self.update_external_system_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_finding_opt(
        &self,
        req: &super::securitycenter_service::UpdateFindingRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::finding::Finding> {
        self.client
            .unary_call(&METHOD_SECURITY_CENTER_UPDATE_FINDING, req, opt)
    }

    pub fn update_finding(
        &self,
        req: &super::securitycenter_service::UpdateFindingRequest,
    ) -> ::grpcio::Result<super::finding::Finding> {
        self.update_finding_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_finding_async_opt(
        &self,
        req: &super::securitycenter_service::UpdateFindingRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::finding::Finding>> {
        self.client
            .unary_call_async(&METHOD_SECURITY_CENTER_UPDATE_FINDING, req, opt)
    }

    pub fn update_finding_async(
        &self,
        req: &super::securitycenter_service::UpdateFindingRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::finding::Finding>> {
        self.update_finding_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_mute_config_opt(
        &self,
        req: &super::securitycenter_service::UpdateMuteConfigRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::mute_config::MuteConfig> {
        self.client
            .unary_call(&METHOD_SECURITY_CENTER_UPDATE_MUTE_CONFIG, req, opt)
    }

    pub fn update_mute_config(
        &self,
        req: &super::securitycenter_service::UpdateMuteConfigRequest,
    ) -> ::grpcio::Result<super::mute_config::MuteConfig> {
        self.update_mute_config_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_mute_config_async_opt(
        &self,
        req: &super::securitycenter_service::UpdateMuteConfigRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::mute_config::MuteConfig>> {
        self.client
            .unary_call_async(&METHOD_SECURITY_CENTER_UPDATE_MUTE_CONFIG, req, opt)
    }

    pub fn update_mute_config_async(
        &self,
        req: &super::securitycenter_service::UpdateMuteConfigRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::mute_config::MuteConfig>> {
        self.update_mute_config_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_notification_config_opt(
        &self,
        req: &super::securitycenter_service::UpdateNotificationConfigRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::notification_config::NotificationConfig> {
        self.client
            .unary_call(&METHOD_SECURITY_CENTER_UPDATE_NOTIFICATION_CONFIG, req, opt)
    }

    pub fn update_notification_config(
        &self,
        req: &super::securitycenter_service::UpdateNotificationConfigRequest,
    ) -> ::grpcio::Result<super::notification_config::NotificationConfig> {
        self.update_notification_config_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_notification_config_async_opt(
        &self,
        req: &super::securitycenter_service::UpdateNotificationConfigRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<
        ::grpcio::ClientUnaryReceiver<super::notification_config::NotificationConfig>,
    > {
        self.client
            .unary_call_async(&METHOD_SECURITY_CENTER_UPDATE_NOTIFICATION_CONFIG, req, opt)
    }

    pub fn update_notification_config_async(
        &self,
        req: &super::securitycenter_service::UpdateNotificationConfigRequest,
    ) -> ::grpcio::Result<
        ::grpcio::ClientUnaryReceiver<super::notification_config::NotificationConfig>,
    > {
        self.update_notification_config_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_organization_settings_opt(
        &self,
        req: &super::securitycenter_service::UpdateOrganizationSettingsRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::organization_settings::OrganizationSettings> {
        self.client.unary_call(
            &METHOD_SECURITY_CENTER_UPDATE_ORGANIZATION_SETTINGS,
            req,
            opt,
        )
    }

    pub fn update_organization_settings(
        &self,
        req: &super::securitycenter_service::UpdateOrganizationSettingsRequest,
    ) -> ::grpcio::Result<super::organization_settings::OrganizationSettings> {
        self.update_organization_settings_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_organization_settings_async_opt(
        &self,
        req: &super::securitycenter_service::UpdateOrganizationSettingsRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<
        ::grpcio::ClientUnaryReceiver<super::organization_settings::OrganizationSettings>,
    > {
        self.client.unary_call_async(
            &METHOD_SECURITY_CENTER_UPDATE_ORGANIZATION_SETTINGS,
            req,
            opt,
        )
    }

    pub fn update_organization_settings_async(
        &self,
        req: &super::securitycenter_service::UpdateOrganizationSettingsRequest,
    ) -> ::grpcio::Result<
        ::grpcio::ClientUnaryReceiver<super::organization_settings::OrganizationSettings>,
    > {
        self.update_organization_settings_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_security_health_analytics_custom_module_opt(
        &self,
        req: &super::securitycenter_service::UpdateSecurityHealthAnalyticsCustomModuleRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<
        super::security_health_analytics_custom_module::SecurityHealthAnalyticsCustomModule,
    > {
        self.client.unary_call(
            &METHOD_SECURITY_CENTER_UPDATE_SECURITY_HEALTH_ANALYTICS_CUSTOM_MODULE,
            req,
            opt,
        )
    }

    pub fn update_security_health_analytics_custom_module(
        &self,
        req: &super::securitycenter_service::UpdateSecurityHealthAnalyticsCustomModuleRequest,
    ) -> ::grpcio::Result<
        super::security_health_analytics_custom_module::SecurityHealthAnalyticsCustomModule,
    > {
        self.update_security_health_analytics_custom_module_opt(
            req,
            ::grpcio::CallOption::default(),
        )
    }

    pub fn update_security_health_analytics_custom_module_async_opt(
        &self,
        req: &super::securitycenter_service::UpdateSecurityHealthAnalyticsCustomModuleRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<
        ::grpcio::ClientUnaryReceiver<
            super::security_health_analytics_custom_module::SecurityHealthAnalyticsCustomModule,
        >,
    > {
        self.client.unary_call_async(
            &METHOD_SECURITY_CENTER_UPDATE_SECURITY_HEALTH_ANALYTICS_CUSTOM_MODULE,
            req,
            opt,
        )
    }

    pub fn update_security_health_analytics_custom_module_async(
        &self,
        req: &super::securitycenter_service::UpdateSecurityHealthAnalyticsCustomModuleRequest,
    ) -> ::grpcio::Result<
        ::grpcio::ClientUnaryReceiver<
            super::security_health_analytics_custom_module::SecurityHealthAnalyticsCustomModule,
        >,
    > {
        self.update_security_health_analytics_custom_module_async_opt(
            req,
            ::grpcio::CallOption::default(),
        )
    }

    pub fn update_source_opt(
        &self,
        req: &super::securitycenter_service::UpdateSourceRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::source::Source> {
        self.client
            .unary_call(&METHOD_SECURITY_CENTER_UPDATE_SOURCE, req, opt)
    }

    pub fn update_source(
        &self,
        req: &super::securitycenter_service::UpdateSourceRequest,
    ) -> ::grpcio::Result<super::source::Source> {
        self.update_source_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_source_async_opt(
        &self,
        req: &super::securitycenter_service::UpdateSourceRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::source::Source>> {
        self.client
            .unary_call_async(&METHOD_SECURITY_CENTER_UPDATE_SOURCE, req, opt)
    }

    pub fn update_source_async(
        &self,
        req: &super::securitycenter_service::UpdateSourceRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::source::Source>> {
        self.update_source_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_security_marks_opt(
        &self,
        req: &super::securitycenter_service::UpdateSecurityMarksRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::security_marks::SecurityMarks> {
        self.client
            .unary_call(&METHOD_SECURITY_CENTER_UPDATE_SECURITY_MARKS, req, opt)
    }

    pub fn update_security_marks(
        &self,
        req: &super::securitycenter_service::UpdateSecurityMarksRequest,
    ) -> ::grpcio::Result<super::security_marks::SecurityMarks> {
        self.update_security_marks_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_security_marks_async_opt(
        &self,
        req: &super::securitycenter_service::UpdateSecurityMarksRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::security_marks::SecurityMarks>> {
        self.client
            .unary_call_async(&METHOD_SECURITY_CENTER_UPDATE_SECURITY_MARKS, req, opt)
    }

    pub fn update_security_marks_async(
        &self,
        req: &super::securitycenter_service::UpdateSecurityMarksRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::security_marks::SecurityMarks>> {
        self.update_security_marks_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_big_query_export_opt(
        &self,
        req: &super::securitycenter_service::CreateBigQueryExportRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::bigquery_export::BigQueryExport> {
        self.client
            .unary_call(&METHOD_SECURITY_CENTER_CREATE_BIG_QUERY_EXPORT, req, opt)
    }

    pub fn create_big_query_export(
        &self,
        req: &super::securitycenter_service::CreateBigQueryExportRequest,
    ) -> ::grpcio::Result<super::bigquery_export::BigQueryExport> {
        self.create_big_query_export_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_big_query_export_async_opt(
        &self,
        req: &super::securitycenter_service::CreateBigQueryExportRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::bigquery_export::BigQueryExport>>
    {
        self.client
            .unary_call_async(&METHOD_SECURITY_CENTER_CREATE_BIG_QUERY_EXPORT, req, opt)
    }

    pub fn create_big_query_export_async(
        &self,
        req: &super::securitycenter_service::CreateBigQueryExportRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::bigquery_export::BigQueryExport>>
    {
        self.create_big_query_export_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_big_query_export_opt(
        &self,
        req: &super::securitycenter_service::DeleteBigQueryExportRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::empty::Empty> {
        self.client
            .unary_call(&METHOD_SECURITY_CENTER_DELETE_BIG_QUERY_EXPORT, req, opt)
    }

    pub fn delete_big_query_export(
        &self,
        req: &super::securitycenter_service::DeleteBigQueryExportRequest,
    ) -> ::grpcio::Result<super::empty::Empty> {
        self.delete_big_query_export_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_big_query_export_async_opt(
        &self,
        req: &super::securitycenter_service::DeleteBigQueryExportRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.client
            .unary_call_async(&METHOD_SECURITY_CENTER_DELETE_BIG_QUERY_EXPORT, req, opt)
    }

    pub fn delete_big_query_export_async(
        &self,
        req: &super::securitycenter_service::DeleteBigQueryExportRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.delete_big_query_export_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_big_query_export_opt(
        &self,
        req: &super::securitycenter_service::UpdateBigQueryExportRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::bigquery_export::BigQueryExport> {
        self.client
            .unary_call(&METHOD_SECURITY_CENTER_UPDATE_BIG_QUERY_EXPORT, req, opt)
    }

    pub fn update_big_query_export(
        &self,
        req: &super::securitycenter_service::UpdateBigQueryExportRequest,
    ) -> ::grpcio::Result<super::bigquery_export::BigQueryExport> {
        self.update_big_query_export_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_big_query_export_async_opt(
        &self,
        req: &super::securitycenter_service::UpdateBigQueryExportRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::bigquery_export::BigQueryExport>>
    {
        self.client
            .unary_call_async(&METHOD_SECURITY_CENTER_UPDATE_BIG_QUERY_EXPORT, req, opt)
    }

    pub fn update_big_query_export_async(
        &self,
        req: &super::securitycenter_service::UpdateBigQueryExportRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::bigquery_export::BigQueryExport>>
    {
        self.update_big_query_export_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_big_query_exports_opt(
        &self,
        req: &super::securitycenter_service::ListBigQueryExportsRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::securitycenter_service::ListBigQueryExportsResponse> {
        self.client
            .unary_call(&METHOD_SECURITY_CENTER_LIST_BIG_QUERY_EXPORTS, req, opt)
    }

    pub fn list_big_query_exports(
        &self,
        req: &super::securitycenter_service::ListBigQueryExportsRequest,
    ) -> ::grpcio::Result<super::securitycenter_service::ListBigQueryExportsResponse> {
        self.list_big_query_exports_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_big_query_exports_async_opt(
        &self,
        req: &super::securitycenter_service::ListBigQueryExportsRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<
        ::grpcio::ClientUnaryReceiver<super::securitycenter_service::ListBigQueryExportsResponse>,
    > {
        self.client
            .unary_call_async(&METHOD_SECURITY_CENTER_LIST_BIG_QUERY_EXPORTS, req, opt)
    }

    pub fn list_big_query_exports_async(
        &self,
        req: &super::securitycenter_service::ListBigQueryExportsRequest,
    ) -> ::grpcio::Result<
        ::grpcio::ClientUnaryReceiver<super::securitycenter_service::ListBigQueryExportsResponse>,
    > {
        self.list_big_query_exports_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_event_threat_detection_custom_module_opt(
        &self,
        req: &super::securitycenter_service::CreateEventThreatDetectionCustomModuleRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<
        super::event_threat_detection_custom_module::EventThreatDetectionCustomModule,
    > {
        self.client.unary_call(
            &METHOD_SECURITY_CENTER_CREATE_EVENT_THREAT_DETECTION_CUSTOM_MODULE,
            req,
            opt,
        )
    }

    pub fn create_event_threat_detection_custom_module(
        &self,
        req: &super::securitycenter_service::CreateEventThreatDetectionCustomModuleRequest,
    ) -> ::grpcio::Result<
        super::event_threat_detection_custom_module::EventThreatDetectionCustomModule,
    > {
        self.create_event_threat_detection_custom_module_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_event_threat_detection_custom_module_async_opt(
        &self,
        req: &super::securitycenter_service::CreateEventThreatDetectionCustomModuleRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<
        ::grpcio::ClientUnaryReceiver<
            super::event_threat_detection_custom_module::EventThreatDetectionCustomModule,
        >,
    > {
        self.client.unary_call_async(
            &METHOD_SECURITY_CENTER_CREATE_EVENT_THREAT_DETECTION_CUSTOM_MODULE,
            req,
            opt,
        )
    }

    pub fn create_event_threat_detection_custom_module_async(
        &self,
        req: &super::securitycenter_service::CreateEventThreatDetectionCustomModuleRequest,
    ) -> ::grpcio::Result<
        ::grpcio::ClientUnaryReceiver<
            super::event_threat_detection_custom_module::EventThreatDetectionCustomModule,
        >,
    > {
        self.create_event_threat_detection_custom_module_async_opt(
            req,
            ::grpcio::CallOption::default(),
        )
    }

    pub fn delete_event_threat_detection_custom_module_opt(
        &self,
        req: &super::securitycenter_service::DeleteEventThreatDetectionCustomModuleRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::empty::Empty> {
        self.client.unary_call(
            &METHOD_SECURITY_CENTER_DELETE_EVENT_THREAT_DETECTION_CUSTOM_MODULE,
            req,
            opt,
        )
    }

    pub fn delete_event_threat_detection_custom_module(
        &self,
        req: &super::securitycenter_service::DeleteEventThreatDetectionCustomModuleRequest,
    ) -> ::grpcio::Result<super::empty::Empty> {
        self.delete_event_threat_detection_custom_module_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_event_threat_detection_custom_module_async_opt(
        &self,
        req: &super::securitycenter_service::DeleteEventThreatDetectionCustomModuleRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.client.unary_call_async(
            &METHOD_SECURITY_CENTER_DELETE_EVENT_THREAT_DETECTION_CUSTOM_MODULE,
            req,
            opt,
        )
    }

    pub fn delete_event_threat_detection_custom_module_async(
        &self,
        req: &super::securitycenter_service::DeleteEventThreatDetectionCustomModuleRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.delete_event_threat_detection_custom_module_async_opt(
            req,
            ::grpcio::CallOption::default(),
        )
    }

    pub fn get_event_threat_detection_custom_module_opt(
        &self,
        req: &super::securitycenter_service::GetEventThreatDetectionCustomModuleRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<
        super::event_threat_detection_custom_module::EventThreatDetectionCustomModule,
    > {
        self.client.unary_call(
            &METHOD_SECURITY_CENTER_GET_EVENT_THREAT_DETECTION_CUSTOM_MODULE,
            req,
            opt,
        )
    }

    pub fn get_event_threat_detection_custom_module(
        &self,
        req: &super::securitycenter_service::GetEventThreatDetectionCustomModuleRequest,
    ) -> ::grpcio::Result<
        super::event_threat_detection_custom_module::EventThreatDetectionCustomModule,
    > {
        self.get_event_threat_detection_custom_module_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_event_threat_detection_custom_module_async_opt(
        &self,
        req: &super::securitycenter_service::GetEventThreatDetectionCustomModuleRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<
        ::grpcio::ClientUnaryReceiver<
            super::event_threat_detection_custom_module::EventThreatDetectionCustomModule,
        >,
    > {
        self.client.unary_call_async(
            &METHOD_SECURITY_CENTER_GET_EVENT_THREAT_DETECTION_CUSTOM_MODULE,
            req,
            opt,
        )
    }

    pub fn get_event_threat_detection_custom_module_async(
        &self,
        req: &super::securitycenter_service::GetEventThreatDetectionCustomModuleRequest,
    ) -> ::grpcio::Result<
        ::grpcio::ClientUnaryReceiver<
            super::event_threat_detection_custom_module::EventThreatDetectionCustomModule,
        >,
    > {
        self.get_event_threat_detection_custom_module_async_opt(
            req,
            ::grpcio::CallOption::default(),
        )
    }

    pub fn list_descendant_event_threat_detection_custom_modules_opt(
        &self,
        req: &super::securitycenter_service::ListDescendantEventThreatDetectionCustomModulesRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<
        super::securitycenter_service::ListDescendantEventThreatDetectionCustomModulesResponse,
    > {
        self.client.unary_call(
            &METHOD_SECURITY_CENTER_LIST_DESCENDANT_EVENT_THREAT_DETECTION_CUSTOM_MODULES,
            req,
            opt,
        )
    }

    pub fn list_descendant_event_threat_detection_custom_modules(
        &self,
        req: &super::securitycenter_service::ListDescendantEventThreatDetectionCustomModulesRequest,
    ) -> ::grpcio::Result<
        super::securitycenter_service::ListDescendantEventThreatDetectionCustomModulesResponse,
    > {
        self.list_descendant_event_threat_detection_custom_modules_opt(
            req,
            ::grpcio::CallOption::default(),
        )
    }

    pub fn list_descendant_event_threat_detection_custom_modules_async_opt(
        &self,
        req: &super::securitycenter_service::ListDescendantEventThreatDetectionCustomModulesRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<
        ::grpcio::ClientUnaryReceiver<
            super::securitycenter_service::ListDescendantEventThreatDetectionCustomModulesResponse,
        >,
    > {
        self.client.unary_call_async(
            &METHOD_SECURITY_CENTER_LIST_DESCENDANT_EVENT_THREAT_DETECTION_CUSTOM_MODULES,
            req,
            opt,
        )
    }

    pub fn list_descendant_event_threat_detection_custom_modules_async(
        &self,
        req: &super::securitycenter_service::ListDescendantEventThreatDetectionCustomModulesRequest,
    ) -> ::grpcio::Result<
        ::grpcio::ClientUnaryReceiver<
            super::securitycenter_service::ListDescendantEventThreatDetectionCustomModulesResponse,
        >,
    > {
        self.list_descendant_event_threat_detection_custom_modules_async_opt(
            req,
            ::grpcio::CallOption::default(),
        )
    }

    pub fn list_event_threat_detection_custom_modules_opt(
        &self,
        req: &super::securitycenter_service::ListEventThreatDetectionCustomModulesRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<
        super::securitycenter_service::ListEventThreatDetectionCustomModulesResponse,
    > {
        self.client.unary_call(
            &METHOD_SECURITY_CENTER_LIST_EVENT_THREAT_DETECTION_CUSTOM_MODULES,
            req,
            opt,
        )
    }

    pub fn list_event_threat_detection_custom_modules(
        &self,
        req: &super::securitycenter_service::ListEventThreatDetectionCustomModulesRequest,
    ) -> ::grpcio::Result<
        super::securitycenter_service::ListEventThreatDetectionCustomModulesResponse,
    > {
        self.list_event_threat_detection_custom_modules_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_event_threat_detection_custom_modules_async_opt(
        &self,
        req: &super::securitycenter_service::ListEventThreatDetectionCustomModulesRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<
        ::grpcio::ClientUnaryReceiver<
            super::securitycenter_service::ListEventThreatDetectionCustomModulesResponse,
        >,
    > {
        self.client.unary_call_async(
            &METHOD_SECURITY_CENTER_LIST_EVENT_THREAT_DETECTION_CUSTOM_MODULES,
            req,
            opt,
        )
    }

    pub fn list_event_threat_detection_custom_modules_async(
        &self,
        req: &super::securitycenter_service::ListEventThreatDetectionCustomModulesRequest,
    ) -> ::grpcio::Result<
        ::grpcio::ClientUnaryReceiver<
            super::securitycenter_service::ListEventThreatDetectionCustomModulesResponse,
        >,
    > {
        self.list_event_threat_detection_custom_modules_async_opt(
            req,
            ::grpcio::CallOption::default(),
        )
    }

    pub fn update_event_threat_detection_custom_module_opt(
        &self,
        req: &super::securitycenter_service::UpdateEventThreatDetectionCustomModuleRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<
        super::event_threat_detection_custom_module::EventThreatDetectionCustomModule,
    > {
        self.client.unary_call(
            &METHOD_SECURITY_CENTER_UPDATE_EVENT_THREAT_DETECTION_CUSTOM_MODULE,
            req,
            opt,
        )
    }

    pub fn update_event_threat_detection_custom_module(
        &self,
        req: &super::securitycenter_service::UpdateEventThreatDetectionCustomModuleRequest,
    ) -> ::grpcio::Result<
        super::event_threat_detection_custom_module::EventThreatDetectionCustomModule,
    > {
        self.update_event_threat_detection_custom_module_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_event_threat_detection_custom_module_async_opt(
        &self,
        req: &super::securitycenter_service::UpdateEventThreatDetectionCustomModuleRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<
        ::grpcio::ClientUnaryReceiver<
            super::event_threat_detection_custom_module::EventThreatDetectionCustomModule,
        >,
    > {
        self.client.unary_call_async(
            &METHOD_SECURITY_CENTER_UPDATE_EVENT_THREAT_DETECTION_CUSTOM_MODULE,
            req,
            opt,
        )
    }

    pub fn update_event_threat_detection_custom_module_async(
        &self,
        req: &super::securitycenter_service::UpdateEventThreatDetectionCustomModuleRequest,
    ) -> ::grpcio::Result<
        ::grpcio::ClientUnaryReceiver<
            super::event_threat_detection_custom_module::EventThreatDetectionCustomModule,
        >,
    > {
        self.update_event_threat_detection_custom_module_async_opt(
            req,
            ::grpcio::CallOption::default(),
        )
    }

    pub fn validate_event_threat_detection_custom_module_opt(
        &self,
        req: &super::securitycenter_service::ValidateEventThreatDetectionCustomModuleRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<
        super::securitycenter_service::ValidateEventThreatDetectionCustomModuleResponse,
    > {
        self.client.unary_call(
            &METHOD_SECURITY_CENTER_VALIDATE_EVENT_THREAT_DETECTION_CUSTOM_MODULE,
            req,
            opt,
        )
    }

    pub fn validate_event_threat_detection_custom_module(
        &self,
        req: &super::securitycenter_service::ValidateEventThreatDetectionCustomModuleRequest,
    ) -> ::grpcio::Result<
        super::securitycenter_service::ValidateEventThreatDetectionCustomModuleResponse,
    > {
        self.validate_event_threat_detection_custom_module_opt(req, ::grpcio::CallOption::default())
    }

    pub fn validate_event_threat_detection_custom_module_async_opt(
        &self,
        req: &super::securitycenter_service::ValidateEventThreatDetectionCustomModuleRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<
        ::grpcio::ClientUnaryReceiver<
            super::securitycenter_service::ValidateEventThreatDetectionCustomModuleResponse,
        >,
    > {
        self.client.unary_call_async(
            &METHOD_SECURITY_CENTER_VALIDATE_EVENT_THREAT_DETECTION_CUSTOM_MODULE,
            req,
            opt,
        )
    }

    pub fn validate_event_threat_detection_custom_module_async(
        &self,
        req: &super::securitycenter_service::ValidateEventThreatDetectionCustomModuleRequest,
    ) -> ::grpcio::Result<
        ::grpcio::ClientUnaryReceiver<
            super::securitycenter_service::ValidateEventThreatDetectionCustomModuleResponse,
        >,
    > {
        self.validate_event_threat_detection_custom_module_async_opt(
            req,
            ::grpcio::CallOption::default(),
        )
    }

    pub fn get_effective_event_threat_detection_custom_module_opt(&self, req: &super::securitycenter_service::GetEffectiveEventThreatDetectionCustomModuleRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::effective_event_threat_detection_custom_module::EffectiveEventThreatDetectionCustomModule>{
        self.client.unary_call(
            &METHOD_SECURITY_CENTER_GET_EFFECTIVE_EVENT_THREAT_DETECTION_CUSTOM_MODULE,
            req,
            opt,
        )
    }

    pub fn get_effective_event_threat_detection_custom_module(&self, req: &super::securitycenter_service::GetEffectiveEventThreatDetectionCustomModuleRequest) -> ::grpcio::Result<super::effective_event_threat_detection_custom_module::EffectiveEventThreatDetectionCustomModule>{
        self.get_effective_event_threat_detection_custom_module_opt(
            req,
            ::grpcio::CallOption::default(),
        )
    }

    pub fn get_effective_event_threat_detection_custom_module_async_opt(&self, req: &super::securitycenter_service::GetEffectiveEventThreatDetectionCustomModuleRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::effective_event_threat_detection_custom_module::EffectiveEventThreatDetectionCustomModule>>{
        self.client.unary_call_async(
            &METHOD_SECURITY_CENTER_GET_EFFECTIVE_EVENT_THREAT_DETECTION_CUSTOM_MODULE,
            req,
            opt,
        )
    }

    pub fn get_effective_event_threat_detection_custom_module_async(&self, req: &super::securitycenter_service::GetEffectiveEventThreatDetectionCustomModuleRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::effective_event_threat_detection_custom_module::EffectiveEventThreatDetectionCustomModule>>{
        self.get_effective_event_threat_detection_custom_module_async_opt(
            req,
            ::grpcio::CallOption::default(),
        )
    }

    pub fn list_effective_event_threat_detection_custom_modules_opt(
        &self,
        req: &super::securitycenter_service::ListEffectiveEventThreatDetectionCustomModulesRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<
        super::securitycenter_service::ListEffectiveEventThreatDetectionCustomModulesResponse,
    > {
        self.client.unary_call(
            &METHOD_SECURITY_CENTER_LIST_EFFECTIVE_EVENT_THREAT_DETECTION_CUSTOM_MODULES,
            req,
            opt,
        )
    }

    pub fn list_effective_event_threat_detection_custom_modules(
        &self,
        req: &super::securitycenter_service::ListEffectiveEventThreatDetectionCustomModulesRequest,
    ) -> ::grpcio::Result<
        super::securitycenter_service::ListEffectiveEventThreatDetectionCustomModulesResponse,
    > {
        self.list_effective_event_threat_detection_custom_modules_opt(
            req,
            ::grpcio::CallOption::default(),
        )
    }

    pub fn list_effective_event_threat_detection_custom_modules_async_opt(
        &self,
        req: &super::securitycenter_service::ListEffectiveEventThreatDetectionCustomModulesRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<
        ::grpcio::ClientUnaryReceiver<
            super::securitycenter_service::ListEffectiveEventThreatDetectionCustomModulesResponse,
        >,
    > {
        self.client.unary_call_async(
            &METHOD_SECURITY_CENTER_LIST_EFFECTIVE_EVENT_THREAT_DETECTION_CUSTOM_MODULES,
            req,
            opt,
        )
    }

    pub fn list_effective_event_threat_detection_custom_modules_async(
        &self,
        req: &super::securitycenter_service::ListEffectiveEventThreatDetectionCustomModulesRequest,
    ) -> ::grpcio::Result<
        ::grpcio::ClientUnaryReceiver<
            super::securitycenter_service::ListEffectiveEventThreatDetectionCustomModulesResponse,
        >,
    > {
        self.list_effective_event_threat_detection_custom_modules_async_opt(
            req,
            ::grpcio::CallOption::default(),
        )
    }

    pub fn batch_create_resource_value_configs_opt(
        &self,
        req: &super::securitycenter_service::BatchCreateResourceValueConfigsRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::securitycenter_service::BatchCreateResourceValueConfigsResponse>
    {
        self.client.unary_call(
            &METHOD_SECURITY_CENTER_BATCH_CREATE_RESOURCE_VALUE_CONFIGS,
            req,
            opt,
        )
    }

    pub fn batch_create_resource_value_configs(
        &self,
        req: &super::securitycenter_service::BatchCreateResourceValueConfigsRequest,
    ) -> ::grpcio::Result<super::securitycenter_service::BatchCreateResourceValueConfigsResponse>
    {
        self.batch_create_resource_value_configs_opt(req, ::grpcio::CallOption::default())
    }

    pub fn batch_create_resource_value_configs_async_opt(
        &self,
        req: &super::securitycenter_service::BatchCreateResourceValueConfigsRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<
        ::grpcio::ClientUnaryReceiver<
            super::securitycenter_service::BatchCreateResourceValueConfigsResponse,
        >,
    > {
        self.client.unary_call_async(
            &METHOD_SECURITY_CENTER_BATCH_CREATE_RESOURCE_VALUE_CONFIGS,
            req,
            opt,
        )
    }

    pub fn batch_create_resource_value_configs_async(
        &self,
        req: &super::securitycenter_service::BatchCreateResourceValueConfigsRequest,
    ) -> ::grpcio::Result<
        ::grpcio::ClientUnaryReceiver<
            super::securitycenter_service::BatchCreateResourceValueConfigsResponse,
        >,
    > {
        self.batch_create_resource_value_configs_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_resource_value_config_opt(
        &self,
        req: &super::securitycenter_service::DeleteResourceValueConfigRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::empty::Empty> {
        self.client.unary_call(
            &METHOD_SECURITY_CENTER_DELETE_RESOURCE_VALUE_CONFIG,
            req,
            opt,
        )
    }

    pub fn delete_resource_value_config(
        &self,
        req: &super::securitycenter_service::DeleteResourceValueConfigRequest,
    ) -> ::grpcio::Result<super::empty::Empty> {
        self.delete_resource_value_config_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_resource_value_config_async_opt(
        &self,
        req: &super::securitycenter_service::DeleteResourceValueConfigRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.client.unary_call_async(
            &METHOD_SECURITY_CENTER_DELETE_RESOURCE_VALUE_CONFIG,
            req,
            opt,
        )
    }

    pub fn delete_resource_value_config_async(
        &self,
        req: &super::securitycenter_service::DeleteResourceValueConfigRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.delete_resource_value_config_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_resource_value_config_opt(
        &self,
        req: &super::securitycenter_service::GetResourceValueConfigRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::resource_value_config::ResourceValueConfig> {
        self.client
            .unary_call(&METHOD_SECURITY_CENTER_GET_RESOURCE_VALUE_CONFIG, req, opt)
    }

    pub fn get_resource_value_config(
        &self,
        req: &super::securitycenter_service::GetResourceValueConfigRequest,
    ) -> ::grpcio::Result<super::resource_value_config::ResourceValueConfig> {
        self.get_resource_value_config_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_resource_value_config_async_opt(
        &self,
        req: &super::securitycenter_service::GetResourceValueConfigRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<
        ::grpcio::ClientUnaryReceiver<super::resource_value_config::ResourceValueConfig>,
    > {
        self.client
            .unary_call_async(&METHOD_SECURITY_CENTER_GET_RESOURCE_VALUE_CONFIG, req, opt)
    }

    pub fn get_resource_value_config_async(
        &self,
        req: &super::securitycenter_service::GetResourceValueConfigRequest,
    ) -> ::grpcio::Result<
        ::grpcio::ClientUnaryReceiver<super::resource_value_config::ResourceValueConfig>,
    > {
        self.get_resource_value_config_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_resource_value_configs_opt(
        &self,
        req: &super::securitycenter_service::ListResourceValueConfigsRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::securitycenter_service::ListResourceValueConfigsResponse> {
        self.client.unary_call(
            &METHOD_SECURITY_CENTER_LIST_RESOURCE_VALUE_CONFIGS,
            req,
            opt,
        )
    }

    pub fn list_resource_value_configs(
        &self,
        req: &super::securitycenter_service::ListResourceValueConfigsRequest,
    ) -> ::grpcio::Result<super::securitycenter_service::ListResourceValueConfigsResponse> {
        self.list_resource_value_configs_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_resource_value_configs_async_opt(
        &self,
        req: &super::securitycenter_service::ListResourceValueConfigsRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<
        ::grpcio::ClientUnaryReceiver<
            super::securitycenter_service::ListResourceValueConfigsResponse,
        >,
    > {
        self.client.unary_call_async(
            &METHOD_SECURITY_CENTER_LIST_RESOURCE_VALUE_CONFIGS,
            req,
            opt,
        )
    }

    pub fn list_resource_value_configs_async(
        &self,
        req: &super::securitycenter_service::ListResourceValueConfigsRequest,
    ) -> ::grpcio::Result<
        ::grpcio::ClientUnaryReceiver<
            super::securitycenter_service::ListResourceValueConfigsResponse,
        >,
    > {
        self.list_resource_value_configs_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_resource_value_config_opt(
        &self,
        req: &super::securitycenter_service::UpdateResourceValueConfigRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::resource_value_config::ResourceValueConfig> {
        self.client.unary_call(
            &METHOD_SECURITY_CENTER_UPDATE_RESOURCE_VALUE_CONFIG,
            req,
            opt,
        )
    }

    pub fn update_resource_value_config(
        &self,
        req: &super::securitycenter_service::UpdateResourceValueConfigRequest,
    ) -> ::grpcio::Result<super::resource_value_config::ResourceValueConfig> {
        self.update_resource_value_config_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_resource_value_config_async_opt(
        &self,
        req: &super::securitycenter_service::UpdateResourceValueConfigRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<
        ::grpcio::ClientUnaryReceiver<super::resource_value_config::ResourceValueConfig>,
    > {
        self.client.unary_call_async(
            &METHOD_SECURITY_CENTER_UPDATE_RESOURCE_VALUE_CONFIG,
            req,
            opt,
        )
    }

    pub fn update_resource_value_config_async(
        &self,
        req: &super::securitycenter_service::UpdateResourceValueConfigRequest,
    ) -> ::grpcio::Result<
        ::grpcio::ClientUnaryReceiver<super::resource_value_config::ResourceValueConfig>,
    > {
        self.update_resource_value_config_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_valued_resources_opt(
        &self,
        req: &super::securitycenter_service::ListValuedResourcesRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::securitycenter_service::ListValuedResourcesResponse> {
        self.client
            .unary_call(&METHOD_SECURITY_CENTER_LIST_VALUED_RESOURCES, req, opt)
    }

    pub fn list_valued_resources(
        &self,
        req: &super::securitycenter_service::ListValuedResourcesRequest,
    ) -> ::grpcio::Result<super::securitycenter_service::ListValuedResourcesResponse> {
        self.list_valued_resources_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_valued_resources_async_opt(
        &self,
        req: &super::securitycenter_service::ListValuedResourcesRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<
        ::grpcio::ClientUnaryReceiver<super::securitycenter_service::ListValuedResourcesResponse>,
    > {
        self.client
            .unary_call_async(&METHOD_SECURITY_CENTER_LIST_VALUED_RESOURCES, req, opt)
    }

    pub fn list_valued_resources_async(
        &self,
        req: &super::securitycenter_service::ListValuedResourcesRequest,
    ) -> ::grpcio::Result<
        ::grpcio::ClientUnaryReceiver<super::securitycenter_service::ListValuedResourcesResponse>,
    > {
        self.list_valued_resources_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_attack_paths_opt(
        &self,
        req: &super::securitycenter_service::ListAttackPathsRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::securitycenter_service::ListAttackPathsResponse> {
        self.client
            .unary_call(&METHOD_SECURITY_CENTER_LIST_ATTACK_PATHS, req, opt)
    }

    pub fn list_attack_paths(
        &self,
        req: &super::securitycenter_service::ListAttackPathsRequest,
    ) -> ::grpcio::Result<super::securitycenter_service::ListAttackPathsResponse> {
        self.list_attack_paths_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_attack_paths_async_opt(
        &self,
        req: &super::securitycenter_service::ListAttackPathsRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<
        ::grpcio::ClientUnaryReceiver<super::securitycenter_service::ListAttackPathsResponse>,
    > {
        self.client
            .unary_call_async(&METHOD_SECURITY_CENTER_LIST_ATTACK_PATHS, req, opt)
    }

    pub fn list_attack_paths_async(
        &self,
        req: &super::securitycenter_service::ListAttackPathsRequest,
    ) -> ::grpcio::Result<
        ::grpcio::ClientUnaryReceiver<super::securitycenter_service::ListAttackPathsResponse>,
    > {
        self.list_attack_paths_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F)
    where
        F: ::std::future::Future<Output = ()> + Send + 'static,
    {
        self.client.spawn(f)
    }
}

pub trait SecurityCenter {
    fn bulk_mute_findings(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: super::securitycenter_service::BulkMuteFindingsRequest,
        sink: ::grpcio::UnarySink<super::operations::Operation>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn create_security_health_analytics_custom_module(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: super::securitycenter_service::CreateSecurityHealthAnalyticsCustomModuleRequest,
        sink: ::grpcio::UnarySink<
            super::security_health_analytics_custom_module::SecurityHealthAnalyticsCustomModule,
        >,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn create_source(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: super::securitycenter_service::CreateSourceRequest,
        sink: ::grpcio::UnarySink<super::source::Source>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn create_finding(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: super::securitycenter_service::CreateFindingRequest,
        sink: ::grpcio::UnarySink<super::finding::Finding>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn create_mute_config(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: super::securitycenter_service::CreateMuteConfigRequest,
        sink: ::grpcio::UnarySink<super::mute_config::MuteConfig>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn create_notification_config(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: super::securitycenter_service::CreateNotificationConfigRequest,
        sink: ::grpcio::UnarySink<super::notification_config::NotificationConfig>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn delete_mute_config(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: super::securitycenter_service::DeleteMuteConfigRequest,
        sink: ::grpcio::UnarySink<super::empty::Empty>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn delete_notification_config(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: super::securitycenter_service::DeleteNotificationConfigRequest,
        sink: ::grpcio::UnarySink<super::empty::Empty>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn delete_security_health_analytics_custom_module(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: super::securitycenter_service::DeleteSecurityHealthAnalyticsCustomModuleRequest,
        sink: ::grpcio::UnarySink<super::empty::Empty>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_simulation(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: super::securitycenter_service::GetSimulationRequest,
        sink: ::grpcio::UnarySink<super::simulation::Simulation>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_valued_resource(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: super::securitycenter_service::GetValuedResourceRequest,
        sink: ::grpcio::UnarySink<super::valued_resource::ValuedResource>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_big_query_export(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: super::securitycenter_service::GetBigQueryExportRequest,
        sink: ::grpcio::UnarySink<super::bigquery_export::BigQueryExport>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_iam_policy(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: super::iam_policy::GetIamPolicyRequest,
        sink: ::grpcio::UnarySink<super::policy::Policy>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_mute_config(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: super::securitycenter_service::GetMuteConfigRequest,
        sink: ::grpcio::UnarySink<super::mute_config::MuteConfig>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_notification_config(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: super::securitycenter_service::GetNotificationConfigRequest,
        sink: ::grpcio::UnarySink<super::notification_config::NotificationConfig>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_organization_settings(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: super::securitycenter_service::GetOrganizationSettingsRequest,
        sink: ::grpcio::UnarySink<super::organization_settings::OrganizationSettings>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_effective_security_health_analytics_custom_module(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: super::securitycenter_service::GetEffectiveSecurityHealthAnalyticsCustomModuleRequest,
        sink: ::grpcio::UnarySink<super::effective_security_health_analytics_custom_module::EffectiveSecurityHealthAnalyticsCustomModule>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_security_health_analytics_custom_module(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: super::securitycenter_service::GetSecurityHealthAnalyticsCustomModuleRequest,
        sink: ::grpcio::UnarySink<
            super::security_health_analytics_custom_module::SecurityHealthAnalyticsCustomModule,
        >,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_source(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: super::securitycenter_service::GetSourceRequest,
        sink: ::grpcio::UnarySink<super::source::Source>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn group_assets(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: super::securitycenter_service::GroupAssetsRequest,
        sink: ::grpcio::UnarySink<super::securitycenter_service::GroupAssetsResponse>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn group_findings(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: super::securitycenter_service::GroupFindingsRequest,
        sink: ::grpcio::UnarySink<super::securitycenter_service::GroupFindingsResponse>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_assets(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: super::securitycenter_service::ListAssetsRequest,
        sink: ::grpcio::UnarySink<super::securitycenter_service::ListAssetsResponse>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_descendant_security_health_analytics_custom_modules(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: super::securitycenter_service::ListDescendantSecurityHealthAnalyticsCustomModulesRequest,
        sink: ::grpcio::UnarySink<super::securitycenter_service::ListDescendantSecurityHealthAnalyticsCustomModulesResponse>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_findings(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: super::securitycenter_service::ListFindingsRequest,
        sink: ::grpcio::UnarySink<super::securitycenter_service::ListFindingsResponse>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_mute_configs(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: super::securitycenter_service::ListMuteConfigsRequest,
        sink: ::grpcio::UnarySink<super::securitycenter_service::ListMuteConfigsResponse>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_notification_configs(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: super::securitycenter_service::ListNotificationConfigsRequest,
        sink: ::grpcio::UnarySink<super::securitycenter_service::ListNotificationConfigsResponse>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_effective_security_health_analytics_custom_modules(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: super::securitycenter_service::ListEffectiveSecurityHealthAnalyticsCustomModulesRequest,
        sink: ::grpcio::UnarySink<super::securitycenter_service::ListEffectiveSecurityHealthAnalyticsCustomModulesResponse>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_security_health_analytics_custom_modules(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: super::securitycenter_service::ListSecurityHealthAnalyticsCustomModulesRequest,
        sink: ::grpcio::UnarySink<
            super::securitycenter_service::ListSecurityHealthAnalyticsCustomModulesResponse,
        >,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_sources(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: super::securitycenter_service::ListSourcesRequest,
        sink: ::grpcio::UnarySink<super::securitycenter_service::ListSourcesResponse>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn run_asset_discovery(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: super::securitycenter_service::RunAssetDiscoveryRequest,
        sink: ::grpcio::UnarySink<super::operations::Operation>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn set_finding_state(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: super::securitycenter_service::SetFindingStateRequest,
        sink: ::grpcio::UnarySink<super::finding::Finding>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn set_mute(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: super::securitycenter_service::SetMuteRequest,
        sink: ::grpcio::UnarySink<super::finding::Finding>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn set_iam_policy(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: super::iam_policy::SetIamPolicyRequest,
        sink: ::grpcio::UnarySink<super::policy::Policy>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn test_iam_permissions(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: super::iam_policy::TestIamPermissionsRequest,
        sink: ::grpcio::UnarySink<super::iam_policy::TestIamPermissionsResponse>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn simulate_security_health_analytics_custom_module(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: super::securitycenter_service::SimulateSecurityHealthAnalyticsCustomModuleRequest,
        sink: ::grpcio::UnarySink<
            super::securitycenter_service::SimulateSecurityHealthAnalyticsCustomModuleResponse,
        >,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn update_external_system(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: super::securitycenter_service::UpdateExternalSystemRequest,
        sink: ::grpcio::UnarySink<super::external_system::ExternalSystem>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn update_finding(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: super::securitycenter_service::UpdateFindingRequest,
        sink: ::grpcio::UnarySink<super::finding::Finding>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn update_mute_config(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: super::securitycenter_service::UpdateMuteConfigRequest,
        sink: ::grpcio::UnarySink<super::mute_config::MuteConfig>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn update_notification_config(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: super::securitycenter_service::UpdateNotificationConfigRequest,
        sink: ::grpcio::UnarySink<super::notification_config::NotificationConfig>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn update_organization_settings(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: super::securitycenter_service::UpdateOrganizationSettingsRequest,
        sink: ::grpcio::UnarySink<super::organization_settings::OrganizationSettings>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn update_security_health_analytics_custom_module(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: super::securitycenter_service::UpdateSecurityHealthAnalyticsCustomModuleRequest,
        sink: ::grpcio::UnarySink<
            super::security_health_analytics_custom_module::SecurityHealthAnalyticsCustomModule,
        >,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn update_source(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: super::securitycenter_service::UpdateSourceRequest,
        sink: ::grpcio::UnarySink<super::source::Source>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn update_security_marks(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: super::securitycenter_service::UpdateSecurityMarksRequest,
        sink: ::grpcio::UnarySink<super::security_marks::SecurityMarks>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn create_big_query_export(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: super::securitycenter_service::CreateBigQueryExportRequest,
        sink: ::grpcio::UnarySink<super::bigquery_export::BigQueryExport>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn delete_big_query_export(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: super::securitycenter_service::DeleteBigQueryExportRequest,
        sink: ::grpcio::UnarySink<super::empty::Empty>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn update_big_query_export(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: super::securitycenter_service::UpdateBigQueryExportRequest,
        sink: ::grpcio::UnarySink<super::bigquery_export::BigQueryExport>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_big_query_exports(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: super::securitycenter_service::ListBigQueryExportsRequest,
        sink: ::grpcio::UnarySink<super::securitycenter_service::ListBigQueryExportsResponse>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn create_event_threat_detection_custom_module(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: super::securitycenter_service::CreateEventThreatDetectionCustomModuleRequest,
        sink: ::grpcio::UnarySink<
            super::event_threat_detection_custom_module::EventThreatDetectionCustomModule,
        >,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn delete_event_threat_detection_custom_module(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: super::securitycenter_service::DeleteEventThreatDetectionCustomModuleRequest,
        sink: ::grpcio::UnarySink<super::empty::Empty>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_event_threat_detection_custom_module(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: super::securitycenter_service::GetEventThreatDetectionCustomModuleRequest,
        sink: ::grpcio::UnarySink<
            super::event_threat_detection_custom_module::EventThreatDetectionCustomModule,
        >,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_descendant_event_threat_detection_custom_modules(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: super::securitycenter_service::ListDescendantEventThreatDetectionCustomModulesRequest,
        sink: ::grpcio::UnarySink<
            super::securitycenter_service::ListDescendantEventThreatDetectionCustomModulesResponse,
        >,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_event_threat_detection_custom_modules(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: super::securitycenter_service::ListEventThreatDetectionCustomModulesRequest,
        sink: ::grpcio::UnarySink<
            super::securitycenter_service::ListEventThreatDetectionCustomModulesResponse,
        >,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn update_event_threat_detection_custom_module(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: super::securitycenter_service::UpdateEventThreatDetectionCustomModuleRequest,
        sink: ::grpcio::UnarySink<
            super::event_threat_detection_custom_module::EventThreatDetectionCustomModule,
        >,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn validate_event_threat_detection_custom_module(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: super::securitycenter_service::ValidateEventThreatDetectionCustomModuleRequest,
        sink: ::grpcio::UnarySink<
            super::securitycenter_service::ValidateEventThreatDetectionCustomModuleResponse,
        >,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_effective_event_threat_detection_custom_module(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: super::securitycenter_service::GetEffectiveEventThreatDetectionCustomModuleRequest,
        sink: ::grpcio::UnarySink<super::effective_event_threat_detection_custom_module::EffectiveEventThreatDetectionCustomModule>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_effective_event_threat_detection_custom_modules(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: super::securitycenter_service::ListEffectiveEventThreatDetectionCustomModulesRequest,
        sink: ::grpcio::UnarySink<
            super::securitycenter_service::ListEffectiveEventThreatDetectionCustomModulesResponse,
        >,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn batch_create_resource_value_configs(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: super::securitycenter_service::BatchCreateResourceValueConfigsRequest,
        sink: ::grpcio::UnarySink<
            super::securitycenter_service::BatchCreateResourceValueConfigsResponse,
        >,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn delete_resource_value_config(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: super::securitycenter_service::DeleteResourceValueConfigRequest,
        sink: ::grpcio::UnarySink<super::empty::Empty>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_resource_value_config(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: super::securitycenter_service::GetResourceValueConfigRequest,
        sink: ::grpcio::UnarySink<super::resource_value_config::ResourceValueConfig>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_resource_value_configs(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: super::securitycenter_service::ListResourceValueConfigsRequest,
        sink: ::grpcio::UnarySink<super::securitycenter_service::ListResourceValueConfigsResponse>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn update_resource_value_config(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: super::securitycenter_service::UpdateResourceValueConfigRequest,
        sink: ::grpcio::UnarySink<super::resource_value_config::ResourceValueConfig>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_valued_resources(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: super::securitycenter_service::ListValuedResourcesRequest,
        sink: ::grpcio::UnarySink<super::securitycenter_service::ListValuedResourcesResponse>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_attack_paths(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: super::securitycenter_service::ListAttackPathsRequest,
        sink: ::grpcio::UnarySink<super::securitycenter_service::ListAttackPathsResponse>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
}

pub fn create_security_center<S: SecurityCenter + Send + Clone + 'static>(
    s: S,
) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(
        &METHOD_SECURITY_CENTER_BULK_MUTE_FINDINGS,
        move |ctx, req, resp| instance.bulk_mute_findings(ctx, req, resp),
    );
    let mut instance = s.clone();
    builder = builder.add_unary_handler(
        &METHOD_SECURITY_CENTER_CREATE_SECURITY_HEALTH_ANALYTICS_CUSTOM_MODULE,
        move |ctx, req, resp| {
            instance.create_security_health_analytics_custom_module(ctx, req, resp)
        },
    );
    let mut instance = s.clone();
    builder = builder.add_unary_handler(
        &METHOD_SECURITY_CENTER_CREATE_SOURCE,
        move |ctx, req, resp| instance.create_source(ctx, req, resp),
    );
    let mut instance = s.clone();
    builder = builder.add_unary_handler(
        &METHOD_SECURITY_CENTER_CREATE_FINDING,
        move |ctx, req, resp| instance.create_finding(ctx, req, resp),
    );
    let mut instance = s.clone();
    builder = builder.add_unary_handler(
        &METHOD_SECURITY_CENTER_CREATE_MUTE_CONFIG,
        move |ctx, req, resp| instance.create_mute_config(ctx, req, resp),
    );
    let mut instance = s.clone();
    builder = builder.add_unary_handler(
        &METHOD_SECURITY_CENTER_CREATE_NOTIFICATION_CONFIG,
        move |ctx, req, resp| instance.create_notification_config(ctx, req, resp),
    );
    let mut instance = s.clone();
    builder = builder.add_unary_handler(
        &METHOD_SECURITY_CENTER_DELETE_MUTE_CONFIG,
        move |ctx, req, resp| instance.delete_mute_config(ctx, req, resp),
    );
    let mut instance = s.clone();
    builder = builder.add_unary_handler(
        &METHOD_SECURITY_CENTER_DELETE_NOTIFICATION_CONFIG,
        move |ctx, req, resp| instance.delete_notification_config(ctx, req, resp),
    );
    let mut instance = s.clone();
    builder = builder.add_unary_handler(
        &METHOD_SECURITY_CENTER_DELETE_SECURITY_HEALTH_ANALYTICS_CUSTOM_MODULE,
        move |ctx, req, resp| {
            instance.delete_security_health_analytics_custom_module(ctx, req, resp)
        },
    );
    let mut instance = s.clone();
    builder = builder.add_unary_handler(
        &METHOD_SECURITY_CENTER_GET_SIMULATION,
        move |ctx, req, resp| instance.get_simulation(ctx, req, resp),
    );
    let mut instance = s.clone();
    builder = builder.add_unary_handler(
        &METHOD_SECURITY_CENTER_GET_VALUED_RESOURCE,
        move |ctx, req, resp| instance.get_valued_resource(ctx, req, resp),
    );
    let mut instance = s.clone();
    builder = builder.add_unary_handler(
        &METHOD_SECURITY_CENTER_GET_BIG_QUERY_EXPORT,
        move |ctx, req, resp| instance.get_big_query_export(ctx, req, resp),
    );
    let mut instance = s.clone();
    builder = builder.add_unary_handler(
        &METHOD_SECURITY_CENTER_GET_IAM_POLICY,
        move |ctx, req, resp| instance.get_iam_policy(ctx, req, resp),
    );
    let mut instance = s.clone();
    builder = builder.add_unary_handler(
        &METHOD_SECURITY_CENTER_GET_MUTE_CONFIG,
        move |ctx, req, resp| instance.get_mute_config(ctx, req, resp),
    );
    let mut instance = s.clone();
    builder = builder.add_unary_handler(
        &METHOD_SECURITY_CENTER_GET_NOTIFICATION_CONFIG,
        move |ctx, req, resp| instance.get_notification_config(ctx, req, resp),
    );
    let mut instance = s.clone();
    builder = builder.add_unary_handler(
        &METHOD_SECURITY_CENTER_GET_ORGANIZATION_SETTINGS,
        move |ctx, req, resp| instance.get_organization_settings(ctx, req, resp),
    );
    let mut instance = s.clone();
    builder = builder.add_unary_handler(
        &METHOD_SECURITY_CENTER_GET_EFFECTIVE_SECURITY_HEALTH_ANALYTICS_CUSTOM_MODULE,
        move |ctx, req, resp| {
            instance.get_effective_security_health_analytics_custom_module(ctx, req, resp)
        },
    );
    let mut instance = s.clone();
    builder = builder.add_unary_handler(
        &METHOD_SECURITY_CENTER_GET_SECURITY_HEALTH_ANALYTICS_CUSTOM_MODULE,
        move |ctx, req, resp| instance.get_security_health_analytics_custom_module(ctx, req, resp),
    );
    let mut instance = s.clone();
    builder = builder
        .add_unary_handler(&METHOD_SECURITY_CENTER_GET_SOURCE, move |ctx, req, resp| {
            instance.get_source(ctx, req, resp)
        });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(
        &METHOD_SECURITY_CENTER_GROUP_ASSETS,
        move |ctx, req, resp| instance.group_assets(ctx, req, resp),
    );
    let mut instance = s.clone();
    builder = builder.add_unary_handler(
        &METHOD_SECURITY_CENTER_GROUP_FINDINGS,
        move |ctx, req, resp| instance.group_findings(ctx, req, resp),
    );
    let mut instance = s.clone();
    builder = builder.add_unary_handler(
        &METHOD_SECURITY_CENTER_LIST_ASSETS,
        move |ctx, req, resp| instance.list_assets(ctx, req, resp),
    );
    let mut instance = s.clone();
    builder = builder.add_unary_handler(
        &METHOD_SECURITY_CENTER_LIST_DESCENDANT_SECURITY_HEALTH_ANALYTICS_CUSTOM_MODULES,
        move |ctx, req, resp| {
            instance.list_descendant_security_health_analytics_custom_modules(ctx, req, resp)
        },
    );
    let mut instance = s.clone();
    builder = builder.add_unary_handler(
        &METHOD_SECURITY_CENTER_LIST_FINDINGS,
        move |ctx, req, resp| instance.list_findings(ctx, req, resp),
    );
    let mut instance = s.clone();
    builder = builder.add_unary_handler(
        &METHOD_SECURITY_CENTER_LIST_MUTE_CONFIGS,
        move |ctx, req, resp| instance.list_mute_configs(ctx, req, resp),
    );
    let mut instance = s.clone();
    builder = builder.add_unary_handler(
        &METHOD_SECURITY_CENTER_LIST_NOTIFICATION_CONFIGS,
        move |ctx, req, resp| instance.list_notification_configs(ctx, req, resp),
    );
    let mut instance = s.clone();
    builder = builder.add_unary_handler(
        &METHOD_SECURITY_CENTER_LIST_EFFECTIVE_SECURITY_HEALTH_ANALYTICS_CUSTOM_MODULES,
        move |ctx, req, resp| {
            instance.list_effective_security_health_analytics_custom_modules(ctx, req, resp)
        },
    );
    let mut instance = s.clone();
    builder = builder.add_unary_handler(
        &METHOD_SECURITY_CENTER_LIST_SECURITY_HEALTH_ANALYTICS_CUSTOM_MODULES,
        move |ctx, req, resp| {
            instance.list_security_health_analytics_custom_modules(ctx, req, resp)
        },
    );
    let mut instance = s.clone();
    builder = builder.add_unary_handler(
        &METHOD_SECURITY_CENTER_LIST_SOURCES,
        move |ctx, req, resp| instance.list_sources(ctx, req, resp),
    );
    let mut instance = s.clone();
    builder = builder.add_unary_handler(
        &METHOD_SECURITY_CENTER_RUN_ASSET_DISCOVERY,
        move |ctx, req, resp| instance.run_asset_discovery(ctx, req, resp),
    );
    let mut instance = s.clone();
    builder = builder.add_unary_handler(
        &METHOD_SECURITY_CENTER_SET_FINDING_STATE,
        move |ctx, req, resp| instance.set_finding_state(ctx, req, resp),
    );
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SECURITY_CENTER_SET_MUTE, move |ctx, req, resp| {
        instance.set_mute(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(
        &METHOD_SECURITY_CENTER_SET_IAM_POLICY,
        move |ctx, req, resp| instance.set_iam_policy(ctx, req, resp),
    );
    let mut instance = s.clone();
    builder = builder.add_unary_handler(
        &METHOD_SECURITY_CENTER_TEST_IAM_PERMISSIONS,
        move |ctx, req, resp| instance.test_iam_permissions(ctx, req, resp),
    );
    let mut instance = s.clone();
    builder = builder.add_unary_handler(
        &METHOD_SECURITY_CENTER_SIMULATE_SECURITY_HEALTH_ANALYTICS_CUSTOM_MODULE,
        move |ctx, req, resp| {
            instance.simulate_security_health_analytics_custom_module(ctx, req, resp)
        },
    );
    let mut instance = s.clone();
    builder = builder.add_unary_handler(
        &METHOD_SECURITY_CENTER_UPDATE_EXTERNAL_SYSTEM,
        move |ctx, req, resp| instance.update_external_system(ctx, req, resp),
    );
    let mut instance = s.clone();
    builder = builder.add_unary_handler(
        &METHOD_SECURITY_CENTER_UPDATE_FINDING,
        move |ctx, req, resp| instance.update_finding(ctx, req, resp),
    );
    let mut instance = s.clone();
    builder = builder.add_unary_handler(
        &METHOD_SECURITY_CENTER_UPDATE_MUTE_CONFIG,
        move |ctx, req, resp| instance.update_mute_config(ctx, req, resp),
    );
    let mut instance = s.clone();
    builder = builder.add_unary_handler(
        &METHOD_SECURITY_CENTER_UPDATE_NOTIFICATION_CONFIG,
        move |ctx, req, resp| instance.update_notification_config(ctx, req, resp),
    );
    let mut instance = s.clone();
    builder = builder.add_unary_handler(
        &METHOD_SECURITY_CENTER_UPDATE_ORGANIZATION_SETTINGS,
        move |ctx, req, resp| instance.update_organization_settings(ctx, req, resp),
    );
    let mut instance = s.clone();
    builder = builder.add_unary_handler(
        &METHOD_SECURITY_CENTER_UPDATE_SECURITY_HEALTH_ANALYTICS_CUSTOM_MODULE,
        move |ctx, req, resp| {
            instance.update_security_health_analytics_custom_module(ctx, req, resp)
        },
    );
    let mut instance = s.clone();
    builder = builder.add_unary_handler(
        &METHOD_SECURITY_CENTER_UPDATE_SOURCE,
        move |ctx, req, resp| instance.update_source(ctx, req, resp),
    );
    let mut instance = s.clone();
    builder = builder.add_unary_handler(
        &METHOD_SECURITY_CENTER_UPDATE_SECURITY_MARKS,
        move |ctx, req, resp| instance.update_security_marks(ctx, req, resp),
    );
    let mut instance = s.clone();
    builder = builder.add_unary_handler(
        &METHOD_SECURITY_CENTER_CREATE_BIG_QUERY_EXPORT,
        move |ctx, req, resp| instance.create_big_query_export(ctx, req, resp),
    );
    let mut instance = s.clone();
    builder = builder.add_unary_handler(
        &METHOD_SECURITY_CENTER_DELETE_BIG_QUERY_EXPORT,
        move |ctx, req, resp| instance.delete_big_query_export(ctx, req, resp),
    );
    let mut instance = s.clone();
    builder = builder.add_unary_handler(
        &METHOD_SECURITY_CENTER_UPDATE_BIG_QUERY_EXPORT,
        move |ctx, req, resp| instance.update_big_query_export(ctx, req, resp),
    );
    let mut instance = s.clone();
    builder = builder.add_unary_handler(
        &METHOD_SECURITY_CENTER_LIST_BIG_QUERY_EXPORTS,
        move |ctx, req, resp| instance.list_big_query_exports(ctx, req, resp),
    );
    let mut instance = s.clone();
    builder = builder.add_unary_handler(
        &METHOD_SECURITY_CENTER_CREATE_EVENT_THREAT_DETECTION_CUSTOM_MODULE,
        move |ctx, req, resp| instance.create_event_threat_detection_custom_module(ctx, req, resp),
    );
    let mut instance = s.clone();
    builder = builder.add_unary_handler(
        &METHOD_SECURITY_CENTER_DELETE_EVENT_THREAT_DETECTION_CUSTOM_MODULE,
        move |ctx, req, resp| instance.delete_event_threat_detection_custom_module(ctx, req, resp),
    );
    let mut instance = s.clone();
    builder = builder.add_unary_handler(
        &METHOD_SECURITY_CENTER_GET_EVENT_THREAT_DETECTION_CUSTOM_MODULE,
        move |ctx, req, resp| instance.get_event_threat_detection_custom_module(ctx, req, resp),
    );
    let mut instance = s.clone();
    builder = builder.add_unary_handler(
        &METHOD_SECURITY_CENTER_LIST_DESCENDANT_EVENT_THREAT_DETECTION_CUSTOM_MODULES,
        move |ctx, req, resp| {
            instance.list_descendant_event_threat_detection_custom_modules(ctx, req, resp)
        },
    );
    let mut instance = s.clone();
    builder = builder.add_unary_handler(
        &METHOD_SECURITY_CENTER_LIST_EVENT_THREAT_DETECTION_CUSTOM_MODULES,
        move |ctx, req, resp| instance.list_event_threat_detection_custom_modules(ctx, req, resp),
    );
    let mut instance = s.clone();
    builder = builder.add_unary_handler(
        &METHOD_SECURITY_CENTER_UPDATE_EVENT_THREAT_DETECTION_CUSTOM_MODULE,
        move |ctx, req, resp| instance.update_event_threat_detection_custom_module(ctx, req, resp),
    );
    let mut instance = s.clone();
    builder = builder.add_unary_handler(
        &METHOD_SECURITY_CENTER_VALIDATE_EVENT_THREAT_DETECTION_CUSTOM_MODULE,
        move |ctx, req, resp| {
            instance.validate_event_threat_detection_custom_module(ctx, req, resp)
        },
    );
    let mut instance = s.clone();
    builder = builder.add_unary_handler(
        &METHOD_SECURITY_CENTER_GET_EFFECTIVE_EVENT_THREAT_DETECTION_CUSTOM_MODULE,
        move |ctx, req, resp| {
            instance.get_effective_event_threat_detection_custom_module(ctx, req, resp)
        },
    );
    let mut instance = s.clone();
    builder = builder.add_unary_handler(
        &METHOD_SECURITY_CENTER_LIST_EFFECTIVE_EVENT_THREAT_DETECTION_CUSTOM_MODULES,
        move |ctx, req, resp| {
            instance.list_effective_event_threat_detection_custom_modules(ctx, req, resp)
        },
    );
    let mut instance = s.clone();
    builder = builder.add_unary_handler(
        &METHOD_SECURITY_CENTER_BATCH_CREATE_RESOURCE_VALUE_CONFIGS,
        move |ctx, req, resp| instance.batch_create_resource_value_configs(ctx, req, resp),
    );
    let mut instance = s.clone();
    builder = builder.add_unary_handler(
        &METHOD_SECURITY_CENTER_DELETE_RESOURCE_VALUE_CONFIG,
        move |ctx, req, resp| instance.delete_resource_value_config(ctx, req, resp),
    );
    let mut instance = s.clone();
    builder = builder.add_unary_handler(
        &METHOD_SECURITY_CENTER_GET_RESOURCE_VALUE_CONFIG,
        move |ctx, req, resp| instance.get_resource_value_config(ctx, req, resp),
    );
    let mut instance = s.clone();
    builder = builder.add_unary_handler(
        &METHOD_SECURITY_CENTER_LIST_RESOURCE_VALUE_CONFIGS,
        move |ctx, req, resp| instance.list_resource_value_configs(ctx, req, resp),
    );
    let mut instance = s.clone();
    builder = builder.add_unary_handler(
        &METHOD_SECURITY_CENTER_UPDATE_RESOURCE_VALUE_CONFIG,
        move |ctx, req, resp| instance.update_resource_value_config(ctx, req, resp),
    );
    let mut instance = s.clone();
    builder = builder.add_unary_handler(
        &METHOD_SECURITY_CENTER_LIST_VALUED_RESOURCES,
        move |ctx, req, resp| instance.list_valued_resources(ctx, req, resp),
    );
    let mut instance = s;
    builder = builder.add_unary_handler(
        &METHOD_SECURITY_CENTER_LIST_ATTACK_PATHS,
        move |ctx, req, resp| instance.list_attack_paths(ctx, req, resp),
    );
    builder.build()
}
