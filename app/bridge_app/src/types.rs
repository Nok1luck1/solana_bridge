use std::fmt::Display;
use std::{fmt, str::FromStr};

use alloy::primitives::{Address, U256};
use anchor_lang::prelude::Pubkey;

#[derive(Debug)]
pub struct OrderFormatter {
    pub time_started: i64,
    pub time_executed: i64,
    pub token0: String,
    pub token1: String,
    pub amount0: u64,
    pub amount1: u64,
    pub sender: String,
    pub receiver: String,
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
        token1: String,
        amount0: u64,
        amount1: u64,
        sender: String,
        receiver: String,
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
    pub fn format_for_evm(self) -> (Address, String, Address, String, u64, U256) {
        let receiver_form = Address::from_str(&self.receiver).unwrap();
        let token_form = Address::from_str(&self.token0).unwrap();
        let amount1 = U256::from(self.amount1);
        return (
            receiver_form,
            self.token0,
            token_form,
            self.sender,
            self.amount0,
            amount1,
        );
    }
    pub fn format_for_solana(self) -> (i64, i64, Pubkey, String, i64, i64, String, Pubkey) {
        let receiver_form = Pubkey::from_str(&self.receiver).unwrap();
        let token_form = Pubkey::from_str(&self.token0).unwrap();

        return (
            self.time_started,
            self.time_executed,
            token_form,
            self.token1,
            self.amount0 as i64,
            self.amount1 as i64,
            self.sender,
            receiver_form,
        );
    }
}
