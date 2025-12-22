// use crate::order;
// use anchor_lang::prelude::*;
// use order::Order;

// use anchor_spl::{
//     associated_token::AssociatedToken,
//     token::{transfer_checked, TransferChecked},
//     token_interface::{Mint, TokenAccount, TokenInterface},
// };

// #[derive(Accounts)]
// #[instruction(id:u64)]
// pub struct CreateOrder<'info> {
//     #[account(mut)]
//     pub maker: Signer<'info>,
//     #[account(mint::token_program = token_program0)]
//     pub token0: InterfaceAccount<'info, Mint>,
//     #[account(
//         mut,
//         associated_token::mint = token0,
//         associated_token::authority = maker,
//         associated_token::token_program = token_program0
// )]
//     pub token_maker_account0: InterfaceAccount<'info, TokenAccount>,
//     #[account(
//         init,
//         space = 8 + Order::INIT_SPACE,
//         payer = maker,
//         seeds = [b"order",maker.key().as_ref(),id.to_le_bytes().as_ref()],
//         bump
//     )]
//     pub order: Account<'info, Order>,
//     #[account(
//         init,
//         payer = maker,
//         associated_token::mint = token0,
//         associated_token::authority = order,
//         associated_token::token_program = token_program0
// )]
//     pub token_vault: InterfaceAccount<'info, TokenAccount>,
//     pub system_program: Program<'info, System>,
//     pub token_program0: Interface<'info, TokenInterface>,
//     pub associated_token_program: Program<'info, AssociatedToken>,
// }

// pub fn create_order(ctx: Context<CreateOrder>, id: u64) -> Result<()> {
//     ctx.accounts.order.set_inner(Order {
//         id: id,
//         maker: ctx.accounts.maker.key(),
//         token0: ctx.accounts.token0.key(),
//         token1: ctx.accounts.order.token1.clone(),
//         receiver: ctx.accounts.order.receiver.clone(),
//         token0amount: ctx.accounts.order.token0amount,
//         token1amount: ctx.accounts.order.token1amount,
//         status: order::StatusOrder::CREATED,
//         bump: ctx.bumps.order,
//     });

//     let _ = transfer_tokens(
//         &ctx.accounts.token_maker_account0,
//         &ctx.accounts.token_vault,
//         &ctx.accounts.order.token0amount,
//         &ctx.accounts.token0,
//         &ctx.accounts.maker,
//         &ctx.accounts.token_program0,
//     );
//     Ok(())
// }

// pub fn transfer_tokens<'info>(
//     from: &InterfaceAccount<'info, TokenAccount>,
//     to: &InterfaceAccount<'info, TokenAccount>,
//     amount: &u64,
//     mint: &InterfaceAccount<'info, Mint>,
//     authority: &Signer<'info>,
//     token_program: &Interface<'info, TokenInterface>,
// ) -> Result<()> {
//     let transfer_accounts_options = TransferChecked {
//         from: from.to_account_info(),
//         mint: mint.to_account_info(),
//         to: to.to_account_info(),
//         authority: authority.to_account_info(),
//     };

//     let cpi_context = CpiContext::new(token_program.to_account_info(), transfer_accounts_options);
//     transfer_checked(cpi_context, *amount, mint.decimals)
// }
