use std::sync::Arc;

use axum::extract::{Path, State};
use axum::response::{IntoResponse, Response};
use axum::Json;
use hyper::StatusCode;
use uuid::Uuid;

use crate::book::db as book_db;
use crate::book::model::Book;
use crate::borrow::db as borrow_db;
use crate::borrow::model::{Borrow, BorrowDTO};
use crate::config::Ctx;
use crate::error::Message;
use crate::student::db as student_db;

pub async fn save_borrow(State(ctx): State<Arc<Ctx>>, Json(borrow): Json<BorrowDTO>) -> Response {
    let Some(student) = student_db::select_student_by_id(&ctx, borrow.student_id).await else {
        return (StatusCode::BAD_REQUEST, Message::json("student not found")).into_response();
    };

    let Some(book) = book_db::select_book_by_id(&ctx, borrow.book_id).await else {
        return (StatusCode::BAD_REQUEST, Message::json("book not found")).into_response();
    };

    let pending_borrows = borrow_db::find_pending_borrows_by_book(&ctx, book.id).await;

    if book_is_out_of_stock(book, pending_borrows) {
        return (StatusCode::BAD_REQUEST, Message::json("book out of stock")).into_response();
    }

    let student_borrows = borrow_db::find_pending_borrows_by_student(&ctx, student.id).await;

    if student_borrows.len() > 0 {
        return (StatusCode::FORBIDDEN, Message::json("Students can borrow only one book per time.")).into_response();
    }


    let borrow = borrow_db::insert_new_borrow(&ctx, borrow).await;

    (StatusCode::CREATED, Json(borrow)).into_response()
}

pub async fn return_borrow(
    State(ctx): State<Arc<Ctx>>,
    Path(borrow_id): Path<Uuid>,
) -> impl IntoResponse {
    let opt_borrow = borrow_db::select_borrow_by_id(&ctx, borrow_id).await;

    match opt_borrow {
        Some(_) => {
            borrow_db::delete_borrow_by_id(&ctx, borrow_id).await;

            StatusCode::NO_CONTENT
        }
        None => StatusCode::NOT_FOUND,
    }
}

fn book_is_out_of_stock(book: Book, pending_borrows: Vec<Borrow>) -> bool {
    book.units_available as usize <= pending_borrows.len()
}
