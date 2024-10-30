use super::{AkinatorError, AkinatorSession};
use regex::Regex;

//responseからsession情報を取り出す
pub async fn session_from_body(body: &str) -> Result<AkinatorSession, AkinatorError> {
    let session_regex: Regex = Regex::new(r#"session:\s*'(.+)'"#).unwrap();
    let signature_regex: Regex = Regex::new(r#"signature:\s*'(.+)'"#).unwrap();

    let session = session_regex
        .captures(body)
        .ok_or(AkinatorError::SessionNotFound)?
        .get(1)
        .ok_or(AkinatorError::UnexpectedError)?
        .as_str();

    let signature = signature_regex
        .captures(body)
        .ok_or(AkinatorError::UnexpectedError)?
        .get(1)
        .ok_or(AkinatorError::UnexpectedError)?
        .as_str();

    Ok(AkinatorSession {
        session: session.to_string(),
        signature: signature.to_string(),
    })
}

pub async fn get_question_from_body(body: &str) -> Result<String, AkinatorError> {
    let question_regex: Regex =
        Regex::new(r#"<p class=\"question-text\" id=\"question-label\">\s*(.+)\s*</p>"#).unwrap();
    let question = question_regex
        .captures(body)
        .ok_or(AkinatorError::UnexpectedError)?
        .get(1)
        .ok_or(AkinatorError::UnexpectedError)?
        .as_str();

    Ok(question.to_string())
}
