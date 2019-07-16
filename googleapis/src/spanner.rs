//! Spanner client.

use std::sync::Arc;

use googleapis_raw::spanner::v1::{
    spanner::{CreateSessionRequest, Session},
    spanner_grpc::SpannerClient,
};
use grpcio::{CallOption, ChannelBuilder, ChannelCredentials, EnvBuilder, MetadataBuilder};

pub struct Client {
    database: String,
    client: SpannerClient,
    session: Session,
}

impl Client {
    /// Creates a new Spanner client.
    ///
    /// The database argument should be formatted as
    /// `projects/PROJECT_ID/instances/INSTANCE_ID/databases/DATABASE_ID`.
    pub fn new(database: &str) -> crate::Result<Client> {
        let database = database.to_string();
        let endpoint = "spanner.googleapis.com:443";

        // Set up the gRPC environment.
        let env = Arc::new(EnvBuilder::new().build());
        let creds = ChannelCredentials::google_default_credentials()?;

        // Create a Spanner client.
        let chan = ChannelBuilder::new(env.clone())
            .max_send_message_len(100 << 20)
            .max_receive_message_len(100 << 20)
            .secure_connect(&endpoint, creds);
        let client = SpannerClient::new(chan);

        let mut req = CreateSessionRequest::new();
        req.set_database(database.to_string());
        let mut meta = MetadataBuilder::new();
        meta.add_str("google-cloud-resource-prefix", &database)?;
        meta.add_str("x-goog-api-client", "googleapis-rs")?;
        let opt = CallOption::default().headers(meta.build());
        let session = client.create_session_opt(&req, opt)?;

        Ok(Client {
            database,
            client,
            session,
        })
    }
}
