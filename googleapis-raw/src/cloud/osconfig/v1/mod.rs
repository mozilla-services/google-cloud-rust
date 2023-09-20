use crate::{
    empty,
    longrunning::operations,
    r#type::{
        date,
        datetime,
        dayofweek,
        timeofday,
    },
};

pub mod inventory;
pub mod os_policy;
pub mod os_policy_assignment_reports;
pub mod os_policy_assignments;
pub mod osconfig_common;
pub mod osconfig_service;
pub mod osconfig_service_grpc;
pub mod osconfig_zonal_service;
pub mod osconfig_zonal_service_grpc;
pub mod patch_deployments;
pub mod patch_jobs;
pub mod vulnerability;
