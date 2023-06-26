#[macro_use] extern crate rocket;

mod schema;
mod models;
mod ai;

use models::*;
use models::RecordedMessage;
use openai_api_rust::Message;
use openai_api_rust::Role;
use rocket::form::Form;
use rocket::response::stream::{EventStream, Event};
use rocket::tokio::sync::broadcast::{channel, Sender, error::RecvError};
use rocket::{State, Shutdown};
use rocket::serde::json::Json;
use rocket::serde::json::serde_json;
use rocket_sync_db_pools::diesel::*;
use rocket_sync_db_pools::database;
use rand::seq::SliceRandom;
use rocket::serde::Serialize;
use uuid::Uuid;
use rocket::tokio::select;

#[database("db")]
struct PgDatabase(PgConnection);

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct StartResponse {
    chat_id: Uuid,
}

#[derive(Debug, Clone, FromForm)]
struct ReplyRequest {
    content: String,
}

#[derive(Debug, Clone)]
struct QueueMessage {
    chat_id: Uuid,
    message: Message
}

fn select_random_defect(connection: &mut PgConnection) -> String {
    let categories = self::schema::categories::dsl::categories.load::<Categories>(connection).expect("Issue retrieving categories");
    let category = categories.choose(&mut rand::thread_rng()).expect("Issue selecting category");
    let defects = self::schema::defects::dsl::defects.filter(schema::defects::category_id.eq(category.id)).load::<Defect>(connection).expect("Issue retrieving defects");
    let defect = defects.choose(&mut rand::thread_rng()).expect("Issue selecting defect");
    defect.text.clone()
}

fn create_chat(connection: &mut PgConnection) -> Uuid {
    diesel::insert_into(self::schema::chats::dsl::chats).default_values().returning(schema::chats::id).get_result(connection).expect("Failed to create chat")
}

fn get_chat_messages(connection: &mut PgConnection, chat_id: &Uuid) -> Vec<Message> {
    let messages: Vec<RecordedMessage> = self::schema::messages::dsl::messages.filter(self::schema::messages::dsl::chat_id.eq(chat_id)).load::<RecordedMessage>(connection).expect("Issue retrieving chat history");
    messages.iter().map(|msg| Message { content: msg.content.clone(), role: serde_json::from_str(&msg.role).unwrap() }).collect()
}

fn record_message(connection: &mut PgConnection, chat_id: &Uuid, message: &Message) {
    let role = serde_json::to_string(&message.role).unwrap(); // TODO store without ""
    println!("{}", role);
    diesel::insert_into(self::schema::messages::dsl::messages)
        .values((
            self::schema::messages::dsl::content.eq(message.content.clone()),
            self::schema::messages::dsl::role.eq(role),
            self::schema::messages::dsl::chat_id.eq(chat_id),
        ))
        .execute(connection)
        .expect("Failed to insert message");
}

#[post("/start")]
async fn start(db: PgDatabase) -> Json<StartResponse> {
    // Create a starting prompt and record chat and messages
    let (chat_id, prompt) = db.run(|connection| {
        let defect = select_random_defect(connection);
        let prompt = format!("You are a bot with defect '{}'", defect);
        let chat_id = create_chat(connection);
        (chat_id, prompt) 
    }).await;
    // Send to AI
    let system_message = Message { role: Role::System, content: prompt };
    let ai_response = ai::chat_completion(vec![ system_message.clone() ]).unwrap();
    // Record messages to DB
    db.run(move |connection| {
        record_message(connection, &chat_id, &system_message);
        record_message(connection, &chat_id, &ai_response);
    }).await;
    Json(StartResponse {
        chat_id
    })
}

#[get("/join/<chat_id>")]
async fn join(db: PgDatabase, chat_id: Uuid, queue: &State<Sender<QueueMessage>>, mut end: Shutdown) -> EventStream![] {
    let mut rx = queue.subscribe();
    // Get historical messages
    let records = db.run(move |connection| get_chat_messages(connection, &chat_id)).await;
    EventStream! {
        // Pipe out existing
        for record in records {
            yield Event::json(&record);
        }
        // Listen for new messages
        loop {
            let msg = select! {
                msg = rx.recv() => match msg {
                    Ok(msg) => {
                        if msg.chat_id != chat_id {
                            continue;
                        }
                        msg.message
                    },
                    Err(RecvError::Closed) => break,
                    Err(RecvError::Lagged(_)) => continue,
                },
                _ = &mut end => break,
            };

            yield Event::json(&msg);
        }
    }
}

#[post("/reply/<chat_id>", data = "<form>")]
async fn reply(db: PgDatabase, chat_id: Uuid, form: Form<ReplyRequest>, queue: &State<Sender<QueueMessage>>) {
    // Get history
    let mut history = db.run(move |connection| get_chat_messages(connection, &chat_id)).await;
    // Append new message    
    let user_message = Message { role: Role::User, content: form.content.clone() };
    history.push(user_message.clone());
    // Notify queue - A send 'fails' if there are no active subscribers. That's okay.
    let _res = queue.send(QueueMessage { chat_id: chat_id.clone(), message: user_message.clone() });
    // Send to AI
    let ai_response = ai::chat_completion(history).unwrap();
    // Notify queue - A send 'fails' if there are no active subscribers. That's okay.
    let _res = queue.send(QueueMessage { chat_id: chat_id.clone(), message: ai_response.clone() });
    // Record
    db.run(move |connection| {
        record_message(connection, &chat_id, &user_message);
        record_message(connection, &chat_id, &ai_response);
    }).await;
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(PgDatabase::fairing())
        .manage(channel::<QueueMessage>(1024).0)
        .mount("/", routes![start, join, reply])
}


