use rocket::serde::{Serialize, Deserialize};
use uuid::Uuid;
use openai_api_rust::Message;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct StartResponse {
    pub chat_id: Uuid,
    pub questions: Vec<String>,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ReplyRequest {
    pub content: String,
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
    pub defect: Option<String>,
}

#[derive(Debug, Clone)]
pub struct QueueMessage {
    pub chat_id: Uuid,
    pub message: Message
}