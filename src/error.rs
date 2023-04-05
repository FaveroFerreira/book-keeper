use axum::Json;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Message<'a> {
    pub message: &'a str,
}

impl<'a> Message<'a> {
    pub fn json(message: &'a str) -> Json<Self> {
        Json(Message { message })
    }
}
