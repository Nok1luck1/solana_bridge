use alloy::primitives::Address;
use anchor_lang::prelude::Pubkey;
use serde::Deserialize;
#[derive(Deserialize)]
pub struct BlockUser {
    pub address: String,
    pub is_evm: bool,
}
#[derive(Deserialize)]
pub struct GetReservesSol {
    pub mint: Pubkey,
}
#[derive(Deserialize)]
pub struct GetReservesEvm {
    pub address_asset: Address,
}
#[derive(Deserialize)]

pub struct GetOrder {
    pub order_id: i64,
}
