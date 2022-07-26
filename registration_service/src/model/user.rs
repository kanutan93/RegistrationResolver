use serde::{Serialize, Deserialize};
use crate::schema::users;

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name="users"]
pub struct User {
    pub login: String,
    pub password: String,
    pub email: String,
    pub full_name: String,
}

impl User {
    pub fn default() -> User {
        User {
            login: "".to_string(),
            password: "".to_string(),
            email: "".to_string(),
            full_name: "".to_string(),
        }
    }
}

impl ToString for User {
    fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}