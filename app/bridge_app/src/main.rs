pub mod bridge;
pub mod db;
pub mod entity;
pub mod errors;
pub mod eth;
pub mod local;
pub mod routes;
pub mod solana;
pub mod types;
use crate::solana::utils;
use crate::types::OrderFormatter;
use dotenv::dotenv;
use entity::orders;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    local::run_evm_local_validator();
    local::run_solana_local_validator();
    tracing_subscriber::fmt::init();
    let _ = bridge::run_bridge().await;
    Ok(())
}
