pub mod eth;
pub mod solana;
use crate::solana::utils;
use crate::utils;
use alloy::{primitives::Address, providers::ProviderBuilder, signers::local::PrivateKeySigner};
use dotenv::dotenv;
use std::env;
use std::error::Error;
use std::rc::Rc;
use std::str::FromStr;

use anchor_client::{
    solana_sdk::signature::{read_keypair_file, Keypair},
    Client, Cluster,
};
use anchor_lang::prelude::Pubkey;
use bridge::{instruction, OrderId};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    tracing_subscriber::fmt::init();
    let provider_end_point = std::env::var("PROVIDER_BNB_ENDPOINT").expect("PROVIDER DID NOT SET");
    let private_key = std::env::var("PRIVATE_KEY_BNB").expect("private missing");
    let signer: PrivateKeySigner = private_key.parse().expect("private key parse error");
    let provider = ProviderBuilder::new()
        .wallet(signer)
        .connect(provider_end_point.as_str())
        .await
        .expect("Failed to connect to BSC");

    let token_addr = std::env::var("TOKEN_BNB_ADDRESS").expect("Contract addr must be set in .env");
    let token_address = Address::from_str(token_addr.as_str());
    let result1 = eth::allowance_checker::check_balance(token_address.unwrap(), &provider);
    let latest_block = eth::latest_block(&provider);
    println!("{}token addr", token_address.unwrap());
    println!("Latest block: {}", latest_block.await.unwrap());
    let payer = read_keypair_file("../../bridge/tests/keys/admin1.json")?;
    let client = Client::new(Cluster::Localnet, Rc::new(payer));
    let program: anchor_client::Program<Rc<Keypair>> = client.program(bridge::ID)?;
    let admin = solana::utils::get_admin_config(&program).await?;
    let order_id = solana::utils::get_current_order_id(&program).await?;
    Ok(())
}
