use serde::{Serialize, Deserialize};
use schema::users;
use crate::schema;

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name="users"]
pub struct User {
    pub login: String,
    pub password: String,
    pub email: String,
    pub full_name: String,
}

impl ToString for User {
    fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}