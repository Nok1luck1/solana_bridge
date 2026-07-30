use crate::db::database;
use crate::errors;
use crate::errors::FormatError;
use crate::eth;
use crate::solana;
use crate::state;

use anchor_lang::prelude::Pubkey;
use std::str::FromStr;
use tokio::time::{timeout, Duration};

pub async fn run_solana_worker(solana_interval: u64) -> Result<(), anyhow::Error> {
    loop {
        match timeout(
            Duration::from_secs(solana_interval),
            solana::scan_for_order_sol(),
        )
        .await
        {
            Ok(Ok(Some((order, order_pda)))) => {
                let pool = state::get_pool();
                let order_id = solana::get_current_order_id()
                    .await
                    .expect("Solana order id missing");
                let verify_order = solana::get_specific_order(
                    Pubkey::from_str(order.sender.as_str()).expect("Sender sol missing"),
                    order_id.1.counter,
                )
                .await
                .expect(&FormatError::ParseError.to_string());
                if verify_order.1.time_started != order.time_started {
                    println!("Error in parsing order");
                }
                database::create_order(
                    &pool,
                    false,
                    verify_order.1.sender.clone(),
                    verify_order.1.receiver.clone(),
                    verify_order.1.token0.clone(),
                    verify_order.1.token1.clone(),
                    verify_order
                        .1
                        .amount0
                        .try_into()
                        .expect(&FormatError::ParseError.to_string()),
                    verify_order
                        .1
                        .amount1
                        .try_into()
                        .expect(&FormatError::ParseError.to_string()),
                    verify_order.1.time_started,
                    0,
                    String::from_utf8(order_pda).expect(&FormatError::ParseError.to_string()),
                    "_".to_string(),
                )
                .await?;
                println!("{:?} Order in solana gettet", order);
                let order_formattet = verify_order.1.format_for_evm();
                let bridge_token_balance = eth::check_balance(order_formattet.2.clone())
                    .await
                    .map_err(|err| {
                        tracing::error!("{err:?}");
                        FormatError::BlockchainError
                    })?;
                if bridge_token_balance < order_formattet.5 {
                    let _ = errors::FormatError::BalanceError {
                        has: bridge_token_balance.to_string(),
                        neeed: order_formattet.5.to_string(),
                    };
                }
                let (execute, _order_id_evm) = eth::execute_order_evm(
                    order_formattet.0,
                    order_formattet.1,
                    order_formattet.2,
                    order_formattet.3,
                    order_formattet.4,
                    order_formattet.5,
                )
                .await
                .map_err(|err| {
                    tracing::error!("{err:?}");
                    FormatError::DBError
                })?;
                println!("{execute:?}");
                database::update_order_with_hash_evm(
                    &pool,
                    order_id
                        .1
                        .counter
                        .clone()
                        .try_into()
                        .expect(&FormatError::OrderError.to_string()),
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
