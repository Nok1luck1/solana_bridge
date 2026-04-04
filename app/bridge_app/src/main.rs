pub mod db;
pub mod entity;

pub mod eth;
pub mod solana;
pub mod types;
use crate::solana::utils;
use crate::types::OrderFormatter;

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
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(5);

    loop {
        interval.tick().await;

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
                //let execute_order_solana =
            }

            Ok(Ok(None)) => {}

            Ok(Err(e)) => {
                println!("Scan error: {:?}", e);
            }

            Err(_) => {
                println!("Timeout EVM");
            }
        }

        // let solana_interval: u64 = std::env::var("SOLANA_INTERVAL")
        //     .unwrap_or("5".to_string())
        //     .parse()
        //     .unwrap();
        // tokio::spawn(async {
        //     if timeout(
        //         Duration::from_secs(solana_interval),
        //         solana::scan_for_order_sol(),
        //     )
        //     .await
        //     .is_ok()
        //     {
        //         println!("Success Solana");
        //     } else {
        //         println!("Timeout Solana");
        //     }
        // });
    }
}
