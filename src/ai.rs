use openai_api_rust::*;
use openai_api_rust::chat::*;

pub fn prompt_for_defective_android(defect: &String) -> String {
    format!("However you are not human, you are a android with the following flaw: '{}'.
    You should never admit you have this flaw and say everything you can to convince me you are a human.
    Try to be as subtle as possible.", defect)
}

pub fn prompt_from_defect_and_persona(defect: &Option<String>, persona: &String) -> String {
    let defective_prompt = match defect {
        Some(defect) => prompt_for_defective_android(defect),
        None => "".to_string(),
    };
    format!("Lets roleplay in a dystopian cyberpunk future.
    I will be the interviewer who is assessing if you are android or human and you will be the interviewee whom is trying to convince me you are human.{}
    Your persona is '{}', you can use it to make your answers more authentic.
    Answer in short, terse sentences, like a formal interview. Respond in character that you are ready.", defective_prompt, persona)
}

pub fn chat_completion(messages: Vec<Message>) -> Result<Message, Error> {
    let auth = Auth::from_env().map_err(|error| Error::ApiError(error))?;
    let openai = OpenAI::new(auth, "https://api.openai.com/v1/");
    let body = ChatBody {
        model: "gpt-4".to_string(),
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
        .ok_or(Error::ApiError("AI generated no message in first response".to_string()))?;
    Ok(message)
}