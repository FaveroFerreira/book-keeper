use std::sync::Arc;

use sqlx::postgres::PgQueryResult;
use sqlx::{query, query_as};
use uuid::Uuid;

use crate::book::model::{Book, BookDTO, PartialUpdateBookDTO};
use crate::config::Ctx;

pub async fn select_all_books(ctx: &Arc<Ctx>) -> Vec<Book> {
    const SELECT_BOOKS_QUERY: &str = "SELECT * FROM book WHERE deleted = false";

    query_as::<_, Book>(SELECT_BOOKS_QUERY)
        .fetch_all(&ctx.db_pool)
        .await
        .unwrap()
}

pub async fn insert_new_book(ctx: &Arc<Ctx>, book: &BookDTO) -> Book {
    const INSERT_BOOK_QUERY: &str = "
    INSERT INTO book(id, name, author, publisher, units_available, deleted)
    VALUES($1, $2, $3, $4, $5, $6)
    RETURNING id, name, author, publisher, units_available, deleted";

    query_as::<_, Book>(INSERT_BOOK_QUERY)
        .bind(Uuid::new_v4())
        .bind(&book.name)
        .bind(&book.author)
        .bind(&book.publisher)
        .bind(book.units_available)
        .bind(false)
        .fetch_one(&ctx.db_pool)
        .await
        .unwrap()
}

pub async fn select_book_by_id(ctx: &Arc<Ctx>, book_id: Uuid) -> Option<Book> {
    const FIND_BY_ID_QUERY: &str = "SELECT * FROM book WHERE id = $1";

    query_as::<_, Book>(FIND_BY_ID_QUERY)
        .bind(book_id)
        .fetch_optional(&ctx.db_pool)
        .await
        .unwrap()
}

pub async fn update_book_full_by_id(ctx: &Arc<Ctx>, book_id: Uuid, book: &BookDTO) {
    const UPDATE_BY_ID_QUERY: &str = "
        UPDATE book
        SET name = $1, author = $2, publisher = $3, units_available = $4
        WHERE id = $5
    ";

    query(UPDATE_BY_ID_QUERY)
        .bind(&book.name)
        .bind(&book.author)
        .bind(&book.publisher)
        .bind(book.units_available)
        .bind(book_id)
        .execute(&ctx.db_pool)
        .await
        .unwrap();
}

pub async fn update_book_partial_by_id(ctx: &Arc<Ctx>, book_id: Uuid, book: &PartialUpdateBookDTO) {
    const UPDATE_BOOK_PARTIAL_QUERY: &str = r#"
        UPDATE book
        SET
            name = COALESCE($1, name),
            author = COALESCE($2, author),
            publisher = COALESCE($3, publisher),
            units_available = COALESCE($4, units_available)
        WHERE id = $5
    "#;

    query(UPDATE_BOOK_PARTIAL_QUERY)
        .bind(&book.name)
        .bind(&book.author)
        .bind(&book.publisher)
        .bind(book.units_available)
        .bind(book_id)
        .execute(&ctx.db_pool)
        .await
        .unwrap();
}

pub async fn delete_book_by_id(ctx: &Arc<Ctx>, book_id: Uuid) -> PgQueryResult {
    const DELETE_BY_ID_QUERY: &str = "DELETE FROM book WHERE id = $1";

    query(DELETE_BY_ID_QUERY)
        .bind(book_id)
        .execute(&ctx.db_pool)
        .await
        .unwrap()
}
