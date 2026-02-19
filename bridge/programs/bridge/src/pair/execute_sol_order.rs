use super::ErrorCode;
use crate::order;
use crate::{AdminConfig, OrderCompleted, OrderExecution, OrderId, StatusOrder};
use anchor_lang::prelude::*;
use anchor_lang::system_program;

#[derive(Accounts)]
pub struct ExecuteSolOrder<'info> {
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
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        seeds = [b"adminconfig"],
        bump = admin_config.bump,
        constraint = admin_config.is_admin(&admin.key()) @ ErrorCode::UnauthorizedAdmin,
    )]
    pub admin_config: Account<'info, AdminConfig>,
    /// CHECK:just address of user that wiil receive payment in solana
    pub recepient: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
pub fn execute_orde_sol(
    ctx: Context<ExecuteSolOrder>,
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
        admin_conf.is_admin(&ctx.accounts.admin.key()),
        ErrorCode::UnauthorizedAdmin
    );
    let bump = ctx.accounts.admin_config.bump;
    let seeds = &[b"adminconfig".as_ref(), &[bump]];
    let signer_seeds: &[&[&[u8]]] = &[seeds];
    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.system_program.to_account_info(),
                to: ctx.accounts.recepient.to_account_info(),
            },
        ),
        _token1amount,
    )?;
    _order.id = _order_id.counter;
    _order.maker = _sender.clone();
    _order.token0 = _token0.clone();
    _order.token1 = order::WSOL_MINT;
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
