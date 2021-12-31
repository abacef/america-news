
use serde::Serialize;

use rocket::http::{Header, ContentType};

#[derive(Serialize)]
pub struct Greeting {
    pub greeting: String
}

#[derive(Responder)]
pub struct Body {
    pub content: String
}

#[derive(Responder)]
#[response(status = 200, content_type = "json")]
pub struct MyResponder {
    pub body: Body
}