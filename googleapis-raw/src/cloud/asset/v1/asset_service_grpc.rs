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

const METHOD_ASSET_SERVICE_EXPORT_ASSETS: ::grpcio::Method<super::asset_service::ExportAssetsRequest, super::operations::Operation> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.asset.v1.AssetService/ExportAssets",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ASSET_SERVICE_LIST_ASSETS: ::grpcio::Method<super::asset_service::ListAssetsRequest, super::asset_service::ListAssetsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.asset.v1.AssetService/ListAssets",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ASSET_SERVICE_BATCH_GET_ASSETS_HISTORY: ::grpcio::Method<super::asset_service::BatchGetAssetsHistoryRequest, super::asset_service::BatchGetAssetsHistoryResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.asset.v1.AssetService/BatchGetAssetsHistory",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ASSET_SERVICE_CREATE_FEED: ::grpcio::Method<super::asset_service::CreateFeedRequest, super::asset_service::Feed> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.asset.v1.AssetService/CreateFeed",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ASSET_SERVICE_GET_FEED: ::grpcio::Method<super::asset_service::GetFeedRequest, super::asset_service::Feed> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.asset.v1.AssetService/GetFeed",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ASSET_SERVICE_LIST_FEEDS: ::grpcio::Method<super::asset_service::ListFeedsRequest, super::asset_service::ListFeedsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.asset.v1.AssetService/ListFeeds",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ASSET_SERVICE_UPDATE_FEED: ::grpcio::Method<super::asset_service::UpdateFeedRequest, super::asset_service::Feed> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.asset.v1.AssetService/UpdateFeed",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ASSET_SERVICE_DELETE_FEED: ::grpcio::Method<super::asset_service::DeleteFeedRequest, super::empty::Empty> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.asset.v1.AssetService/DeleteFeed",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ASSET_SERVICE_SEARCH_ALL_RESOURCES: ::grpcio::Method<super::asset_service::SearchAllResourcesRequest, super::asset_service::SearchAllResourcesResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.asset.v1.AssetService/SearchAllResources",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ASSET_SERVICE_SEARCH_ALL_IAM_POLICIES: ::grpcio::Method<super::asset_service::SearchAllIamPoliciesRequest, super::asset_service::SearchAllIamPoliciesResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.asset.v1.AssetService/SearchAllIamPolicies",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ASSET_SERVICE_ANALYZE_IAM_POLICY: ::grpcio::Method<super::asset_service::AnalyzeIamPolicyRequest, super::asset_service::AnalyzeIamPolicyResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.asset.v1.AssetService/AnalyzeIamPolicy",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ASSET_SERVICE_ANALYZE_IAM_POLICY_LONGRUNNING: ::grpcio::Method<super::asset_service::AnalyzeIamPolicyLongrunningRequest, super::operations::Operation> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.asset.v1.AssetService/AnalyzeIamPolicyLongrunning",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ASSET_SERVICE_ANALYZE_MOVE: ::grpcio::Method<super::asset_service::AnalyzeMoveRequest, super::asset_service::AnalyzeMoveResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.asset.v1.AssetService/AnalyzeMove",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ASSET_SERVICE_QUERY_ASSETS: ::grpcio::Method<super::asset_service::QueryAssetsRequest, super::asset_service::QueryAssetsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.asset.v1.AssetService/QueryAssets",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ASSET_SERVICE_CREATE_SAVED_QUERY: ::grpcio::Method<super::asset_service::CreateSavedQueryRequest, super::asset_service::SavedQuery> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.asset.v1.AssetService/CreateSavedQuery",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ASSET_SERVICE_GET_SAVED_QUERY: ::grpcio::Method<super::asset_service::GetSavedQueryRequest, super::asset_service::SavedQuery> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.asset.v1.AssetService/GetSavedQuery",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ASSET_SERVICE_LIST_SAVED_QUERIES: ::grpcio::Method<super::asset_service::ListSavedQueriesRequest, super::asset_service::ListSavedQueriesResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.asset.v1.AssetService/ListSavedQueries",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ASSET_SERVICE_UPDATE_SAVED_QUERY: ::grpcio::Method<super::asset_service::UpdateSavedQueryRequest, super::asset_service::SavedQuery> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.asset.v1.AssetService/UpdateSavedQuery",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ASSET_SERVICE_DELETE_SAVED_QUERY: ::grpcio::Method<super::asset_service::DeleteSavedQueryRequest, super::empty::Empty> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.asset.v1.AssetService/DeleteSavedQuery",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ASSET_SERVICE_BATCH_GET_EFFECTIVE_IAM_POLICIES: ::grpcio::Method<super::asset_service::BatchGetEffectiveIamPoliciesRequest, super::asset_service::BatchGetEffectiveIamPoliciesResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.asset.v1.AssetService/BatchGetEffectiveIamPolicies",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ASSET_SERVICE_ANALYZE_ORG_POLICIES: ::grpcio::Method<super::asset_service::AnalyzeOrgPoliciesRequest, super::asset_service::AnalyzeOrgPoliciesResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.asset.v1.AssetService/AnalyzeOrgPolicies",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ASSET_SERVICE_ANALYZE_ORG_POLICY_GOVERNED_CONTAINERS: ::grpcio::Method<super::asset_service::AnalyzeOrgPolicyGovernedContainersRequest, super::asset_service::AnalyzeOrgPolicyGovernedContainersResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.asset.v1.AssetService/AnalyzeOrgPolicyGovernedContainers",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ASSET_SERVICE_ANALYZE_ORG_POLICY_GOVERNED_ASSETS: ::grpcio::Method<super::asset_service::AnalyzeOrgPolicyGovernedAssetsRequest, super::asset_service::AnalyzeOrgPolicyGovernedAssetsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.asset.v1.AssetService/AnalyzeOrgPolicyGovernedAssets",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct AssetServiceClient {
    pub client: ::grpcio::Client,
}

