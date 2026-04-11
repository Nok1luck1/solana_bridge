pub mod db;
pub mod entity;

pub mod eth;
pub mod solana;
pub mod types;
use crate::types::OrderFormatter;
use crate::{db::database, solana::utils};
use anchor_lang::prelude::Pubkey;
use dotenv::dotenv;
use entity::orders;
use std::str::FromStr;
pub mod errors;
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
        match timeout(Duration::from_secs(evm_interval), eth::scan_for_orders()).await {
            Ok(Ok(Some((order_id, tx_hash)))) => {
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
                let _save_order_in_db = database::create_order(
                    order_id.to::<i32>(),
                    true,
                    struct_order.sender.clone(),
                    struct_order.receiver.clone(),
                    struct_order.token0.clone(),
                    struct_order.token1.clone(),
                    struct_order.amount0 as i64,
                    struct_order.amount1 as i64,
                    struct_order.time_started,
                    0,
                    "".to_string(),
                    tx_hash.to_string(),
                );
                let (arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8) =
                    struct_order.format_for_solana();
                let execute_order_solana =
                    solana::sender::execute_order(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)
                        .await?;
                database::update_order_with_hash_sol(
                    order_id.to::<i32>(),
                    execute_order_solana.to_string(),
                )
                .await?;
                println!("Transaction execution in solana network succesfully completed,hash is {execute_order_solana:?}");
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
                Ok(Ok(Some((order, order_pda)))) => {
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
                    database::create_order(
                        order_id.1.counter.clone().try_into().unwrap(),
                        false,
                        verify_order.1.sender.clone(),
                        verify_order.1.receiver.clone(),
                        verify_order.1.token0.clone(),
                        verify_order.1.token1.clone(),
                        verify_order.1.amount0.try_into().unwrap(),
                        verify_order.1.amount1.try_into().unwrap(),
                        verify_order.1.time_started,
                        0,
                        String::from_utf8(order_pda).unwrap(),
                        "_".to_string(),
                    )
                    .await?;
                    println!("{:?} Order in solana gettet", order);
                    let order_formattet = verify_order.1.format_for_evm();
                    let bridge_token_balance =
                        eth::check_balance(order_formattet.2.clone()).await?;
                    if bridge_token_balance < order_formattet.5 {
                        let _ = errors::FormatError::BalanceError {
                            has: bridge_token_balance.to_string(),
                            neeed: order_formattet.5.to_string(),
                        };
                    }
                    let execute = eth::execute_order_evm(
                        order_formattet.0,
                        order_formattet.1,
                        order_formattet.2,
                        order_formattet.3,
                        order_formattet.4,
                        order_formattet.5,
                    )
                    .await?;
                    println!("{execute:?}");
                    database::update_order_with_hash_evm(
                        order_id.1.counter.clone().try_into().unwrap(),
                        execute.to_string(),
                    )
                    .await?;
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
