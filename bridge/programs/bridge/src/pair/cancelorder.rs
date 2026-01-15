use super::ErrorCode;
use crate::{transfer_tokens, Order, OrderCancelled, StatusOrder};
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

#[derive(Accounts)]
pub struct CancelOrder<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"order", user.key().as_ref(), order.id.to_le_bytes().as_ref()],
        bump = order.bump,
        constraint = order.maker == user.key() @ ErrorCode::UnauthorizedError,
        constraint = order.status == StatusOrder::CREATED @ ErrorCode::InvalidOrderStatusError,
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
        mut,
        token::mint = token_0_mint,
        token::authority = vault_authority,
        token::token_program = token_program,
        seeds = [b"vault", token_0_mint.key().as_ref()],
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
    pub vault_authority: Signer<'info>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

pub fn cancel_order(ctx: Context<CancelOrder>) -> Result<()> {
    let order = &ctx.accounts.order;

    transfer_tokens(
        &ctx.accounts.vault_token_account,
        &ctx.accounts.maker_token_account,
        &ctx.accounts.order.token0amount,
        &ctx.accounts.token_0_mint,
        &ctx.accounts.vault_authority,
        &ctx.accounts.token_program,
    )?;

    emit!(OrderCancelled {
        order_id: order.id,
        maker: order.maker,
        token0: order.token0,
        token1: order.token1.clone(),
        amount0: order.token0amount,
        time_cancelled: Clock::get()?.unix_timestamp,
    });

    Ok(())
}
