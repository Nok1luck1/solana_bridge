pub mod bridge;
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
use dotenv::dotenv;
use entity::orders;
use tracing::Level;
use tracing_subscriber;
#[tokio::main]

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //  let client = redis::Client::open("redis://127.0.0.1/")?;
    // let mut con = client.get_connection()?;

    dotenv().ok();
    state::init_db_pool().await;
    state::init_redis().await;
    state::init_auth_config().await;
    let state = state::AppState::from_static_pools();
    //local::run_evm_local_validator();
    //local::run_solana_local_validator();
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();
    //let _ = bridge::run_bridge().await;

    let app = routes::all_routes(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect(&errors::FormatError::AppError.to_string());

    axum::serve(listener, app).await?;
    Ok(())
}
