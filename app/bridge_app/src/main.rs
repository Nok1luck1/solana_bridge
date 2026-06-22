pub mod bridge;
pub mod db;
pub mod entity;
pub mod errors;
pub mod eth;
pub mod solana;
pub mod types;
use crate::types::OrderFormatter;
use crate::solana::utils;
use dotenv::dotenv;
use entity::orders;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    tracing_subscriber::fmt::init();
    bridge::run_bridge();
    Ok(())
}
