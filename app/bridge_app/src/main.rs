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
use crate::dto::admin::GetOrder;
use crate::handlers::{admin_routes, users_routes};
use crate::solana::utils;
use crate::types::OrderFormatter;
use axum::http::HeaderValue;
use axum::routing::{get, post};
use axum::Json;
use dotenv::dotenv;
use entity::orders;
#[tokio::main]

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    //local::run_evm_local_validator();
    //local::run_solana_local_validator();
    tracing_subscriber::fmt::init();
    //let _ = bridge::run_bridge().await;
    let app = axum::Router::new()
        //.layer(CorsLayer::new().allow_origin("pizda_jopa".parse::<HeaderValue>().unwrap()))
        .route(
            "/get_specific_order",
            post(admin_routes::get_spicific_order),
        );
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect(FormatError::AppError);
    axum::serve(listener, app).await?;
    Ok(())
}
