use std::ops::Deref;
use std::time::Duration;
use actix_web::{HttpResponse, post, web::{Json, Data}};
use diesel::associations::HasTable;
use diesel::{RunQueryDsl};
use log::info;
use rdkafka::producer::{BaseRecord, Producer};
use config::db_config::DbConfig;
use config::kafka_config::KafkaConfig;
use crate::model::user::User;
use crate::schema::users::dsl::users;

#[post("/register")]
pub async fn register(mut user: Json<User>, app_config: Data<(KafkaConfig, DbConfig)>) -> HttpResponse {
    let (kafka_config, db_config) = app_config.get_ref();
    let user = &mut user.0;

    save_user_to_db(user, db_config);
    produce_message(user, kafka_config);

    HttpResponse::Created().finish()
}

pub fn save_user_to_db(user: &mut User, db_config: &DbConfig) {
    let DbConfig {conn} = db_config;

    info!("Trying to save user: \"{}\" to db...", user.login);

    user.password = bcrypt::hash(&user.password, 4).unwrap();

    diesel::insert_into(users::table())
        .values(user.deref())
        .execute(conn)
        .expect("Can't save user to db!");

    info!("User \"{}\" has been successfully saved to db!", user.login);
}

pub fn produce_message(user: &User, kafka_config: &KafkaConfig) {
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