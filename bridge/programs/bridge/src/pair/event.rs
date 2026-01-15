use anchor_lang::prelude::*;

#[event]
pub struct OrderCreated {
    pub timecreation: i64,
    pub token0: Pubkey,
    pub token1: String,
    pub amount0: u64,
    pub amount1: u64,
    pub sender: Pubkey,
    pub receiver: String,
}

#[event]
pub struct OrderCompleted {
    pub timeexecuted: i64,
    pub token0: String,
    pub token1: Pubkey,
    pub amount0: u64,
    pub amount1: u64,
    pub sender: String,
    pub receiver: Pubkey,
}
#[event]
pub struct OrderCancelled {
    pub order_id: u64,
    pub maker: Pubkey,
    pub token0: Pubkey,
    pub token1: String,
    pub amount0: u64,
    pub time_cancelled: i64,
}
