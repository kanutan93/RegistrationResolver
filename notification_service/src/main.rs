use std::env;
use dotenv::dotenv;
use log::error;
use rdkafka::consumer::Consumer;
use config::kafka_config::KafkaConfig;
use notification_service::{consume_message, send_email};

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