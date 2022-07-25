use std::env;
use rdkafka::ClientConfig;
use rdkafka::error::KafkaResult;
use rdkafka::producer::BaseProducer;

pub struct KafkaConfig {
    pub producer: BaseProducer,
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
            .expect("Can't create kafka producer!")
    }

    pub fn new(topic: String) -> KafkaConfig {
        KafkaConfig {
            producer: KafkaConfig::producer(),
            topic
        }
    }
}