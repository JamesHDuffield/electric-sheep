#[macro_use] extern crate rocket;

mod schema;
mod models;
mod ai;

use models::*;
use openai_api_rust::Role;
use rocket::serde::json::Json;
use rocket::serde::json::serde_json;
use rocket_sync_db_pools::diesel::*;
use rocket_sync_db_pools::database;
use rand::seq::SliceRandom;
use rocket::serde::Serialize;
use uuid::Uuid;

#[database("db")]
struct PgDatabase(PgConnection);

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct StartResponse {
    chat_id: Uuid,
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

fn record_message(connection: &mut PgConnection, chat_id: &Uuid, role: &Role, content: &str) {
    diesel::insert_into(self::schema::messages::dsl::messages)
        .values((
            self::schema::messages::dsl::content.eq(content),
            self::schema::messages::dsl::role.eq(serde_json::to_string(role).unwrap()),
            self::schema::messages::dsl::chat_id.eq(chat_id),
        ))
        .execute(connection)
        .expect("Failed to insert message");
}

#[get("/")]
fn index() -> String {
    ai::chat_completion("Hello!".to_string()).unwrap()
}

#[get("/defect")]
async fn defect(db: PgDatabase) -> String {
    db.run(|connection| select_random_defect(connection)).await
}

#[post("/start")]
async fn start(db: PgDatabase) -> Json<StartResponse> {
    // Create a starting prompt and record chat and messages
    let (chat_id, prompt) = db.run(|connection| {
        let defect = select_random_defect(connection);
        let prompt = format!("You are a bot with defect '{}'", defect);
        let chat_id = create_chat(connection);
        record_message(connection, &chat_id, &Role::System, &prompt);
        (chat_id, prompt) 
    }).await;
    // TODO Send to AI to get started and post to queue
    println!("{}", prompt);
    // Response
    Json(StartResponse {
        chat_id
    })
}


#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(PgDatabase::fairing())
        .mount("/", routes![index, defect, start])
}


