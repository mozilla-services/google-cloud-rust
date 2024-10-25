// @generated
use crate::{
    api::{annotations, client, field_behavior},
    empty,
    iam::v1::{iam_policy, policy},
    longrunning::operations,
    r#type::expr,
    rpc::status,
};

pub mod access;
pub mod application;
pub mod asset;
pub mod attack_exposure;
pub mod attack_path;
pub mod backup_disaster_recovery;
pub mod bigquery_export;
pub mod cloud_armor;
pub mod cloud_dlp_data_profile;
pub mod cloud_dlp_inspection;
pub mod compliance;
pub mod connection;
pub mod contact_details;
pub mod container;
pub mod database;
pub mod effective_event_threat_detection_custom_module;
pub mod effective_security_health_analytics_custom_module;
pub mod event_threat_detection_custom_module;
pub mod event_threat_detection_custom_module_validation_errors;
pub mod exfiltration;
pub mod external_system;
pub mod file;
pub mod finding;
pub mod folder;
pub mod group_membership;
pub mod iam_binding;
pub mod indicator;
pub mod kernel_rootkit;
pub mod kubernetes;
pub mod label;
pub mod load_balancer;
pub mod log_entry;
pub mod mitre_attack;
pub mod mute_config;
pub mod notebook;
pub mod notification_config;
pub mod notification_message;
pub mod org_policy;
pub mod organization_settings;
pub mod process;
pub mod resource;
pub mod resource_value_config;
pub mod run_asset_discovery_response;
pub mod security_health_analytics_custom_config;
pub mod security_health_analytics_custom_module;
pub mod security_marks;
pub mod security_posture;
pub mod securitycenter_service;
pub mod securitycenter_service_grpc;
pub mod simulation;
pub mod source;
pub mod toxic_combination;
pub mod valued_resource;
pub mod vulnerability;
