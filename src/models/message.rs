use diesel::prelude::*;
use rocket::serde::Serialize;
use std::time::SystemTime;
use uuid::Uuid;

#[derive(Queryable, Serialize, Clone, Debug)]
#[diesel(table_name = messages)]
pub struct RecordedMessage {
    pub id: i32,
    pub role: String,
    pub content: String,
    pub created_at: SystemTime,
    pub chat_id: Uuid,
}
