use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ApprovalMessage {
    pub email: String,
    pub text: String
}