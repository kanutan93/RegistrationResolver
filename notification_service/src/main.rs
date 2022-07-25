mod model;

use std::env;
use dotenv::dotenv;
use log::{error, info};
use rdkafka::consumer::{CommitMode, Consumer, StreamConsumer};
use rdkafka::Message;
use rdkafka::message::BorrowedMessage;
use config::kafka_config::KafkaConfig;
use crate::model::approve_result::ApprovalMessage;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let resolver_topic = env::var("KAFKA_RESOLVER_TOPIC")
        .expect("KAFKA_RESOLVER_TOPIC env var isn't set!");

    let KafkaConfig {consumer, ..} = KafkaConfig::new(resolver_topic.clone());

    consumer
        .subscribe(&[&resolver_topic])
        .expect("Can't subscribe to topic!");

    loop {
        match consumer.recv().await {
            Ok(m) => {
                let message = consume_message(&m, &consumer);
                send_email(message)
            }
            Err(e) => error!("Kafka error: {}", e)
        }
    }
}

fn consume_message<'a>(m: &'a BorrowedMessage, consumer: &'a StreamConsumer) -> &'a str{
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

fn send_email(message: &str) {
    let approve_result: ApprovalMessage  = serde_json::from_str(message)
        .expect("Can't deserialize message");
    info!("Sending email: {:?}", approve_result)
}