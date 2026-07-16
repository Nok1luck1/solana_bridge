use crate::errors;
use redis::aio::ConnectionManager;
use sea_orm::{Database, DatabaseConnection};
use std::sync::Arc;
use tokio::sync::OnceCell;

static DB_POOL: OnceCell<Arc<DatabaseConnection>> = OnceCell::const_new();
static REDIS_POOL: OnceCell<ConnectionManager> = OnceCell::const_new();

pub async fn init_db_pool() {
    let pool = Database::connect(std::env::var("DATABASE_URL").expect("DATABASE_URL not set"))
        .await
        .expect(&errors::FormatError::InitError.to_string());

    DB_POOL
        .set(Arc::new(pool))
        .expect("DB_POOL already initialized");
}

pub fn get_pool() -> Arc<DatabaseConnection> {
    DB_POOL
        .get()
        .expect("DB_POOL not initialized! Call init_db_pool() first")
        .clone()
}

pub async fn init_redis() {
    let client = redis::Client::open(std::env::var("REDIS_URL").expect("REDIS_URL not set"))
        .expect(&errors::FormatError::InitError.to_string());
    let manager = ConnectionManager::new(client)
        .await
        .expect(&errors::FormatError::RedisError.to_string());
    REDIS_POOL
        .set(manager)
        .expect("REDIS_POOL already initialized");
}

pub fn get_redis() -> ConnectionManager {
    REDIS_POOL
        .get()
        .expect("REDIS_POOL not initialized! Call init_redis() first")
        .clone()
}
#[derive(Clone)]
pub struct AppState {
    pub db: Arc<DatabaseConnection>,
    pub redis: ConnectionManager,
}

impl AppState {
    pub fn from_static_pools() -> Self {
        Self {
            db: get_pool(),
            redis: get_redis(),
        }
    }
}
