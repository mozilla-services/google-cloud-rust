#[allow(unused_imports)]

use std::error::Error;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use futures::prelude::*;
use grpcio::{
    Channel,
    ChannelBuilder,
    ChannelCredentials,
    ClientUnaryReceiver,
    EnvBuilder,
};
use googleapis_raw::empty::Empty;
use googleapis_raw::spanner::v1::{
    spanner_grpc::SpannerClient,
};
use googleapis_raw::longrunning::operations::{
    GetOperationRequest,
    Operation,
};
use googleapis_raw::longrunning::operations_grpc::{
    OperationsClient,
};
use googleapis_raw::spanner::admin::database::v1::{
    spanner_database_admin::Database,
    spanner_database_admin::CreateDatabaseRequest,
    spanner_database_admin::DropDatabaseRequest,
    spanner_database_admin::GetDatabaseRequest,
    spanner_database_admin_grpc::DatabaseAdminClient,
};
use protobuf::RepeatedField;

const CREATE_DATABASE: &str =
    "CREATE DATABASE music";

const CREATE_SINGER_TABLE: &str =
    "CREATE TABLE Singers (
      SingerId   INT64 NOT NULL,
      FirstName  STRING(1024),
      LastName   STRING(1024),
      SingerInfo BYTES(MAX),
    ) PRIMARY KEY (SingerId)";

const CREATE_ALBUMS_TABLE: &str =
    "CREATE TABLE Albums (
      SingerId     INT64 NOT NULL,
      AlbumId      INT64 NOT NULL,
      AlbumTitle   STRING(MAX),
    ) PRIMARY KEY (SingerId, AlbumId),
      INTERLEAVE IN PARENT Singers ON DELETE CASCADE";

/// Waits until the operation is finished
///
fn wait_operation_finished(channel: &Channel, operation: &str) {
    let operations_client = OperationsClient::new(channel.clone());

    let mut request = GetOperationRequest::new();
    request.set_name(operation.to_string());

    loop {
        println!("Checking operation: {}", operation);
        match operations_client.get_operation(&request) {
            Ok(response) => {
                if response.get_done() {
                    println!("Operation {} finished", operation);
                    break;
                }

                // wait instead
                let wait_time = Duration::from_millis(250);
                std::thread::sleep(wait_time);
            }
            Err(error) => {
                println!("Failed to get operation");
                dbg!(error);
            }
        }
    }
}

/// Creates a new database if it does not exist yet.
///
fn create_database_if_not_exists(channel: &Channel, database_name: &str, instance_id: &str) {
    let client = DatabaseAdminClient::new(channel.clone());
    // find database
    println!("Finding database {}", database_name);
    let mut request = GetDatabaseRequest::new();
    request.set_name(database_name.to_string());
    if let Ok(database) = client.get_database(&request) {
        println!("Found database: {}", database.get_name());
        return;
    } else {
        println!("Database not found");
    }

    // create a new database
    println!("Create database {}", database_name);
    let statements = vec![CREATE_SINGER_TABLE, CREATE_ALBUMS_TABLE]
        .iter()
        .map(|s| s.to_string())
        .collect();

    let mut request = CreateDatabaseRequest::new();
    request.set_parent(instance_id.to_string());
    request.set_create_statement(CREATE_DATABASE.to_string());
    request.set_extra_statements(RepeatedField::from_vec(statements));
    let operation = client.create_database(&request).expect("Failed to create database");
    dbg!(operation.clone());

    // check that operation is finished
    wait_operation_finished(&channel, operation.get_name());
}

/// Deletes a given database
///
fn drop_database(channel: &Channel, database_name: &str) -> ::grpcio::Result<ClientUnaryReceiver<Empty>> {
    let client = DatabaseAdminClient::new(channel.clone());

    let mut request = DropDatabaseRequest::new();
    request.set_database(database_name.to_string());

    client.drop_database_async(&request)
}

/// Create a new channel used for the different types of clients
///
fn connect(endpoint: &str) -> Channel {
    // Set up the gRPC environment.
    let env = Arc::new(EnvBuilder::new().build());
    let creds = ChannelCredentials::google_default_credentials().expect("No Google credentials found");

    // Create a channel to connect to Gcloud.
    ChannelBuilder::new(env.clone())
        // Set the max size to correspond to server-side limits.
        .max_send_message_len(1 << 28)
        .max_receive_message_len(1 << 28)
        .secure_connect(&endpoint, creds)
}

fn main() -> Result<(), Box<dyn Error>> {
    let endpoint = "spanner.googleapis.com";

    // global project_id
    let project_id = "projects/mozilla-rust-sdk-dev";
    // spanner instance id
    let instance_id = "projects/mozilla-rust-sdk-dev/instances/mozilla-spanner-dev";
    // database name
    let database_name = "projects/mozilla-rust-sdk-dev/instances/mozilla-spanner-dev/databases/music";

    // create spanner admin client
    let channel = connect(endpoint);

    // create database if it not already exists
    create_database_if_not_exists(&channel, database_name, instance_id);

    // delete database
    drop_database(&channel, database_name)?.wait()?;

    Ok(())
}
