use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct StudentDTO {
    pub name: String,
    pub class: String,
    pub ra: String,
}

#[derive(Debug, Serialize, FromRow)]
pub struct Student {
    pub id: Uuid,
    pub name: String,
    pub class: String,
    pub ra: String,
    pub deleted: bool,
}
