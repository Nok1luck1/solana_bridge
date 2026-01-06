use super::ErrorCode;
use crate::{transfer_tokens, Order, OrderId, StatusOrder};
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

#[derive(Accounts)]
pub struct ExecuteOrderFromAnotherChain<'info> {
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
        space = Order::INIT_SPACE,
        seeds = [b"order",admin.key().as_ref(),order_id.counter.to_le_bytes().as_ref()],
        bump
    )]
    pub order: Account<'info, Order>,
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
    pub vault_authority: UncheckedAccount<'info>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}
