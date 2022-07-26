use std::env;
use log::{error};
use config::kafka_config::KafkaConfig;
use rdkafka::consumer::{Consumer};
use dotenv::*;
use resolver_service::{consume_message, produce_approval_result};

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
                produce_approval_result(&message, &resolver_topic, &producer)
            }
            Err(e) => error!("Kafka error: {e}")
        }
    }
}
