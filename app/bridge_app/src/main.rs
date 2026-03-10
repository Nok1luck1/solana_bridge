pub mod eth;
pub mod solana;
pub mod types;
use crate::solana::utils;

//use crate::eth::latest_block;
// use alloy::{
//     primitives::{
//         map::foldhash::{HashMap, HashMapExt},
//         Address,
//     },
//     providers::ProviderBuilder,
//     signers::local::PrivateKeySigner,
// };
use anchor_client::{
    solana_sdk::signature::{read_keypair_file, Keypair},
    Client, Cluster,
};
use dotenv::dotenv;
use std::rc::Rc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    tracing_subscriber::fmt::init();
    // let provider_end_point = std::env::var("PROVIDER_ETH_ENDPOINT").expect("PROVIDER DID NOT SET");
    // let private_key = std::env::var("PRIVATE_KEY_ETH").expect("private missing");
    // let signer: PrivateKeySigner = private_key.parse().expect("private key parse error");
    // let provider = ProviderBuilder::new()
    //     .wallet(signer)
    //     .connect_ws(provider_end_point.as_str())
    //     .await
    //     .expect("Failed to connect to EVM");

    // let token_addr = std::env::var("TOKEN_ADDRESS").expect("Contract addr must be set in .env");
    // let token_address = Address::from_str(token_addr.as_str());
    // let result1 = eth::allowance_checker::check_balance(token_address.unwrap(), &provider);
    // let latest_block = eth::latest_block(&provider);
    // println!("{}token addr",token_address.unwrap());
    // println!("Latest block: {}",latest_block.await.unwrap());
    let _payer = read_keypair_file("../../bridge/tests/keys/admin1.json")?;
    let _client = Client::new(Cluster::Localnet, Rc::new(_payer));
    let _program: anchor_client::Program<Rc<Keypair>> = _client.program(bridge::ID)?;
    let _admin = solana::utils::get_admin_config(&_program).await?;
    let _order_id = solana::utils::get_current_order_id(&_program).await?;
    let _check = solana::scanner::checking();
    _check
        .await
        .unwrap()
        .inspect(|x| println!("got: {x:?}"))
        .expect("list should be long enough");

    Ok(())
}
