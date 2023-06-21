#[macro_use] extern crate rocket;
extern crate dotenv;

use rocket::serde::{Deserialize, json::Json};
use openai_api_rust::*;
use openai_api_rust::chat::*;
use dotenv::dotenv;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct Data<'a> {
    description: &'a str,
    complete: bool
}

#[get("/")]
fn index() -> String {
    chat("Hello!".to_string()).unwrap()
}

#[post("/todo", data = "<data>")]
fn todo(data: Json<Data<'_>>) -> &str {
    data.description.clone()
}

#[get("/hello/<name>")]
fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![todo])
        .mount("/", routes![hello])
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

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::local::blocking::Client;
    use rocket::http::Status;

    #[test]
    fn index() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get(uri!(crate::index)).dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), "Hello, world!");
    }
}