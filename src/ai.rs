use openai_api_rust::chat::*;
use openai_api_rust::*;

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

pub fn chat_completion(messages: Vec<Message>) -> Result<Message, Error> {
    let auth = Auth::from_env().map_err(|error| Error::ApiError(error))?;
    let openai = OpenAI::new(auth, "https://api.openai.com/v1/");
    let body = ChatBody {
        model: "gpt-4o".to_string(),
        max_tokens: Some(100),
        temperature: Some(0_f32),
        top_p: Some(0_f32),
        stream: None,
        stop: None,
        presence_penalty: None,
        frequency_penalty: None,
        logit_bias: None,
        user: None,
        n: None,
        messages,
    };
    let response: completions::Completion = openai.chat_completion_create(&body)?;
    let message = response
        .choices
        .first()
        .ok_or(Error::ApiError("AI generated no responses".to_string()))?
        .message
        .clone()
        .ok_or(Error::ApiError(
            "AI generated no message in first response".to_string(),
        ))?;
    Ok(message)
}
