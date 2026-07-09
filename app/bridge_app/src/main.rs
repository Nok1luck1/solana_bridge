pub mod bridge;
pub mod db;
pub mod dto;
pub mod entity;
pub mod errors;
pub mod eth;
pub mod handlers;
pub mod local;
pub mod solana;
pub mod types;
use crate::handlers::admin_routes;
use crate::solana::utils;
use crate::types::OrderFormatter;
use axum::handler::Handler;
use axum::routing::post;
use dotenv::dotenv;
use entity::orders;
use tower_http::trace::TraceLayer;
use tracing::Level;
use tracing_subscriber;
#[tokio::main]

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    //local::run_evm_local_validator();
    //local::run_solana_local_validator();
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();
    //let _ = bridge::run_bridge().await;

    let app = axum::Router::new().route(
        "/get_specific_order",
        post(admin_routes::get_spicific_order).layer(TraceLayer::new_for_http()),
    );
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect(&errors::FormatError::AppError.to_string());
    axum::serve(listener, app).await?;
    Ok(())
}
