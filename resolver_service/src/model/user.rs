use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub login: String,
    pub email: String,
    pub full_name: String,
}

impl ToString for User {
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}