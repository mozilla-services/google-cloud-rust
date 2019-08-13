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
use googleapis_raw::spanner::v1::{
    spanner_grpc::SpannerClient,
};
use googleapis_raw::spanner::admin::database::v1::{
    spanner_database_admin::Database,
    spanner_database_admin::CreateDatabaseRequest,
    spanner_database_admin::GetDatabaseRequest,
    spanner_database_admin_grpc::DatabaseAdminClient,
};

const CREATE_STATEMENTS: &str =
    "CREATE TABLE Singers (
      SingerId   INT64 NOT NULL,
      FirstName  STRING(1024),
      LastName   STRING(1024),
      SingerInfo BYTES(MAX),
    ) PRIMARY KEY (SingerId)
    ;
    CREATE TABLE Albums (
      SingerId     INT64 NOT NULL,
      AlbumId      INT64 NOT NULL,
      AlbumTitle   STRING(MAX),
    ) PRIMARY KEY (SingerId, AlbumId),
      INTERLEAVE IN PARENT Singers ON DELETE CASCADE
    ;
";

/// Finds or creates a database, returns it
///
fn find_or_create_database(client: &DatabaseAdminClient, database_name: &str, instance_id: &str) /*-> ::grpcio::Result<Database> */{
    // find database
    println!("Finding database {}", database_name);
    let mut request = GetDatabaseRequest::new();
    request.set_name(database_name.to_string());
    if let Ok(database) = client.get_database(&request) {
        println!("Found database: {}", database.get_name());
        // return Ok(database);
        return;
    } else {
        println!("Database not found");
    }

    // create a new database
    println!("Create database {}", database_name);
    let mut request = CreateDatabaseRequest::new();
    request.set_parent(instance_id.to_string());
    request.set_create_statement(CREATE_STATEMENTS.to_string());
    request.set_extra_statements(CREATE_STATEMENTS);
    let operation = client.create_database(&request);
    dbg!(operation);
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
    let admin_client = DatabaseAdminClient::new(channel);

    find_or_create_database(&admin_client, database_name, instance_id);

    Ok(())
}
