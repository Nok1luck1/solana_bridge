use alloy::primitives::Address;
use anchor_client::solana_sdk::pubkey::Pubkey;
use serde::{Deserialize, Serialize};

use crate::handlers::helpers::Network;

#[derive(Deserialize)]
pub struct CreateUser {
    pub pub_key: String,
    pub network: Network,
}

#[derive(Deserialize)]
pub struct GetUserOrders {
    pub address_sol: Pubkey,
    pub address_evm: Address,
    pub maker: bool,
    pub network: Network,
    pub limit: u64,
    pub offset: u64,
}
#[derive(Serialize)]
pub struct User {
    pub id: i64,
    pub address_sol: String,
    pub address_evm: String,
    pub blocked: bool,
}
