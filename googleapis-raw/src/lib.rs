// Copyright 2020 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![allow(bare_trait_objects)]

// This appears as a comment in each generated file. Add it once here
// to save a bit of time and effort.

const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_3_0;

pub const API_CLIENT_ID: &str = "gcp-grpc-rs";

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub(crate) mod iam;
pub(crate) mod rpc;
pub(crate) mod r#type;

// pub mod empty;
#[cfg(feature = "bigtable")]
#[cfg(any(feature = "bigtable", feature = "pubsub", feature = "spanner"))]
#[cfg(feature = "pubsub")]
#[cfg(feature = "spanner")]
pub mod empty;
pub mod api;
pub mod bigtable;
pub mod cloud;
pub mod identity;
pub mod logging;
pub mod longrunning;
pub mod orgpolicy;
pub mod pubsub;
pub mod spanner;
