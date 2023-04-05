use std::sync::Arc;

use sqlx::migrate::Migrator;
use sqlx::PgPool;

pub struct Ctx {
    pub db_pool: PgPool,
}

impl Ctx {
    pub async fn new(env: Environment) -> Self {
        let db_pool = PgPool::connect(&env.database_url).await.unwrap();

        Self { db_pool }
    }
}

#[derive(serde::Deserialize)]
pub struct Environment {
    pub database_url: String,
}

impl Environment {
    pub fn load() -> Self {
        dotenvy::dotenv().unwrap();

        envy::from_env().unwrap()
    }
}

pub async fn apply_db_migrations(ctx: &Arc<Ctx>) {
    Migrator::new(std::path::Path::new("./migrations"))
        .await
        .unwrap()
        .run(&ctx.db_pool)
        .await
        .unwrap();
}
