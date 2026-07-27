use alloy::primitives::Address;
use anchor_lang::prelude::Pubkey;
use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize)]
pub struct BlockUser {
    pub address: String,
    pub is_evm: bool,
}
#[derive(Deserialize,Serialize)]
pub struct GetReservesSol {
    pub mint: Pubkey,
}
#[derive(Deserialize,Serialize)]
pub struct GetReservesEvm {
    pub address_asset: Address,
}
#[derive(Deserialize,Serialize)]

pub struct GetOrder {
    pub order_id: i64,
}
