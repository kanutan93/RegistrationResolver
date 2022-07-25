mod handler;
mod model;
mod config;

use std::env;
use actix_web::{App, HttpServer, web::Data};
use rdkafka::ClientConfig;
use rdkafka::producer::BaseProducer;
use config::kafka_config;
use crate::handler::register_handler;
use crate::kafka_config::KafkaConfig;

struct AppConfig {
    kafka_config: KafkaConfig,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let ip = env::var("APP_IP")
        .expect("APP_IP env var isn't set!");
    let port = env::var("APP_PORT")
        .expect("APP_PORT env var isn't set!");
    let topic = env::var("KAFKA_REGISTRATION_TOPIC")
        .expect("KAFKA_REGISTRATION_TOPIC env var isn't set!");

    HttpServer::new(move || App::new()
        .app_data(Data::new(AppConfig {
            kafka_config: KafkaConfig::new(topic.clone())
        }))
        .service(register_handler::register))
        .bind((ip, port.parse().unwrap()))?
        .run()
        .await
}