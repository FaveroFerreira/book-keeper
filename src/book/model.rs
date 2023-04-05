use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct BookDTO {
    pub name: String,
    pub author: String,
    pub publisher: String,
    pub units_available: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PartialUpdateBookDTO {
    pub name: Option<String>,
    pub author: Option<String>,
    pub publisher: Option<String>,
    pub units_available: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Book {
    pub id: Uuid,
    pub name: String,
    pub author: String,
    pub publisher: String,
    pub units_available: i32,
    pub deleted: bool,
}
