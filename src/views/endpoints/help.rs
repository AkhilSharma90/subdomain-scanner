//use actix_web::{web, Responder};
//use actix_web::{web, HttpResponse};
use crate::json_serialization::message::Message;

const HELP_MESSAGE: &str = r#"
    Usage:\n
    * '\api\v1\scan\{domain}' to scan for a particular domain\n
    * '\api\v1\about' to view application information.
    * '\api\v1\help' to view help details.
"#;

pub async fn help() -> Message {
    Message::new(HELP_MESSAGE.to_string())
}
