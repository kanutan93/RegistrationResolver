use std::time::Duration;
use actix_web::{HttpResponse, post, web::{Json, Data}};
use rdkafka::producer::{BaseRecord, Producer};
use crate::model::user::User;
use crate::{AppConfig, KafkaConfig};

#[post("/register")]
pub async fn register(user: Json<User>, app_config: Data<AppConfig>) -> HttpResponse {
    let AppConfig {kafka_config} = app_config.get_ref();
    let KafkaConfig {producer, topic} = kafka_config;

    let payload = user.to_string();
    let record = BaseRecord::to(&topic)
        .key(user.get_login())
        .payload(payload.as_bytes());
    producer.send(record).expect("Can't send message!");
    producer.flush(Duration::from_secs(1));

    HttpResponse::Created().finish()
}