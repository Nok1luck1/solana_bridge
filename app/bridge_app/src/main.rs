pub mod db;
pub mod entity;
pub mod eth;
pub mod solana;
pub mod types;
use crate::solana::utils;
use anchor_client::solana_sdk::signature::read_keypair_file;
use anchor_client::Client;
use anchor_client::Cluster;
use dotenv::dotenv;
use entity::orders;
use std::rc::Rc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    tracing_subscriber::fmt::init();
    db::save_order(
        1,
        true,
        "erfververb".to_string(),
        "erberberb".to_string(),
        "erberberb".to_string(),
        "erberberb".to_string(),
        123123,
        123123,
        345345,
        34534534,
        "erberberb".to_string(),
        "erberberb".to_string(),
    )
    .await?;
    println!("erberberb");

    // loop {
    //     let orders = eth::scan_for_orders().await?;
    // }

    // let _admin = solana::utils::get_admin_config(&_program).await?;
    // let _order_id = solana::utils::get_current_order_id(&_program).await?;
    // let _check = solana::scanner::checking();
    // _check
    //     .await
    //     .unwrap()
    //     .inspect(|x| println!("got: {x:?}"))
    //     .expect("list should be long enough");

    Ok(())
}
