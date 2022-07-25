#[macro_use]
extern crate diesel;

mod handler;
mod model;
mod config;
mod schema;

use std::env;
use actix_web::{App, HttpServer, web::Data};
use tracing_actix_web::{DefaultRootSpanBuilder, TracingLogger};
use config::kafka_config;
use crate::config::db_config::DbConfig;
use crate::handler::register_handler;
use crate::kafka_config::KafkaConfig;

pub struct AppConfig {
    kafka_config: KafkaConfig,
    db_config: DbConfig,
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
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL env var isn't set!");

    HttpServer::new(move || App::new()
        .wrap(TracingLogger::<DefaultRootSpanBuilder>::new())
        .app_data(Data::new(AppConfig {
            kafka_config: KafkaConfig::new(topic.clone()),
            db_config: DbConfig::new(database_url.clone()),
        }))
        .service(register_handler::register))
        .bind((ip, port.parse().unwrap()))?
        .run()
        .await
}