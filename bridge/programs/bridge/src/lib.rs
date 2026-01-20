#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;
pub mod pair;
pub use pair::*;
declare_id!("FnKGHW1EPKnMUL8FgauV1v5xK4Evmy9cQDKFAMvdQdXz");

#[program]
pub mod bridge {

    use super::*;
    #[derive(Accounts)]
    pub struct Initialize<'info> {
        #[account(mut)]
        pub authority: Signer<'info>,
        #[account(
        init_if_needed,
        payer = authority,
        space = 8 + AdminConfig::INIT_SPACE,
        seeds = [b"adminconfig"],
        bump
    )]
        pub admin_config: Account<'info, AdminConfig>,
        pub system_program: Program<'info, System>,
    }

    pub fn initialize(ctx: Context<Initialize>, admins: Vec<Pubkey>) -> Result<()> {
        require!(admins.len() <= 10, errors::ErrorCode::TooManyAdmins);
        let admin_config = &mut ctx.accounts.admin_config;
        admin_config.admins = admins;
        admin_config.bump = ctx.bumps.admin_config;
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
    pub fn order_for_execution(
        ctx: Context<ExecuteOrder>,
        receiver: Pubkey,
        token0amount: u64,
        token1amount: u64,
        token0: String,
        sender: String,
        timestart: i64,
    ) -> Result<()> {
        execute_order(
            ctx,
            receiver,
            token0amount,
            token1amount,
            token0,
            sender,
            timestart,
        )
    }
    pub fn cancel_existing_order(ctx: Context<CancelOrder>) -> Result<()> {
        cancel_order(ctx)
    }
}
