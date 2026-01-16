#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;
pub mod pair;
pub use pair::*;
declare_id!("ERxSBFf5xhxPibsvdTrSSGw5wLZwSs8DmWa5GyWLe7oW");

#[program]
pub mod bridge {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    ////fucntion signature
    #[derive(Accounts)]

    pub struct Initialize<'info> {
        #[account(mut)]
        pub authority: Signer<'info>,
        pub system_program: Program<'info, System>,
        pub bump: u8,
    }
}
