use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Debug)]
#[diesel(table_name = chats)]
pub struct Chat {
    pub id: Uuid,
    pub defective: bool,
    pub defect: Option<String>,
    pub persona: String,
    pub name: String,
}
