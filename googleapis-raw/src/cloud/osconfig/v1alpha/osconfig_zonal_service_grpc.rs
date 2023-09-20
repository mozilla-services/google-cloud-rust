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

const METHOD_OS_CONFIG_ZONAL_SERVICE_CREATE_OS_POLICY_ASSIGNMENT: ::grpcio::Method<super::os_policy_assignments::CreateOSPolicyAssignmentRequest, super::operations::Operation> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.osconfig.v1alpha.OsConfigZonalService/CreateOSPolicyAssignment",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_OS_CONFIG_ZONAL_SERVICE_UPDATE_OS_POLICY_ASSIGNMENT: ::grpcio::Method<super::os_policy_assignments::UpdateOSPolicyAssignmentRequest, super::operations::Operation> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.osconfig.v1alpha.OsConfigZonalService/UpdateOSPolicyAssignment",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_OS_CONFIG_ZONAL_SERVICE_GET_OS_POLICY_ASSIGNMENT: ::grpcio::Method<super::os_policy_assignments::GetOSPolicyAssignmentRequest, super::os_policy_assignments::OSPolicyAssignment> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.osconfig.v1alpha.OsConfigZonalService/GetOSPolicyAssignment",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_OS_CONFIG_ZONAL_SERVICE_LIST_OS_POLICY_ASSIGNMENTS: ::grpcio::Method<super::os_policy_assignments::ListOSPolicyAssignmentsRequest, super::os_policy_assignments::ListOSPolicyAssignmentsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.osconfig.v1alpha.OsConfigZonalService/ListOSPolicyAssignments",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_OS_CONFIG_ZONAL_SERVICE_LIST_OS_POLICY_ASSIGNMENT_REVISIONS: ::grpcio::Method<super::os_policy_assignments::ListOSPolicyAssignmentRevisionsRequest, super::os_policy_assignments::ListOSPolicyAssignmentRevisionsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.osconfig.v1alpha.OsConfigZonalService/ListOSPolicyAssignmentRevisions",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_OS_CONFIG_ZONAL_SERVICE_DELETE_OS_POLICY_ASSIGNMENT: ::grpcio::Method<super::os_policy_assignments::DeleteOSPolicyAssignmentRequest, super::operations::Operation> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.osconfig.v1alpha.OsConfigZonalService/DeleteOSPolicyAssignment",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_OS_CONFIG_ZONAL_SERVICE_GET_INSTANCE_OS_POLICIES_COMPLIANCE: ::grpcio::Method<super::instance_os_policies_compliance::GetInstanceOSPoliciesComplianceRequest, super::instance_os_policies_compliance::InstanceOSPoliciesCompliance> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.osconfig.v1alpha.OsConfigZonalService/GetInstanceOSPoliciesCompliance",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_OS_CONFIG_ZONAL_SERVICE_LIST_INSTANCE_OS_POLICIES_COMPLIANCES: ::grpcio::Method<super::instance_os_policies_compliance::ListInstanceOSPoliciesCompliancesRequest, super::instance_os_policies_compliance::ListInstanceOSPoliciesCompliancesResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.osconfig.v1alpha.OsConfigZonalService/ListInstanceOSPoliciesCompliances",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_OS_CONFIG_ZONAL_SERVICE_GET_OS_POLICY_ASSIGNMENT_REPORT: ::grpcio::Method<super::os_policy_assignment_reports::GetOSPolicyAssignmentReportRequest, super::os_policy_assignment_reports::OSPolicyAssignmentReport> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.osconfig.v1alpha.OsConfigZonalService/GetOSPolicyAssignmentReport",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_OS_CONFIG_ZONAL_SERVICE_LIST_OS_POLICY_ASSIGNMENT_REPORTS: ::grpcio::Method<super::os_policy_assignment_reports::ListOSPolicyAssignmentReportsRequest, super::os_policy_assignment_reports::ListOSPolicyAssignmentReportsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.osconfig.v1alpha.OsConfigZonalService/ListOSPolicyAssignmentReports",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_OS_CONFIG_ZONAL_SERVICE_GET_INVENTORY: ::grpcio::Method<super::inventory::GetInventoryRequest, super::inventory::Inventory> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.osconfig.v1alpha.OsConfigZonalService/GetInventory",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_OS_CONFIG_ZONAL_SERVICE_LIST_INVENTORIES: ::grpcio::Method<super::inventory::ListInventoriesRequest, super::inventory::ListInventoriesResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.osconfig.v1alpha.OsConfigZonalService/ListInventories",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_OS_CONFIG_ZONAL_SERVICE_GET_VULNERABILITY_REPORT: ::grpcio::Method<super::vulnerability::GetVulnerabilityReportRequest, super::vulnerability::VulnerabilityReport> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.osconfig.v1alpha.OsConfigZonalService/GetVulnerabilityReport",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_OS_CONFIG_ZONAL_SERVICE_LIST_VULNERABILITY_REPORTS: ::grpcio::Method<super::vulnerability::ListVulnerabilityReportsRequest, super::vulnerability::ListVulnerabilityReportsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.osconfig.v1alpha.OsConfigZonalService/ListVulnerabilityReports",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct OsConfigZonalServiceClient {
    client: ::grpcio::Client,
}

