use super::ErrorCode;
use crate::{transfer_tokens, Order, OrderCreated, OrderId, StatusOrder};
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

#[derive(Accounts)]

pub struct CreateOrder<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init_if_needed,
        payer = user,
        space = 8 + OrderId::INIT_SPACE,
        seeds = [b"order_id"],
        bump
    )]
    pub order_id: Account<'info, OrderId>,
    #[account(
        init,
        payer = user,
        space = 8 + Order::INIT_SPACE,
        seeds = [b"order",user.key().as_ref(),order_id.counter.to_le_bytes().as_ref()],
        bump
    )]
    pub order: Account<'info, Order>,
    #[account(
        mint::token_program = token_program
    )]
    pub token_0_mint: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        token::mint = token_0_mint,
        token::authority = user,
        token::token_program = token_program
    )]
    pub maker_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = user,
        token::mint = token_0_mint,
        token::authority = vault_authority,
        token::token_program = token_program,
        seeds = [b"vault",token_0_mint.key().as_ref()],
        bump
    )]
    pub vault_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(
        seeds = [b"vault_authority"],
        bump
    )]
    /// CHECK: This is a token account that can hold any SPL token.
    /// We verify it's a valid token account through CPI calls but don't
    /// deserialize it as Account<TokenAccount> to support multiple token types
    pub vault_authority: UncheckedAccount<'info>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

pub fn create_order(
    ctx: Context<CreateOrder>,
    _token1: String,
    _receiver: String,
    _token0amount: u64,
    _token1amount: u64,
) -> Result<()> {
    let order = &mut ctx.accounts.order;
    let order_id = &mut ctx.accounts.order_id;

    require!(_token0amount > 0, ErrorCode::ZeroAmountError);

    require!(_token1amount > 0, ErrorCode::ZeroAmountError);
    require!(_token1.len() == 20, ErrorCode::AddressLengthError);
    require!(_receiver.len() == 20, ErrorCode::AddressLengthError);

    if order_id.counter == 0 {
        order_id.counter = 1;
        order_id.bump = ctx.bumps.order_id;
    }
    order.id = order_id.counter;
    order.maker = ctx.accounts.user.key();
    order.token0 = ctx.accounts.token_0_mint.key();
    order.token1 = _token1;
    order.receiver = _receiver;
    order.token0amount = _token0amount;
    order.token1amount = _token1amount;
    order.status = StatusOrder::CREATED;
    order.bump = ctx.bumps.order;
    order.timestart = Clock::get()?.unix_timestamp;

    order_id.counter = order_id
        .counter
        .checked_add(1)
        .ok_or(ErrorCode::OveflowError)?;
    transfer_tokens(
        &ctx.accounts.maker_token_account,
        &ctx.accounts.vault_token_account,
        &_token0amount,
        &ctx.accounts.token_0_mint,
        &ctx.accounts.user,
        &ctx.accounts.token_program,
    )?;
    emit!(OrderCreated {
        timecreation: order.timestart,
        token0: order.token0,
        token1: order.token1.clone(),
        amount0: order.token0amount,
        amount1: order.token1amount,
        sender: order.maker,
        receiver: order.receiver.clone()
    });
    Ok(())
}
