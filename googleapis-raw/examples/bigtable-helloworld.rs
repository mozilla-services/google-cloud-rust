use std::error::Error;
use std::sync::Arc;

use futures::prelude::*;
use googleapis_raw::bigtable::v2::{bigtable::ReadRowsRequest, bigtable_grpc::BigtableClient};
use googleapis_raw::bigtable::admin::v2::{
    bigtable_instance_admin::GetInstanceRequest,
    bigtable_instance_admin_grpc::BigtableInstanceAdminClient,
    bigtable_table_admin::CreateTableRequest,
};
use grpcio::{ChannelBuilder, ChannelCredentials, EnvBuilder};

fn main() -> Result<(), Box<dyn Error>> {
    // An example database inside Mozilla's Bigtable instance.
    let table = "projects/mozilla-rust-sdk-dev/instances/mozilla-rust-sdk/tables/prezzy";

    // The BigTable instance id
    let instance_id = String::from("mozilla-rust-sdk");
    // Google Cloud configuration.
    let endpoint = "bigtable.googleapis.com:443";
    // The table name
    let table_name = String::from("hello-world");

    // Set up the gRPC environment.
    let env = Arc::new(EnvBuilder::new().build());
    let creds = ChannelCredentials::google_default_credentials()?;

    // Create a Bigtable client.
    let channel = ChannelBuilder::new(env.clone())
        // Set the max size to correspond to server-side limits.
        .max_send_message_len(1 << 28)
        .max_receive_message_len(1 << 28)
        .secure_connect(&endpoint, creds);
    let client = BigtableInstanceAdminClient::new(channel);

    println!("Get existing BigTable instance");
    let mut instance_request = GetInstanceRequest::new();
    instance_request.set_name(instance_id);
    println!("Get existing BigTable instance 1");
    let instance = client.get_instance(&instance_request)?;
    println!("Get existing BigTable instance 2");
    dbg!(instance);

    println!("Creating the {} table", table_name);
    let mut table_request = CreateTableRequest::new();
    table_request.set_table_id(table_name);
    // client.create_table()

    // println!("Creating ");

    Ok(())
}
