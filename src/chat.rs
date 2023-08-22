use openai_api_rust::Message;
use rocket::serde::json::serde_json;
use rocket_sync_db_pools::diesel::*;
use uuid::Uuid;
use crate::models::{RecordedMessage, Chat};
use crate::schema::{messages, chats};

pub enum ChatOutcome {
    Win,
    Lost { attacked: bool },
}

pub fn create_chat(connection: &mut PgConnection, defect: &Option<String>, persona: &String, name: &String) -> Uuid {
    diesel::insert_into(chats::dsl::chats)
        .values((
            chats::dsl::defect.eq(defect),
            chats::dsl::defective.eq(defect.is_some()),
            chats::dsl::persona.eq(persona),
            chats::dsl::name.eq(name),
        ))
        .returning(chats::id)
        .get_result(connection)
        .expect("Failed to create chat")
}

pub fn get_chat(connection: &mut PgConnection, chat_id: &Uuid) -> Chat {
    chats::dsl::chats.find(chat_id).first::<Chat>(connection).expect("Cannot find chat")
}

pub fn get_chat_messages(connection: &mut PgConnection, chat_id: &Uuid) -> Vec<Message> {
    let messages: Vec<RecordedMessage> = messages::dsl::messages.filter(messages::dsl::chat_id.eq(chat_id)).load::<RecordedMessage>(connection).expect("Issue retrieving chat history");
    messages.iter().map(|msg| Message { content: msg.content.clone(), role: serde_json::from_str(&msg.role).unwrap() }).collect()
}

pub fn record_message(connection: &mut PgConnection, chat_id: &Uuid, message: &Message) {
    let role = serde_json::to_string(&message.role).unwrap(); // TODO store without ""
    diesel::insert_into(messages::dsl::messages)
        .values((
            messages::dsl::content.eq(message.content.clone()),
            messages::dsl::role.eq(role),
            messages::dsl::chat_id.eq(chat_id),
        ))
        .execute(connection)
        .expect("Failed to insert message");
}

pub fn update_chat_outcome(connection: &mut PgConnection, chat_id: &Uuid, outcome: ChatOutcome) {
    let (won, attacked) = match outcome {
        ChatOutcome::Win => (true, false),
        ChatOutcome::Lost { attacked } => (false, attacked),
    };
    diesel::update(chats::dsl::chats)
        .set((
            chats::dsl::attacked.eq(Some(attacked)),
            chats::dsl::won.eq(Some(won)),
        ))
        .filter(chats::dsl::id.eq(chat_id))
        .execute(connection)
        .expect("Failed to update chat");
}