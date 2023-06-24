#[macro_use] extern crate rocket;

mod schema;
mod models;
mod ai;

use models::*;
use rocket::serde::json::Json;
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
    let chat_id = db.run(|connection| create_chat(connection)).await;
    
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


