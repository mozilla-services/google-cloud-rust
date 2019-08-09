#[allow(unused_imports)]

use std::error::Error;
use std::sync::Arc;
use std::time::{SystemTime};
use futures::prelude::*;
use grpcio::{
    Channel,
    ChannelBuilder,
    ChannelCredentials,
    ClientUnaryReceiver,
    EnvBuilder,
};

/// Create a new channel used for the different types of clients
///
fn connect(endpoint: &str) -> Channel {
    // Set up the gRPC environment.
    let env = Arc::new(EnvBuilder::new().build());
    let creds = ChannelCredentials::google_default_credentials().expect("No Google redentials found");

    // Create a channel to connect to Gcloud.
    ChannelBuilder::new(env.clone())
        // Set the max size to correspond to server-side limits.
        .max_send_message_len(1 << 28)
        .max_receive_message_len(1 << 28)
        .secure_connect(&endpoint, creds)
}

fn main() -> Result<(), Box<dyn Error>> {
    let endpoint = "";

    Ok(())
}
