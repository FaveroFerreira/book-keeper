use crate::borrow::model::{Borrow, BorrowDTO};
use crate::config::Ctx;
use sqlx::postgres::PgQueryResult;
use sqlx::{query, query_as};
use std::sync::Arc;
use uuid::Uuid;

pub async fn select_borrow_by_id(ctx: &Arc<Ctx>, borrow_id: Uuid) -> Option<Borrow> {
    const FIND_BY_ID_QUERY: &str = "SELECT * FROM borrow WHERE id = $1";

    query_as::<_, Borrow>(FIND_BY_ID_QUERY)
        .bind(borrow_id)
        .fetch_optional(&ctx.db_pool)
        .await
        .unwrap()
}

pub async fn delete_borrow_by_id(ctx: &Arc<Ctx>, borrow_id: Uuid) -> PgQueryResult {
    const DELETE_BY_ID_QUERY: &str = "DELETE FROM borrow WHERE id = $1";

    query(DELETE_BY_ID_QUERY)
        .bind(borrow_id)
        .execute(&ctx.db_pool)
        .await
        .unwrap()
}

pub async fn insert_new_borrow(ctx: &Ctx, borrow: BorrowDTO) -> Borrow {
    const INSERT_BORROW_QUERY: &str = "
    INSERT INTO borrow(id, return_date, student_id, book_id)
    VALUES($1, $2, $3, $4)
    RETURNING id, return_date, student_id, book_id";

    query_as::<_, Borrow>(INSERT_BORROW_QUERY)
        .bind(Uuid::new_v4())
        .bind(&borrow.return_date)
        .bind(&borrow.student_id)
        .bind(&borrow.book_id)
        .fetch_one(&ctx.db_pool)
        .await
        .unwrap()
}

pub async fn find_pending_borrows_by_book(ctx: &Ctx, book_id: Uuid) -> Vec<Borrow> {
    const FIND_BY_ID_QUERY: &str = "SELECT * FROM borrow WHERE book_id = $1";

    query_as::<_, Borrow>(FIND_BY_ID_QUERY)
        .bind(book_id)
        .fetch_all(&ctx.db_pool)
        .await
        .unwrap()
}

pub async fn find_pending_borrows_by_student(ctx: &Ctx, student_id: Uuid) -> Vec<Borrow> {
    const FIND_BY_ID_QUERY: &str = "SELECT * FROM borrow WHERE student_id = $1";

    query_as::<_, Borrow>(FIND_BY_ID_QUERY)
        .bind(student_id)
        .fetch_all(&ctx.db_pool)
        .await
        .unwrap()
}
