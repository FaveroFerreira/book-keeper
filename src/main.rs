use std::net::{Ipv4Addr, SocketAddr};
use std::sync::Arc;

use axum::routing::{delete, get, patch, post, put};
use axum::Router;

use config::{Ctx, Environment};

mod book;
mod borrow;
mod config;
mod error;
mod student;

#[tokio::main]
async fn main() {
    let env = Environment::load();

    let ctx = Arc::new(Ctx::new(env).await);

    config::apply_db_migrations(&ctx).await;

    let router = Router::new()
        .route("/book", get(book::handler::list_books))
        .route("/book", post(book::handler::save_book))
        .route("/book/:id", delete(book::handler::delete_book))
        .route("/book/:id", put(book::handler::update_book))
        .route("/book/:id", patch(book::handler::patch_book))
        .route("/borrow", post(borrow::handler::save_borrow))
        .route("/borrow/:id", delete(borrow::handler::return_borrow))
        .route("/student", get(student::handler::list_students))
        .route("/student", post(student::handler::save_student))
        .with_state(ctx);

    axum::Server::bind(&SocketAddr::from((Ipv4Addr::UNSPECIFIED, 8080)))
        .serve(router.into_make_service())
        .await
        .unwrap()
}
