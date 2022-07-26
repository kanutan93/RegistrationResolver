pub mod model;

use std::time::Duration;
use log::{error, info};
use rand::Rng;
use rdkafka::consumer::{CommitMode, Consumer, StreamConsumer};
use rdkafka::Message;
use rdkafka::message::BorrowedMessage;
use rdkafka::producer::{BaseProducer, BaseRecord, Producer};
use crate::model::approve_result::ApprovalMessage;
use crate::model::user::User;

pub fn produce_approval_result(approval_message: &ApprovalMessage, resolver_topic: &str, producer: &BaseProducer) {
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

pub fn consume_message(consumer: & StreamConsumer, m: &BorrowedMessage) -> ApprovalMessage {
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

    let is_approved = approve();

    let user: User = serde_json::from_str(message)
        .expect("Error while deserializing message");
    let approval_message = if is_approved {
        "Your request was approved"
    } else {
        "Your request was rejected"
    };

    ApprovalMessage {
        email: user.email,
        text: format!("Hello {} ({}). {}", user.login, user.full_name, approval_message),
    }
}

fn approve() -> bool {
    rand::thread_rng().gen_range(0..10) != 1
}