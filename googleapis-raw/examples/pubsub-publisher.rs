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

use std::collections::HashMap;
use std::sync::Arc;
use std::time::SystemTime;

use futures::executor::block_on;
use google_cloud_rust_raw::pubsub::v1::{
    pubsub::AcknowledgeRequest, pubsub::ExpirationPolicy, pubsub::GetSubscriptionRequest,
    pubsub::GetTopicRequest, pubsub::PublishRequest, pubsub::PublishResponse,
    pubsub::PubsubMessage, pubsub::PullRequest, pubsub::PushConfig, pubsub::Subscription,
    pubsub::Topic, pubsub_grpc::PublisherClient, pubsub_grpc::SubscriberClient,
};
use grpcio::{Channel, ChannelBuilder, ChannelCredentials, ClientUnaryReceiver, EnvBuilder};
use protobuf::{well_known_types::duration::Duration, MessageField};

/// Creates a topic or finds an existing one, then returns the topic
///
fn find_or_create_topic(client: &PublisherClient, topic_name: &str) -> ::grpcio::Result<Topic> {
    // find topic
    println!("Finding topic {}", topic_name);
    let request = GetTopicRequest {
        topic: topic_name.to_string(),
        ..Default::default()
    };
    if let Ok(topic) = client.get_topic(&request) {
        println!("Found topic: {}", topic.name);
        return Ok(topic);
    } else {
        println!("Topic not found");
    }

    // otherwise create topic
    println!("Creating topic {}", topic_name);
    let mut labels = HashMap::new();
    labels.insert("environment".to_string(), "test".to_string());
    let topic = Topic {
        name: topic_name.to_string(),
        labels,
        ..Default::default()
    };
    client.create_topic(&topic)
}

/// Creates a subscription or finds an existing one
///
fn find_or_create_subscription(
    client: &SubscriberClient,
    subscription_name: &str,
    topic_name: &str,
) -> ::grpcio::Result<Subscription> {
    // find subscription
    println!(
        "Finding subscription {} for topic {}",
        subscription_name, topic_name
    );
    let request = GetSubscriptionRequest {
        subscription: subscription_name.to_string(),
        ..Default::default()
    };
    if let Ok(subscription) = client.get_subscription(&request) {
        println!("Found subscription: {}", subscription.name);
        return Ok(subscription);
    } else {
        println!("Subscription not found");
    }

    // create a new subscription
    println!("Creating a new subscription {}", subscription_name);
    let mut labels = HashMap::new();
    labels.insert("environment".to_string(), "test".to_string());
    let mut attributes = HashMap::new();
    attributes.insert("attribute".to_string(), "hello".to_string());
    let expiration_duration = Duration {
        seconds: (60 * 60 * 48),
        ..Default::default()
    };
    let expiration_policy = ExpirationPolicy {
        ttl: protobuf::MessageField(Some(Box::new(expiration_duration))),
        ..Default::default()
    };
    let push_config = PushConfig {
        attributes,
        ..Default::default()
    };
    let subscription = Subscription {
        name: subscription_name.to_string(),
        topic: topic_name.to_string(),
        ack_deadline_seconds: 20,
        expiration_policy: MessageField(Some(Box::new(expiration_policy))),
        push_config: MessageField(Some(Box::new(push_config))),
        ..Default::default()
    };
    // subscription.set_expiration_policy(expiration_policy);
    // subscription.set_message_retention_duration(expiration_duration.clone());
    // subscription.set_push_config(push_config);
    // subscription.set_labels(labels);

    client.create_subscription(&subscription)
}

fn timestamp_in_seconds() -> u64 {
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    timestamp.as_secs()
}

/// Creates a new PubSubMessage instance
///
fn create_pubsub_msg(message: &str) -> PubsubMessage {
    println!("Publishing message: {}", message);
    let timestamp = ::protobuf::well_known_types::timestamp::Timestamp {
        seconds: timestamp_in_seconds() as i64,
        ..Default::default()
    };

    let pubsub_msg = PubsubMessage {
        data: message.to_owned().into_bytes(),
        publish_time: protobuf::MessageField(Some(Box::new(timestamp))),
        ..Default::default()
    };
    pubsub_msg
}

/// Publishes a message asynchronously, returning a future
///
fn publish_msg_async(
    client: &PublisherClient,
    topic: &Topic,
    messages: Vec<String>,
) -> ::grpcio::Result<ClientUnaryReceiver<PublishResponse>> {
    let pub_messages = messages.iter().map(|msg| create_pubsub_msg(msg)).collect();

    let request = PublishRequest {
        topic: topic.name.to_string(),
        messages: pub_messages,
        ..Default::default()
    };
    client.publish_async(&request)
}

/// Create a new channel used for the different types of clients
///
fn connect(endpoint: &str) -> Channel {
    // Set up the gRPC environment.
    let env = Arc::new(EnvBuilder::new().build());
    let creds =
        ChannelCredentials::google_default_credentials().expect("No Google redentials found");

    // Create a channel to connect to Gcloud.
    ChannelBuilder::new(env)
        // Set the max size to correspond to server-side limits.
        .max_send_message_len(1 << 28)
        .max_receive_message_len(1 << 28)
        .set_credentials(creds)
        .connect(&endpoint)
}

async fn async_main() {
    // API endpoint
    let endpoint = "pubsub.googleapis.com";
    // GCloud project id
    let project_id = "mozilla-rust-sdk-dev";

    // create client
    let channel = connect(&endpoint);
    let publisher = PublisherClient::new(channel.clone());

    // get topic
    let topic_name = format!("projects/{}/topics/greetings", project_id);
    let topic = dbg!(find_or_create_topic(&publisher, &topic_name).unwrap());

    // publish a number of greeting messages
    let greetings = vec!["hello", "hi", "hola", "bonjour", "ahoi"];
    let messages = greetings.iter().map(|g| g.to_string()).collect();
    publish_msg_async(&publisher, &topic, messages)
        .unwrap()
        .await
        .unwrap();

    // create a subscriber to consume these messages
    let subscription_name = format!("projects/{}/subscriptions/sub-greetings", project_id);
    let subscriber = SubscriberClient::new(channel.clone());

    // get subscription
    let subscription =
        find_or_create_subscription(&subscriber, &subscription_name, &topic_name).unwrap();

    // Pubsub Subscription Pull, receive all messages
    println!("Pulling messages from subscription {:?}", subscription);
    let request = PullRequest {
        subscription: subscription_name.to_string(),
        max_messages: 10,
        ..Default::default()
    };

    loop {
        let future = subscriber.pull_async(&request).unwrap();
        let response = future.await.unwrap();
        let pubsub_messages = response.received_messages;

        println!("Handling {} messages", pubsub_messages.len());
        for pubsub_message in pubsub_messages.clone() {
            println!("  >> message: {:?}", pubsub_message);
            let ack_id = pubsub_message.ack_id;

            let request = AcknowledgeRequest {
                subscription: subscription_name.clone(),
                ack_ids: vec![ack_id],
                ..Default::default()
            };
            subscriber.acknowledge(&request).unwrap();
        }

        // once all messages are handled leave
        if pubsub_messages.is_empty() {
            break;
        }
    }
}

fn main() {
    block_on(async_main());
}
