use sqlx::PgPool;

pub mod database;
pub mod route;
pub mod services;

pub struct AppState {
    pub db_pool: PgPool,
    pub config: Config,
}

pub struct Config {
    pub creation_attempts: u8,
}

impl AppState {
    pub async fn initialize() -> AppState {
        // Todo: Read environment variables for config

        AppState { 
            db_pool: PgPool::connect("postgres://username:password@db:5432/palm").await.unwrap(),
            config: Config { creation_attempts: 5 }
        }
    }
}