pub mod eth;
pub mod solana;
use crate::solana::utils;

use crate::eth::latest_block;
use alloy::{
    primitives::{
        map::foldhash::{HashMap, HashMapExt},
        Address,
    },
    providers::ProviderBuilder,
    signers::local::PrivateKeySigner,
};
use anchor_client::{
    solana_sdk::signature::{read_keypair_file, Keypair, Signature},
    Client, Cluster,
};
use anchor_lang::prelude::Pubkey;
use bridge::{instruction, OrderId};
use dotenv::dotenv;
use futures::{SinkExt, StreamExt};
use std::collections::HashMap;
use std::error::Error;
use std::rc::Rc;
use std::str::FromStr;
use std::{collections::HashMap, env};
use yellowstone_grpc_client::{ClientTlsConfig, GeyserGrpcClient};
use yellowstone_grpc_proto::geyser::{
    subscribe_update::UpdateOneof, CommitmentLevel, SubscribeRequest,
    SubscribeRequestFilterTransactions,
};
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
    //     .expect("Failed to connect to BSC");

    // let token_addr = std::env::var("TOKEN_ADDRESS").expect("Contract addr must be set in .env");
    // let token_address = Address::from_str(token_addr.as_str());
    // let result1 = eth::allowance_checker::check_balance(token_address.unwrap(), &provider);
    // let latest_block = eth::latest_block(&provider);
    // println!("{}token addr",token_address.unwrap());
    // println!("Latest block: {}",latest_block.await.unwrap());
    let payer = read_keypair_file("../../bridge/tests/keys/admin1.json")?;
    let client = Client::new(Cluster::Localnet, Rc::new(payer));
    let program: anchor_client::Program<Rc<Keypair>> = client.program(bridge::ID)?;
    let admin = solana::utils::get_admin_config(&program).await?;
    let order_id = solana::utils::get_current_order_id(&program).await?;
    let grpc_client = GeyserGrpcClient::build_from_static("http://127.0.0.1:10000")
        .connect()
        .await?;

    Ok(())
}
