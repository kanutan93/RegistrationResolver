use std::env;
use std::time::Duration;
use rdkafka::consumer::Consumer;
use rdkafka::Message;
use rdkafka::producer::{BaseRecord, Producer};
use testcontainers::clients;
use testcontainers::images::kafka;
use config::kafka_config::KafkaConfig;
use notification_service::{consume_message, send_email};
use notification_service::model::approve_result::ApprovalMessage;

fn init() {
    let _ = env_logger::builder().is_test(true).try_init();
}

#[tokio::test]
async fn consume_message_test() {
    init();

    let docker = clients::Cli::default();
    let kafka_node = docker.run(kafka::Kafka::default());

    let bootstrap_server = format!(
        "127.0.0.1:{}",
        kafka_node.get_host_port_ipv4(kafka::KAFKA_PORT)
    );

    env::set_var("KAFKA_BOOTSTRAP_SERVERS", &bootstrap_server);

    let topic = "resolver";

    let kafka_config = KafkaConfig::new(topic.to_string());

    let KafkaConfig { consumer, producer, .. } = kafka_config;

    let approval_message = ApprovalMessage {
        email: "".to_string(),
        text: "".to_string()
    };

    let approval_message_str = approval_message.to_string();
    let record = BaseRecord::to(topic)
        .key(&approval_message.email)
        .payload(approval_message_str.as_bytes());
    producer.send(record).expect("Can't send message!");
    producer.flush(Duration::from_secs(1));

    consumer
        .subscribe(&[&topic])
        .expect("Failed to subscribe to a topic");

    let borrowed_message = consumer.recv().await.unwrap();
    consume_message(&borrowed_message, &consumer);

    assert_eq!(
        &approval_message_str,
        borrowed_message
            .payload_view::<str>()
            .unwrap()
            .unwrap()
    );
}

#[tokio::test]
async fn send_email_test() {
    init();

    let approval_message = ApprovalMessage {
        email: "".to_string(),
        text: "".to_string()
    };
    send_email(approval_message);
}