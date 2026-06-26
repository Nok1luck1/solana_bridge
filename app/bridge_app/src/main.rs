pub mod bridge;
pub mod db;
pub mod entity;
pub mod errors;
pub mod eth;
pub mod handlers;
pub mod local;
pub mod solana;
pub mod types;
use crate::handlers::users_routes;
use crate::solana::utils;
use crate::types::OrderFormatter;
use axum::routing::get;
use dotenv::dotenv;
use entity::orders;
use tokio::net::TcpListener;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    //local::run_evm_local_validator();
    //local::run_solana_local_validator();
    tracing_subscriber::fmt::init();
    //let _ = bridge::run_bridge().await;
    let app = axum::Router::new().route("/create_user", get(users_routes::create_user));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    //let listener = tokio::net::
    axum::serve(listener, app).await?;
    Ok(())
}
