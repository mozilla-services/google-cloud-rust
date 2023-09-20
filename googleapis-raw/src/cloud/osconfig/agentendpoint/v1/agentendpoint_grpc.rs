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

const METHOD_AGENT_ENDPOINT_SERVICE_RECEIVE_TASK_NOTIFICATION: ::grpcio::Method<super::agentendpoint::ReceiveTaskNotificationRequest, super::agentendpoint::ReceiveTaskNotificationResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/google.cloud.osconfig.agentendpoint.v1.AgentEndpointService/ReceiveTaskNotification",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_AGENT_ENDPOINT_SERVICE_START_NEXT_TASK: ::grpcio::Method<super::agentendpoint::StartNextTaskRequest, super::agentendpoint::StartNextTaskResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.osconfig.agentendpoint.v1.AgentEndpointService/StartNextTask",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_AGENT_ENDPOINT_SERVICE_REPORT_TASK_PROGRESS: ::grpcio::Method<super::agentendpoint::ReportTaskProgressRequest, super::agentendpoint::ReportTaskProgressResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.osconfig.agentendpoint.v1.AgentEndpointService/ReportTaskProgress",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_AGENT_ENDPOINT_SERVICE_REPORT_TASK_COMPLETE: ::grpcio::Method<super::agentendpoint::ReportTaskCompleteRequest, super::agentendpoint::ReportTaskCompleteResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.osconfig.agentendpoint.v1.AgentEndpointService/ReportTaskComplete",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_AGENT_ENDPOINT_SERVICE_REGISTER_AGENT: ::grpcio::Method<super::agentendpoint::RegisterAgentRequest, super::agentendpoint::RegisterAgentResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.osconfig.agentendpoint.v1.AgentEndpointService/RegisterAgent",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_AGENT_ENDPOINT_SERVICE_REPORT_INVENTORY: ::grpcio::Method<super::agentendpoint::ReportInventoryRequest, super::agentendpoint::ReportInventoryResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.osconfig.agentendpoint.v1.AgentEndpointService/ReportInventory",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct AgentEndpointServiceClient {
    pub client: ::grpcio::Client,
}

