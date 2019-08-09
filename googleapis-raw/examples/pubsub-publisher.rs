#[allow(unused_imports)]
use std::collections::HashMap;
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
use googleapis_raw::pubsub::v1::{
    pubsub::GetTopicRequest,
    pubsub::PublishRequest,
    pubsub::PublishResponse,
    pubsub::PubsubMessage,
    pubsub::Topic,
    pubsub_grpc::PublisherClient,
};
use protobuf::RepeatedField;

/// Creates a topic or finds an existing one, then returns the topic
///
fn find_or_create_topic(client: &PublisherClient, topic_name: &str, label: &str) -> ::grpcio::Result<Topic> {
    // find topic
    println!("Finding topic {}", topic_name);
    let mut request = GetTopicRequest::new();
    request.set_topic(topic_name.to_string());
    if let Ok(topic) = client.get_topic(&request) {
        println!("Found topic: {}", topic.get_name());
        return Ok(topic);
    } else {
        println!("Topic not found");
    }

    // otherwise create topic
    println!("Creating topic {}", topic_name);
    let mut labels = HashMap::new();
    labels.insert("environment".to_string(), label.to_string());
    let mut topic = Topic::new();
    topic.set_name(topic_name.to_string());
    topic.set_labels(labels);
    client.create_topic(&topic)
}

fn timestamp_in_seconds() -> u64 {
    let timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    timestamp.as_secs()
}

/// Creates a new PubSubMessage instance
fn create_pubsub_msg(message: &str) -> PubsubMessage {
    println!("Publishing message: {}", message);
    let mut timestamp = ::protobuf::well_known_types::Timestamp::new();
    timestamp.set_seconds(timestamp_in_seconds() as i64);

    let mut pubsub_msg = PubsubMessage::new();
    pubsub_msg.set_data(message.to_string().into_bytes());
    pubsub_msg.set_publish_time(timestamp);
    pubsub_msg
}

/// Publishes a message asynchronously, returning a future
///
fn publish_msg_async(client: &PublisherClient, topic: &Topic, messages: Vec<String>) -> ::grpcio::Result<ClientUnaryReceiver<PublishResponse>> {
    let pub_messages = messages.iter().map(|msg| create_pubsub_msg(msg) ).collect();
    
    let mut request = PublishRequest::new();
    request.set_topic(topic.get_name().to_string());
    request.set_messages(RepeatedField::from_vec(pub_messages));
    client.publish_async(&request)
}

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
    // API endpoint
    let endpoint = "pubsub.googleapis.com";
    // GCloud project id
    let project_id = "mozilla-rust-sdk-dev";

    // create client
    let channel = connect(&endpoint);
    let publisher = PublisherClient::new(channel.clone());

    // get topic
    let topic_name = format!("projects/{}/topics/greetings", project_id);
    let topic = find_or_create_topic(&publisher, &topic_name, "test")?;
    dbg!(topic.clone());

    // publish a number of greeting messages
    let greetings = vec!["hello", "hi", "hola", "bonjour", "ahoi"];
    for greeting in greetings {
        publish_msg_async(&publisher, &topic, vec![greeting.to_string()])?;
    }

    // create a subscriber to consume these messages
    

    Ok(())
}
