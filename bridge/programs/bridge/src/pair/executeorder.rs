use super::ErrorCode;
use crate::{AdminConfig, OrderCompleted, OrderExecution, OrderId, StatusOrder};
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

#[derive(Accounts)]
pub struct ExecuteOrder<'info> {
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
        space = 8+OrderExecution::INIT_SPACE,
        seeds = [b"order_execution",admin.key().as_ref(),order_id.counter.to_le_bytes().as_ref()],
        bump
    )]
    pub order_execution: Account<'info, OrderExecution>,
    pub token_1_mint: InterfaceAccount<'info, Mint>,
    #[account(mut)]
    pub receiver_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        token::mint = token_1_mint,
        token::authority = admin_config,
        token::token_program = token_program
    )]
    pub vault_token_program: InterfaceAccount<'info, TokenAccount>,

    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        seeds = [b"adminconfig"],
        bump = admin_config.bump,
    )]
    pub admin_config: Account<'info, AdminConfig>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
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
    let admin_conf = &ctx.accounts.admin_config;
    require!(_token0amount > 0, ErrorCode::ZeroAmountError);
    require!(_token1amount > 0, ErrorCode::ZeroAmountError);
    require!(_token0.len() == 42, ErrorCode::AddressLengthError);
    require!(_sender.len() == 42, ErrorCode::AddressLengthError);
    require!(
        ctx.accounts.vault_token_program.amount >= _order.token1amount,
        ErrorCode::InsufficientFundsError
    );
    require!(
        admin_conf.is_admin(&ctx.accounts.admin.key()),
        ErrorCode::UnauthorizedAdmin
    );
    if _order_id.counter == 0 {
        _order_id.counter = 1;
        _order_id.bump = ctx.bumps.order_id;
    }

    let bump = ctx.accounts.admin_config.bump;
    let seeds = &[b"adminconfig".as_ref(), &[bump]];
    let signer_seeds: &[&[&[u8]]] = &[seeds];

    anchor_spl::token_interface::transfer_checked(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token_interface::TransferChecked {
                from: ctx.accounts.vault_token_program.to_account_info(),
                to: ctx.accounts.receiver_token_account.to_account_info(),
                authority: ctx.accounts.admin_config.to_account_info(),
                mint: ctx.accounts.token_1_mint.to_account_info(),
            },
            signer_seeds,
        ),
        _order.token1amount,
        ctx.accounts.token_1_mint.decimals,
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
        id: _order.id,
        timestarted: _timestart,
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
