use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ApprovalMessage {
    pub email: String,
    pub text: String
}

impl ToString for ApprovalMessage {
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}