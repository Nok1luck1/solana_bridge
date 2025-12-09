#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

declare_id!("BGveEuTKJLqVRJc9sWaX3fjvhYGWesjsyXEe24nVVizA");

const ADMIN_PUBKEY: Pubkey = pubkey!("DsqQPGmhhySWUFaWDEDVifLGUfe3DwnZ7MnVJcNW5Ykv");

#[program]
pub mod test {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = 0;
        Ok(())
    }
    pub fn increase(ctx: Context<CounterIncrease>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count += 1;
        Ok(())
    }
    ////fucnton signature
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
    pub struct CounterIncrease<'info> {
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
