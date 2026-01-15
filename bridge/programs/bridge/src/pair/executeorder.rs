use super::ErrorCode;
use crate::{Order, OrderExecution, OrderId, StatusOrder};
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

#[derive(Accounts)]
pub struct ExecuteOrder<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
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
    /// CHECK: PDA authority for vault, derived from seeds. No data deserialization needed
    pub vault_authority: UncheckedAccount<'info>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}
pub fn execute_order(
    ctx: Context<ExecuteOrder>,
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
    require!(_order_id.counter > 0, ErrorCode::InsufficientFundsError);
    _order.id = _order_id.counter;
    _order.maker = _sender;
    _order.token0 = _token0;

    _order.status = StatusOrder::COMPLETED;

    Ok(())
}