impl OsConfigZonalServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        OsConfigZonalServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn create_os_policy_assignment_opt(&self, req: &super::os_policy_assignments::CreateOSPolicyAssignmentRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::operations::Operation> {
        self.client.unary_call(&METHOD_OS_CONFIG_ZONAL_SERVICE_CREATE_OS_POLICY_ASSIGNMENT, req, opt)
    }

    pub fn create_os_policy_assignment(&self, req: &super::os_policy_assignments::CreateOSPolicyAssignmentRequest) -> ::grpcio::Result<super::operations::Operation> {
        self.create_os_policy_assignment_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_os_policy_assignment_async_opt(&self, req: &super::os_policy_assignments::CreateOSPolicyAssignmentRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operations::Operation>> {
        self.client.unary_call_async(&METHOD_OS_CONFIG_ZONAL_SERVICE_CREATE_OS_POLICY_ASSIGNMENT, req, opt)
    }

    pub fn create_os_policy_assignment_async(&self, req: &super::os_policy_assignments::CreateOSPolicyAssignmentRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operations::Operation>> {
        self.create_os_policy_assignment_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_os_policy_assignment_opt(&self, req: &super::os_policy_assignments::UpdateOSPolicyAssignmentRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::operations::Operation> {
        self.client.unary_call(&METHOD_OS_CONFIG_ZONAL_SERVICE_UPDATE_OS_POLICY_ASSIGNMENT, req, opt)
    }

    pub fn update_os_policy_assignment(&self, req: &super::os_policy_assignments::UpdateOSPolicyAssignmentRequest) -> ::grpcio::Result<super::operations::Operation> {
        self.update_os_policy_assignment_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_os_policy_assignment_async_opt(&self, req: &super::os_policy_assignments::UpdateOSPolicyAssignmentRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operations::Operation>> {
        self.client.unary_call_async(&METHOD_OS_CONFIG_ZONAL_SERVICE_UPDATE_OS_POLICY_ASSIGNMENT, req, opt)
    }

    pub fn update_os_policy_assignment_async(&self, req: &super::os_policy_assignments::UpdateOSPolicyAssignmentRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operations::Operation>> {
        self.update_os_policy_assignment_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_os_policy_assignment_opt(&self, req: &super::os_policy_assignments::GetOSPolicyAssignmentRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::os_policy_assignments::OSPolicyAssignment> {
        self.client.unary_call(&METHOD_OS_CONFIG_ZONAL_SERVICE_GET_OS_POLICY_ASSIGNMENT, req, opt)
    }

    pub fn get_os_policy_assignment(&self, req: &super::os_policy_assignments::GetOSPolicyAssignmentRequest) -> ::grpcio::Result<super::os_policy_assignments::OSPolicyAssignment> {
        self.get_os_policy_assignment_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_os_policy_assignment_async_opt(&self, req: &super::os_policy_assignments::GetOSPolicyAssignmentRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::os_policy_assignments::OSPolicyAssignment>> {
        self.client.unary_call_async(&METHOD_OS_CONFIG_ZONAL_SERVICE_GET_OS_POLICY_ASSIGNMENT, req, opt)
    }

    pub fn get_os_policy_assignment_async(&self, req: &super::os_policy_assignments::GetOSPolicyAssignmentRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::os_policy_assignments::OSPolicyAssignment>> {
        self.get_os_policy_assignment_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_os_policy_assignments_opt(&self, req: &super::os_policy_assignments::ListOSPolicyAssignmentsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::os_policy_assignments::ListOSPolicyAssignmentsResponse> {
        self.client.unary_call(&METHOD_OS_CONFIG_ZONAL_SERVICE_LIST_OS_POLICY_ASSIGNMENTS, req, opt)
    }

    pub fn list_os_policy_assignments(&self, req: &super::os_policy_assignments::ListOSPolicyAssignmentsRequest) -> ::grpcio::Result<super::os_policy_assignments::ListOSPolicyAssignmentsResponse> {
        self.list_os_policy_assignments_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_os_policy_assignments_async_opt(&self, req: &super::os_policy_assignments::ListOSPolicyAssignmentsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::os_policy_assignments::ListOSPolicyAssignmentsResponse>> {
        self.client.unary_call_async(&METHOD_OS_CONFIG_ZONAL_SERVICE_LIST_OS_POLICY_ASSIGNMENTS, req, opt)
    }

    pub fn list_os_policy_assignments_async(&self, req: &super::os_policy_assignments::ListOSPolicyAssignmentsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::os_policy_assignments::ListOSPolicyAssignmentsResponse>> {
        self.list_os_policy_assignments_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_os_policy_assignment_revisions_opt(&self, req: &super::os_policy_assignments::ListOSPolicyAssignmentRevisionsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::os_policy_assignments::ListOSPolicyAssignmentRevisionsResponse> {
        self.client.unary_call(&METHOD_OS_CONFIG_ZONAL_SERVICE_LIST_OS_POLICY_ASSIGNMENT_REVISIONS, req, opt)
    }

    pub fn list_os_policy_assignment_revisions(&self, req: &super::os_policy_assignments::ListOSPolicyAssignmentRevisionsRequest) -> ::grpcio::Result<super::os_policy_assignments::ListOSPolicyAssignmentRevisionsResponse> {
        self.list_os_policy_assignment_revisions_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_os_policy_assignment_revisions_async_opt(&self, req: &super::os_policy_assignments::ListOSPolicyAssignmentRevisionsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::os_policy_assignments::ListOSPolicyAssignmentRevisionsResponse>> {
        self.client.unary_call_async(&METHOD_OS_CONFIG_ZONAL_SERVICE_LIST_OS_POLICY_ASSIGNMENT_REVISIONS, req, opt)
    }

    pub fn list_os_policy_assignment_revisions_async(&self, req: &super::os_policy_assignments::ListOSPolicyAssignmentRevisionsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::os_policy_assignments::ListOSPolicyAssignmentRevisionsResponse>> {
        self.list_os_policy_assignment_revisions_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_os_policy_assignment_opt(&self, req: &super::os_policy_assignments::DeleteOSPolicyAssignmentRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::operations::Operation> {
        self.client.unary_call(&METHOD_OS_CONFIG_ZONAL_SERVICE_DELETE_OS_POLICY_ASSIGNMENT, req, opt)
    }

    pub fn delete_os_policy_assignment(&self, req: &super::os_policy_assignments::DeleteOSPolicyAssignmentRequest) -> ::grpcio::Result<super::operations::Operation> {
        self.delete_os_policy_assignment_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_os_policy_assignment_async_opt(&self, req: &super::os_policy_assignments::DeleteOSPolicyAssignmentRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operations::Operation>> {
        self.client.unary_call_async(&METHOD_OS_CONFIG_ZONAL_SERVICE_DELETE_OS_POLICY_ASSIGNMENT, req, opt)
    }

    pub fn delete_os_policy_assignment_async(&self, req: &super::os_policy_assignments::DeleteOSPolicyAssignmentRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operations::Operation>> {
        self.delete_os_policy_assignment_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_instance_os_policies_compliance_opt(&self, req: &super::instance_os_policies_compliance::GetInstanceOSPoliciesComplianceRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::instance_os_policies_compliance::InstanceOSPoliciesCompliance> {
        self.client.unary_call(&METHOD_OS_CONFIG_ZONAL_SERVICE_GET_INSTANCE_OS_POLICIES_COMPLIANCE, req, opt)
    }

    pub fn get_instance_os_policies_compliance(&self, req: &super::instance_os_policies_compliance::GetInstanceOSPoliciesComplianceRequest) -> ::grpcio::Result<super::instance_os_policies_compliance::InstanceOSPoliciesCompliance> {
        self.get_instance_os_policies_compliance_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_instance_os_policies_compliance_async_opt(&self, req: &super::instance_os_policies_compliance::GetInstanceOSPoliciesComplianceRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::instance_os_policies_compliance::InstanceOSPoliciesCompliance>> {
        self.client.unary_call_async(&METHOD_OS_CONFIG_ZONAL_SERVICE_GET_INSTANCE_OS_POLICIES_COMPLIANCE, req, opt)
    }

    pub fn get_instance_os_policies_compliance_async(&self, req: &super::instance_os_policies_compliance::GetInstanceOSPoliciesComplianceRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::instance_os_policies_compliance::InstanceOSPoliciesCompliance>> {
        self.get_instance_os_policies_compliance_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_instance_os_policies_compliances_opt(&self, req: &super::instance_os_policies_compliance::ListInstanceOSPoliciesCompliancesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::instance_os_policies_compliance::ListInstanceOSPoliciesCompliancesResponse> {
        self.client.unary_call(&METHOD_OS_CONFIG_ZONAL_SERVICE_LIST_INSTANCE_OS_POLICIES_COMPLIANCES, req, opt)
    }

    pub fn list_instance_os_policies_compliances(&self, req: &super::instance_os_policies_compliance::ListInstanceOSPoliciesCompliancesRequest) -> ::grpcio::Result<super::instance_os_policies_compliance::ListInstanceOSPoliciesCompliancesResponse> {
        self.list_instance_os_policies_compliances_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_instance_os_policies_compliances_async_opt(&self, req: &super::instance_os_policies_compliance::ListInstanceOSPoliciesCompliancesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::instance_os_policies_compliance::ListInstanceOSPoliciesCompliancesResponse>> {
        self.client.unary_call_async(&METHOD_OS_CONFIG_ZONAL_SERVICE_LIST_INSTANCE_OS_POLICIES_COMPLIANCES, req, opt)
    }

    pub fn list_instance_os_policies_compliances_async(&self, req: &super::instance_os_policies_compliance::ListInstanceOSPoliciesCompliancesRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::instance_os_policies_compliance::ListInstanceOSPoliciesCompliancesResponse>> {
        self.list_instance_os_policies_compliances_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_os_policy_assignment_report_opt(&self, req: &super::os_policy_assignment_reports::GetOSPolicyAssignmentReportRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::os_policy_assignment_reports::OSPolicyAssignmentReport> {
        self.client.unary_call(&METHOD_OS_CONFIG_ZONAL_SERVICE_GET_OS_POLICY_ASSIGNMENT_REPORT, req, opt)
    }

    pub fn get_os_policy_assignment_report(&self, req: &super::os_policy_assignment_reports::GetOSPolicyAssignmentReportRequest) -> ::grpcio::Result<super::os_policy_assignment_reports::OSPolicyAssignmentReport> {
        self.get_os_policy_assignment_report_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_os_policy_assignment_report_async_opt(&self, req: &super::os_policy_assignment_reports::GetOSPolicyAssignmentReportRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::os_policy_assignment_reports::OSPolicyAssignmentReport>> {
        self.client.unary_call_async(&METHOD_OS_CONFIG_ZONAL_SERVICE_GET_OS_POLICY_ASSIGNMENT_REPORT, req, opt)
    }

    pub fn get_os_policy_assignment_report_async(&self, req: &super::os_policy_assignment_reports::GetOSPolicyAssignmentReportRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::os_policy_assignment_reports::OSPolicyAssignmentReport>> {
        self.get_os_policy_assignment_report_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_os_policy_assignment_reports_opt(&self, req: &super::os_policy_assignment_reports::ListOSPolicyAssignmentReportsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::os_policy_assignment_reports::ListOSPolicyAssignmentReportsResponse> {
        self.client.unary_call(&METHOD_OS_CONFIG_ZONAL_SERVICE_LIST_OS_POLICY_ASSIGNMENT_REPORTS, req, opt)
    }

    pub fn list_os_policy_assignment_reports(&self, req: &super::os_policy_assignment_reports::ListOSPolicyAssignmentReportsRequest) -> ::grpcio::Result<super::os_policy_assignment_reports::ListOSPolicyAssignmentReportsResponse> {
        self.list_os_policy_assignment_reports_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_os_policy_assignment_reports_async_opt(&self, req: &super::os_policy_assignment_reports::ListOSPolicyAssignmentReportsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::os_policy_assignment_reports::ListOSPolicyAssignmentReportsResponse>> {
        self.client.unary_call_async(&METHOD_OS_CONFIG_ZONAL_SERVICE_LIST_OS_POLICY_ASSIGNMENT_REPORTS, req, opt)
    }

    pub fn list_os_policy_assignment_reports_async(&self, req: &super::os_policy_assignment_reports::ListOSPolicyAssignmentReportsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::os_policy_assignment_reports::ListOSPolicyAssignmentReportsResponse>> {
        self.list_os_policy_assignment_reports_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_inventory_opt(&self, req: &super::inventory::GetInventoryRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::inventory::Inventory> {
        self.client.unary_call(&METHOD_OS_CONFIG_ZONAL_SERVICE_GET_INVENTORY, req, opt)
    }

    pub fn get_inventory(&self, req: &super::inventory::GetInventoryRequest) -> ::grpcio::Result<super::inventory::Inventory> {
        self.get_inventory_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_inventory_async_opt(&self, req: &super::inventory::GetInventoryRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::inventory::Inventory>> {
        self.client.unary_call_async(&METHOD_OS_CONFIG_ZONAL_SERVICE_GET_INVENTORY, req, opt)
    }

    pub fn get_inventory_async(&self, req: &super::inventory::GetInventoryRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::inventory::Inventory>> {
        self.get_inventory_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_inventories_opt(&self, req: &super::inventory::ListInventoriesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::inventory::ListInventoriesResponse> {
        self.client.unary_call(&METHOD_OS_CONFIG_ZONAL_SERVICE_LIST_INVENTORIES, req, opt)
    }

    pub fn list_inventories(&self, req: &super::inventory::ListInventoriesRequest) -> ::grpcio::Result<super::inventory::ListInventoriesResponse> {
        self.list_inventories_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_inventories_async_opt(&self, req: &super::inventory::ListInventoriesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::inventory::ListInventoriesResponse>> {
        self.client.unary_call_async(&METHOD_OS_CONFIG_ZONAL_SERVICE_LIST_INVENTORIES, req, opt)
    }

    pub fn list_inventories_async(&self, req: &super::inventory::ListInventoriesRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::inventory::ListInventoriesResponse>> {
        self.list_inventories_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_vulnerability_report_opt(&self, req: &super::vulnerability::GetVulnerabilityReportRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::vulnerability::VulnerabilityReport> {
        self.client.unary_call(&METHOD_OS_CONFIG_ZONAL_SERVICE_GET_VULNERABILITY_REPORT, req, opt)
    }

    pub fn get_vulnerability_report(&self, req: &super::vulnerability::GetVulnerabilityReportRequest) -> ::grpcio::Result<super::vulnerability::VulnerabilityReport> {
        self.get_vulnerability_report_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_vulnerability_report_async_opt(&self, req: &super::vulnerability::GetVulnerabilityReportRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::vulnerability::VulnerabilityReport>> {
        self.client.unary_call_async(&METHOD_OS_CONFIG_ZONAL_SERVICE_GET_VULNERABILITY_REPORT, req, opt)
    }

    pub fn get_vulnerability_report_async(&self, req: &super::vulnerability::GetVulnerabilityReportRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::vulnerability::VulnerabilityReport>> {
        self.get_vulnerability_report_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_vulnerability_reports_opt(&self, req: &super::vulnerability::ListVulnerabilityReportsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::vulnerability::ListVulnerabilityReportsResponse> {
        self.client.unary_call(&METHOD_OS_CONFIG_ZONAL_SERVICE_LIST_VULNERABILITY_REPORTS, req, opt)
    }

    pub fn list_vulnerability_reports(&self, req: &super::vulnerability::ListVulnerabilityReportsRequest) -> ::grpcio::Result<super::vulnerability::ListVulnerabilityReportsResponse> {
        self.list_vulnerability_reports_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_vulnerability_reports_async_opt(&self, req: &super::vulnerability::ListVulnerabilityReportsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::vulnerability::ListVulnerabilityReportsResponse>> {
        self.client.unary_call_async(&METHOD_OS_CONFIG_ZONAL_SERVICE_LIST_VULNERABILITY_REPORTS, req, opt)
    }

    pub fn list_vulnerability_reports_async(&self, req: &super::vulnerability::ListVulnerabilityReportsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::vulnerability::ListVulnerabilityReportsResponse>> {
        self.list_vulnerability_reports_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::std::future::Future<Output = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait OsConfigZonalService {
    fn create_os_policy_assignment(&mut self, ctx: ::grpcio::RpcContext, _req: super::os_policy_assignments::CreateOSPolicyAssignmentRequest, sink: ::grpcio::UnarySink<super::operations::Operation>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn update_os_policy_assignment(&mut self, ctx: ::grpcio::RpcContext, _req: super::os_policy_assignments::UpdateOSPolicyAssignmentRequest, sink: ::grpcio::UnarySink<super::operations::Operation>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_os_policy_assignment(&mut self, ctx: ::grpcio::RpcContext, _req: super::os_policy_assignments::GetOSPolicyAssignmentRequest, sink: ::grpcio::UnarySink<super::os_policy_assignments::OSPolicyAssignment>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_os_policy_assignments(&mut self, ctx: ::grpcio::RpcContext, _req: super::os_policy_assignments::ListOSPolicyAssignmentsRequest, sink: ::grpcio::UnarySink<super::os_policy_assignments::ListOSPolicyAssignmentsResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_os_policy_assignment_revisions(&mut self, ctx: ::grpcio::RpcContext, _req: super::os_policy_assignments::ListOSPolicyAssignmentRevisionsRequest, sink: ::grpcio::UnarySink<super::os_policy_assignments::ListOSPolicyAssignmentRevisionsResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn delete_os_policy_assignment(&mut self, ctx: ::grpcio::RpcContext, _req: super::os_policy_assignments::DeleteOSPolicyAssignmentRequest, sink: ::grpcio::UnarySink<super::operations::Operation>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_instance_os_policies_compliance(&mut self, ctx: ::grpcio::RpcContext, _req: super::instance_os_policies_compliance::GetInstanceOSPoliciesComplianceRequest, sink: ::grpcio::UnarySink<super::instance_os_policies_compliance::InstanceOSPoliciesCompliance>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_instance_os_policies_compliances(&mut self, ctx: ::grpcio::RpcContext, _req: super::instance_os_policies_compliance::ListInstanceOSPoliciesCompliancesRequest, sink: ::grpcio::UnarySink<super::instance_os_policies_compliance::ListInstanceOSPoliciesCompliancesResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_os_policy_assignment_report(&mut self, ctx: ::grpcio::RpcContext, _req: super::os_policy_assignment_reports::GetOSPolicyAssignmentReportRequest, sink: ::grpcio::UnarySink<super::os_policy_assignment_reports::OSPolicyAssignmentReport>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_os_policy_assignment_reports(&mut self, ctx: ::grpcio::RpcContext, _req: super::os_policy_assignment_reports::ListOSPolicyAssignmentReportsRequest, sink: ::grpcio::UnarySink<super::os_policy_assignment_reports::ListOSPolicyAssignmentReportsResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_inventory(&mut self, ctx: ::grpcio::RpcContext, _req: super::inventory::GetInventoryRequest, sink: ::grpcio::UnarySink<super::inventory::Inventory>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_inventories(&mut self, ctx: ::grpcio::RpcContext, _req: super::inventory::ListInventoriesRequest, sink: ::grpcio::UnarySink<super::inventory::ListInventoriesResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_vulnerability_report(&mut self, ctx: ::grpcio::RpcContext, _req: super::vulnerability::GetVulnerabilityReportRequest, sink: ::grpcio::UnarySink<super::vulnerability::VulnerabilityReport>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_vulnerability_reports(&mut self, ctx: ::grpcio::RpcContext, _req: super::vulnerability::ListVulnerabilityReportsRequest, sink: ::grpcio::UnarySink<super::vulnerability::ListVulnerabilityReportsResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
}

pub fn create_os_config_zonal_service<S: OsConfigZonalService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_OS_CONFIG_ZONAL_SERVICE_CREATE_OS_POLICY_ASSIGNMENT, move |ctx, req, resp| {
        instance.create_os_policy_assignment(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_OS_CONFIG_ZONAL_SERVICE_UPDATE_OS_POLICY_ASSIGNMENT, move |ctx, req, resp| {
        instance.update_os_policy_assignment(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_OS_CONFIG_ZONAL_SERVICE_GET_OS_POLICY_ASSIGNMENT, move |ctx, req, resp| {
        instance.get_os_policy_assignment(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_OS_CONFIG_ZONAL_SERVICE_LIST_OS_POLICY_ASSIGNMENTS, move |ctx, req, resp| {
        instance.list_os_policy_assignments(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_OS_CONFIG_ZONAL_SERVICE_LIST_OS_POLICY_ASSIGNMENT_REVISIONS, move |ctx, req, resp| {
        instance.list_os_policy_assignment_revisions(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_OS_CONFIG_ZONAL_SERVICE_DELETE_OS_POLICY_ASSIGNMENT, move |ctx, req, resp| {
        instance.delete_os_policy_assignment(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_OS_CONFIG_ZONAL_SERVICE_GET_INSTANCE_OS_POLICIES_COMPLIANCE, move |ctx, req, resp| {
        instance.get_instance_os_policies_compliance(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_OS_CONFIG_ZONAL_SERVICE_LIST_INSTANCE_OS_POLICIES_COMPLIANCES, move |ctx, req, resp| {
        instance.list_instance_os_policies_compliances(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_OS_CONFIG_ZONAL_SERVICE_GET_OS_POLICY_ASSIGNMENT_REPORT, move |ctx, req, resp| {
        instance.get_os_policy_assignment_report(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_OS_CONFIG_ZONAL_SERVICE_LIST_OS_POLICY_ASSIGNMENT_REPORTS, move |ctx, req, resp| {
        instance.list_os_policy_assignment_reports(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_OS_CONFIG_ZONAL_SERVICE_GET_INVENTORY, move |ctx, req, resp| {
        instance.get_inventory(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_OS_CONFIG_ZONAL_SERVICE_LIST_INVENTORIES, move |ctx, req, resp| {
        instance.list_inventories(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_OS_CONFIG_ZONAL_SERVICE_GET_VULNERABILITY_REPORT, move |ctx, req, resp| {
        instance.get_vulnerability_report(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_OS_CONFIG_ZONAL_SERVICE_LIST_VULNERABILITY_REPORTS, move |ctx, req, resp| {
        instance.list_vulnerability_reports(ctx, req, resp)
    });
    builder.build()
}
