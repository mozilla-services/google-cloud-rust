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

const METHOD_ASSET_SERVICE_BATCH_GET_ASSETS_HISTORY: ::grpcio::Method<super::asset_service::BatchGetAssetsHistoryRequest, super::asset_service::BatchGetAssetsHistoryResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.asset.v1.AssetService/BatchGetAssetsHistory",
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
    pub fn spawn<F>(&self, f: F) where F: ::std::future::Future<Output = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait AssetService {
    fn export_assets(&mut self, ctx: ::grpcio::RpcContext, _req: super::asset_service::ExportAssetsRequest, sink: ::grpcio::UnarySink<super::operations::Operation>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn batch_get_assets_history(&mut self, ctx: ::grpcio::RpcContext, _req: super::asset_service::BatchGetAssetsHistoryRequest, sink: ::grpcio::UnarySink<super::asset_service::BatchGetAssetsHistoryResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
}

pub fn create_asset_service<S: AssetService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ASSET_SERVICE_EXPORT_ASSETS, move |ctx, req, resp| {
        instance.export_assets(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_ASSET_SERVICE_BATCH_GET_ASSETS_HISTORY, move |ctx, req, resp| {
        instance.batch_get_assets_history(ctx, req, resp)
    });
    builder.build()
}
