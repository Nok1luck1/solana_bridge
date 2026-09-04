use crate::dto::auth::AuthConfig;
use redis::aio::ConnectionManager;
use sea_orm::{Database, DatabaseConnection};

use std::sync::Arc;
use tokio::sync::OnceCell;
static DB_POOL: OnceCell<Arc<DatabaseConnection>> = OnceCell::const_new();
static REDIS_POOL: OnceCell<ConnectionManager> = OnceCell::const_new();
static AUTH_CONFIG: OnceCell<AuthConfig> = OnceCell::const_new();

pub async fn init_db_pool() -> Result<(), String> {
    let database_url = std::env::var("DATABASE_URL").map_err(|_| "DATABASE_URL not set".to_string())?;
    let pool = Database::connect(database_url)
        .await
        .map_err(|err| format!("failed to connect to database: {err}"))?;

    DB_POOL
        .set(Arc::new(pool))
        .map_err(|_| "DB_POOL already initialized".to_string())?;
    Ok(())
}

pub fn get_pool() -> Arc<DatabaseConnection> {
    DB_POOL
        .get()
        .expect("DB_POOL not initialized! Call init_db_pool() first")
        .clone()
}

pub async fn init_redis() -> Result<(), String> {
    let redis_url = std::env::var("REDIS_URL").map_err(|_| "REDIS_URL not set".to_string())?;
    let client = redis::Client::open(redis_url).map_err(|err| format!("invalid Redis URL: {err}"))?;
    let manager = ConnectionManager::new(client)
        .await
        .map_err(|err| format!("failed to connect to Redis: {err}"))?;
    REDIS_POOL
        .set(manager)
        .map_err(|_| "REDIS_POOL already initialized".to_string())?;
    Ok(())
}

pub fn get_redis() -> ConnectionManager {
    REDIS_POOL
        .get()
        .expect("REDIS_POOL not initialized! Call init_redis() first")
        .clone()
}
pub async fn init_auth_config() -> Result<(), String> {
    let jwt_secret = std::env::var("JWT_SECRET").map_err(|_| "JWT_SECRET not set".to_string())?;
    let jwt_expiry_hours = std::env::var("JWT_EXPIRY_HOURS")
        .map_err(|_| "JWT_EXPIRY_HOURS not set".to_string())?
        .parse::<i64>()
        .map_err(|_| "Invalid JWT_EXPIRY_HOURS".to_string())?;
    AUTH_CONFIG
        .set(AuthConfig {
            jwt_secret,
            jwt_expiry_hours,
        })
        .map_err(|_| "AUTH_CONFIG already initialized".to_string())?;
    Ok(())
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

pub type SharedAppState = Arc<AppState>;

impl AppState {
    pub fn from_static_pools() -> SharedAppState {
        Arc::new(Self {
            db: get_pool(),
            redis: get_redis(),
            auth: get_auth_config(),
        })
    }
}
