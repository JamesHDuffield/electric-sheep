#[macro_use]
extern crate rocket;

mod ai;
mod chat;
mod errors;
mod messages;
mod models;
mod schema;

use std::path::Path;

use crate::errors::*;
use ai::*;
use chat::*;
use fake::faker::name::en::Name;
use fake::Fake;
use messages::*;
use models::*;
use openai_api_rust::Message;
use openai_api_rust::Role;
use rand::Rng;
use rocket::fs::NamedFile;
use rocket::fs::{relative, FileServer};
use rocket::response::status;
use rocket::response::stream::{Event, EventStream};
use rocket::serde::json::Json;
use rocket::tokio::select;
use rocket::tokio::sync::broadcast::{channel, error::RecvError, Sender};
use rocket::{Shutdown, State};
use rocket_sync_db_pools::database;
use rocket_sync_db_pools::diesel::*;
use uuid::Uuid;

#[database("db")]
struct PgDatabase(PgConnection);

#[post("/start")]
async fn start(db: PgDatabase) -> Result<Json<StartResponse>, ApiError> {
    // Create a starting prompt and record chat and messages
    let chat_id = db
        .run(|connection| {
            // Flip a coin to determine if android is going be a defect
            let is_defective = rand::thread_rng().gen_bool(0.5);
            let defect = if is_defective {
                let category = Categories::select_random(connection)?;
                Some(Defect::select_random_from_category(connection, &category)?)
            } else {
                None
            };
            let persona = Persona::select_random_persona(connection)?;
            let name: String = Name().fake();
            let prompt = prompt_from_defect_and_persona_and_name(&defect, &persona, &name);
            let chat_id = create_chat(connection, &defect, &persona, &name)?;
            let system_message = Message {
                role: Role::System,
                content: prompt,
            };
            record_message(connection, &chat_id, &system_message)?;
            Ok(chat_id) as QueryResult<Uuid>
        })
        .await?;
    Ok(Json(StartResponse { chat_id }))
}

#[get("/chat/<chat_id>")]
async fn get_chat_details(
    db: PgDatabase,
    chat_id: Uuid,
) -> Result<Json<ChatDetailsResponse>, ApiError> {
    let chat = db
        .run(move |connection| get_chat(connection, &chat_id))
        .await?;
    let result = chat.won.map(|win| SubmitResponse {
        defective: chat.defective,
        defect: chat.defect,
        win,
        attacked: chat.attacked.unwrap_or(true),
    });
    Ok(Json(ChatDetailsResponse {
        name: chat.name,
        persona: chat.persona,
        result,
    }))
}

#[get("/join/<chat_id>")]
async fn join(
    db: PgDatabase,
    chat_id: Uuid,
    queue: &State<Sender<QueueMessage>>,
    mut end: Shutdown,
) -> Result<EventStream![], ApiError> {
    let mut rx = queue.subscribe();
    // Get historical messages
    let records = db
        .run(move |connection| get_chat_messages(connection, &chat_id))
        .await?;

    Ok(EventStream! {
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
    })
}

#[post("/reply/<chat_id>", format = "json", data = "<body>")]
async fn reply(
    db: PgDatabase,
    chat_id: Uuid,
    body: Json<ReplyRequest>,
    queue: &State<Sender<QueueMessage>>,
) -> Result<Json<ReplyResponse>, ApiError> {
    let user_message = Message {
        role: Role::User,
        content: body.content.to_owned(),
    };
    // Notify queue - A send 'fails' if there are no active subscribers. That's okay.
    let _res = queue.send(QueueMessage {
        chat_id: chat_id.to_owned(),
        message: user_message.to_owned(),
    });
    // Get history
    let mut history = db
        .run(move |connection| get_chat_messages(connection, &chat_id))
        .await?;
    // Append new message
    history.push(user_message.clone());
    // Send to AI
    let ai_response = ai::chat_completion(history)?;
    // Check to see if ai won
    let lose = ai_response.content.contains(GAME_OVER_MESSAGE);
    // Notify queue - A send 'fails' if there are no active subscribers. That's okay.
    let _res = queue.send(QueueMessage {
        chat_id: chat_id.clone(),
        message: ai_response.clone(),
    });
    // Record
    db.run(move |connection| {
        record_message(connection, &chat_id, &user_message)?;
        record_message(connection, &chat_id, &ai_response)?;
        Ok(()) as QueryResult<()>
    })
    .await?;

    let result = match lose {
        true => {
            let chat = db
                .run(move |connection| {
                    update_chat_outcome(
                        connection,
                        &chat_id,
                        ChatOutcome::Lost { attacked: true },
                    )?;
                    get_chat(connection, &chat_id)
                })
                .await?;
            Some(SubmitResponse {
                defective: chat.defective,
                defect: chat.defect,
                win: chat.won.unwrap_or(false),
                attacked: chat.attacked.unwrap_or(true),
            })
        }
        false => None,
    };

    Ok(Json(ReplyResponse { result }))
}

#[post("/submit/<chat_id>", format = "json", data = "<body>")]
async fn submit(
    db: PgDatabase,
    chat_id: Uuid,
    body: Json<SubmitRequest>,
) -> Result<Json<SubmitResponse>, ApiError> {
    // Get chat
    let chat = db
        .run(move |connection| {
            let chat = get_chat(connection, &chat_id)?;
            let outcome = match chat.defective == body.defective {
                true => ChatOutcome::Win,
                false => ChatOutcome::Lost { attacked: false },
            };
            update_chat_outcome(connection, &chat_id, outcome)?;
            get_chat(connection, &chat_id)
        })
        .await?;
    // Compare result
    Ok(Json(SubmitResponse {
        defective: chat.defective,
        defect: chat.defect,
        win: chat.won.unwrap_or(false),
        attacked: chat.attacked.unwrap_or(true),
    }))
}

#[catch(404)]
async fn fallback_to_index() -> status::Custom<Option<NamedFile>> {
    status::Custom(
        rocket::http::Status::Ok,
        NamedFile::open(Path::new(relative!("/site/build/index.html")))
            .await
            .ok(),
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(PgDatabase::fairing())
        .manage(channel::<QueueMessage>(1024).0)
        .mount(
            "/api",
            routes![start, get_chat_details, join, reply, submit],
        )
        .mount("/", FileServer::from(relative!("/site/build")))
        .register("/", catchers![fallback_to_index])
}
