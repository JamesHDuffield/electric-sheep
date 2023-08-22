#[macro_use] extern crate rocket;

mod schema;
mod models;
mod ai;
mod chat;
mod messages;

use std::path::Path;

use ai::*;
use fake::Fake;
use fake::faker::name::en::Name;
use models::*;
use chat::*;
use messages::*;
use openai_api_rust::Message;
use openai_api_rust::Role;
use rocket::fs::NamedFile;
use rocket::response::status;
use rocket::response::stream::{EventStream, Event};
use rocket::tokio::sync::broadcast::{channel, Sender, error::RecvError};
use rocket::{State, Shutdown};
use rocket::fs::{FileServer, relative};
use rocket::serde::json::Json;
use rocket_sync_db_pools::diesel::*;
use rocket_sync_db_pools::database;
use uuid::Uuid;
use rocket::tokio::select;
use rand::Rng;

#[database("db")]
struct PgDatabase(PgConnection);

#[post("/start")]
async fn start(db: PgDatabase) -> Json<StartResponse> {
    // Create a starting prompt and record chat and messages
    let chat_id = db.run(|connection| {
        // Flip a coin to determine if android is going be a defect
        let is_defective = rand::thread_rng().gen_bool(0.5);
        let category = Categories::select_random(connection);
        let defect = match is_defective {
            true => Some(Defect::select_random_from_category(connection, &category)),
            false => None
        };
        let persona = Persona::select_random_persona(connection);
        let name: String = Name().fake();
        let prompt = prompt_from_defect_and_persona_and_name(&defect, &persona, &name);
        let chat_id = create_chat(connection, &defect, &persona, &name);
        let system_message = Message { role: Role::System, content: prompt };
        record_message(connection, &chat_id, &system_message);
        chat_id
    }).await;
    Json(StartResponse {
        chat_id,
    })
}

#[get("/chat/<chat_id>")]
async fn get_chat_details(db: PgDatabase, chat_id: Uuid) -> Json<ChatDetailsResponse> {
    let chat = db.run(move |connection| get_chat(connection, &chat_id)).await;
    Json(ChatDetailsResponse {
        name: chat.name,
        persona: chat.persona,
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
            match record.role {
                Role::System => continue,
                _ => yield Event::json(&record),
            }
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

#[post("/reply/<chat_id>", format = "json", data = "<body>")]
async fn reply(db: PgDatabase, chat_id: Uuid, body: Json<ReplyRequest>, queue: &State<Sender<QueueMessage>>) -> Json<ReplyResponse> {    
    let user_message = Message { role: Role::User, content: body.content.clone() };
    // Notify queue - A send 'fails' if there are no active subscribers. That's okay.
    let _res = queue.send(QueueMessage { chat_id: chat_id.clone(), message: user_message.clone() });
    // Get history
    let mut history = db.run(move |connection| get_chat_messages(connection, &chat_id)).await;
    // Append new message
    history.push(user_message.clone());
    // Send to AI
    let ai_response = ai::chat_completion(history).unwrap();
    // Check to see if ai won
    let lose = ai_response.content.contains(GAME_OVER_MESSAGE);
    // Notify queue - A send 'fails' if there are no active subscribers. That's okay.
    let _res = queue.send(QueueMessage { chat_id: chat_id.clone(), message: ai_response.clone() });
    // Record
    db.run(move |connection| {
        record_message(connection, &chat_id, &user_message);
        record_message(connection, &chat_id, &ai_response);
    }).await;

    let result = match lose {
        true => {
            let chat = db.run(move |connection| get_chat(connection, &chat_id)).await;
            Some(SubmitResponse { win: false, defective: chat.defective, defect: chat.defect})
        },
        false => None,
    };

    Json(ReplyResponse { result })
}

#[post("/submit/<chat_id>", format = "json", data = "<body>")]
async fn submit(db: PgDatabase, chat_id: Uuid, body: Json<SubmitRequest>) -> Json<SubmitResponse> {
    // Get chat
    let chat = db.run(move |connection| get_chat(connection, &chat_id)).await;
    // Compare result
    Json(SubmitResponse {
        defective: chat.defective,
        defect: chat.defect,
        win: chat.defective == body.defective,
    })
}

#[catch(404)]
async fn fallback_to_index() -> status::Custom<Option<NamedFile>> {
    status::Custom(rocket::http::Status::Ok, NamedFile::open(Path::new(relative!("/site/build/index.html"))).await.ok())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(PgDatabase::fairing())
        .manage(channel::<QueueMessage>(1024).0)
        .mount("/api", routes![start, get_chat_details, join, reply, submit])
        .mount("/", FileServer::from(relative!("/site/build")))
        .register("/", catchers![fallback_to_index])
}


