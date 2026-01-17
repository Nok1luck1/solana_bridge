#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;
pub mod pair;
pub use pair::*;
declare_id!("ERxSBFf5xhxPibsvdTrSSGw5wLZwSs8DmWa5GyWLe7oW");

#[program]
pub mod bridge {

    use super::*;

    pub fn initialize(ctx: Context<AdminConfig>) -> Result<()> {
        Ok(())
    }
    pub fn order_for_transfer(
        ctx: Context<CreateOrder>,
        token1: String,
        receiver: String,
        token0amount: u64,
        token1amount: u64,
    ) -> Result<()> {
        create_order(ctx, token1, receiver, token0amount, token1amount)
    }
}
