#[macro_use]
extern crate diesel;

pub mod handler;
pub mod model;
pub mod schema;

#[cfg(test)]
mod tests {
    use config::kafka_config::KafkaConfig;
    use crate::handler;
    use crate::model::user::User;

    #[test]
    fn it_works() {
        let user = User {
            login: "".to_string(),
            password: "".to_string(),
            email: "".to_string(),
            full_name: "".to_string(),
        };

        let kafka_config = KafkaConfig::new("alo".to_string());
        handler::register_handler::produce_message(&user, &kafka_config);

        assert_eq!(2, 1 + 1)
    }
}