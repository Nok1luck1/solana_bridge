use crate::db::database;
use crate::errors::FormatError;
use crate::types::OrderFormatter;
use crate::{eth, solana, state, types};
use alloy::primitives::{Address, U256};
use eth::constant;
use std::str::FromStr;
use std::time::Duration;
use tokio::time::timeout;

pub async fn run_evm_worker_from_rpc(rpc_url: String, interval: u64) -> Result<(), anyhow::Error> {
    loop {
        match timeout(
            Duration::from_secs(interval),
            eth::scan_for_orders(&rpc_url),
        )
        .await
        {
            Ok(Ok(Some((order_id, tx_hash)))) => {
                let result_order = eth::get_order_info(order_id)
                    .await
                    .expect("Order Info EVM missing");
                let struct_order: OrderFormatter = types::OrderFormatter::new(
                    result_order
                        .timestamp
                        .try_into()
                        .expect("timestamp missing"),
                    0,
                    result_order.token0.to_string(),
                    result_order.token1.to_string(),
                    result_order
                        .amount0
                        .try_into()
                        .expect("amount0 evm missing"),
                    result_order
                        .amount1
                        .try_into()
                        .expect("amount1 evm missing"),
                    result_order.maker.to_string(),
                    result_order.receiver.to_string(),
                );
                println!("{:?} Order EVM gettet", struct_order);
                let pool = state::get_pool();
                let _save_order_in_db = database::create_order(
                    &pool,
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
                match result_order.orderType {
                    constant::Bridge::OrderType::FromEVMtoSol => {
                        let (arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8) =
                            struct_order.format_for_solana();
                        let execute_order_solana = solana::sender::execute_order(
                            arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8,
                        )
                        .await?;
                        database::update_order_with_hash_sol(
                            &pool,
                            order_id.to::<i64>(),
                            execute_order_solana.to_string(),
                        )
                        .await
                        .map_err(|err| {
                            tracing::error!("{err:?}");
                            FormatError::DBError
                        })?;
                        println!("Transaction execution in solana network succesfully completed,hash is {execute_order_solana:?}");
                    }
                    constant::Bridge::OrderType::FromEVMtoEVM => {
                        //need to override to multiple scaner for many different chains
                        let (receipt, order_id) = eth::utils::execute_order_evm(
                            Address::from_str(struct_order.receiver.as_str())?,
                            struct_order.token0,
                            Address::from_str(struct_order.token1.as_str())?,
                            struct_order.sender,
                            struct_order.amount0,
                            U256::from(struct_order.amount1),
                        )
                        .await
                        .map_err(|err| {
                            tracing::error!("{err:?}");
                            FormatError::DBError
                        })?;
                        database::update_order_with_hash_evm(&pool, order_id, receipt.to_string())
                            .await
                            .map_err(|err| {
                                tracing::error!("{err:?}");
                                FormatError::DBError
                            })?;
                        println!("Transaction execution in solana network succesfully completed,hash is {receipt}");
                    }
                    constant::Bridge::OrderType::FomrSolToEVM => {
                        ////do i need this?
                    }
                    _ => {}
                }
            }
            Ok(Ok(None)) => {}
            Ok(Err(e)) => {
                println!("Scan evm error: {:?}", e);
            }
            Err(_) => {
                println!("Timeout EVM");
            }
        }
    }
}
