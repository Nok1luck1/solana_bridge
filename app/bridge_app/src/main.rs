pub mod eth;
pub mod solana;
use std::error::Error;
use std::str::FromStr;

use alloy::{primitives::Address, providers::ProviderBuilder, signers::local::PrivateKeySigner};

use crate::eth::latest_block;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
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

    Ok(())
}
