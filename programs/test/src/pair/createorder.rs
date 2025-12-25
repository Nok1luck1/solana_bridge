//use crate::order;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

use crate::Order;

#[derive(Accounts)]
#[instruction(id:u64)]
pub struct CreateOrder<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    #[account(
            mut,
            associated_token::mint = token0mint,
            associated_token::authority = maker,
    )]
    pub token_maker_account0: Account<'info, TokenAccount>,
    #[account(
        init,
        space = 8 + Order::INIT_SPACE,
        payer = maker,
        seeds = [b"order",maker.key().as_ref(),id.to_le_bytes().as_ref()],
        bump
    )]
    pub order: Account<'info, Order>,
    #[account(
        init_if_needed,
        payer = maker,
        associated_token::mint = token0mint,
        associated_token::authority = order,
)]
    pub token0mint: Account<'info, Mint>,
    pub token_program0: Program<'info, Token>,
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