impl AssetServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        AssetServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn export_assets_opt(&self, req: &super::asset_service::ExportAssetsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::operations::Operation> {
        self.client.unary_call(&METHOD_ASSET_SERVICE_EXPORT_ASSETS, req, opt)
    }

    pub fn export_assets(&self, req: &super::asset_service::ExportAssetsRequest) -> ::grpcio::Result<super::operations::Operation> {
        self.export_assets_opt(req, ::grpcio::CallOption::default())
    }

    pub fn export_assets_async_opt(&self, req: &super::asset_service::ExportAssetsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operations::Operation>> {
        self.client.unary_call_async(&METHOD_ASSET_SERVICE_EXPORT_ASSETS, req, opt)
    }

    pub fn export_assets_async(&self, req: &super::asset_service::ExportAssetsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operations::Operation>> {
        self.export_assets_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_assets_opt(&self, req: &super::asset_service::ListAssetsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::asset_service::ListAssetsResponse> {
        self.client.unary_call(&METHOD_ASSET_SERVICE_LIST_ASSETS, req, opt)
    }

    pub fn list_assets(&self, req: &super::asset_service::ListAssetsRequest) -> ::grpcio::Result<super::asset_service::ListAssetsResponse> {
        self.list_assets_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_assets_async_opt(&self, req: &super::asset_service::ListAssetsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::asset_service::ListAssetsResponse>> {
        self.client.unary_call_async(&METHOD_ASSET_SERVICE_LIST_ASSETS, req, opt)
    }

    pub fn list_assets_async(&self, req: &super::asset_service::ListAssetsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::asset_service::ListAssetsResponse>> {
        self.list_assets_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn batch_get_assets_history_opt(&self, req: &super::asset_service::BatchGetAssetsHistoryRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::asset_service::BatchGetAssetsHistoryResponse> {
        self.client.unary_call(&METHOD_ASSET_SERVICE_BATCH_GET_ASSETS_HISTORY, req, opt)
    }

    pub fn batch_get_assets_history(&self, req: &super::asset_service::BatchGetAssetsHistoryRequest) -> ::grpcio::Result<super::asset_service::BatchGetAssetsHistoryResponse> {
        self.batch_get_assets_history_opt(req, ::grpcio::CallOption::default())
    }

    pub fn batch_get_assets_history_async_opt(&self, req: &super::asset_service::BatchGetAssetsHistoryRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::asset_service::BatchGetAssetsHistoryResponse>> {
        self.client.unary_call_async(&METHOD_ASSET_SERVICE_BATCH_GET_ASSETS_HISTORY, req, opt)
    }

    pub fn batch_get_assets_history_async(&self, req: &super::asset_service::BatchGetAssetsHistoryRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::asset_service::BatchGetAssetsHistoryResponse>> {
        self.batch_get_assets_history_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_feed_opt(&self, req: &super::asset_service::CreateFeedRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::asset_service::Feed> {
        self.client.unary_call(&METHOD_ASSET_SERVICE_CREATE_FEED, req, opt)
    }

    pub fn create_feed(&self, req: &super::asset_service::CreateFeedRequest) -> ::grpcio::Result<super::asset_service::Feed> {
        self.create_feed_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_feed_async_opt(&self, req: &super::asset_service::CreateFeedRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::asset_service::Feed>> {
        self.client.unary_call_async(&METHOD_ASSET_SERVICE_CREATE_FEED, req, opt)
    }

    pub fn create_feed_async(&self, req: &super::asset_service::CreateFeedRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::asset_service::Feed>> {
        self.create_feed_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_feed_opt(&self, req: &super::asset_service::GetFeedRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::asset_service::Feed> {
        self.client.unary_call(&METHOD_ASSET_SERVICE_GET_FEED, req, opt)
    }

    pub fn get_feed(&self, req: &super::asset_service::GetFeedRequest) -> ::grpcio::Result<super::asset_service::Feed> {
        self.get_feed_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_feed_async_opt(&self, req: &super::asset_service::GetFeedRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::asset_service::Feed>> {
        self.client.unary_call_async(&METHOD_ASSET_SERVICE_GET_FEED, req, opt)
    }

    pub fn get_feed_async(&self, req: &super::asset_service::GetFeedRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::asset_service::Feed>> {
        self.get_feed_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_feeds_opt(&self, req: &super::asset_service::ListFeedsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::asset_service::ListFeedsResponse> {
        self.client.unary_call(&METHOD_ASSET_SERVICE_LIST_FEEDS, req, opt)
    }

    pub fn list_feeds(&self, req: &super::asset_service::ListFeedsRequest) -> ::grpcio::Result<super::asset_service::ListFeedsResponse> {
        self.list_feeds_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_feeds_async_opt(&self, req: &super::asset_service::ListFeedsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::asset_service::ListFeedsResponse>> {
        self.client.unary_call_async(&METHOD_ASSET_SERVICE_LIST_FEEDS, req, opt)
    }

    pub fn list_feeds_async(&self, req: &super::asset_service::ListFeedsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::asset_service::ListFeedsResponse>> {
        self.list_feeds_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_feed_opt(&self, req: &super::asset_service::UpdateFeedRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::asset_service::Feed> {
        self.client.unary_call(&METHOD_ASSET_SERVICE_UPDATE_FEED, req, opt)
    }

    pub fn update_feed(&self, req: &super::asset_service::UpdateFeedRequest) -> ::grpcio::Result<super::asset_service::Feed> {
        self.update_feed_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_feed_async_opt(&self, req: &super::asset_service::UpdateFeedRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::asset_service::Feed>> {
        self.client.unary_call_async(&METHOD_ASSET_SERVICE_UPDATE_FEED, req, opt)
    }

    pub fn update_feed_async(&self, req: &super::asset_service::UpdateFeedRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::asset_service::Feed>> {
        self.update_feed_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_feed_opt(&self, req: &super::asset_service::DeleteFeedRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::empty::Empty> {
        self.client.unary_call(&METHOD_ASSET_SERVICE_DELETE_FEED, req, opt)
    }

    pub fn delete_feed(&self, req: &super::asset_service::DeleteFeedRequest) -> ::grpcio::Result<super::empty::Empty> {
        self.delete_feed_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_feed_async_opt(&self, req: &super::asset_service::DeleteFeedRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.client.unary_call_async(&METHOD_ASSET_SERVICE_DELETE_FEED, req, opt)
    }

    pub fn delete_feed_async(&self, req: &super::asset_service::DeleteFeedRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.delete_feed_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn search_all_resources_opt(&self, req: &super::asset_service::SearchAllResourcesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::asset_service::SearchAllResourcesResponse> {
        self.client.unary_call(&METHOD_ASSET_SERVICE_SEARCH_ALL_RESOURCES, req, opt)
    }

    pub fn search_all_resources(&self, req: &super::asset_service::SearchAllResourcesRequest) -> ::grpcio::Result<super::asset_service::SearchAllResourcesResponse> {
        self.search_all_resources_opt(req, ::grpcio::CallOption::default())
    }

    pub fn search_all_resources_async_opt(&self, req: &super::asset_service::SearchAllResourcesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::asset_service::SearchAllResourcesResponse>> {
        self.client.unary_call_async(&METHOD_ASSET_SERVICE_SEARCH_ALL_RESOURCES, req, opt)
    }

    pub fn search_all_resources_async(&self, req: &super::asset_service::SearchAllResourcesRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::asset_service::SearchAllResourcesResponse>> {
        self.search_all_resources_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn search_all_iam_policies_opt(&self, req: &super::asset_service::SearchAllIamPoliciesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::asset_service::SearchAllIamPoliciesResponse> {
        self.client.unary_call(&METHOD_ASSET_SERVICE_SEARCH_ALL_IAM_POLICIES, req, opt)
    }

    pub fn search_all_iam_policies(&self, req: &super::asset_service::SearchAllIamPoliciesRequest) -> ::grpcio::Result<super::asset_service::SearchAllIamPoliciesResponse> {
        self.search_all_iam_policies_opt(req, ::grpcio::CallOption::default())
    }

    pub fn search_all_iam_policies_async_opt(&self, req: &super::asset_service::SearchAllIamPoliciesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::asset_service::SearchAllIamPoliciesResponse>> {
        self.client.unary_call_async(&METHOD_ASSET_SERVICE_SEARCH_ALL_IAM_POLICIES, req, opt)
    }

    pub fn search_all_iam_policies_async(&self, req: &super::asset_service::SearchAllIamPoliciesRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::asset_service::SearchAllIamPoliciesResponse>> {
        self.search_all_iam_policies_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn analyze_iam_policy_opt(&self, req: &super::asset_service::AnalyzeIamPolicyRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::asset_service::AnalyzeIamPolicyResponse> {
        self.client.unary_call(&METHOD_ASSET_SERVICE_ANALYZE_IAM_POLICY, req, opt)
    }

    pub fn analyze_iam_policy(&self, req: &super::asset_service::AnalyzeIamPolicyRequest) -> ::grpcio::Result<super::asset_service::AnalyzeIamPolicyResponse> {
        self.analyze_iam_policy_opt(req, ::grpcio::CallOption::default())
    }

    pub fn analyze_iam_policy_async_opt(&self, req: &super::asset_service::AnalyzeIamPolicyRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::asset_service::AnalyzeIamPolicyResponse>> {
        self.client.unary_call_async(&METHOD_ASSET_SERVICE_ANALYZE_IAM_POLICY, req, opt)
    }

    pub fn analyze_iam_policy_async(&self, req: &super::asset_service::AnalyzeIamPolicyRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::asset_service::AnalyzeIamPolicyResponse>> {
        self.analyze_iam_policy_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn analyze_iam_policy_longrunning_opt(&self, req: &super::asset_service::AnalyzeIamPolicyLongrunningRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::operations::Operation> {
        self.client.unary_call(&METHOD_ASSET_SERVICE_ANALYZE_IAM_POLICY_LONGRUNNING, req, opt)
    }

    pub fn analyze_iam_policy_longrunning(&self, req: &super::asset_service::AnalyzeIamPolicyLongrunningRequest) -> ::grpcio::Result<super::operations::Operation> {
        self.analyze_iam_policy_longrunning_opt(req, ::grpcio::CallOption::default())
    }

    pub fn analyze_iam_policy_longrunning_async_opt(&self, req: &super::asset_service::AnalyzeIamPolicyLongrunningRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operations::Operation>> {
        self.client.unary_call_async(&METHOD_ASSET_SERVICE_ANALYZE_IAM_POLICY_LONGRUNNING, req, opt)
    }

    pub fn analyze_iam_policy_longrunning_async(&self, req: &super::asset_service::AnalyzeIamPolicyLongrunningRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operations::Operation>> {
        self.analyze_iam_policy_longrunning_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn analyze_move_opt(&self, req: &super::asset_service::AnalyzeMoveRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::asset_service::AnalyzeMoveResponse> {
        self.client.unary_call(&METHOD_ASSET_SERVICE_ANALYZE_MOVE, req, opt)
    }

    pub fn analyze_move(&self, req: &super::asset_service::AnalyzeMoveRequest) -> ::grpcio::Result<super::asset_service::AnalyzeMoveResponse> {
        self.analyze_move_opt(req, ::grpcio::CallOption::default())
    }

    pub fn analyze_move_async_opt(&self, req: &super::asset_service::AnalyzeMoveRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::asset_service::AnalyzeMoveResponse>> {
        self.client.unary_call_async(&METHOD_ASSET_SERVICE_ANALYZE_MOVE, req, opt)
    }

    pub fn analyze_move_async(&self, req: &super::asset_service::AnalyzeMoveRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::asset_service::AnalyzeMoveResponse>> {
        self.analyze_move_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn query_assets_opt(&self, req: &super::asset_service::QueryAssetsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::asset_service::QueryAssetsResponse> {
        self.client.unary_call(&METHOD_ASSET_SERVICE_QUERY_ASSETS, req, opt)
    }

    pub fn query_assets(&self, req: &super::asset_service::QueryAssetsRequest) -> ::grpcio::Result<super::asset_service::QueryAssetsResponse> {
        self.query_assets_opt(req, ::grpcio::CallOption::default())
    }

    pub fn query_assets_async_opt(&self, req: &super::asset_service::QueryAssetsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::asset_service::QueryAssetsResponse>> {
        self.client.unary_call_async(&METHOD_ASSET_SERVICE_QUERY_ASSETS, req, opt)
    }

    pub fn query_assets_async(&self, req: &super::asset_service::QueryAssetsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::asset_service::QueryAssetsResponse>> {
        self.query_assets_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_saved_query_opt(&self, req: &super::asset_service::CreateSavedQueryRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::asset_service::SavedQuery> {
        self.client.unary_call(&METHOD_ASSET_SERVICE_CREATE_SAVED_QUERY, req, opt)
    }

    pub fn create_saved_query(&self, req: &super::asset_service::CreateSavedQueryRequest) -> ::grpcio::Result<super::asset_service::SavedQuery> {
        self.create_saved_query_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_saved_query_async_opt(&self, req: &super::asset_service::CreateSavedQueryRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::asset_service::SavedQuery>> {
        self.client.unary_call_async(&METHOD_ASSET_SERVICE_CREATE_SAVED_QUERY, req, opt)
    }

    pub fn create_saved_query_async(&self, req: &super::asset_service::CreateSavedQueryRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::asset_service::SavedQuery>> {
        self.create_saved_query_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_saved_query_opt(&self, req: &super::asset_service::GetSavedQueryRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::asset_service::SavedQuery> {
        self.client.unary_call(&METHOD_ASSET_SERVICE_GET_SAVED_QUERY, req, opt)
    }

    pub fn get_saved_query(&self, req: &super::asset_service::GetSavedQueryRequest) -> ::grpcio::Result<super::asset_service::SavedQuery> {
        self.get_saved_query_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_saved_query_async_opt(&self, req: &super::asset_service::GetSavedQueryRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::asset_service::SavedQuery>> {
        self.client.unary_call_async(&METHOD_ASSET_SERVICE_GET_SAVED_QUERY, req, opt)
    }

    pub fn get_saved_query_async(&self, req: &super::asset_service::GetSavedQueryRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::asset_service::SavedQuery>> {
        self.get_saved_query_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_saved_queries_opt(&self, req: &super::asset_service::ListSavedQueriesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::asset_service::ListSavedQueriesResponse> {
        self.client.unary_call(&METHOD_ASSET_SERVICE_LIST_SAVED_QUERIES, req, opt)
    }

    pub fn list_saved_queries(&self, req: &super::asset_service::ListSavedQueriesRequest) -> ::grpcio::Result<super::asset_service::ListSavedQueriesResponse> {
        self.list_saved_queries_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_saved_queries_async_opt(&self, req: &super::asset_service::ListSavedQueriesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::asset_service::ListSavedQueriesResponse>> {
        self.client.unary_call_async(&METHOD_ASSET_SERVICE_LIST_SAVED_QUERIES, req, opt)
    }

    pub fn list_saved_queries_async(&self, req: &super::asset_service::ListSavedQueriesRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::asset_service::ListSavedQueriesResponse>> {
        self.list_saved_queries_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_saved_query_opt(&self, req: &super::asset_service::UpdateSavedQueryRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::asset_service::SavedQuery> {
        self.client.unary_call(&METHOD_ASSET_SERVICE_UPDATE_SAVED_QUERY, req, opt)
    }

    pub fn update_saved_query(&self, req: &super::asset_service::UpdateSavedQueryRequest) -> ::grpcio::Result<super::asset_service::SavedQuery> {
        self.update_saved_query_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_saved_query_async_opt(&self, req: &super::asset_service::UpdateSavedQueryRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::asset_service::SavedQuery>> {
        self.client.unary_call_async(&METHOD_ASSET_SERVICE_UPDATE_SAVED_QUERY, req, opt)
    }

    pub fn update_saved_query_async(&self, req: &super::asset_service::UpdateSavedQueryRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::asset_service::SavedQuery>> {
        self.update_saved_query_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_saved_query_opt(&self, req: &super::asset_service::DeleteSavedQueryRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::empty::Empty> {
        self.client.unary_call(&METHOD_ASSET_SERVICE_DELETE_SAVED_QUERY, req, opt)
    }

    pub fn delete_saved_query(&self, req: &super::asset_service::DeleteSavedQueryRequest) -> ::grpcio::Result<super::empty::Empty> {
        self.delete_saved_query_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_saved_query_async_opt(&self, req: &super::asset_service::DeleteSavedQueryRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.client.unary_call_async(&METHOD_ASSET_SERVICE_DELETE_SAVED_QUERY, req, opt)
    }

    pub fn delete_saved_query_async(&self, req: &super::asset_service::DeleteSavedQueryRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.delete_saved_query_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn batch_get_effective_iam_policies_opt(&self, req: &super::asset_service::BatchGetEffectiveIamPoliciesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::asset_service::BatchGetEffectiveIamPoliciesResponse> {
        self.client.unary_call(&METHOD_ASSET_SERVICE_BATCH_GET_EFFECTIVE_IAM_POLICIES, req, opt)
    }

    pub fn batch_get_effective_iam_policies(&self, req: &super::asset_service::BatchGetEffectiveIamPoliciesRequest) -> ::grpcio::Result<super::asset_service::BatchGetEffectiveIamPoliciesResponse> {
        self.batch_get_effective_iam_policies_opt(req, ::grpcio::CallOption::default())
    }

    pub fn batch_get_effective_iam_policies_async_opt(&self, req: &super::asset_service::BatchGetEffectiveIamPoliciesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::asset_service::BatchGetEffectiveIamPoliciesResponse>> {
        self.client.unary_call_async(&METHOD_ASSET_SERVICE_BATCH_GET_EFFECTIVE_IAM_POLICIES, req, opt)
    }

    pub fn batch_get_effective_iam_policies_async(&self, req: &super::asset_service::BatchGetEffectiveIamPoliciesRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::asset_service::BatchGetEffectiveIamPoliciesResponse>> {
        self.batch_get_effective_iam_policies_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn analyze_org_policies_opt(&self, req: &super::asset_service::AnalyzeOrgPoliciesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::asset_service::AnalyzeOrgPoliciesResponse> {
        self.client.unary_call(&METHOD_ASSET_SERVICE_ANALYZE_ORG_POLICIES, req, opt)
    }

    pub fn analyze_org_policies(&self, req: &super::asset_service::AnalyzeOrgPoliciesRequest) -> ::grpcio::Result<super::asset_service::AnalyzeOrgPoliciesResponse> {
        self.analyze_org_policies_opt(req, ::grpcio::CallOption::default())
    }

    pub fn analyze_org_policies_async_opt(&self, req: &super::asset_service::AnalyzeOrgPoliciesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::asset_service::AnalyzeOrgPoliciesResponse>> {
        self.client.unary_call_async(&METHOD_ASSET_SERVICE_ANALYZE_ORG_POLICIES, req, opt)
    }

    pub fn analyze_org_policies_async(&self, req: &super::asset_service::AnalyzeOrgPoliciesRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::asset_service::AnalyzeOrgPoliciesResponse>> {
        self.analyze_org_policies_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn analyze_org_policy_governed_containers_opt(&self, req: &super::asset_service::AnalyzeOrgPolicyGovernedContainersRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::asset_service::AnalyzeOrgPolicyGovernedContainersResponse> {
        self.client.unary_call(&METHOD_ASSET_SERVICE_ANALYZE_ORG_POLICY_GOVERNED_CONTAINERS, req, opt)
    }

    pub fn analyze_org_policy_governed_containers(&self, req: &super::asset_service::AnalyzeOrgPolicyGovernedContainersRequest) -> ::grpcio::Result<super::asset_service::AnalyzeOrgPolicyGovernedContainersResponse> {
        self.analyze_org_policy_governed_containers_opt(req, ::grpcio::CallOption::default())
    }

    pub fn analyze_org_policy_governed_containers_async_opt(&self, req: &super::asset_service::AnalyzeOrgPolicyGovernedContainersRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::asset_service::AnalyzeOrgPolicyGovernedContainersResponse>> {
        self.client.unary_call_async(&METHOD_ASSET_SERVICE_ANALYZE_ORG_POLICY_GOVERNED_CONTAINERS, req, opt)
    }

    pub fn analyze_org_policy_governed_containers_async(&self, req: &super::asset_service::AnalyzeOrgPolicyGovernedContainersRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::asset_service::AnalyzeOrgPolicyGovernedContainersResponse>> {
        self.analyze_org_policy_governed_containers_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn analyze_org_policy_governed_assets_opt(&self, req: &super::asset_service::AnalyzeOrgPolicyGovernedAssetsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::asset_service::AnalyzeOrgPolicyGovernedAssetsResponse> {
        self.client.unary_call(&METHOD_ASSET_SERVICE_ANALYZE_ORG_POLICY_GOVERNED_ASSETS, req, opt)
    }

    pub fn analyze_org_policy_governed_assets(&self, req: &super::asset_service::AnalyzeOrgPolicyGovernedAssetsRequest) -> ::grpcio::Result<super::asset_service::AnalyzeOrgPolicyGovernedAssetsResponse> {
        self.analyze_org_policy_governed_assets_opt(req, ::grpcio::CallOption::default())
    }

    pub fn analyze_org_policy_governed_assets_async_opt(&self, req: &super::asset_service::AnalyzeOrgPolicyGovernedAssetsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::asset_service::AnalyzeOrgPolicyGovernedAssetsResponse>> {
        self.client.unary_call_async(&METHOD_ASSET_SERVICE_ANALYZE_ORG_POLICY_GOVERNED_ASSETS, req, opt)
    }

    pub fn analyze_org_policy_governed_assets_async(&self, req: &super::asset_service::AnalyzeOrgPolicyGovernedAssetsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::asset_service::AnalyzeOrgPolicyGovernedAssetsResponse>> {
        self.analyze_org_policy_governed_assets_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::std::future::Future<Output = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait AssetService {
    fn export_assets(&mut self, ctx: ::grpcio::RpcContext, _req: super::asset_service::ExportAssetsRequest, sink: ::grpcio::UnarySink<super::operations::Operation>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_assets(&mut self, ctx: ::grpcio::RpcContext, _req: super::asset_service::ListAssetsRequest, sink: ::grpcio::UnarySink<super::asset_service::ListAssetsResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn batch_get_assets_history(&mut self, ctx: ::grpcio::RpcContext, _req: super::asset_service::BatchGetAssetsHistoryRequest, sink: ::grpcio::UnarySink<super::asset_service::BatchGetAssetsHistoryResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn create_feed(&mut self, ctx: ::grpcio::RpcContext, _req: super::asset_service::CreateFeedRequest, sink: ::grpcio::UnarySink<super::asset_service::Feed>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_feed(&mut self, ctx: ::grpcio::RpcContext, _req: super::asset_service::GetFeedRequest, sink: ::grpcio::UnarySink<super::asset_service::Feed>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_feeds(&mut self, ctx: ::grpcio::RpcContext, _req: super::asset_service::ListFeedsRequest, sink: ::grpcio::UnarySink<super::asset_service::ListFeedsResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn update_feed(&mut self, ctx: ::grpcio::RpcContext, _req: super::asset_service::UpdateFeedRequest, sink: ::grpcio::UnarySink<super::asset_service::Feed>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn delete_feed(&mut self, ctx: ::grpcio::RpcContext, _req: super::asset_service::DeleteFeedRequest, sink: ::grpcio::UnarySink<super::empty::Empty>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn search_all_resources(&mut self, ctx: ::grpcio::RpcContext, _req: super::asset_service::SearchAllResourcesRequest, sink: ::grpcio::UnarySink<super::asset_service::SearchAllResourcesResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn search_all_iam_policies(&mut self, ctx: ::grpcio::RpcContext, _req: super::asset_service::SearchAllIamPoliciesRequest, sink: ::grpcio::UnarySink<super::asset_service::SearchAllIamPoliciesResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn analyze_iam_policy(&mut self, ctx: ::grpcio::RpcContext, _req: super::asset_service::AnalyzeIamPolicyRequest, sink: ::grpcio::UnarySink<super::asset_service::AnalyzeIamPolicyResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn analyze_iam_policy_longrunning(&mut self, ctx: ::grpcio::RpcContext, _req: super::asset_service::AnalyzeIamPolicyLongrunningRequest, sink: ::grpcio::UnarySink<super::operations::Operation>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn analyze_move(&mut self, ctx: ::grpcio::RpcContext, _req: super::asset_service::AnalyzeMoveRequest, sink: ::grpcio::UnarySink<super::asset_service::AnalyzeMoveResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn query_assets(&mut self, ctx: ::grpcio::RpcContext, _req: super::asset_service::QueryAssetsRequest, sink: ::grpcio::UnarySink<super::asset_service::QueryAssetsResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn create_saved_query(&mut self, ctx: ::grpcio::RpcContext, _req: super::asset_service::CreateSavedQueryRequest, sink: ::grpcio::UnarySink<super::asset_service::SavedQuery>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_saved_query(&mut self, ctx: ::grpcio::RpcContext, _req: super::asset_service::GetSavedQueryRequest, sink: ::grpcio::UnarySink<super::asset_service::SavedQuery>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_saved_queries(&mut self, ctx: ::grpcio::RpcContext, _req: super::asset_service::ListSavedQueriesRequest, sink: ::grpcio::UnarySink<super::asset_service::ListSavedQueriesResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn update_saved_query(&mut self, ctx: ::grpcio::RpcContext, _req: super::asset_service::UpdateSavedQueryRequest, sink: ::grpcio::UnarySink<super::asset_service::SavedQuery>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn delete_saved_query(&mut self, ctx: ::grpcio::RpcContext, _req: super::asset_service::DeleteSavedQueryRequest, sink: ::grpcio::UnarySink<super::empty::Empty>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn batch_get_effective_iam_policies(&mut self, ctx: ::grpcio::RpcContext, _req: super::asset_service::BatchGetEffectiveIamPoliciesRequest, sink: ::grpcio::UnarySink<super::asset_service::BatchGetEffectiveIamPoliciesResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn analyze_org_policies(&mut self, ctx: ::grpcio::RpcContext, _req: super::asset_service::AnalyzeOrgPoliciesRequest, sink: ::grpcio::UnarySink<super::asset_service::AnalyzeOrgPoliciesResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn analyze_org_policy_governed_containers(&mut self, ctx: ::grpcio::RpcContext, _req: super::asset_service::AnalyzeOrgPolicyGovernedContainersRequest, sink: ::grpcio::UnarySink<super::asset_service::AnalyzeOrgPolicyGovernedContainersResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn analyze_org_policy_governed_assets(&mut self, ctx: ::grpcio::RpcContext, _req: super::asset_service::AnalyzeOrgPolicyGovernedAssetsRequest, sink: ::grpcio::UnarySink<super::asset_service::AnalyzeOrgPolicyGovernedAssetsResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
}

pub fn create_asset_service<S: AssetService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ASSET_SERVICE_EXPORT_ASSETS, move |ctx, req, resp| {
        instance.export_assets(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ASSET_SERVICE_LIST_ASSETS, move |ctx, req, resp| {
        instance.list_assets(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ASSET_SERVICE_BATCH_GET_ASSETS_HISTORY, move |ctx, req, resp| {
        instance.batch_get_assets_history(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ASSET_SERVICE_CREATE_FEED, move |ctx, req, resp| {
        instance.create_feed(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ASSET_SERVICE_GET_FEED, move |ctx, req, resp| {
        instance.get_feed(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ASSET_SERVICE_LIST_FEEDS, move |ctx, req, resp| {
        instance.list_feeds(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ASSET_SERVICE_UPDATE_FEED, move |ctx, req, resp| {
        instance.update_feed(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ASSET_SERVICE_DELETE_FEED, move |ctx, req, resp| {
        instance.delete_feed(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ASSET_SERVICE_SEARCH_ALL_RESOURCES, move |ctx, req, resp| {
        instance.search_all_resources(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ASSET_SERVICE_SEARCH_ALL_IAM_POLICIES, move |ctx, req, resp| {
        instance.search_all_iam_policies(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ASSET_SERVICE_ANALYZE_IAM_POLICY, move |ctx, req, resp| {
        instance.analyze_iam_policy(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ASSET_SERVICE_ANALYZE_IAM_POLICY_LONGRUNNING, move |ctx, req, resp| {
        instance.analyze_iam_policy_longrunning(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ASSET_SERVICE_ANALYZE_MOVE, move |ctx, req, resp| {
        instance.analyze_move(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ASSET_SERVICE_QUERY_ASSETS, move |ctx, req, resp| {
        instance.query_assets(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ASSET_SERVICE_CREATE_SAVED_QUERY, move |ctx, req, resp| {
        instance.create_saved_query(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ASSET_SERVICE_GET_SAVED_QUERY, move |ctx, req, resp| {
        instance.get_saved_query(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ASSET_SERVICE_LIST_SAVED_QUERIES, move |ctx, req, resp| {
        instance.list_saved_queries(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ASSET_SERVICE_UPDATE_SAVED_QUERY, move |ctx, req, resp| {
        instance.update_saved_query(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ASSET_SERVICE_DELETE_SAVED_QUERY, move |ctx, req, resp| {
        instance.delete_saved_query(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ASSET_SERVICE_BATCH_GET_EFFECTIVE_IAM_POLICIES, move |ctx, req, resp| {
        instance.batch_get_effective_iam_policies(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ASSET_SERVICE_ANALYZE_ORG_POLICIES, move |ctx, req, resp| {
        instance.analyze_org_policies(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ASSET_SERVICE_ANALYZE_ORG_POLICY_GOVERNED_CONTAINERS, move |ctx, req, resp| {
        instance.analyze_org_policy_governed_containers(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_ASSET_SERVICE_ANALYZE_ORG_POLICY_GOVERNED_ASSETS, move |ctx, req, resp| {
        instance.analyze_org_policy_governed_assets(ctx, req, resp)
    });
    builder.build()
}
