// Copyright 2020 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use google_cloud_rust_raw::empty::Empty;
use google_cloud_rust_raw::longrunning::operations::GetOperationRequest;
use google_cloud_rust_raw::longrunning::operations_grpc::OperationsClient;
use google_cloud_rust_raw::spanner::admin::database::v1::{
    spanner_database_admin::CreateDatabaseRequest, spanner_database_admin::DropDatabaseRequest,
    spanner_database_admin::GetDatabaseRequest, spanner_database_admin_grpc::DatabaseAdminClient,
};
use google_cloud_rust_raw::spanner::v1::transaction::transaction_options::{Mode, ReadWrite};
use google_cloud_rust_raw::spanner::v1::{
    mutation::mutation::Write,
    mutation::Mutation,
    spanner::{BeginTransactionRequest, CommitRequest, CreateSessionRequest, Session},
    spanner_grpc::SpannerClient,
    transaction::TransactionOptions,
};
use grpcio::{
    CallOption, Channel, ChannelBuilder, ChannelCredentials, ClientUnaryReceiver, EnvBuilder,
    MetadataBuilder,
};
use protobuf::well_known_types::struct_::{ListValue, Value};
use protobuf::MessageField;
#[allow(unused_imports)]
use std::error::Error;
use std::sync::Arc;
use std::time::Duration;

const CREATE_DATABASE: &str = "CREATE DATABASE music";

const CREATE_SINGER_TABLE: &str = "CREATE TABLE Singers (
      SingerId   INT64 NOT NULL,
      FirstName  STRING(1024),
      LastName   STRING(1024),
      SingerInfo BYTES(MAX),
    ) PRIMARY KEY (SingerId)";

const CREATE_ALBUMS_TABLE: &str = "CREATE TABLE Albums (
      SingerId     INT64 NOT NULL,
      AlbumId      INT64 NOT NULL,
      AlbumTitle   STRING(MAX),
    ) PRIMARY KEY (SingerId, AlbumId),
      INTERLEAVE IN PARENT Singers ON DELETE CASCADE";

/// A basic struct for a singer
///
struct Singer {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
}

