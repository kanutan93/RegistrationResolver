use std::env;
use std::future::Future;
use log::{error, info};
use config::kafka_config::KafkaConfig;
use rdkafka::{ClientConfig, Message};
use rdkafka::consumer::{CommitMode, Consumer};
use dotenv::*;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let registration_topic = env::var("KAFKA_REGISTRATION_TOPIC")
        .expect("KAFKA_REGISTRATION_TOPIC env var isn't set!");
    let resolver_topic = env::var("KAFKA_RESOLVER_TOPIC")
        .expect("KAFKA_RESOLVER_TOPIC env var isn't set!");

    let KafkaConfig{ consumer, producer, .. } = KafkaConfig::new(registration_topic.clone());

    consumer
        .subscribe(&[registration_topic.as_str()])
        .expect("Can't subscribe to specified topics");

    loop {
        match consumer.recv().await {
            Ok(m) => {
                let payload = match m.payload_view::<str>() {
                    None => "",
                    Some(Ok(s)) => s,
                    Some(Err(e)) => {
                        error!("Error while deserializing message payload: {:?}", e);
                        ""
                    }
                };
                info!("Received message: payload: '{}', topic: {}, partition: {}, offset: {}, timestamp: {:?}",
                      payload, m.topic(), m.partition(), m.offset(), m.timestamp());

                consumer.commit_message(&m, CommitMode::Async).unwrap();
            },
            Err(e) => error!("Kafka error: {e}")
        }
    }
}
