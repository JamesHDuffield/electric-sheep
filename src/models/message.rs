use std::time::SystemTime;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Debug)]
#[diesel(table_name = messages)]
pub struct Message {
    pub id: i32,
    pub role: String,
    pub content: String,
    pub created_at: SystemTime,
    pub chat_id: Uuid,
}