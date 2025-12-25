use crate::{test::Vault, Order};
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

#[derive(Accounts)]

pub struct CreateOrder<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        seeds = [b"vault",vault.owner.key().as_ref()],
        bump = vault.bump)]
    pub vault: Account<'info, Vault>,
    #[account(
        init,
        space = 8 + Order::INIT_SPACE,
        payer = user,
        seeds = [b"order",user.key().as_ref(),id.to_le_bytes().as_ref()],
        bump
    )]
    pub order: Account<'info, Order>,
    #[account(
        mint::token_program = token_program
    )]
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        token::mint = mint,
        token::authority = vault,
        token::token_program = token_program
    )]
    pub vault_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        token::mint = mint,
        token::authority = user,
        token::token_program = token_program
    )]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_account: Program<'info, System>,
}

// // pub fn create_order(ctx: Context<CreateOrder>, id: u64) -> Result<()> {
// //     ctx.accounts.order.set_inner(Order {
// //         id: id,
// //         maker: ctx.accounts.maker.key(),
// //         token0: ctx.accounts.token0.key(),
// //         token1: ctx.accounts.order.token1.clone(),
// //         receiver: ctx.accounts.order.receiver.clone(),
// //         token0amount: ctx.accounts.order.token0amount,
// //         token1amount: ctx.accounts.order.token1amount,
// //         status: order::StatusOrder::CREATED,
// //         bump: ctx.bumps.order,
// //     });

// //     let _ = transfer_tokens(
// //         &ctx.accounts.token_maker_account0,
// //         &ctx.accounts.token_vault,
// //         &ctx.accounts.order.token0amount,
// //         &ctx.accounts.token0,
// //         &ctx.accounts.maker,
// //         &ctx.accounts.token_program0,
// //     );
// //     Ok(())
// // }
