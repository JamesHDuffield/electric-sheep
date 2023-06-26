use openai_api_rust::Message;
use rocket::serde::json::serde_json;
use rocket_sync_db_pools::diesel::*;
use uuid::Uuid;
use crate::models::RecordedMessage;

pub fn create_chat(connection: &mut PgConnection) -> Uuid {
    diesel::insert_into(crate::schema::chats::dsl::chats).default_values().returning(crate::schema::chats::id).get_result(connection).expect("Failed to create chat")
}

pub fn get_chat_messages(connection: &mut PgConnection, chat_id: &Uuid) -> Vec<Message> {
    let messages: Vec<RecordedMessage> = crate::schema::messages::dsl::messages.filter(crate::schema::messages::dsl::chat_id.eq(chat_id)).load::<RecordedMessage>(connection).expect("Issue retrieving chat history");
    messages.iter().map(|msg| Message { content: msg.content.clone(), role: serde_json::from_str(&msg.role).unwrap() }).collect()
}

pub fn record_message(connection: &mut PgConnection, chat_id: &Uuid, message: &Message) {
    let role = serde_json::to_string(&message.role).unwrap(); // TODO store without ""
    println!("{}", role);
    diesel::insert_into(crate::schema::messages::dsl::messages)
        .values((
            crate::schema::messages::dsl::content.eq(message.content.clone()),
            crate::schema::messages::dsl::role.eq(role),
            crate::schema::messages::dsl::chat_id.eq(chat_id),
        ))
        .execute(connection)
        .expect("Failed to insert message");
}