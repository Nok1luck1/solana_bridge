use anchor_lang::prelude::Pubkey;
use std::fmt;
use std::fmt::Display;

use crate::solana::sender;
#[derive(Debug)]
pub struct OrderFormatter {
    pub time_started: i64,
    pub time_executed: i64,
    pub token0: String,
    pub token1: Pubkey,
    pub amount0: u64,
    pub amount1: u64,
    pub sender: String,
    pub receiver: Pubkey,
}
impl Display for OrderFormatter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Order has {},{},{},{},{},{},{},{},",
            self.time_started,
            self.time_executed,
            self.token0,
            self.token1,
            self.amount0,
            self.amount1,
            self.sender,
            self.receiver
        )
    }
}
impl OrderFormatter {
    pub fn new(
        time_started: i64,
        time_executed: i64,
        token0: String,
        token1: Pubkey,
        amount0: u64,
        amount1: u64,
        sender: String,
        receiver: Pubkey,
    ) -> Self {
        Self {
            time_started,
            time_executed,
            token0,
            token1,
            amount0,
            amount1,
            sender,
            receiver,
        }
    }
}
