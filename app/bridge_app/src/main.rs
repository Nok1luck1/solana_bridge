pub mod db;
pub mod entity;

pub mod eth;
pub mod solana;
pub mod types;
use crate::solana::utils;
use crate::types::OrderFormatter;
use anchor_client::solana_sdk::signature::Keypair;
use anchor_lang::prelude::Pubkey;
use dotenv::dotenv;
use entity::orders;
use std::str::FromStr;
use tokio::time::{timeout, Duration};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    tracing_subscriber::fmt::init();
    let evm_interval: u64 = std::env::var("EVM_INTERVAL")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(5);

    loop {
        // interval.tick().await;

        match timeout(Duration::from_secs(evm_interval), eth::scan_for_orders()).await {
            Ok(Ok(Some(order_id))) => {
                let result_order = eth::get_order_info(order_id).await.unwrap();
                let struct_order: OrderFormatter = types::OrderFormatter::new(
                    result_order.timestamp.try_into().unwrap(),
                    0,
                    result_order.token0.to_string(),
                    result_order.token1.to_string(),
                    result_order.amount0.try_into().unwrap(),
                    result_order.amount1.try_into().unwrap(),
                    result_order.maker.to_string(),
                    result_order.receiver.to_string(),
                );
                println!("{:?} Order EVM gettet", struct_order);
                let (arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8) =
                    struct_order.format_for_solana();
                let execute_order_solana =
                    solana::sender::execute_order(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8);
            }
            Ok(Ok(None)) => {}
            Ok(Err(e)) => {
                println!("Scan evm error: {:?}", e);
            }
            Err(_) => {
                println!("Timeout EVM");
            }
        }

        let solana_interval: u64 = std::env::var("SOLANA_INTERVAL")
            .unwrap_or("5".to_string())
            .parse()
            .unwrap();

        loop {
            match timeout(
                Duration::from_secs(solana_interval),
                solana::scan_for_order_sol(),
            )
            .await
            {
                Ok(Ok(Some(order))) => {
                    let order_id = solana::get_current_order_id().await.unwrap();
                    let verify_order = solana::get_specific_order(
                        Pubkey::from_str(order.sender.as_str()).expect("pidoras"),
                        order_id.1.counter,
                    )
                    .await
                    .unwrap();
                    if verify_order.1.time_started != order.time_started {
                        println!("Error in parsing order");
                    }
                    println!("{:?} Order in solana gettet", order);
                    let order_formattet = verify_order.1.format_for_evm();
                    // let execute = eth::execute_order_evm(order_formattet.)
                }
                Ok(Ok(None)) => {}
                Ok(Err(e)) => {
                    println!("Scan sol error: {:?}", e);
                }
                Err(_) => {
                    println!("Solana timeout");
                }
            }
        }
    }
}
