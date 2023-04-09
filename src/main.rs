use anyhow::Result;
use axum::extract::State;
use axum::response::IntoResponse;

use book_keeper::config::environment::Environment;
use book_keeper::config::observability::init_tracing;
use book_keeper::core::state::SharedState;
use tracing::debug;

#[tokio::main]
async fn main() -> Result<()> {
    let _guard = init_tracing()?;

    let env = Environment::load()?;

    let _state = book_keeper::core::state::State::new(env).await?;

    debug!("done");

    Ok(())
}

pub async fn test(State(_state): State<SharedState>) -> impl IntoResponse {
    "Ok"
}
