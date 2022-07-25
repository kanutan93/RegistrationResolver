use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub login: String,
    pub email: String,
    pub full_name: String,
}