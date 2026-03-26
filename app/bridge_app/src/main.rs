pub mod db;
pub mod entity;

pub mod eth;
pub mod solana;
pub mod types;
use crate::solana::utils;
use dotenv::dotenv;
use entity::orders;
use tokio::time::{timeout, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    tracing_subscriber::fmt::init();
    // db::save_order(
    //     1,
    //     true,
    //     "erfververb".to_string(),
    //     "erberberb".to_string(),
    //     "erberberb".to_string(),
    //     "erberberb".to_string(),
    //     123123,
    //     123123,
    //     345345,
    //     34534534,
    //     "erberberb".to_string(),
    //     "erberberb".to_string(),
    // )
    // .await?;
    // println!("erberberb");
    let mut interval = tokio::time::interval(Duration::from_secs(5));
    let evm_interval: u64 = std::env::var("EVM_INTERVAL")
        .unwrap_or("5".to_string())
        .parse()
        .unwrap();
    let solana_interval: u64 = std::env::var("SOLANA_INTERVAL")
        .unwrap_or("5".to_string())
        .parse()
        .unwrap();
    loop {
        interval.tick().await;
        tokio::spawn(async move {
            if timeout(
                Duration::from_secs(evm_interval),
                eth::scan_for_orders().await,
            )
            .await
            .is_ok()
            {
                println!("Success EVM");
            } else {
                println!("Timeout EVM");
            }
        });

        tokio::spawn(async {
            if timeout(
                Duration::from_secs(solana_interval),
                solana::scan_for_order_sol(),
            )
            .await
            .is_ok()
            {
                println!("Success Solana");
            } else {
                println!("Timeout Solana");
            }
        });
    }
    Ok(())
}
