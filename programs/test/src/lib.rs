#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

declare_id!("BGveEuTKJLqVRJc9sWaX3fjvhYGWesjsyXEe24nVVizA");

//const ADMIN_PUBKEY: Pubkey = pubkey!("DsqQPGmhhySWUFaWDEDVifLGUfe3DwnZ7MnVJcNW5Ykv");

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
    pub fn initcandidate(ctx: Context<InitCandidate>) -> Result<()> {
        Ok(())
    }
    pub fn vote(ctx: Context<Vote>, voices: u64) -> Result<()> {
        let candidate = &mut ctx.accounts.candidate;
        candidate.candidatevoices = voices;
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
    #[derive(Accounts)]
    pub struct InitCandidate<'info> {
        #[account(
            init,
            payer = creator,
            seeds = [b"vote-list"],
            bump,
            space = 8+72,
        )]
        pub candidate: Account<'info, VoteList>,
        #[account(mut)]
        pub creator: Signer<'info>,
        pub system_program: Program<'info, System>,
    }
    #[derive(Accounts)]
    #[instruction(numberofvoice:u64)]
    pub struct Vote<'info> {
        #[account(mut,
        seeds = [b"vote-list",candidate.candidatename.as_ref()],
        bump = candidate.bump
        )]
        pub candidate: Account<'info, VoteList>,
    }

    #[account]
    #[derive(InitSpace)]
    pub struct VoteList {
        #[max_len(50)]
        pub candidatename: String,
        pub candidatepubkey: Pubkey,
        pub candidatevoices: u64,
        pub bump: u8,
    }
}
