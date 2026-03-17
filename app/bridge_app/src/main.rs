pub mod db;
pub mod entity;
pub mod eth;
pub mod solana;
pub mod types;
use crate::solana::utils;
use alloy::{primitives::Address, providers::ProviderBuilder, signers::local::PrivateKeySigner};
use bridge::createorder;
use dotenv::dotenv;
use entity::orders;
use sea_orm::{prelude::Decimal, ActiveModelTrait, Database, DatabaseConnection, Set};
use std::str::FromStr;
use tokio::sync::OnceCell;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    tracing_subscriber::fmt::init();
    db::connect_static_db();
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

    //let get_ord_info = eth::get_order_info(_check.unwrap(), &provider);
    // let _latest_block = eth::latest_block(&provider);

    // let _payer = read_keypair_file("../../bridge/tests/keys/admin1.json")?;
    // let _client = Client::new(Cluster::Localnet, Rc::new(_payer));
    // let _program: anchor_client::Program<Rc<Keypair>> = _client.program(bridge::ID)?;
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
