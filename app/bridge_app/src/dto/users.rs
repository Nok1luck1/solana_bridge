use alloy::primitives::Address;
use anchor_lang::prelude::Pubkey;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateUser {
    pub pub_key: String,
    pub is_evm: bool,
}

#[derive(Deserialize)]
pub struct GetUserOrders {
    pub address_sol: Pubkey,
    pub address_evm: Address,
    pub maker: bool,
    pub is_evm: bool,
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
