#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;
pub mod pair;
pub use pair::*;
declare_id!("BGveEuTKJLqVRJc9sWaX3fjvhYGWesjsyXEe24nVVizA");

#[program]
pub mod test {

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
            space = 24+8,
            seeds = [b"counter-acc"],
            bump
        )]
        pub counter: Account<'info, CounterAcc>,
        #[account(mut)]
        pub authority: Signer<'info>,
        pub system_program: Program<'info, System>,
        #[account(init,
        payer = authority,
        space = 8 + OrderId::INIT_SPACE,
        seeds = [b"orderid"],
        bump
        )]
        pub orderid: Account<'info, OrderId>,
    }
    //function signature
    #[derive(Accounts)]
    #[instruction(amount:u64)]
    pub struct CounterChange<'info> {
        #[account(
            mut,
            seeds = [b"counter-acc"],
            bump = counter.bump,
            constraint = counter.count >= amount || amount == 0,
            constraint = amount <= 1000,
            constraint = amount > 0,
        )]
        pub counter: Account<'info, CounterAcc>,
    }

    #[account]
    pub struct CounterAcc {
        pub count: u64,
        pub bump: u8,
    }
}
