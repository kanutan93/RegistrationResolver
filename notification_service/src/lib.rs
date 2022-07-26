pub mod model;

use log::{error, info};
use rdkafka::consumer::{CommitMode, Consumer, StreamConsumer};
use rdkafka::Message;
use rdkafka::message::BorrowedMessage;
use crate::model::approve_result::ApprovalMessage;

pub fn consume_message(m: & BorrowedMessage, consumer: &StreamConsumer) -> ApprovalMessage{
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

    serde_json::from_str(message)
        .expect("Can't deserialize message")
}

pub fn send_email(approve_result: ApprovalMessage) {
    info!("Sending email: {:?}", approve_result)
}