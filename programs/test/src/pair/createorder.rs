use crate::{test::Vault, Order, OrderId};
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
        token::authority = order,
        token::token_program = token_program,
        seeds = [b"vault",token_0_mint.key().as_ref()],
        bump
    )]
    pub vault_token_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
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
