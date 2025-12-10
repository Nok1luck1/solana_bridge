#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

declare_id!("BGveEuTKJLqVRJc9sWaX3fjvhYGWesjsyXEe24nVVizA");

const ADMIN_PUBKEY: Pubkey = pubkey!("DsqQPGmhhySWUFaWDEDVifLGUfe3DwnZ7MnVJcNW5Ykv");

#[program]
pub mod test {

    use anchor_lang::accounts::account;

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.bump = ctx.bumps.counter;
        counter.count = 586786;
        Ok(())
    }
    pub fn increase(ctx: Context<CounterChange>, amount: u64) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count += amount;
        Ok(())
    }
    pub fn decrease(ctx: Context<CounterChange>, amount: u64) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count -= amount;
        Ok(())
    }
    ////fucntion signature
    #[derive(Accounts)]

    pub struct Initialize<'info> {
        #[account(init,
            payer = authority,
            space = 16+1,
            seeds = [b"counter-acc"],
            bump
        )]
        pub counter: Account<'info, CounterAcc>,
        #[account(mut)]
        pub authority: Signer<'info>,
        pub system_program: Program<'info, System>,
    }
    //function signature
    #[derive(Accounts)]
    pub struct CounterChange<'info> {
        #[account(
            mut,
            seeds = [b"counter-acc"],
            bump = counter.bump
        )]
        pub counter: Account<'info, CounterAcc>,
    }

    #[account]
    pub struct CounterAcc {
        pub count: u64,
        pub bump: u8,
    }
    // #[error_code]
    // pub enum Errors {
    //     #[msg("invalid owner")]
    //     NotOwnerCalled,
    // }
}
