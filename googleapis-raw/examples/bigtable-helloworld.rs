#[allow(unused_imports)]
use std::error::Error;
use std::sync::Arc;

use futures::prelude::*;
use googleapis_raw::bigtable::admin::v2::{
    bigtable_instance_admin::GetClusterRequest,
    bigtable_table_admin::CreateTableRequest,
    bigtable_table_admin::DeleteTableRequest,
    bigtable_table_admin::ListTablesRequest,
    bigtable_table_admin::ListTablesResponse,
    bigtable_table_admin_grpc::BigtableTableAdminClient,
    bigtable_instance_admin_grpc::BigtableInstanceAdminClient,
    table::ColumnFamily,
    table::Table,
};
use grpcio::{ChannelBuilder, ChannelCredentials, EnvBuilder};

/// Lists all tables for a given cluster
///
fn list_tables(client: &BigtableTableAdminClient, instance_id: &String) {
    println!("List all existing tables");
    let mut request = ListTablesRequest::new();
    request.set_parent(instance_id.clone());
    match client.list_tables(&request) {
        Ok(response) => {
            response.get_tables()
                .iter()
                .for_each(|table| println!("  table: {:?}", table) );
        },
        Err(error) => println!("Failed to list tables: {}", error),
    }
}

/// Create a new table in the BigTable cluster
///
fn create_table(client: &BigtableTableAdminClient, table_name: &String) {

}

fn main() -> Result<(), Box<dyn Error>> {
    // BigTable project id
    let project_id = String::from("mozilla-rust-sdk-dev");
    // The BigTable instance id
    let instance_id = String::from("projects/mozilla-rust-sdk-dev/instances/mozilla-rust-sdk");
    // The cluster id
    let cluster_id = String::from("projects/mozilla-rust-sdk-dev/instances/mozilla-rust-sdk/clusters/mozilla-rust-sdk-c1");
    // Google Cloud configuration.
    let endpoint = "bigtableadmin.googleapis.com";
    // The table name
    let table_name = String::from("projects/mozilla-rust-sdk-dev/instances/mozilla-rust-sdk/tables/hello-world");

    // Set up the gRPC environment.
    let env = Arc::new(EnvBuilder::new().build());
    let creds = ChannelCredentials::google_default_credentials()?;

    // Create a Bigtable client.
    let channel = ChannelBuilder::new(env.clone())
        // Set the max size to correspond to server-side limits.
        .max_send_message_len(1 << 28)
        .max_receive_message_len(1 << 28)
        .secure_connect(&endpoint, creds);
    let client = BigtableInstanceAdminClient::new(channel.clone());

    println!("Get cluster information");
    let mut request = GetClusterRequest::new();
    request.set_name(cluster_id.to_string());
    let cluster = client.get_cluster(&request)?;
    dbg!(cluster);

    // create admin client for tables
    let client = BigtableTableAdminClient::new(channel.clone());
    list_tables(&client, &instance_id);

    println!("Creating the {} table", table_name);
    let mut request = CreateTableRequest::new();
    request.set_parent(instance_id.clone());
    request.set_table(Table::new());
    request.set_table_id("hello-world".to_string());
    match client.create_table(&request) {
        Ok(table) => println!("  table {:?} created", table),
        Err(error) => println!("  failed to created table: {}", error),
    }

    list_tables(&client, &instance_id);

    println!("Deleting the {} table", table_name);
    let mut request = DeleteTableRequest::new();
    request.set_name(table_name);
    let future = client.delete_table_async(&request)?;
    let response = future.wait()?;
    dbg!(response);

    list_tables(&client, &instance_id);

    Ok(())
}
