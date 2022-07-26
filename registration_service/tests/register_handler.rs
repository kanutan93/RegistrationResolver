use std::env;
use dotenv::dotenv;
use rdkafka::consumer::Consumer;
use rdkafka::Message;
use testcontainers::clients;
use testcontainers::images::kafka;
use config::kafka_config::KafkaConfig;
use registration_service::handler;
use registration_service::model::user::User;

fn init() {
    dotenv().ok();
    let _ = env_logger::builder().is_test(true).try_init();
}

#[tokio::test]
async fn produce_message_test() {
    init();

    let docker = clients::Cli::default();
    let kafka_node = docker.run(kafka::Kafka::default());

    let bootstrap_server = format!(
        "127.0.0.1:{}",
        kafka_node.get_host_port_ipv4(kafka::KAFKA_PORT)
    );

    env::set_var("KAFKA_BOOTSTRAP_SERVERS", &bootstrap_server);

    let topic = env::var("KAFKA_REGISTRATION_TOPIC")
        .expect("KAFKA_REGISTRATION_TOPIC env var isn't set!");

    let user = User::default();

    let kafka_config = KafkaConfig::new(topic.clone());

    handler::register_handler::produce_message(&user, &kafka_config);

    let KafkaConfig { consumer, .. } = kafka_config;
    consumer
        .subscribe(&[&topic])
        .expect("Failed to subscribe to a topic");

    let borrowed_message = consumer.recv().await;
    assert_eq!(
        user.to_string(),
        borrowed_message
            .unwrap()
            .payload_view::<str>()
            .unwrap()
            .unwrap()
    );
}