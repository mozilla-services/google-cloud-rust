#[allow(unused_imports)]
use std::error::Error;
use std::sync::Arc;

use futures::prelude::*;
use googleapis_raw::bigtable::admin::v2::{
    bigtable_table_admin::CreateTableRequest,
    bigtable_table_admin::ListTablesRequest,
    bigtable_table_admin::ListTablesResponse,
    bigtable_table_admin_grpc::BigtableTableAdminClient,
};
use grpcio::{ChannelBuilder, ChannelCredentials, EnvBuilder};

fn main() -> Result<(), Box<dyn Error>> {
    // An example database inside Mozilla's Bigtable instance.
    let table = "projects/mozilla-rust-sdk-dev/instances/mozilla-rust-sdk/tables/prezzy";

    // BigTable project id
    let project_id = String::from("mozilla-rust-sdk-dev");
    // The BigTable instance id
    let instance_id = String::from("projects/mozilla-rust-sdk-dev/instances/mozilla-rust-sdk");
    // The cluster id
    let cluster_id = String::from("mozilla-rust-sdk-c1");
    // Google Cloud configuration.
    let endpoint = "bigtableadmin.googleapis.com";
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
    let client = BigtableTableAdminClient::new(channel);

    println!("List all existing tables");
    let mut request = ListTablesRequest::new();
    request.set_parent(instance_id);
    match client.list_tables(&request) {
        Ok(response) => {
            response.get_tables()
                .iter()
                .for_each(|table| println!("  table: {:?}", table) );
        },
        Err(error) => println!("Failed to list tables: {}", error),
    }

    println!("Creating the {} table", table_name);
    let mut request = CreateTableRequest::new();
    request.set_table_id(table_name);
    match client.create_table(&request) {
        Ok(table) => println!("  table {:?} created", table),
        Err(error) => println!("  ")
    }

    // println!("Creating ");

    Ok(())
}
