use std::sync::Arc;

use anyhow::Result;

use crate::config::environment::Environment;
use crate::persistence::scylla_db::ScyllaDb;

pub type SharedState = Arc<State>;

pub struct State {
    pub scylla: ScyllaDb,
}

impl State {
    pub async fn new(env: Environment) -> Result<SharedState> {
        let scylla = ScyllaDb::new(&env).await?;

        let state = Self { scylla };

        Ok(Arc::new(state))
    }
}
