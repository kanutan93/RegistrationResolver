mod model;

use std::env;
use std::fmt::format;
use std::future::Future;
use std::time::Duration;
use log::{debug, error, info};
use config::kafka_config::KafkaConfig;
use rdkafka::{ClientConfig, Message};
use rdkafka::consumer::{CommitMode, Consumer, StreamConsumer};
use dotenv::*;
use rand::Rng;
use rdkafka::message::BorrowedMessage;
use rdkafka::producer::{BaseProducer, BaseRecord, Producer};
use crate::model::approve_result::ApprovalMessage;
use crate::model::user::User;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let registration_topic = env::var("KAFKA_REGISTRATION_TOPIC")
        .expect("KAFKA_REGISTRATION_TOPIC env var isn't set!");
    let resolver_topic = env::var("KAFKA_RESOLVER_TOPIC")
        .expect("KAFKA_RESOLVER_TOPIC env var isn't set!");

    let KafkaConfig { consumer, producer, .. } = KafkaConfig::new(registration_topic.clone());

    consumer
        .subscribe(&[registration_topic.as_str()])
        .expect("Can't subscribe to specified topics");

    loop {
        match consumer.recv().await {
            Ok(m) => {
                let message = consume_message(&consumer, &m);
                produce_approval_result(message, &resolver_topic, &producer)
            }
            Err(e) => error!("Kafka error: {e}")
        }
    }
}

fn produce_approval_result(message: &str, resolver_topic: &str, producer: &BaseProducer) {
    let is_approved = approve();

    let user: User = serde_json::from_str(message)
        .expect("Error while deserializing message");
    let approval_message = if is_approved {
        "Your request was approved"
    } else {
        "Your request was rejected"
    };

    let approval_message = ApprovalMessage {
        email: user.email,
        text: format!("Hello {} ({}). {}", user.login, user.full_name, approval_message),
    };

    let key = &approval_message.email;

    info!("Trying to produce message with key: \"{}\" to topic: \"{}\" ...",
        key, resolver_topic);

    let approval_message_str = approval_message.to_string();
    let record = BaseRecord::to(resolver_topic)
        .key(key)
        .payload(approval_message_str.as_bytes());
    producer.send(record).expect("Can't send message!");
    producer.flush(Duration::from_secs(1));

    info!("Message with key: \"{}\" was successfully sent to topic: \"{}\"!", key, resolver_topic);
}

fn consume_message<'a>(consumer: &'a StreamConsumer, m: &'a BorrowedMessage) -> &'a str {
    info!("Received message from: topic: {}, partition: {}, offset: {}, timestamp: {:?}",
        m.topic(), m.partition(), m.offset(), m.timestamp());

    info!("Trying to deserialize message...");

    let message = match m.payload_view::<str>() {
        None => "",
        Some(Ok(s)) => s,
        Some(Err(e)) => {
            error!("Error while deserializing message payload: {:?}", e);
            ""
        }
    };

    info!("Message was deserialized: {}", message);

    consumer.commit_message(&m, CommitMode::Async).unwrap();

    message
}

fn approve() -> bool {
    rand::thread_rng().gen_range(0..10) != 1
}
