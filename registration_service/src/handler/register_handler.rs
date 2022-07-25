use std::time::Duration;
use actix_web::{HttpResponse, post, web::{Json, Data}};
use diesel::associations::HasTable;
use diesel::{RunQueryDsl};
use log::info;
use rdkafka::producer::{BaseRecord, Producer};
use config::db_config::DbConfig;
use config::kafka_config::KafkaConfig;
use crate::AppConfig;
use crate::model::user::User;
use crate::schema::users::dsl::users;

#[post("/register")]
pub async fn register(user: Json<User>, app_config: Data<AppConfig>) -> HttpResponse {
    let AppConfig {kafka_config, db_config} = app_config.get_ref();
    let user = &user.0;

    save_user_to_db(user, db_config);
    produce_message(user, kafka_config);

    HttpResponse::Created().finish()
}

fn save_user_to_db(user: &User, db_config: &DbConfig) {
    let DbConfig {conn} = db_config;

    info!("Trying to save user: \"{}\" to db...", user.login);

    diesel::insert_into(users::table())
        .values(user)
        .execute(conn)
        .expect("Can't save user to db!");

    info!("User \"{}\" was saved successfully to db!", user.login);
}

fn produce_message(user: &User, kafka_config: &KafkaConfig) {
    let KafkaConfig {producer, topic, ..} = kafka_config;

    let key = &user.login;
    let payload = user.to_string();

    info!("Trying to produce message with key: \"{}\" to topic: \"{}\" ...", key, topic);

    let record = BaseRecord::to(&topic)
        .key(key)
        .payload(payload.as_bytes());

    producer.send(record).expect("Can't send message!");
    producer.flush(Duration::from_secs(1));

    info!("Message with key: \"{}\" was successfully sent to topic: \"{}\"!", key, topic);
}