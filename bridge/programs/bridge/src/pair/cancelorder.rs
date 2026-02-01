use super::ErrorCode;
use crate::{order::transfer_tokens, AdminConfig, Order, OrderCancelled, StatusOrder};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

#[derive(Accounts)]
pub struct CancelOrder<'info> {
    #[account(
        mut,
        seeds = [b"order", order.maker.as_ref(), order.id.to_le_bytes().as_ref()],
        bump = order.bump,
        constraint = order.status == StatusOrder::CREATED @ ErrorCode::InvalidOrderStatusError,
    )]
    pub order: Account<'info, Order>,

    #[account(
        mint::token_program = token_program
    )]
    pub token_0_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = token_0_mint,
        associated_token::authority = order.maker,
        associated_token::token_program = token_program
    )]
    pub maker_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = token_0_mint,
        associated_token::authority = vault_authority,
        associated_token::token_program = token_program,
    )]
    pub vault_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(
        seeds = [b"adminconfig"],
        bump
    )]
    pub vault_authority: Account<'info, AdminConfig>,
    pub admin_ref: Signer<'info>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn cancel_order(ctx: Context<CancelOrder>) -> Result<()> {
    let order = &mut ctx.accounts.order;
    require!(!order.locked, ErrorCode::ReentrancyDetected);
    order.locked = true;
    require!(
        order.status == StatusOrder::CREATED,
        ErrorCode::OrderStatusError
    );
    // require!(
    //     ctx.accounts
    //         .vault_authority
    //         .is_admin(&ctx.accounts.admin_ref.key()),
    //     ErrorCode::UnauthorizedAdmin
    // );
    require!(
        ctx.accounts.vault_token_account.amount >= order.token0amount,
        ErrorCode::InsufficientFundsError
    );

    let bump = ctx.bumps.vault_authority;
    let seeds = &[b"adminconfig".as_ref(), &[bump]];
    let signer_seeds = &[&seeds[..]];

    anchor_spl::token_interface::transfer_checked(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token_interface::TransferChecked {
                from: ctx.accounts.vault_token_account.to_account_info(),
                to: ctx.accounts.maker_token_account.to_account_info(),
                authority: ctx.accounts.vault_authority.to_account_info(),
                mint: ctx.accounts.token_0_mint.to_account_info(),
            },
            signer_seeds,
        ),
        order.token0amount,
        ctx.accounts.token_0_mint.decimals,
    )?;

    order.status = StatusOrder::CANCELED;
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
