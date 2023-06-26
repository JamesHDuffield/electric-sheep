use openai_api_rust::*;
use openai_api_rust::chat::*;

pub fn prompt_from_defect(defect: String) -> String {
    format!("Let's play a game. It is like the Voight-Kampff from Bladerunner.
    I will be the interviewer and you will be the android. You should never admit you are an android and say everything you can to convince me you are a non-defective android.
    However as an android you have a flaw: '{}'. Try to be as subtle as possible. Answer in short, terse sentences, like a casual chat. Respond in character that you are ready.", defect)
}

pub fn chat_completion(messages: Vec<Message>) -> Result<Message, Error> {
    let auth = Auth::from_env().map_err(|error| Error::ApiError(error))?;
    let openai = OpenAI::new(auth, "https://api.openai.com/v1/");
    let body = ChatBody {
        model: "gpt-3.5-turbo".to_string(),
        max_tokens: Some(50),
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