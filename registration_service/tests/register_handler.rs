use std::env;
use rdkafka::consumer::Consumer;
use rdkafka::Message;
use testcontainers::{clients, images};
use testcontainers::core::WaitFor;
use testcontainers::images::kafka;
use config::db_config::DbConfig;
use config::kafka_config::KafkaConfig;
use registration_service::handler;
use registration_service::model::user::User;
#[macro_use]
extern crate diesel_migrations;
use diesel_migrations::embed_migrations;

embed_migrations!("migrations/");

fn init() {
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

    let topic = "registration";

    let user = User::default();

    let kafka_config = KafkaConfig::new(topic.to_string());

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

#[test]
fn save_user_to_db_test() {
    init();

    let docker = clients::Cli::default();
    let db = "postgres-db-test";
    let user = "postgres-user-test";
    let password = "postgres-password-test";

    let generic_postgres = images::generic::GenericImage::new("postgres", "9.6-alpine")
        .with_wait_for(WaitFor::message_on_stderr(
            "database system is ready to accept connections",
        ))
        .with_env_var("POSTGRES_DB", db)
        .with_env_var("POSTGRES_USER", user)
        .with_env_var("POSTGRES_PASSWORD", password);
    let postgres_node = docker.run(generic_postgres);

    let database_url = &format!(
        "postgres://{}:{}@127.0.0.1:{}/{}",
        user,
        password,
        postgres_node.get_host_port_ipv4(5432),
        db
    );
    env::set_var("DATABASE_URL", database_url);

    let db_config = DbConfig::new(database_url.clone());

    let DbConfig {conn} = &db_config;
    embedded_migrations::run(conn).expect("Can't run migrations");

    let mut user = User::default();
    handler::register_handler::save_user_to_db(&mut user, &db_config)
}