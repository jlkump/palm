use sqlx::PgPool;

pub mod database;
pub mod route;
pub mod services;

pub struct AppState {
    pub db_pool: PgPool,
}

impl AppState {
    pub async fn initialize() -> AppState {
        // Todo: Read environment variables for config

        AppState { db_pool: PgPool::connect("postgres://username:password@db:5432/palm").await.unwrap() }
    }
}