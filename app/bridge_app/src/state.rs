use crate::dto::auth::AuthConfig;
use crate::{errors, handlers::helpers::Role};
use redis::aio::ConnectionManager;
use sea_orm::{Database, DatabaseConnection};
use serde::{Deserialize, Serialize};

use std::sync::Arc;
use tokio::sync::OnceCell;
static DB_POOL: OnceCell<Arc<DatabaseConnection>> = OnceCell::const_new();
static REDIS_POOL: OnceCell<ConnectionManager> = OnceCell::const_new();
static AUTH_CONFIG: OnceCell<AuthConfig> = OnceCell::const_new();

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
pub async fn init_auth_config() {
    AUTH_CONFIG
        .set(AuthConfig {
            jwt_secret: std::env::var("JWT_SECRET").expect("JWT_SECRET not set"),
            jwt_expiry_hours: std::env::var("JWT_EXPIRY_HOURS")
                .expect("JWT_EXPIRY_HOURS not set")
                .parse()
                .expect("Invalid JWT_EXPIRY_HOURS"),
        })
        .expect("AUTH_CONFIG already initialized");
}

pub fn get_auth_config() -> AuthConfig {
    AUTH_CONFIG
        .get()
        .expect("AUTH_CONFIG not initialized")
        .clone()
}

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<DatabaseConnection>,
    pub redis: ConnectionManager,
    pub auth: AuthConfig,
}

impl AppState {
    pub fn from_static_pools() -> Self {
        Self {
            db: get_pool(),
            redis: get_redis(),
            auth: get_auth_config(),
        }
    }
}
