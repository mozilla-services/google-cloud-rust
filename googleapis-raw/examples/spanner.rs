use std::sync::Arc;

use googleapis_raw::spanner::v1::{
    spanner::CreateSessionRequest, spanner::ExecuteSqlRequest, spanner::Session,
    spanner_grpc::SpannerClient,
};
use grpcio::{CallOption, ChannelBuilder, ChannelCredentials, EnvBuilder, MetadataBuilder};
use slog::{Drain, Logger, OwnedKV};
use slog_scope::GlobalLoggerGuard;
use slog_term::{Decorator, FullFormat, PlainSyncDecorator, TermDecorator};

pub fn init_log() -> GlobalLoggerGuard {
    let decorator = TermDecorator::new().build();
    let drain = FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let logger = Logger::root(drain, OwnedKV(()));
    let guard = slog_scope::set_global_logger(logger);
    slog_stdlog::init().unwrap();
    guard
}

fn main() {
    let _guard = init_log();

    let endpoint = "spanner.googleapis.com:443";
    let db = "projects/mozilla-rust-sdk-dev/instances/mozilla-spanner-dev/databases/mydb";

    let env = Arc::new(EnvBuilder::new().build());
    let creds = ChannelCredentials::google_default_credentials().unwrap();
    let chan = ChannelBuilder::new(env.clone())
        .max_send_message_len(100 << 20)
        .max_receive_message_len(100 << 20)
        .secure_connect(&endpoint, creds);

    let client = SpannerClient::new(chan);

    let mut req = CreateSessionRequest::new();
    req.set_database(db.to_string());
    let mut meta = MetadataBuilder::new();
    meta.add_str("google-cloud-resource-prefix", &db);
    // meta.add_str("x-goog-api-client", "TODO");
    let opt = CallOption::default().headers(meta.build());
    let session = client.create_session_opt(&req, opt).unwrap();

    let mut req = ExecuteSqlRequest::new();
    req.set_session(session.get_name().to_string());
    req.set_sql("select * from people".to_string());
    let fut = client.execute_sql_async(&req).unwrap();

    dbg!(futures::Future::wait(fut));

    // let mut req = HelloRequest::new_();
    // req.set_name("world".to_owned());
    // let reply = client.say_hello(&req).expect("rpc");
    // info!("Greeter received: {}", reply.get_message());
}