/// Waits until the operation is finished
///
fn wait_operation_finished(channel: &Channel, operation: &str) {
    let operations_client = OperationsClient::new(channel.clone());

    let request = GetOperationRequest {
        name: operation.to_string(),
        ..Default::default()
    };

    loop {
        println!("Checking operation: {}", operation);
        match operations_client.get_operation(&request) {
            Ok(response) => {
                if response.done {
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
    let request = GetDatabaseRequest {
        name: database_name.to_string(),
        ..Default::default()
    };
    if let Ok(database) = client.get_database(&request) {
        println!("Found database: {}", database.name);
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

    let request = CreateDatabaseRequest {
        parent: instance_id.to_string(),
        create_statement: CREATE_DATABASE.to_string(),
        extra_statements: vec![statements],
        ..Default::default()
    };
    let operation = client
        .create_database(&request)
        .expect("Failed to create database");
    dbg!(operation.clone());

    // check that operation is finished
    wait_operation_finished(&channel, &operation.name);
}

/// Deletes a given database
///
#[allow(dead_code)]
fn drop_database(
    channel: &Channel,
    database_name: &str,
) -> ::grpcio::Result<ClientUnaryReceiver<Empty>> {
    println!("Drop database {}", database_name);
    let client = DatabaseAdminClient::new(channel.clone());

    let request = DropDatabaseRequest {
        database: database_name.to_string(),
        ..Default::default()
    };

    client.drop_database_async(&request)
}

/// Create a new session to communicate with Spanner
///
fn create_session(client: &SpannerClient, database_name: &str) -> ::grpcio::Result<Session> {
    let request = CreateSessionRequest {
        database: database_name.to_string(),
        ..Default::default()
    };
    let mut meta = MetadataBuilder::new();
    meta.add_str("google-cloud-resource-prefix", database_name)
        .expect("Failed to set meta data");
    meta.add_str("x-goog-api-client", "googleapis-rs")
        .expect("Failed to set meta data");
    let opt = CallOption::default().headers(meta.build());
    client.create_session_opt(&request, opt)
}

/// Create a new channel used for the different types of clients
///
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
        .connect(&endpoint)
}

fn main() -> Result<(), Box<dyn Error>> {
    let endpoint = "spanner.googleapis.com";

    // global project_id
    let _project_id = "projects/mozilla-rust-sdk-dev";
    // spanner instance id
    let instance_id = "projects/mozilla-rust-sdk-dev/instances/mozilla-spanner-dev";
    // database name
    let database_name =
        "projects/mozilla-rust-sdk-dev/instances/mozilla-spanner-dev/databases/music";

    // create spanner admin client
    let channel = connect(endpoint);

    // create database if it not already exists
    create_database_if_not_exists(&channel, database_name, instance_id);

    // create session to communicate
    let client = SpannerClient::new(channel.clone());
    let session = create_session(&client, database_name)?;

    // insert data into database by using a transaction
    let client = SpannerClient::new(channel);
    let read_write = TransactionOptions {
        mode: Some(Mode::ReadWrite(ReadWrite::default())),
        ..Default::default()
    };
    let request = BeginTransactionRequest {
        session: session.name.to_string(),
        options: MessageField::from(Some(read_write)),
        ..Default::default()
    };
    let transaction = client.begin_transaction(&request)?;

    // the list of singers to add
    let columns = vec![
        "SingerId".to_string(),
        "FirstName".to_string(),
        "LastName".to_string(),
    ];
    let singers = vec![
        Singer {
            id: 1,
            first_name: "Marc".to_string(),
            last_name: "Richards".to_string(),
        },
        Singer {
            id: 2,
            first_name: "Catalina".to_string(),
            last_name: "Smith".to_string(),
        },
        Singer {
            id: 3,
            first_name: "Alice".to_string(),
            last_name: "Trentor".to_string(),
        },
        Singer {
            id: 4,
            first_name: "Lea".to_string(),
            last_name: "Martin".to_string(),
        },
        Singer {
            id: 5,
            first_name: "David".to_string(),
            last_name: "Lomond".to_string(),
        },
    ];

    // collect all values
    let mut list_values = Vec::new();
    for singer in singers {
        let mut id = Value::new();
        id.set_string_value(singer.id.to_string());
        let mut first_name = Value::new();
        first_name.set_string_value(singer.first_name.clone());
        let mut last_name = Value::new();
        last_name.set_string_value(singer.last_name.clone());

        let list = ListValue {
            values: vec![id, first_name, last_name],
            ..Default::default()
        };
        list_values.push(list);
    }

    // create a suitable mutation with all values
    println!("Preparing write mutation to add singers");
    /*
    let mut mutation_write = Mutation_Write::new();
    mutation_write.set_table("Singers".to_string());
    mutation_write.set_columns(RepeatedField::from_vec(columns));
    mutation_write.set_values(RepeatedField::from_vec(list_values));
    println!("Mutation write object");
    dbg!(mutation_write.clone());
    */
    let mutation_write = Write {
        table: "Singers".to_string(),
        columns,
        values: list_values,
        ..Default::default()
    };
    // finally commit to database
    println!("Commit data to database {}", database_name);
    let mut mutation = Mutation::new();
    mutation.set_insert_or_update(mutation_write);
    let commit = CommitRequest {
        session: session.name.clone(),
        mutations: vec![mutation],
        transaction: Some(
            google_cloud_rust_raw::spanner::v1::spanner::commit_request::Transaction::TransactionId(
                transaction.id,
            ),
        ),
        ..Default::default()
    };
    let response = client.commit(&commit)?;
    dbg!(response);

    // delete database
    // drop_database(&channel, database_name)?.wait()?;

    Ok(())
}
