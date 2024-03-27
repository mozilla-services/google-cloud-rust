use std::sync::Arc;

use google_cloud_rust_raw::cloud::asset::v1::{
    asset_service::ExportAssetsRequest, asset_service_grpc::AssetServiceClient,
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
        .set_credentials(creds)
        .connect(endpoint)
}

fn main() -> Result<(), Box<dyn Error>> {
    let endpoint = "cloudasset.googleapis.com";
    let channel = connect(endpoint);
    let client = AssetServiceClient::new(channel);

    let req = ExportAssetsRequest {
        parent: format!("projects/{}", "mozilla-rust-sdk-dev"),
        ..Default::default()
    };

    match client.export_assets(&req) {
        Ok(response) => {
            // NOTE: the API for this recently changd. Please refer to
            // GCP documentation for details about these changes.
            print!("{:?}", response);
        }
        Err(error) => println!("Failed to list assets: {}", error),
    }

    Ok(())
}
