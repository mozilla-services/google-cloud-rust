use std::sync::Arc;

use google_cloud_rust_raw::cloud::asset::v1::{
    asset_service_grpc::AssetServiceClient,
    asset_service::ListAssetsRequest,
};
use grpcio::{Channel, ChannelBuilder, ChannelCredentials, EnvBuilder};
use std::error::Error;


fn connect(endpoint: &str) -> Channel {
    // Set up the gRPC environment.
    let env = Arc::new(EnvBuilder::new().build());
    let creds =
        ChannelCredentials::google_default_credentials().expect("No Google credentials found");

    // Create a channel to connect to Gcloud.
    ChannelBuilder::new(env)
        // Set the max size to correspond to server-side limits.
        .max_send_message_len(1 << 28)
        .max_receive_message_len(1 << 28)
        .secure_connect(endpoint, creds)
}

fn main() -> Result<(), Box<dyn Error>> {
    let endpoint = "cloudasset.googleapis.com";
    let channel = connect(endpoint);
    let client = AssetServiceClient::new(channel);

    let mut req = ListAssetsRequest::new();
    req.set_parent(format!("projects/{}", "mozilla-rust-sdk-dev"));

    match client.list_assets(&req) {
        Ok(response) => {
            response
                .get_assets()
                .iter()
                .for_each(|asset| println!("  Asset: {:?}", asset));
        },
        Err(error) => println!("Failed to list assets: {}", error),
    }

    Ok(())
}
