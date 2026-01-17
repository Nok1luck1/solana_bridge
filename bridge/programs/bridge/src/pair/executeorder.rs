use super::ErrorCode;
use crate::{transfer_tokens, AdminConfig, OrderCompleted, OrderExecution, OrderId, StatusOrder};
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

#[derive(Accounts)]
pub struct ExecuteOrder<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        seeds = [b"admin_config"],
        bump = admin_config.bump,
        constraint = admin_config.is_admin(&admin.key()) @ ErrorCode::UnauthorizedAdmin,
    )]
    pub admin_config: Account<'info, AdminConfig>,
    #[account(
        init_if_needed,
        payer = admin,
        space = 8 + OrderId::INIT_SPACE,
        seeds = [b"order_id"],
        bump
    )]
    pub order_id: Account<'info, OrderId>,
    #[account(
        init,
        payer = admin,
        space = OrderExecution::INIT_SPACE,
        seeds = [b"order_execution",admin.key().as_ref(),order_id.counter.to_le_bytes().as_ref()],
        bump
    )]
    pub order_execution: Account<'info, OrderExecution>,
    pub token_1_mint: InterfaceAccount<'info, Mint>,
    pub receiver_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(
        token::mint = token_1_mint,
        token::authority = vault_authority,
        token::token_program = token_program
    )]
    pub vault_token_program: InterfaceAccount<'info, TokenAccount>,
    #[account(
        seeds = [b"vault_authority"],
        bump
    )]
    /// CHECK: This is a token account that can hold any SPL token.
    /// We verify it's a valid token account through CPI calls but don't
    /// deserialize it as Account<TokenAccount> to support multiple token types
    pub vault_authority: Signer<'info>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}
pub fn execute_order(
    ctx: Context<ExecuteOrder>,
    _receiver: Pubkey,
    _token0amount: u64,
    _token1amount: u64,
    _token0: String,
    _sender: String,
    _timestart: i64,
) -> Result<()> {
    let _order = &mut ctx.accounts.order_execution;
    let _order_id = &mut ctx.accounts.order_id;

    require!(_token0amount > 0, ErrorCode::ZeroAmountError);
    require!(_token1amount > 0, ErrorCode::ZeroAmountError);
    require!(_token0.len() == 20, ErrorCode::AddressLengthError);
    require!(_sender.len() == 20, ErrorCode::AddressLengthError);
    if _order_id.counter == 0 {
        _order_id.counter = 1;
        _order_id.bump = ctx.bumps.order_id;
    }
    transfer_tokens(
        &ctx.accounts.vault_token_program,
        &ctx.accounts.receiver_token_account,
        &_token1amount,
        &ctx.accounts.token_1_mint,
        &ctx.accounts.vault_authority,
        &ctx.accounts.token_program,
    )?;
    _order.id = _order_id.counter;
    _order.maker = _sender.clone();
    _order.token0 = _token0.clone();
    _order.token1 = ctx.accounts.token_1_mint.key();
    _order.receiver = _receiver;
    _order.token0amount = _token0amount;
    _order.token1amount = _token1amount;
    _order.status = StatusOrder::COMPLETED;
    _order.timeend = Clock::get()?.unix_timestamp;
    _order_id.counter = _order_id
        .counter
        .checked_add(1)
        .ok_or(ErrorCode::OveflowError)?;
    emit!(OrderCompleted {
        timeexecuted: _order.timeend,
        token0: _token0,
        token1: _order.token1,
        amount0: _order.token0amount,
        amount1: _order.token1amount,
        sender: _sender,
        receiver: _order.receiver
    });
    Ok(())
}
