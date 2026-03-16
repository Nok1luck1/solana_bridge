pub mod entity;
pub mod eth;
pub mod solana;
pub mod types;
use crate::solana::utils;
use alloy::{primitives::Address, providers::ProviderBuilder, signers::local::PrivateKeySigner};
use bridge::createorder;
use dotenv::dotenv;
use entity::orders;
use sea_orm::{prelude::Decimal, ActiveModelTrait, Database, Set};

use std::str::FromStr;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    tracing_subscriber::fmt::init();
    // let provider_end_point = std::env::var("PROVIDER_ETH_ENDPOINT").expect("PROVIDER DID NOT SET");
    // let private_key = std::env::var("PRIVATE_KEY_ETH").expect("private missing");
    // let signer: PrivateKeySigner = private_key.parse().expect("private key parse error");
    // let provider = ProviderBuilder::new()
    //     .wallet(signer)
    //     .connect(provider_end_point.as_str())
    //     .await
    //     .expect("Failed to connect to EVM");

    // let _token_addr = std::env::var("TOKEN_ADDRESS").expect("Contract addr must be set in .env");
    // let _token_address = Address::from_str(_token_addr.as_str());
    // let _result1 = eth::utils::check_balance(_token_address.unwrap(), &provider);
    // loop {
    //     let orders = eth::scan_for_orders().await?;
    // }

    let db = Database::connect(
        std::env::var("DATABASE_URL")
            .expect("DB URL DID NOT SETTET")
            .as_str(),
    )
    .await?;
    println!("Connected successfully!");
    let create_order = orders::ActiveModel {
        fromevmtosol: Set(true),
        maker: Set("0xabc".to_string()),
        receiver: Set("0xabc".to_string()),
        token0: Set("0xabc".to_string()),
        token1: Set("0xabc".to_string()),
        token0amount: Set(1000000),
        token1amount: Set(1000000),
        timestart: Set(Decimal::from(1000000)),
        timeendl: Set(Decimal::from(1000000)),
        tx_hash_solana: Set("0xabc".to_string()),
        tx_hash_evm: Set("0xabc".to_string()),
        id: Set(1),
    };
    let check = create_order.insert(&db).await?;
    println!("Inserted: {:?}", check);
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
