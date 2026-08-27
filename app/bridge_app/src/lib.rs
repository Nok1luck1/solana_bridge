pub mod bridges;
pub mod db;
pub mod dto;
pub mod entity;
pub mod errors;
pub mod eth;
pub mod handlers;
pub mod local;
pub mod middleware;
pub mod routes;
pub mod solana;
pub mod state;
pub mod types;

use crate::solana::utils;
use crate::state::AppState;
use crate::types::OrderFormatter;
use entity::orders;

use axum::Router;
use tracing::Level;

pub async fn init() -> state::AppState {
    dotenv::dotenv().ok();

    let _ = state::init_db_pool().await;
    let _ = state::init_redis().await;
    let _ = state::init_auth_config().await;
    state::AppState::from_static_pools()
}

pub fn build_app(state: state::AppState) -> Router {
    routes::all_routes(state)
}

pub async fn build_app_default() -> Router {
    let state = init().await;
    build_app(state)
}

pub fn init_tracing() {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();
}

pub async fn run(addr: &str) -> Result<(), Box<dyn std::error::Error>> {
    let app = build_app_default().await;

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect(&errors::FormatError::AppError.to_string());

    axum::serve(listener, app).await?;
    Ok(())
}
