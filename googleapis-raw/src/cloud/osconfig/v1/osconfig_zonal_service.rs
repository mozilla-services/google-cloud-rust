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
//! Generated file from `google/cloud/osconfig/v1/osconfig_zonal_service.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_28_0;

static file_descriptor_proto_data: &'static [u8] = b"\
    \n5google/cloud/osconfig/v1/osconfig_zonal_service.proto\x12\x18google.c\
    loud.osconfig.v1\x1a\x1cgoogle/api/annotations.proto\x1a\x17google/api/c\
    lient.proto\x1a(google/cloud/osconfig/v1/inventory.proto\x1a;google/clou\
    d/osconfig/v1/os_policy_assignment_reports.proto\x1a4google/cloud/osconf\
    ig/v1/os_policy_assignments.proto\x1a,google/cloud/osconfig/v1/vulnerabi\
    lity.proto\x1a#google/longrunning/operations.proto2\xac\x17\n\x14OsConfi\
    gZonalService\x12\xbe\x02\n\x18CreateOSPolicyAssignment\x129.google.clou\
    d.osconfig.v1.CreateOSPolicyAssignmentRequest\x1a\x1d.google.longrunning\
    .Operation\"\xc7\x01\xcaA9\n\x12OSPolicyAssignment\x12#OSPolicyAssignmen\
    tOperationMetadata\x82\xd3\xe4\x93\x02O\"7/v1/{parent=projects/*/locatio\
    ns/*}/osPolicyAssignments:\x14os_policy_assignment\xdaA3parent,os_policy\
    _assignment,os_policy_assignment_id\x12\xc0\x02\n\x18UpdateOSPolicyAssig\
    nment\x129.google.cloud.osconfig.v1.UpdateOSPolicyAssignmentRequest\x1a\
    \x1d.google.longrunning.Operation\"\xc9\x01\xcaA9\n\x12OSPolicyAssignmen\
    t\x12#OSPolicyAssignmentOperationMetadata\x82\xd3\xe4\x93\x02d2L/v1/{os_\
    policy_assignment.name=projects/*/locations/*/osPolicyAssignments/*}:\
    \x14os_policy_assignment\xdaA\x20os_policy_assignment,update_mask\x12\
    \xc5\x01\n\x15GetOSPolicyAssignment\x126.google.cloud.osconfig.v1.GetOSP\
    olicyAssignmentRequest\x1a,.google.cloud.osconfig.v1.OSPolicyAssignment\
    \"F\x82\xd3\xe4\x93\x029\x127/v1/{name=projects/*/locations/*/osPolicyAs\
    signments/*}\xdaA\x04name\x12\xd8\x01\n\x17ListOSPolicyAssignments\x128.\
    google.cloud.osconfig.v1.ListOSPolicyAssignmentsRequest\x1a9.google.clou\
    d.osconfig.v1.ListOSPolicyAssignmentsResponse\"H\x82\xd3\xe4\x93\x029\
    \x127/v1/{parent=projects/*/locations/*}/osPolicyAssignments\xdaA\x06par\
    ent\x12\xfc\x01\n\x1fListOSPolicyAssignmentRevisions\x12@.google.cloud.o\
    sconfig.v1.ListOSPolicyAssignmentRevisionsRequest\x1aA.google.cloud.osco\
    nfig.v1.ListOSPolicyAssignmentRevisionsResponse\"T\x82\xd3\xe4\x93\x02G\
    \x12E/v1/{name=projects/*/locations/*/osPolicyAssignments/*}:listRevisio\
    ns\xdaA\x04name\x12\xfc\x01\n\x18DeleteOSPolicyAssignment\x129.google.cl\
    oud.osconfig.v1.DeleteOSPolicyAssignmentRequest\x1a\x1d.google.longrunni\
    ng.Operation\"\x85\x01\xcaA<\n\x15google.protobuf.Empty\x12#OSPolicyAssi\
    gnmentOperationMetadata\x82\xd3\xe4\x93\x029*7/v1/{name=projects/*/locat\
    ions/*/osPolicyAssignments/*}\xdaA\x04name\x12\xea\x01\n\x1bGetOSPolicyA\
    ssignmentReport\x12<.google.cloud.osconfig.v1.GetOSPolicyAssignmentRepor\
    tRequest\x1a2.google.cloud.osconfig.v1.OSPolicyAssignmentReport\"Y\x82\
    \xd3\xe4\x93\x02L\x12J/v1/{name=projects/*/locations/*/instances/*/osPol\
    icyAssignments/*/report}\xdaA\x04name\x12\x80\x02\n\x1dListOSPolicyAssig\
    nmentReports\x12>.google.cloud.osconfig.v1.ListOSPolicyAssignmentReports\
    Request\x1a?.google.cloud.osconfig.v1.ListOSPolicyAssignmentReportsRespo\
    nse\"^\x82\xd3\xe4\x93\x02O\x12M/v1/{parent=projects/*/locations/*/insta\
    nces/*/osPolicyAssignments/*}/reports\xdaA\x06parent\x12\xaa\x01\n\x0cGe\
    tInventory\x12-.google.cloud.osconfig.v1.GetInventoryRequest\x1a#.google\
    .cloud.osconfig.v1.Inventory\"F\x82\xd3\xe4\x93\x029\x127/v1/{name=proje\
    cts/*/locations/*/instances/*/inventory}\xdaA\x04name\x12\xc4\x01\n\x0fL\
    istInventories\x120.google.cloud.osconfig.v1.ListInventoriesRequest\x1a1\
    .google.cloud.osconfig.v1.ListInventoriesResponse\"L\x82\xd3\xe4\x93\x02\
    =\x12;/v1/{parent=projects/*/locations/*/instances/*}/inventories\xdaA\
    \x06parent\x12\xd2\x01\n\x16GetVulnerabilityReport\x127.google.cloud.osc\
    onfig.v1.GetVulnerabilityReportRequest\x1a-.google.cloud.osconfig.v1.Vul\
    nerabilityReport\"P\x82\xd3\xe4\x93\x02C\x12A/v1/{name=projects/*/locati\
    ons/*/instances/*/vulnerabilityReport}\xdaA\x04name\x12\xe8\x01\n\x18Lis\
    tVulnerabilityReports\x129.google.cloud.osconfig.v1.ListVulnerabilityRep\
    ortsRequest\x1a:.google.cloud.osconfig.v1.ListVulnerabilityReportsRespon\
    se\"U\x82\xd3\xe4\x93\x02F\x12D/v1/{parent=projects/*/locations/*/instan\
    ces/*}/vulnerabilityReports\xdaA\x06parent\x1aK\xd2A.https://www.googlea\
    pis.com/auth/cloud-platform\xcaA\x17osconfig.googleapis.comB\xc9\x01\n\
    \x1ccom.google.cloud.osconfig.v1B\x19OsConfigZonalServiceProtoP\x01Z8clo\
    ud.google.com/go/osconfig/apiv1/osconfigpb;osconfigpb\xaa\x02\x18Google.\
    Cloud.OsConfig.V1\xca\x02\x18Google\\Cloud\\OsConfig\\V1\xea\x02\x1bGoog\
    le::Cloud::OsConfig::V1J\xe6(\n\x07\x12\x05\x0e\0\xc9\x01\x01\n\xbc\x04\
    \n\x01\x0c\x12\x03\x0e\0\x122\xb1\x04\x20Copyright\x202021\x20Google\x20\
    LLC\n\n\x20Licensed\x20under\x20the\x20Apache\x20License,\x20Version\x20\
    2.0\x20(the\x20\"License\");\n\x20you\x20may\x20not\x20use\x20this\x20fi\
    le\x20except\x20in\x20compliance\x20with\x20the\x20License.\n\x20You\x20\
    may\x20obtain\x20a\x20copy\x20of\x20the\x20License\x20at\n\n\x20\x20\x20\
    \x20\x20http://www.apache.org/licenses/LICENSE-2.0\n\n\x20Unless\x20requ\
    ired\x20by\x20applicable\x20law\x20or\x20agreed\x20to\x20in\x20writing,\
    \x20software\n\x20distributed\x20under\x20the\x20License\x20is\x20distri\
    buted\x20on\x20an\x20\"AS\x20IS\"\x20BASIS,\n\x20WITHOUT\x20WARRANTIES\
    \x20OR\x20CONDITIONS\x20OF\x20ANY\x20KIND,\x20either\x20express\x20or\
    \x20implied.\n\x20See\x20the\x20License\x20for\x20the\x20specific\x20lan\
    guage\x20governing\x20permissions\x20and\n\x20limitations\x20under\x20th\
    e\x20License.\n\n\x08\n\x01\x02\x12\x03\x10\0!\n\t\n\x02\x03\0\x12\x03\
    \x12\0&\n\t\n\x02\x03\x01\x12\x03\x13\0!\n\t\n\x02\x03\x02\x12\x03\x14\0\
    2\n\t\n\x02\x03\x03\x12\x03\x15\0E\n\t\n\x02\x03\x04\x12\x03\x16\0>\n\t\
    \n\x02\x03\x05\x12\x03\x17\06\n\t\n\x02\x03\x06\x12\x03\x18\0-\n\x08\n\
    \x01\x08\x12\x03\x1a\05\n\t\n\x02\x08%\x12\x03\x1a\05\n\x08\n\x01\x08\
    \x12\x03\x1b\0O\n\t\n\x02\x08\x0b\x12\x03\x1b\0O\n\x08\n\x01\x08\x12\x03\
    \x1c\0\"\n\t\n\x02\x08\n\x12\x03\x1c\0\"\n\x08\n\x01\x08\x12\x03\x1d\0:\
    \n\t\n\x02\x08\x08\x12\x03\x1d\0:\n\x08\n\x01\x08\x12\x03\x1e\05\n\t\n\
    \x02\x08\x01\x12\x03\x1e\05\n\x08\n\x01\x08\x12\x03\x1f\05\n\t\n\x02\x08\
    )\x12\x03\x1f\05\n\x08\n\x01\x08\x12\x03\x20\04\n\t\n\x02\x08-\x12\x03\
    \x20\04\n\xbb\x01\n\x02\x06\0\x12\x05&\0\xc9\x01\x01\x1a\xad\x01\x20Zona\
    l\x20OS\x20Config\x20API\n\n\x20The\x20OS\x20Config\x20service\x20is\x20\
    the\x20server-side\x20component\x20that\x20allows\x20users\x20to\n\x20ma\
    nage\x20package\x20installations\x20and\x20patch\x20jobs\x20for\x20Compu\
    te\x20Engine\x20VM\x20instances.\n\n\n\n\x03\x06\0\x01\x12\x03&\x08\x1c\
    \n\n\n\x03\x06\0\x03\x12\x03'\x02?\n\x0c\n\x05\x06\0\x03\x99\x08\x12\x03\
    '\x02?\n\x0b\n\x03\x06\0\x03\x12\x04(\x02)7\n\r\n\x05\x06\0\x03\x9a\x08\
    \x12\x04(\x02)7\n\xda\x03\n\x04\x06\0\x02\0\x12\x044\x02@\x03\x1a\xcb\
    \x03\x20Create\x20an\x20OS\x20policy\x20assignment.\n\n\x20This\x20metho\
    d\x20also\x20creates\x20the\x20first\x20revision\x20of\x20the\x20OS\x20p\
    olicy\x20assignment.\n\n\x20This\x20method\x20returns\x20a\x20long\x20ru\
    nning\x20operation\x20(LRO)\x20that\x20contains\x20the\n\x20rollout\x20d\
    etails.\x20The\x20rollout\x20can\x20be\x20cancelled\x20by\x20cancelling\
    \x20the\x20LRO.\n\n\x20For\x20more\x20information,\x20see\x20[Method:\n\
    \x20projects.locations.osPolicyAssignments.operations.cancel](https://cl\
    oud.google.com/compute/docs/osconfig/rest/v1/projects.locations.osPolicy\
    Assignments.operations/cancel).\n\n\x0c\n\x05\x06\0\x02\0\x01\x12\x034\
    \x06\x1e\n\x0c\n\x05\x06\0\x02\0\x02\x12\x034\x1f>\n\x0c\n\x05\x06\0\x02\
    \0\x03\x12\x035\x0f+\n\r\n\x05\x06\0\x02\0\x04\x12\x046\x049\x06\n\x11\n\
    \t\x06\0\x02\0\x04\xb0\xca\xbc\"\x12\x046\x049\x06\n\r\n\x05\x06\0\x02\0\
    \x04\x12\x04:\x04;>\n\x10\n\x08\x06\0\x02\0\x04\x9b\x08\0\x12\x04:\x04;>\
    \n\r\n\x05\x06\0\x02\0\x04\x12\x04<\x04?\x06\n\x0f\n\x07\x06\0\x02\0\x04\
    \x99\x08\x12\x04<\x04?\x06\n\xda\x03\n\x04\x06\0\x02\x01\x12\x04K\x02V\
    \x03\x1a\xcb\x03\x20Update\x20an\x20existing\x20OS\x20policy\x20assignme\
    nt.\n\n\x20This\x20method\x20creates\x20a\x20new\x20revision\x20of\x20th\
    e\x20OS\x20policy\x20assignment.\n\n\x20This\x20method\x20returns\x20a\
    \x20long\x20running\x20operation\x20(LRO)\x20that\x20contains\x20the\n\
    \x20rollout\x20details.\x20The\x20rollout\x20can\x20be\x20cancelled\x20b\
    y\x20cancelling\x20the\x20LRO.\n\n\x20For\x20more\x20information,\x20see\
    \x20[Method:\n\x20projects.locations.osPolicyAssignments.operations.canc\
    el](https://cloud.google.com/compute/docs/osconfig/rest/v1/projects.loca\
    tions.osPolicyAssignments.operations/cancel).\n\n\x0c\n\x05\x06\0\x02\
    \x01\x01\x12\x03K\x06\x1e\n\x0c\n\x05\x06\0\x02\x01\x02\x12\x03K\x1f>\n\
    \x0c\n\x05\x06\0\x02\x01\x03\x12\x03L\x0f+\n\r\n\x05\x06\0\x02\x01\x04\
    \x12\x04M\x04P\x06\n\x11\n\t\x06\0\x02\x01\x04\xb0\xca\xbc\"\x12\x04M\
    \x04P\x06\n\x0c\n\x05\x06\0\x02\x01\x04\x12\x03Q\x04N\n\x0f\n\x08\x06\0\
    \x02\x01\x04\x9b\x08\0\x12\x03Q\x04N\n\r\n\x05\x06\0\x02\x01\x04\x12\x04\
    R\x04U\x06\n\x0f\n\x07\x06\0\x02\x01\x04\x99\x08\x12\x04R\x04U\x06\n\xe1\
    \x01\n\x04\x06\0\x02\x02\x12\x04]\x02c\x03\x1a\xd2\x01\x20Retrieve\x20an\
    \x20existing\x20OS\x20policy\x20assignment.\n\n\x20This\x20method\x20alw\
    ays\x20returns\x20the\x20latest\x20revision.\x20In\x20order\x20to\x20ret\
    rieve\x20a\n\x20previous\x20revision\x20of\x20the\x20assignment,\x20also\
    \x20provide\x20the\x20revision\x20ID\x20in\x20the\n\x20`name`\x20paramet\
    er.\n\n\x0c\n\x05\x06\0\x02\x02\x01\x12\x03]\x06\x1b\n\x0c\n\x05\x06\0\
    \x02\x02\x02\x12\x03]\x1c8\n\x0c\n\x05\x06\0\x02\x02\x03\x12\x03^\x0f!\n\
    \r\n\x05\x06\0\x02\x02\x04\x12\x04_\x04a\x06\n\x11\n\t\x06\0\x02\x02\x04\
    \xb0\xca\xbc\"\x12\x04_\x04a\x06\n\x0c\n\x05\x06\0\x02\x02\x04\x12\x03b\
    \x042\n\x0f\n\x08\x06\0\x02\x02\x04\x9b\x08\0\x12\x03b\x042\n\x8b\x01\n\
    \x04\x06\0\x02\x03\x12\x04h\x02n\x03\x1a}\x20List\x20the\x20OS\x20policy\
    \x20assignments\x20under\x20the\x20parent\x20resource.\n\n\x20For\x20eac\
    h\x20OS\x20policy\x20assignment,\x20the\x20latest\x20revision\x20is\x20r\
    eturned.\n\n\x0c\n\x05\x06\0\x02\x03\x01\x12\x03h\x06\x1d\n\x0c\n\x05\
    \x06\0\x02\x03\x02\x12\x03h\x1e<\n\x0c\n\x05\x06\0\x02\x03\x03\x12\x03i\
    \x0f.\n\r\n\x05\x06\0\x02\x03\x04\x12\x04j\x04l\x06\n\x11\n\t\x06\0\x02\
    \x03\x04\xb0\xca\xbc\"\x12\x04j\x04l\x06\n\x0c\n\x05\x06\0\x02\x03\x04\
    \x12\x03m\x044\n\x0f\n\x08\x06\0\x02\x03\x04\x9b\x08\0\x12\x03m\x044\nY\
    \n\x04\x06\0\x02\x04\x12\x04q\x02w\x03\x1aK\x20List\x20the\x20OS\x20poli\
    cy\x20assignment\x20revisions\x20for\x20a\x20given\x20OS\x20policy\x20as\
    signment.\n\n\x0c\n\x05\x06\0\x02\x04\x01\x12\x03q\x06%\n\x0c\n\x05\x06\
    \0\x02\x04\x02\x12\x03q&L\n\x0c\n\x05\x06\0\x02\x04\x03\x12\x03r\x0f6\n\
    \r\n\x05\x06\0\x02\x04\x04\x12\x04s\x04u\x06\n\x11\n\t\x06\0\x02\x04\x04\
    \xb0\xca\xbc\"\x12\x04s\x04u\x06\n\x0c\n\x05\x06\0\x02\x04\x04\x12\x03v\
    \x042\n\x0f\n\x08\x06\0\x02\x04\x04\x9b\x08\0\x12\x03v\x042\n\xc6\x04\n\
    \x04\x06\0\x02\x05\x12\x06\x85\x01\x02\x8f\x01\x03\x1a\xb5\x04\x20Delete\
    \x20the\x20OS\x20policy\x20assignment.\n\n\x20This\x20method\x20creates\
    \x20a\x20new\x20revision\x20of\x20the\x20OS\x20policy\x20assignment.\n\n\
    \x20This\x20method\x20returns\x20a\x20long\x20running\x20operation\x20(L\
    RO)\x20that\x20contains\x20the\n\x20rollout\x20details.\x20The\x20rollou\
    t\x20can\x20be\x20cancelled\x20by\x20cancelling\x20the\x20LRO.\n\n\x20If\
    \x20the\x20LRO\x20completes\x20and\x20is\x20not\x20cancelled,\x20all\x20\
    revisions\x20associated\x20with\n\x20the\x20OS\x20policy\x20assignment\
    \x20are\x20deleted.\n\n\x20For\x20more\x20information,\x20see\x20[Method\
    :\n\x20projects.locations.osPolicyAssignments.operations.cancel](https:/\
    /cloud.google.com/compute/docs/osconfig/rest/v1/projects.locations.osPol\
    icyAssignments.operations/cancel).\n\n\r\n\x05\x06\0\x02\x05\x01\x12\x04\
    \x85\x01\x06\x1e\n\r\n\x05\x06\0\x02\x05\x02\x12\x04\x85\x01\x1f>\n\r\n\
    \x05\x06\0\x02\x05\x03\x12\x04\x86\x01\x0f+\n\x0f\n\x05\x06\0\x02\x05\
    \x04\x12\x06\x87\x01\x04\x89\x01\x06\n\x13\n\t\x06\0\x02\x05\x04\xb0\xca\
    \xbc\"\x12\x06\x87\x01\x04\x89\x01\x06\n\r\n\x05\x06\0\x02\x05\x04\x12\
    \x04\x8a\x01\x042\n\x10\n\x08\x06\0\x02\x05\x04\x9b\x08\0\x12\x04\x8a\
    \x01\x042\n\x0f\n\x05\x06\0\x02\x05\x04\x12\x06\x8b\x01\x04\x8e\x01\x06\
    \n\x11\n\x07\x06\0\x02\x05\x04\x99\x08\x12\x06\x8b\x01\x04\x8e\x01\x06\n\
    e\n\x04\x06\0\x02\x06\x12\x06\x93\x01\x02\x99\x01\x03\x1aU\x20Get\x20the\
    \x20OS\x20policy\x20asssignment\x20report\x20for\x20the\x20specified\x20\
    Compute\x20Engine\x20VM\n\x20instance.\n\n\r\n\x05\x06\0\x02\x06\x01\x12\
    \x04\x93\x01\x06!\n\r\n\x05\x06\0\x02\x06\x02\x12\x04\x93\x01\"D\n\r\n\
    \x05\x06\0\x02\x06\x03\x12\x04\x94\x01\x0f'\n\x0f\n\x05\x06\0\x02\x06\
    \x04\x12\x06\x95\x01\x04\x97\x01\x06\n\x13\n\t\x06\0\x02\x06\x04\xb0\xca\
    \xbc\"\x12\x06\x95\x01\x04\x97\x01\x06\n\r\n\x05\x06\0\x02\x06\x04\x12\
    \x04\x98\x01\x042\n\x10\n\x08\x06\0\x02\x06\x04\x9b\x08\0\x12\x04\x98\
    \x01\x042\np\n\x04\x06\0\x02\x07\x12\x06\x9d\x01\x02\xa3\x01\x03\x1a`\
    \x20List\x20OS\x20policy\x20asssignment\x20reports\x20for\x20all\x20Comp\
    ute\x20Engine\x20VM\x20instances\x20in\n\x20the\x20specified\x20zone.\n\
    \n\r\n\x05\x06\0\x02\x07\x01\x12\x04\x9d\x01\x06#\n\r\n\x05\x06\0\x02\
    \x07\x02\x12\x04\x9d\x01$H\n\r\n\x05\x06\0\x02\x07\x03\x12\x04\x9e\x01\
    \x0f4\n\x0f\n\x05\x06\0\x02\x07\x04\x12\x06\x9f\x01\x04\xa1\x01\x06\n\
    \x13\n\t\x06\0\x02\x07\x04\xb0\xca\xbc\"\x12\x06\x9f\x01\x04\xa1\x01\x06\
    \n\r\n\x05\x06\0\x02\x07\x04\x12\x04\xa2\x01\x044\n\x10\n\x08\x06\0\x02\
    \x07\x04\x9b\x08\0\x12\x04\xa2\x01\x044\n\x91\x01\n\x04\x06\0\x02\x08\
    \x12\x06\xa7\x01\x02\xac\x01\x03\x1a\x80\x01\x20Get\x20inventory\x20data\
    \x20for\x20the\x20specified\x20VM\x20instance.\x20If\x20the\x20VM\x20has\
    \x20no\n\x20associated\x20inventory,\x20the\x20message\x20`NOT_FOUND`\
    \x20is\x20returned.\n\n\r\n\x05\x06\0\x02\x08\x01\x12\x04\xa7\x01\x06\
    \x12\n\r\n\x05\x06\0\x02\x08\x02\x12\x04\xa7\x01\x13&\n\r\n\x05\x06\0\
    \x02\x08\x03\x12\x04\xa7\x011:\n\x0f\n\x05\x06\0\x02\x08\x04\x12\x06\xa8\
    \x01\x04\xaa\x01\x06\n\x13\n\t\x06\0\x02\x08\x04\xb0\xca\xbc\"\x12\x06\
    \xa8\x01\x04\xaa\x01\x06\n\r\n\x05\x06\0\x02\x08\x04\x12\x04\xab\x01\x04\
    2\n\x10\n\x08\x06\0\x02\x08\x04\x9b\x08\0\x12\x04\xab\x01\x042\nQ\n\x04\
    \x06\0\x02\t\x12\x06\xaf\x01\x02\xb5\x01\x03\x1aA\x20List\x20inventory\
    \x20data\x20for\x20all\x20VM\x20instances\x20in\x20the\x20specified\x20z\
    one.\n\n\r\n\x05\x06\0\x02\t\x01\x12\x04\xaf\x01\x06\x15\n\r\n\x05\x06\0\
    \x02\t\x02\x12\x04\xaf\x01\x16,\n\r\n\x05\x06\0\x02\t\x03\x12\x04\xb0\
    \x01\x0f&\n\x0f\n\x05\x06\0\x02\t\x04\x12\x06\xb1\x01\x04\xb3\x01\x06\n\
    \x13\n\t\x06\0\x02\t\x04\xb0\xca\xbc\"\x12\x06\xb1\x01\x04\xb3\x01\x06\n\
    \r\n\x05\x06\0\x02\t\x04\x12\x04\xb4\x01\x044\n\x10\n\x08\x06\0\x02\t\
    \x04\x9b\x08\0\x12\x04\xb4\x01\x044\n\x9e\x01\n\x04\x06\0\x02\n\x12\x06\
    \xb9\x01\x02\xbf\x01\x03\x1a\x8d\x01\x20Gets\x20the\x20vulnerability\x20\
    report\x20for\x20the\x20specified\x20VM\x20instance.\x20Only\x20VMs\x20w\
    ith\n\x20inventory\x20data\x20have\x20vulnerability\x20reports\x20associ\
    ated\x20with\x20them.\n\n\r\n\x05\x06\0\x02\n\x01\x12\x04\xb9\x01\x06\
    \x1c\n\r\n\x05\x06\0\x02\n\x02\x12\x04\xb9\x01\x1d:\n\r\n\x05\x06\0\x02\
    \n\x03\x12\x04\xba\x01\x0f\"\n\x0f\n\x05\x06\0\x02\n\x04\x12\x06\xbb\x01\
    \x04\xbd\x01\x06\n\x13\n\t\x06\0\x02\n\x04\xb0\xca\xbc\"\x12\x06\xbb\x01\
    \x04\xbd\x01\x06\n\r\n\x05\x06\0\x02\n\x04\x12\x04\xbe\x01\x042\n\x10\n\
    \x08\x06\0\x02\n\x04\x9b\x08\0\x12\x04\xbe\x01\x042\nX\n\x04\x06\0\x02\
    \x0b\x12\x06\xc2\x01\x02\xc8\x01\x03\x1aH\x20List\x20vulnerability\x20re\
    ports\x20for\x20all\x20VM\x20instances\x20in\x20the\x20specified\x20zone\
    .\n\n\r\n\x05\x06\0\x02\x0b\x01\x12\x04\xc2\x01\x06\x1e\n\r\n\x05\x06\0\
    \x02\x0b\x02\x12\x04\xc2\x01\x1f>\n\r\n\x05\x06\0\x02\x0b\x03\x12\x04\
    \xc3\x01\x0f/\n\x0f\n\x05\x06\0\x02\x0b\x04\x12\x06\xc4\x01\x04\xc6\x01\
    \x06\n\x13\n\t\x06\0\x02\x0b\x04\xb0\xca\xbc\"\x12\x06\xc4\x01\x04\xc6\
    \x01\x06\n\r\n\x05\x06\0\x02\x0b\x04\x12\x04\xc7\x01\x044\n\x10\n\x08\
    \x06\0\x02\x0b\x04\x9b\x08\0\x12\x04\xc7\x01\x044b\x06proto3\
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
