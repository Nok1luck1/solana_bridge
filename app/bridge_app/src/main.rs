pub mod eth;
pub mod solana;
<<<<<<< HEAD
pub mod types;
use crate::solana::utils;

use alloy::{primitives::Address, providers::ProviderBuilder, signers::local::PrivateKeySigner};
use dotenv::dotenv;
use std::str::FromStr;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    tracing_subscriber::fmt::init();
    let provider_end_point = std::env::var("PROVIDER_ETH_ENDPOINT").expect("PROVIDER DID NOT SET");
    let private_key = std::env::var("PRIVATE_KEY_ETH").expect("private missing");
=======
use std::error::Error;
use std::str::FromStr;

use alloy::{primitives::Address, providers::ProviderBuilder, signers::local::PrivateKeySigner};

use crate::eth::latest_block;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();
    let provider_end_point = std::env::var("PROVIDER_BNB_ENDPOINT").expect("PROVIDER DID NOT SET");
    let private_key = std::env::var("PRIVATE_KEY_BNB").expect("private missing");
>>>>>>> origin/master
    let signer: PrivateKeySigner = private_key.parse().expect("private key parse error");
    let provider = ProviderBuilder::new()
        .wallet(signer)
        .connect(provider_end_point.as_str())
        .await
<<<<<<< HEAD
        .expect("Failed to connect to EVM");

    let _token_addr = std::env::var("TOKEN_ADDRESS").expect("Contract addr must be set in .env");
    let _token_address = Address::from_str(_token_addr.as_str());
    let _result1 = eth::allowance_checker::check_balance(_token_address.unwrap(), &provider);
    // let _latest_block = eth::latest_block(&provider);
    // println!("Latest block: {}", &_latest_block.await?);
    // let _events = eth::scan_for_orders(0, _latest_block.await?, &provider);
    // //println!("{} token addr", _token_address.unwrap());

    //println!("events {:?}", &_events.await.unwrap());

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
=======
        .expect("Failed to connect to BSC");

    let token_addr = std::env::var("TOKEN_BNB_ADDRESS").expect("Contract addr must be set in .env");
    let token_address = Address::from_str(token_addr.as_str());
    let result1 = eth::allowance_checker::check_balance(token_address.unwrap(), &provider);
    let latest_block = eth::latest_block(&provider);
>>>>>>> origin/master

    Ok(())
}