impl AgentEndpointServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        AgentEndpointServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn receive_task_notification_opt(&self, req: &super::agentendpoint::ReceiveTaskNotificationRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::agentendpoint::ReceiveTaskNotificationResponse>> {
        self.client.server_streaming(&METHOD_AGENT_ENDPOINT_SERVICE_RECEIVE_TASK_NOTIFICATION, req, opt)
    }

    pub fn receive_task_notification(&self, req: &super::agentendpoint::ReceiveTaskNotificationRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::agentendpoint::ReceiveTaskNotificationResponse>> {
        self.receive_task_notification_opt(req, ::grpcio::CallOption::default())
    }

    pub fn start_next_task_opt(&self, req: &super::agentendpoint::StartNextTaskRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::agentendpoint::StartNextTaskResponse> {
        self.client.unary_call(&METHOD_AGENT_ENDPOINT_SERVICE_START_NEXT_TASK, req, opt)
    }

    pub fn start_next_task(&self, req: &super::agentendpoint::StartNextTaskRequest) -> ::grpcio::Result<super::agentendpoint::StartNextTaskResponse> {
        self.start_next_task_opt(req, ::grpcio::CallOption::default())
    }

    pub fn start_next_task_async_opt(&self, req: &super::agentendpoint::StartNextTaskRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::agentendpoint::StartNextTaskResponse>> {
        self.client.unary_call_async(&METHOD_AGENT_ENDPOINT_SERVICE_START_NEXT_TASK, req, opt)
    }

    pub fn start_next_task_async(&self, req: &super::agentendpoint::StartNextTaskRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::agentendpoint::StartNextTaskResponse>> {
        self.start_next_task_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn report_task_progress_opt(&self, req: &super::agentendpoint::ReportTaskProgressRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::agentendpoint::ReportTaskProgressResponse> {
        self.client.unary_call(&METHOD_AGENT_ENDPOINT_SERVICE_REPORT_TASK_PROGRESS, req, opt)
    }

    pub fn report_task_progress(&self, req: &super::agentendpoint::ReportTaskProgressRequest) -> ::grpcio::Result<super::agentendpoint::ReportTaskProgressResponse> {
        self.report_task_progress_opt(req, ::grpcio::CallOption::default())
    }

    pub fn report_task_progress_async_opt(&self, req: &super::agentendpoint::ReportTaskProgressRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::agentendpoint::ReportTaskProgressResponse>> {
        self.client.unary_call_async(&METHOD_AGENT_ENDPOINT_SERVICE_REPORT_TASK_PROGRESS, req, opt)
    }

    pub fn report_task_progress_async(&self, req: &super::agentendpoint::ReportTaskProgressRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::agentendpoint::ReportTaskProgressResponse>> {
        self.report_task_progress_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn report_task_complete_opt(&self, req: &super::agentendpoint::ReportTaskCompleteRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::agentendpoint::ReportTaskCompleteResponse> {
        self.client.unary_call(&METHOD_AGENT_ENDPOINT_SERVICE_REPORT_TASK_COMPLETE, req, opt)
    }

    pub fn report_task_complete(&self, req: &super::agentendpoint::ReportTaskCompleteRequest) -> ::grpcio::Result<super::agentendpoint::ReportTaskCompleteResponse> {
        self.report_task_complete_opt(req, ::grpcio::CallOption::default())
    }

    pub fn report_task_complete_async_opt(&self, req: &super::agentendpoint::ReportTaskCompleteRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::agentendpoint::ReportTaskCompleteResponse>> {
        self.client.unary_call_async(&METHOD_AGENT_ENDPOINT_SERVICE_REPORT_TASK_COMPLETE, req, opt)
    }

    pub fn report_task_complete_async(&self, req: &super::agentendpoint::ReportTaskCompleteRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::agentendpoint::ReportTaskCompleteResponse>> {
        self.report_task_complete_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn register_agent_opt(&self, req: &super::agentendpoint::RegisterAgentRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::agentendpoint::RegisterAgentResponse> {
        self.client.unary_call(&METHOD_AGENT_ENDPOINT_SERVICE_REGISTER_AGENT, req, opt)
    }

    pub fn register_agent(&self, req: &super::agentendpoint::RegisterAgentRequest) -> ::grpcio::Result<super::agentendpoint::RegisterAgentResponse> {
        self.register_agent_opt(req, ::grpcio::CallOption::default())
    }

    pub fn register_agent_async_opt(&self, req: &super::agentendpoint::RegisterAgentRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::agentendpoint::RegisterAgentResponse>> {
        self.client.unary_call_async(&METHOD_AGENT_ENDPOINT_SERVICE_REGISTER_AGENT, req, opt)
    }

    pub fn register_agent_async(&self, req: &super::agentendpoint::RegisterAgentRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::agentendpoint::RegisterAgentResponse>> {
        self.register_agent_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn report_inventory_opt(&self, req: &super::agentendpoint::ReportInventoryRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::agentendpoint::ReportInventoryResponse> {
        self.client.unary_call(&METHOD_AGENT_ENDPOINT_SERVICE_REPORT_INVENTORY, req, opt)
    }

    pub fn report_inventory(&self, req: &super::agentendpoint::ReportInventoryRequest) -> ::grpcio::Result<super::agentendpoint::ReportInventoryResponse> {
        self.report_inventory_opt(req, ::grpcio::CallOption::default())
    }

    pub fn report_inventory_async_opt(&self, req: &super::agentendpoint::ReportInventoryRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::agentendpoint::ReportInventoryResponse>> {
        self.client.unary_call_async(&METHOD_AGENT_ENDPOINT_SERVICE_REPORT_INVENTORY, req, opt)
    }

    pub fn report_inventory_async(&self, req: &super::agentendpoint::ReportInventoryRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::agentendpoint::ReportInventoryResponse>> {
        self.report_inventory_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::std::future::Future<Output = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait AgentEndpointService {
    fn receive_task_notification(&mut self, ctx: ::grpcio::RpcContext, _req: super::agentendpoint::ReceiveTaskNotificationRequest, sink: ::grpcio::ServerStreamingSink<super::agentendpoint::ReceiveTaskNotificationResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn start_next_task(&mut self, ctx: ::grpcio::RpcContext, _req: super::agentendpoint::StartNextTaskRequest, sink: ::grpcio::UnarySink<super::agentendpoint::StartNextTaskResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn report_task_progress(&mut self, ctx: ::grpcio::RpcContext, _req: super::agentendpoint::ReportTaskProgressRequest, sink: ::grpcio::UnarySink<super::agentendpoint::ReportTaskProgressResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn report_task_complete(&mut self, ctx: ::grpcio::RpcContext, _req: super::agentendpoint::ReportTaskCompleteRequest, sink: ::grpcio::UnarySink<super::agentendpoint::ReportTaskCompleteResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn register_agent(&mut self, ctx: ::grpcio::RpcContext, _req: super::agentendpoint::RegisterAgentRequest, sink: ::grpcio::UnarySink<super::agentendpoint::RegisterAgentResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn report_inventory(&mut self, ctx: ::grpcio::RpcContext, _req: super::agentendpoint::ReportInventoryRequest, sink: ::grpcio::UnarySink<super::agentendpoint::ReportInventoryResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
}

pub fn create_agent_endpoint_service<S: AgentEndpointService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_AGENT_ENDPOINT_SERVICE_RECEIVE_TASK_NOTIFICATION, move |ctx, req, resp| {
        instance.receive_task_notification(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_AGENT_ENDPOINT_SERVICE_START_NEXT_TASK, move |ctx, req, resp| {
        instance.start_next_task(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_AGENT_ENDPOINT_SERVICE_REPORT_TASK_PROGRESS, move |ctx, req, resp| {
        instance.report_task_progress(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_AGENT_ENDPOINT_SERVICE_REPORT_TASK_COMPLETE, move |ctx, req, resp| {
        instance.report_task_complete(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_AGENT_ENDPOINT_SERVICE_REGISTER_AGENT, move |ctx, req, resp| {
        instance.register_agent(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_AGENT_ENDPOINT_SERVICE_REPORT_INVENTORY, move |ctx, req, resp| {
        instance.report_inventory(ctx, req, resp)
    });
    builder.build()
}
