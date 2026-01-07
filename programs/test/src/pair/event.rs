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
pub struct OrderCompleted {}
#[event]
pub struct OrderCanceled {}
