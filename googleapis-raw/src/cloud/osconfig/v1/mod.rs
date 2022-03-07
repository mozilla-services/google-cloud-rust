pub(crate) use crate::{
    r#type::{datetime, dayofweek, timeofday},
    empty,
};
pub mod inventory;
pub mod osconfig_common;
pub mod osconfig_service_grpc;
pub mod osconfig_service;
pub mod patch_deployments;
pub mod patch_jobs;