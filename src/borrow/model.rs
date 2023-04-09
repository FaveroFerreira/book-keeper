use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, FromRow)]
pub struct Borrow {
    pub id: Uuid,
    pub return_date: NaiveDate,
    pub student_id: Uuid,
    pub book_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BorrowDTO {
    pub return_date: NaiveDate,
    pub student_id: Uuid,
    pub book_id: Uuid,
}
