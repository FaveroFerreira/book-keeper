use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use uuid::Uuid;

use crate::book::db;
use crate::book::model::{BookDTO, PartialUpdateBookDTO};
use crate::config::Ctx;

pub async fn list_books(State(ctx): State<Arc<Ctx>>) -> impl IntoResponse {
    let books = db::select_all_books(&ctx).await;

    (StatusCode::OK, Json(books))
}

pub async fn save_book(
    State(ctx): State<Arc<Ctx>>,
    Json(book): Json<BookDTO>,
) -> impl IntoResponse {
    let created_book = db::insert_new_book(&ctx, &book).await;

    (StatusCode::CREATED, Json(created_book))
}

pub async fn delete_book(
    State(ctx): State<Arc<Ctx>>,
    Path(book_id): Path<Uuid>,
) -> impl IntoResponse {
    let opt_book = db::select_book_by_id(&ctx, book_id).await;

    match opt_book {
        Some(_) => {
            db::delete_book_by_id(&ctx, book_id).await;

            StatusCode::NO_CONTENT
        }
        None => StatusCode::NOT_FOUND,
    }
}

pub async fn update_book(
    State(ctx): State<Arc<Ctx>>,
    Path(book_id): Path<Uuid>,
    Json(book): Json<BookDTO>,
) -> Response {
    let opt_book = db::select_book_by_id(&ctx, book_id).await;

    match opt_book {
        Some(_) => {
            db::update_book_full_by_id(&ctx, book_id, &book).await;

            (StatusCode::OK, Json(book)).into_response()
        }
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

pub async fn patch_book(
    State(ctx): State<Arc<Ctx>>,
    Path(book_id): Path<Uuid>,
    Json(book): Json<PartialUpdateBookDTO>,
) -> Response {
    let opt_book = db::select_book_by_id(&ctx, book_id).await;

    match opt_book {
        Some(_) => {
            db::update_book_partial_by_id(&ctx, book_id, &book).await;

            (StatusCode::OK, Json(book)).into_response()
        }
        None => StatusCode::NOT_FOUND.into_response(),
    }
}
