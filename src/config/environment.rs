use anyhow::Context;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Environment {
    pub profile: String,
    pub scylla_db_nodes: Vec<String>,
    pub scylla_db_username: Option<String>,
    pub scylla_db_password: Option<String>,
    pub elasticsearch_url: String,
}

impl Environment {
    pub fn load() -> anyhow::Result<Self> {
        let profile = std::env::var("PROFILE").unwrap_or_default();

        if profile.is_empty() {
            dotenvy::dotenv().context("failed to load .env")?;
        }

        envy::from_env().context("unable to fill `Environment` with env var values")
    }
}
