use alloy::primitives::Address;
use anchor_lang::prelude::Pubkey;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateUser {
    address_sol: Pubkey,
    address_evm: Address,
}

#[derive(Deserialize)]
pub struct GetUser {
    pub address_sol: Pubkey,
    pub address_evm: Address,
    pub maker: bool,
    pub is_evm: bool,
    pub limit: u64,
    pub offset: u64,
}
