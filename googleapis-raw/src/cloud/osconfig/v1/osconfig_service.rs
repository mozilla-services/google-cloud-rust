// This file is generated by rust-protobuf 2.28.0. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `google/cloud/osconfig/v1/osconfig_service.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_28_0;

static file_descriptor_proto_data: &'static [u8] = b"\
    \n/google/cloud/osconfig/v1/osconfig_service.proto\x12\x18google.cloud.o\
    sconfig.v1\x1a\x17google/api/client.proto\x1a\x19google/api/resource.pro\
    to\x1a0google/cloud/osconfig/v1/patch_deployments.proto\x1a)google/cloud\
    /osconfig/v1/patch_jobs.proto\x1a\x1bgoogle/protobuf/empty.proto\x1a\x1c\
    google/api/annotations.proto2\xac\x12\n\x0fOsConfigService\x12\x9d\x01\n\
    \x0fExecutePatchJob\x120.google.cloud.osconfig.v1.ExecutePatchJobRequest\
    \x1a\".google.cloud.osconfig.v1.PatchJob\"4\x82\xd3\xe4\x93\x02.\")/v1/{\
    parent=projects/*}/patchJobs:execute:\x01*\x12\x91\x01\n\x0bGetPatchJob\
    \x12,.google.cloud.osconfig.v1.GetPatchJobRequest\x1a\".google.cloud.osc\
    onfig.v1.PatchJob\"0\x82\xd3\xe4\x93\x02#\x12!/v1/{name=projects/*/patch\
    Jobs/*}\xdaA\x04name\x12\x9a\x01\n\x0eCancelPatchJob\x12/.google.cloud.o\
    sconfig.v1.CancelPatchJobRequest\x1a\".google.cloud.osconfig.v1.PatchJob\
    \"3\x82\xd3\xe4\x93\x02-\"(/v1/{name=projects/*/patchJobs/*}:cancel:\x01\
    *\x12\xa4\x01\n\rListPatchJobs\x12..google.cloud.osconfig.v1.ListPatchJo\
    bsRequest\x1a/.google.cloud.osconfig.v1.ListPatchJobsResponse\"2\x82\xd3\
    \xe4\x93\x02#\x12!/v1/{parent=projects/*}/patchJobs\xdaA\x06parent\x12\
    \xe0\x01\n\x1bListPatchJobInstanceDetails\x12<.google.cloud.osconfig.v1.\
    ListPatchJobInstanceDetailsRequest\x1a=.google.cloud.osconfig.v1.ListPat\
    chJobInstanceDetailsResponse\"D\x82\xd3\xe4\x93\x025\x123/v1/{parent=pro\
    jects/*/patchJobs/*}/instanceDetails\xdaA\x06parent\x12\xec\x01\n\x15Cre\
    atePatchDeployment\x126.google.cloud.osconfig.v1.CreatePatchDeploymentRe\
    quest\x1a).google.cloud.osconfig.v1.PatchDeployment\"p\x82\xd3\xe4\x93\
    \x02<\"(/v1/{parent=projects/*}/patchDeployments:\x10patch_deployment\
    \xdaA+parent,patch_deployment,patch_deployment_id\x12\xad\x01\n\x12GetPa\
    tchDeployment\x123.google.cloud.osconfig.v1.GetPatchDeploymentRequest\
    \x1a).google.cloud.osconfig.v1.PatchDeployment\"7\x82\xd3\xe4\x93\x02*\
    \x12(/v1/{name=projects/*/patchDeployments/*}\xdaA\x04name\x12\xc0\x01\n\
    \x14ListPatchDeployments\x125.google.cloud.osconfig.v1.ListPatchDeployme\
    ntsRequest\x1a6.google.cloud.osconfig.v1.ListPatchDeploymentsResponse\"9\
    \x82\xd3\xe4\x93\x02*\x12(/v1/{parent=projects/*}/patchDeployments\xdaA\
    \x06parent\x12\xa0\x01\n\x15DeletePatchDeployment\x126.google.cloud.osco\
    nfig.v1.DeletePatchDeploymentRequest\x1a\x16.google.protobuf.Empty\"7\
    \x82\xd3\xe4\x93\x02**(/v1/{name=projects/*/patchDeployments/*}\xdaA\x04\
    name\x12\xee\x01\n\x15UpdatePatchDeployment\x126.google.cloud.osconfig.v\
    1.UpdatePatchDeploymentRequest\x1a).google.cloud.osconfig.v1.PatchDeploy\
    ment\"r\x82\xd3\xe4\x93\x02M29/v1/{patch_deployment.name=projects/*/patc\
    hDeployments/*}:\x10patch_deployment\xdaA\x1cpatch_deployment,update_mas\
    k\x12\xba\x01\n\x14PausePatchDeployment\x125.google.cloud.osconfig.v1.Pa\
    usePatchDeploymentRequest\x1a).google.cloud.osconfig.v1.PatchDeployment\
    \"@\x82\xd3\xe4\x93\x023\"./v1/{name=projects/*/patchDeployments/*}:paus\
    e:\x01*\xdaA\x04name\x12\xbd\x01\n\x15ResumePatchDeployment\x126.google.\
    cloud.osconfig.v1.ResumePatchDeploymentRequest\x1a).google.cloud.osconfi\
    g.v1.PatchDeployment\"A\x82\xd3\xe4\x93\x024\"//v1/{name=projects/*/patc\
    hDeployments/*}:resume:\x01*\xdaA\x04name\x1aK\xd2A.https://www.googleap\
    is.com/auth/cloud-platform\xcaA\x17osconfig.googleapis.comB\xd4\x02\n\
    \x1ccom.google.cloud.osconfig.v1B\rOsConfigProtoZ8cloud.google.com/go/os\
    config/apiv1/osconfigpb;osconfigpb\xaa\x02\x18Google.Cloud.OsConfig.V1\
    \xca\x02\x18Google\\Cloud\\OsConfig\\V1\xea\x02\x1bGoogle::Cloud::OsConf\
    ig::V1\xeaA\x95\x01\n\x1fcompute.googleapis.com/Instance\x124projects/{p\
    roject}/zones/{zone}/instances/{instance}\x12<projects/{project}/locatio\
    ns/{location}/instances/{instance}J\xd8\x19\n\x07\x12\x05\x0e\0\x9d\x01\
    \x01\n\xbc\x04\n\x01\x0c\x12\x03\x0e\0\x122\xb1\x04\x20Copyright\x202020\
    \x20Google\x20LLC\n\n\x20Licensed\x20under\x20the\x20Apache\x20License,\
    \x20Version\x202.0\x20(the\x20\"License\");\n\x20you\x20may\x20not\x20us\
    e\x20this\x20file\x20except\x20in\x20compliance\x20with\x20the\x20Licens\
    e.\n\x20You\x20may\x20obtain\x20a\x20copy\x20of\x20the\x20License\x20at\
    \n\n\x20\x20\x20\x20\x20http://www.apache.org/licenses/LICENSE-2.0\n\n\
    \x20Unless\x20required\x20by\x20applicable\x20law\x20or\x20agreed\x20to\
    \x20in\x20writing,\x20software\n\x20distributed\x20under\x20the\x20Licen\
    se\x20is\x20distributed\x20on\x20an\x20\"AS\x20IS\"\x20BASIS,\n\x20WITHO\
    UT\x20WARRANTIES\x20OR\x20CONDITIONS\x20OF\x20ANY\x20KIND,\x20either\x20\
    express\x20or\x20implied.\n\x20See\x20the\x20License\x20for\x20the\x20sp\
    ecific\x20language\x20governing\x20permissions\x20and\n\x20limitations\
    \x20under\x20the\x20License.\n\n\x08\n\x01\x02\x12\x03\x10\0!\n\t\n\x02\
    \x03\0\x12\x03\x12\0!\n\t\n\x02\x03\x01\x12\x03\x13\0#\n\t\n\x02\x03\x02\
    \x12\x03\x14\0:\n\t\n\x02\x03\x03\x12\x03\x15\03\n\t\n\x02\x03\x04\x12\
    \x03\x16\0%\n\t\n\x02\x03\x05\x12\x03\x17\0&\n\x08\n\x01\x08\x12\x03\x19\
    \05\n\t\n\x02\x08%\x12\x03\x19\05\n\x08\n\x01\x08\x12\x03\x1a\0O\n\t\n\
    \x02\x08\x0b\x12\x03\x1a\0O\n\x08\n\x01\x08\x12\x03\x1b\0.\n\t\n\x02\x08\
    \x08\x12\x03\x1b\0.\n\x08\n\x01\x08\x12\x03\x1c\05\n\t\n\x02\x08\x01\x12\
    \x03\x1c\05\n\x08\n\x01\x08\x12\x03\x1d\05\n\t\n\x02\x08)\x12\x03\x1d\05\
    \n\x08\n\x01\x08\x12\x03\x1e\04\n\t\n\x02\x08-\x12\x03\x1e\04\n\t\n\x01\
    \x08\x12\x04\x1f\0#\x02\n\x0c\n\x04\x08\x9d\x08\0\x12\x04\x1f\0#\x02\n\
    \xb0\x01\n\x02\x06\0\x12\x05)\0\x9d\x01\x01\x1a\xa2\x01\x20OS\x20Config\
    \x20API\n\n\x20The\x20OS\x20Config\x20service\x20is\x20a\x20server-side\
    \x20component\x20that\x20you\x20can\x20use\x20to\n\x20manage\x20package\
    \x20installations\x20and\x20patch\x20jobs\x20for\x20virtual\x20machine\
    \x20instances.\n\n\n\n\x03\x06\0\x01\x12\x03)\x08\x17\n\n\n\x03\x06\0\
    \x03\x12\x03*\x02?\n\x0c\n\x05\x06\0\x03\x99\x08\x12\x03*\x02?\n\x0b\n\
    \x03\x06\0\x03\x12\x04+\x02,7\n\r\n\x05\x06\0\x03\x9a\x08\x12\x04+\x02,7\
    \nG\n\x04\x06\0\x02\0\x12\x04/\x024\x03\x1a9\x20Patch\x20VM\x20instances\
    \x20by\x20creating\x20and\x20running\x20a\x20patch\x20job.\n\n\x0c\n\x05\
    \x06\0\x02\0\x01\x12\x03/\x06\x15\n\x0c\n\x05\x06\0\x02\0\x02\x12\x03/\
    \x16,\n\x0c\n\x05\x06\0\x02\0\x03\x12\x03/7?\n\r\n\x05\x06\0\x02\0\x04\
    \x12\x040\x043\x06\n\x11\n\t\x06\0\x02\0\x04\xb0\xca\xbc\"\x12\x040\x043\
    \x06\n\x8b\x01\n\x04\x06\0\x02\x01\x12\x048\x02=\x03\x1a}\x20Get\x20the\
    \x20patch\x20job.\x20This\x20can\x20be\x20used\x20to\x20track\x20the\x20\
    progress\x20of\x20an\n\x20ongoing\x20patch\x20job\x20or\x20review\x20the\
    \x20details\x20of\x20completed\x20jobs.\n\n\x0c\n\x05\x06\0\x02\x01\x01\
    \x12\x038\x06\x11\n\x0c\n\x05\x06\0\x02\x01\x02\x12\x038\x12$\n\x0c\n\
    \x05\x06\0\x02\x01\x03\x12\x038/7\n\r\n\x05\x06\0\x02\x01\x04\x12\x049\
    \x04;\x06\n\x11\n\t\x06\0\x02\x01\x04\xb0\xca\xbc\"\x12\x049\x04;\x06\n\
    \x0c\n\x05\x06\0\x02\x01\x04\x12\x03<\x042\n\x0f\n\x08\x06\0\x02\x01\x04\
    \x9b\x08\0\x12\x03<\x042\nk\n\x04\x06\0\x02\x02\x12\x04A\x02F\x03\x1a]\
    \x20Cancel\x20a\x20patch\x20job.\x20The\x20patch\x20job\x20must\x20be\
    \x20active.\x20Canceled\x20patch\x20jobs\n\x20cannot\x20be\x20restarted.\
    \n\n\x0c\n\x05\x06\0\x02\x02\x01\x12\x03A\x06\x14\n\x0c\n\x05\x06\0\x02\
    \x02\x02\x12\x03A\x15*\n\x0c\n\x05\x06\0\x02\x02\x03\x12\x03A5=\n\r\n\
    \x05\x06\0\x02\x02\x04\x12\x04B\x04E\x06\n\x11\n\t\x06\0\x02\x02\x04\xb0\
    \xca\xbc\"\x12\x04B\x04E\x06\n)\n\x04\x06\0\x02\x03\x12\x04I\x02N\x03\
    \x1a\x1b\x20Get\x20a\x20list\x20of\x20patch\x20jobs.\n\n\x0c\n\x05\x06\0\
    \x02\x03\x01\x12\x03I\x06\x13\n\x0c\n\x05\x06\0\x02\x03\x02\x12\x03I\x14\
    (\n\x0c\n\x05\x06\0\x02\x03\x03\x12\x03I3H\n\r\n\x05\x06\0\x02\x03\x04\
    \x12\x04J\x04L\x06\n\x11\n\t\x06\0\x02\x03\x04\xb0\xca\xbc\"\x12\x04J\
    \x04L\x06\n\x0c\n\x05\x06\0\x02\x03\x04\x12\x03M\x044\n\x0f\n\x08\x06\0\
    \x02\x03\x04\x9b\x08\0\x12\x03M\x044\nE\n\x04\x06\0\x02\x04\x12\x04Q\x02\
    W\x03\x1a7\x20Get\x20a\x20list\x20of\x20instance\x20details\x20for\x20a\
    \x20given\x20patch\x20job.\n\n\x0c\n\x05\x06\0\x02\x04\x01\x12\x03Q\x06!\
    \n\x0c\n\x05\x06\0\x02\x04\x02\x12\x03Q\"D\n\x0c\n\x05\x06\0\x02\x04\x03\
    \x12\x03R\x0f2\n\r\n\x05\x06\0\x02\x04\x04\x12\x04S\x04U\x06\n\x11\n\t\
    \x06\0\x02\x04\x04\xb0\xca\xbc\"\x12\x04S\x04U\x06\n\x0c\n\x05\x06\0\x02\
    \x04\x04\x12\x03V\x044\n\x0f\n\x08\x06\0\x02\x04\x04\x9b\x08\0\x12\x03V\
    \x044\n5\n\x04\x06\0\x02\x05\x12\x04Z\x02b\x03\x1a'\x20Create\x20an\x20O\
    S\x20Config\x20patch\x20deployment.\n\n\x0c\n\x05\x06\0\x02\x05\x01\x12\
    \x03Z\x06\x1b\n\x0c\n\x05\x06\0\x02\x05\x02\x12\x03Z\x1c8\n\x0c\n\x05\
    \x06\0\x02\x05\x03\x12\x03[\x0f\x1e\n\r\n\x05\x06\0\x02\x05\x04\x12\x04\
    \\\x04_\x06\n\x11\n\t\x06\0\x02\x05\x04\xb0\xca\xbc\"\x12\x04\\\x04_\x06\
    \n\r\n\x05\x06\0\x02\x05\x04\x12\x04`\x04a6\n\x10\n\x08\x06\0\x02\x05\
    \x04\x9b\x08\0\x12\x04`\x04a6\n2\n\x04\x06\0\x02\x06\x12\x04e\x02j\x03\
    \x1a$\x20Get\x20an\x20OS\x20Config\x20patch\x20deployment.\n\n\x0c\n\x05\
    \x06\0\x02\x06\x01\x12\x03e\x06\x18\n\x0c\n\x05\x06\0\x02\x06\x02\x12\
    \x03e\x192\n\x0c\n\x05\x06\0\x02\x06\x03\x12\x03e=L\n\r\n\x05\x06\0\x02\
    \x06\x04\x12\x04f\x04h\x06\n\x11\n\t\x06\0\x02\x06\x04\xb0\xca\xbc\"\x12\
    \x04f\x04h\x06\n\x0c\n\x05\x06\0\x02\x06\x04\x12\x03i\x042\n\x0f\n\x08\
    \x06\0\x02\x06\x04\x9b\x08\0\x12\x03i\x042\n:\n\x04\x06\0\x02\x07\x12\
    \x04m\x02s\x03\x1a,\x20Get\x20a\x20page\x20of\x20OS\x20Config\x20patch\
    \x20deployments.\n\n\x0c\n\x05\x06\0\x02\x07\x01\x12\x03m\x06\x1a\n\x0c\
    \n\x05\x06\0\x02\x07\x02\x12\x03m\x1b6\n\x0c\n\x05\x06\0\x02\x07\x03\x12\
    \x03n\x0f+\n\r\n\x05\x06\0\x02\x07\x04\x12\x04o\x04q\x06\n\x11\n\t\x06\0\
    \x02\x07\x04\xb0\xca\xbc\"\x12\x04o\x04q\x06\n\x0c\n\x05\x06\0\x02\x07\
    \x04\x12\x03r\x044\n\x0f\n\x08\x06\0\x02\x07\x04\x9b\x08\0\x12\x03r\x044\
    \n5\n\x04\x06\0\x02\x08\x12\x04v\x02|\x03\x1a'\x20Delete\x20an\x20OS\x20\
    Config\x20patch\x20deployment.\n\n\x0c\n\x05\x06\0\x02\x08\x01\x12\x03v\
    \x06\x1b\n\x0c\n\x05\x06\0\x02\x08\x02\x12\x03v\x1c8\n\x0c\n\x05\x06\0\
    \x02\x08\x03\x12\x03w\x0f$\n\r\n\x05\x06\0\x02\x08\x04\x12\x04x\x04z\x06\
    \n\x11\n\t\x06\0\x02\x08\x04\xb0\xca\xbc\"\x12\x04x\x04z\x06\n\x0c\n\x05\
    \x06\0\x02\x08\x04\x12\x03{\x042\n\x0f\n\x08\x06\0\x02\x08\x04\x9b\x08\0\
    \x12\x03{\x042\n6\n\x04\x06\0\x02\t\x12\x05\x7f\x02\x86\x01\x03\x1a'\x20\
    Update\x20an\x20OS\x20Config\x20patch\x20deployment.\n\n\x0c\n\x05\x06\0\
    \x02\t\x01\x12\x03\x7f\x06\x1b\n\x0c\n\x05\x06\0\x02\t\x02\x12\x03\x7f\
    \x1c8\n\r\n\x05\x06\0\x02\t\x03\x12\x04\x80\x01\x0f\x1e\n\x0f\n\x05\x06\
    \0\x02\t\x04\x12\x06\x81\x01\x04\x84\x01\x06\n\x13\n\t\x06\0\x02\t\x04\
    \xb0\xca\xbc\"\x12\x06\x81\x01\x04\x84\x01\x06\n\r\n\x05\x06\0\x02\t\x04\
    \x12\x04\x85\x01\x04J\n\x10\n\x08\x06\0\x02\t\x04\x9b\x08\0\x12\x04\x85\
    \x01\x04J\n~\n\x04\x06\0\x02\n\x12\x06\x8a\x01\x02\x91\x01\x03\x1an\x20C\
    hange\x20state\x20of\x20patch\x20deployment\x20to\x20\"PAUSED\".\n\x20Pa\
    tch\x20deployment\x20in\x20paused\x20state\x20doesn't\x20generate\x20pat\
    ch\x20jobs.\n\n\r\n\x05\x06\0\x02\n\x01\x12\x04\x8a\x01\x06\x1a\n\r\n\
    \x05\x06\0\x02\n\x02\x12\x04\x8a\x01\x1b6\n\r\n\x05\x06\0\x02\n\x03\x12\
    \x04\x8b\x01\x0f\x1e\n\x0f\n\x05\x06\0\x02\n\x04\x12\x06\x8c\x01\x04\x8f\
    \x01\x06\n\x13\n\t\x06\0\x02\n\x04\xb0\xca\xbc\"\x12\x06\x8c\x01\x04\x8f\
    \x01\x06\n\r\n\x05\x06\0\x02\n\x04\x12\x04\x90\x01\x042\n\x10\n\x08\x06\
    \0\x02\n\x04\x9b\x08\0\x12\x04\x90\x01\x042\n\x88\x01\n\x04\x06\0\x02\
    \x0b\x12\x06\x95\x01\x02\x9c\x01\x03\x1ax\x20Change\x20state\x20of\x20pa\
    tch\x20deployment\x20back\x20to\x20\"ACTIVE\".\n\x20Patch\x20deployment\
    \x20in\x20active\x20state\x20continues\x20to\x20generate\x20patch\x20job\
    s.\n\n\r\n\x05\x06\0\x02\x0b\x01\x12\x04\x95\x01\x06\x1b\n\r\n\x05\x06\0\
    \x02\x0b\x02\x12\x04\x95\x01\x1c8\n\r\n\x05\x06\0\x02\x0b\x03\x12\x04\
    \x96\x01\x0f\x1e\n\x0f\n\x05\x06\0\x02\x0b\x04\x12\x06\x97\x01\x04\x9a\
    \x01\x06\n\x13\n\t\x06\0\x02\x0b\x04\xb0\xca\xbc\"\x12\x06\x97\x01\x04\
    \x9a\x01\x06\n\r\n\x05\x06\0\x02\x0b\x04\x12\x04\x9b\x01\x042\n\x10\n\
    \x08\x06\0\x02\x0b\x04\x9b\x08\0\x12\x04\x9b\x01\x042b\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
