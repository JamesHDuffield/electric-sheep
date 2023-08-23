use crate::models::{Chat, RecordedMessage};
use crate::schema::{chats, messages};
use openai_api_rust::Message;
use rocket::serde::json::serde_json;
use rocket_sync_db_pools::diesel::*;
use uuid::Uuid;

pub enum ChatOutcome {
    Win,
    Lost { attacked: bool },
}

pub fn create_chat(
    connection: &mut PgConnection,
    defect: &Option<String>,
    persona: &String,
    name: &String,
) -> QueryResult<Uuid> {
    diesel::insert_into(chats::dsl::chats)
        .values((
            chats::dsl::defect.eq(defect),
            chats::dsl::defective.eq(defect.is_some()),
            chats::dsl::persona.eq(persona),
            chats::dsl::name.eq(name),
        ))
        .returning(chats::id)
        .get_result(connection)
}

pub fn get_chat(connection: &mut PgConnection, chat_id: &Uuid) -> QueryResult<Chat> {
    chats::dsl::chats.find(chat_id).first::<Chat>(connection)
}

pub fn get_chat_messages(
    connection: &mut PgConnection,
    chat_id: &Uuid,
) -> QueryResult<Vec<Message>> {
    let messages: Vec<RecordedMessage> = messages::dsl::messages
        .filter(messages::dsl::chat_id.eq(chat_id))
        .load::<RecordedMessage>(connection)?;
    messages
        .into_iter()
        .map(|msg| {
            Ok::<Message, diesel::result::Error>(Message {
                content: msg.content.clone(),
                role: serde_json::from_str(&msg.role)
                    .map_err(|err| diesel::result::Error::DeserializationError(Box::new(err)))?,
            })
        })
        .collect()
}

pub fn record_message(
    connection: &mut PgConnection,
    chat_id: &Uuid,
    message: &Message,
) -> QueryResult<usize> {
    let role = serde_json::to_string(&message.role).unwrap(); // TODO store without ""
    diesel::insert_into(messages::dsl::messages)
        .values((
            messages::dsl::content.eq(message.content.clone()),
            messages::dsl::role.eq(role),
            messages::dsl::chat_id.eq(chat_id),
        ))
        .execute(connection)
}

pub fn update_chat_outcome(connection: &mut PgConnection, chat_id: &Uuid, outcome: ChatOutcome) -> QueryResult<usize> {
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
}
