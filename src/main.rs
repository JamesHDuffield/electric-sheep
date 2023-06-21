#[macro_use] extern crate rocket;

use rocket_sync_db_pools::{diesel, database};
use openai_api_rust::*;
use openai_api_rust::chat::*;

#[database("db")]
struct PgDatabase(diesel::PgConnection);

fn load_from_db(_connection: &diesel::PgConnection) -> () {
    // Do something with connection, return some data.
    println!("made it");
}

#[get("/")]
async fn index(db: PgDatabase) -> () {
    db.run(|connection| load_from_db(connection)).await;
    chat("Hello!".to_string()).unwrap()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(PgDatabase::fairing())
        .mount("/", routes![index])
}

fn chat(input: String) -> Result<String, Error> {
    let auth = Auth::from_env().map_err(|error| Error::ApiError(error))?;
    let openai = OpenAI::new(auth, "https://api.openai.com/v1/");
    let body = ChatBody {
        model: "gpt-3.5-turbo".to_string(),
        max_tokens: Some(7),
        temperature: Some(0_f32),
        top_p: Some(0_f32),
        stream: None,
        stop: None,
        presence_penalty: None,
        frequency_penalty: None,
        logit_bias: None,
        user: None,
        n: None,
        messages: vec![Message { role: Role::User, content: input }],
    };
    let response: completions::Completion = openai.chat_completion_create(&body)?;
    let message = response
        .choices
        .first()
        .ok_or(Error::ApiError("AI generated no responses".to_string()))?
        .message
        .clone()
        .ok_or(Error::ApiError("AI generated no message in first response".to_string()))?;
    Ok(message.content)
}
