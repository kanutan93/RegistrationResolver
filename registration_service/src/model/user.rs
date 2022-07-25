use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    login: String,
    password: String,
    email: String,
    full_name: String,
}

impl User {
    pub fn get_login(&self) -> &str {
        &self.login
    }
}

impl ToString for User {
    fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}