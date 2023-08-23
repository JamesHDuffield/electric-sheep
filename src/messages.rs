use openai_api_rust::Message;
use rocket::serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct StartResponse {
    pub chat_id: Uuid,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ReplyRequest {
    pub content: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ReplyResponse {
    pub result: Option<SubmitResponse>,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct SubmitRequest {
    pub defective: bool,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct SubmitResponse {
    pub defective: bool,
    pub win: bool,
    pub attacked: bool,
    pub defect: Option<String>,
}

#[derive(Debug, Clone)]
pub struct QueueMessage {
    pub chat_id: Uuid,
    pub message: Message,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ChatDetailsResponse {
    pub name: String,
    pub persona: String,
    pub result: Option<SubmitResponse>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ErrorResponse {
    pub error_message: String,
}
