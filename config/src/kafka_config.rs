use std::env;
use rdkafka::ClientConfig;
use rdkafka::consumer::{StreamConsumer};
use rdkafka::producer::BaseProducer;

pub struct KafkaConfig {
    pub producer: BaseProducer,
    pub consumer: StreamConsumer,
    pub topic: String,
}

impl KafkaConfig {
    fn producer() -> BaseProducer {
        let bootstrap_server = env::var("KAFKA_BOOTSTRAP_SERVERS")
            .expect("KAFKA_BOOTSTRAP_SERVERS env var isn't set!");
        let message_timeout = env::var("KAFKA_MESSAGE_TIMEOUT_MS")
            .unwrap_or(String::from("5000"));

        ClientConfig::new()
            .set("bootstrap.servers", bootstrap_server)
            .set("message.timeout.ms", message_timeout)
            .create()
            .expect("Producer creation failed")
    }

    fn consumer() -> StreamConsumer {
        let bootstrap_server = env::var("KAFKA_BOOTSTRAP_SERVERS")
            .expect("KAFKA_BOOTSTRAP_SERVERS env var isn't set!");

        ClientConfig::new()
            .set("group.id", "resolver")
            .set("bootstrap.servers", bootstrap_server)
            .set("auto.offset.reset", "earliest")
            .set("session.timeout.ms", "6000")
            .create()
            .expect("Consumer creation failed")
    }

    pub fn new(topic: String) -> KafkaConfig {
        KafkaConfig {
            producer: KafkaConfig::producer(),
            consumer: KafkaConfig::consumer(),
            topic
        }
    }
}