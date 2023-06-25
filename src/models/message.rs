use std::time::SystemTime;
use diesel::prelude::*;
use uuid::Uuid;
use rocket::serde::Serialize;

#[derive(Queryable, Serialize, Clone, Debug)]
#[diesel(table_name = messages)]
pub struct RecordedMessage {
    pub id: i32,
    pub role: String,
    pub content: String,
    pub created_at: SystemTime,
    pub chat_id: Uuid,
}