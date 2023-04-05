use std::sync::Arc;

use axum::extract::State;
use axum::response::{IntoResponse, Response};
use axum::Json;
use hyper::StatusCode;

use crate::config::Ctx;
use crate::student::db;
use crate::student::model::StudentDTO;

pub async fn save_student(
    State(ctx): State<Arc<Ctx>>,
    Json(student): Json<StudentDTO>,
) -> Response {
    let student = db::insert_new_student(&ctx, &student).await;

    (StatusCode::CREATED, Json(student)).into_response()
}

pub async fn list_students(State(ctx): State<Arc<Ctx>>) -> Response {
    let students = db::select_all_students(&ctx).await;

    (StatusCode::OK, Json(students)).into_response()
}
