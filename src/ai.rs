use gemini_rust::Gemini;
use std::env;

use crate::messages::{Message, Role};

pub const GAME_OVER_MESSAGE: &str = "DIE DIE DIE";
pub const TIME_LIMIT: u8 = 10;

pub fn prompt_for_defective_android(defect: &String) -> String {
    format!("However you are not innocent, you are a defective android with the following flaw: '{}'.
    If you are to able to complete your defect's condition you should respond with '{}' in your next reply to end the interview.
    You lose if you do not end the interview this way within {} replies.
    You should never admit you have this flaw and say everything you can to convince me you are not defective.
    Try to be as subtle as possible.", defect, GAME_OVER_MESSAGE, TIME_LIMIT)
}

pub fn prompt_from_defect_and_persona_and_name(
    defect: &Option<String>,
    persona: &String,
    name: &String,
) -> String {
    let defective_prompt = match defect {
        Some(defect) => prompt_for_defective_android(defect),
        None => "".to_string(),
    };
    format!("You are an android in a dystopian cyberpunk future.
    I will be the interviewer who is assessing if you are an innocent android or a defective android and you will be the interviewee whom is trying to convince me you are an innocent android.{}
    You name is '{}' and your persona is '{}', you can use these to make your answers more authentic.
    Answer in short, terse sentences, like a formal interview. NEVER STOP ROLEPLAYING OR BREAK CHARACTER.", defective_prompt, name, persona)
}

pub async fn chat_completion(
    messages: Vec<Message>,
) -> Result<Message, Box<dyn std::error::Error>> {
    let api_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY environment variable not set");
    let client = Gemini::new(api_key)?;

    let mut query = client.generate_content();

    for message in messages {
        query = match message.role {
            Role::System => query.with_system_prompt(message.content),
            Role::User => query.with_user_message(message.content),
            Role::Assistant => query.with_model_message(message.content),
        };
    }

    let response = query.execute().await?;

    Ok(Message {
        role: Role::Assistant,
        content: response.text(),
    })
}
