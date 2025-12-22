use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer_checked, TransferChecked},
    token_interface::{Mint, TokenAccount, TokenInterface},
};

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, InitSpace, Eq)]
pub enum StatusOrder {
    CREATED,
    COMPLETED,
    CANCELED,
}

#[account]
#[derive(InitSpace)]
pub struct Order {
    pub id: u64,
    pub maker: Pubkey,
    pub token0: Pubkey,
    #[max_len(20)]
    pub token1: String,
    #[max_len(20)]
    pub receiver: String,
    pub token0amount: u64,
    pub token1amount: u64,
    pub status: StatusOrder,
    pub bump: u8,
}
#[account]
#[derive(InitSpace)]
pub struct OrderId {
    pub counter: u64,
    pub bump: u8,
}
#[derive(Accounts)]
#[instruction(id:u64)]
pub struct OrderInfo<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    #[account(mint::token_program = token_program)]
    pub token0: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = token0,
        associated_token::authority = maker,
        associated_token::token_program = token_program
)]
    pub token_maker_account0: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init,
        space = 8 + Order::INIT_SPACE,
        payer = maker,
        seeds = [b"order",maker.key().as_ref(),id.to_le_bytes().as_ref()],
        bump
    )]
    pub order: Account<'info, Order>,
    #[account(
        init,
        payer = maker,
        associated_token::mint = token0,
        associated_token::authority = order,
        associated_token::token_program = token_program
)]
    pub token_vault: InterfaceAccount<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn create_order(ctx: Context<OrderInfo>, id: u64, amount: u64) -> Result<()> {
    ctx.accounts.order.set_inner(Order {
        id: id,
        maker: ctx.accounts.maker.key(),
        token0: ctx.accounts.token0.key(),
        token1: ctx.accounts.order.token1.clone(),
        receiver: ctx.accounts.order.receiver.clone(),
        token0amount: ctx.accounts.order.token0amount,
        token1amount: ctx.accounts.order.token1amount,
        status: StatusOrder::CREATED,
        bump: ctx.bumps.order,
    });

    let _ = transfer_tokens(
        &ctx.accounts.token_maker_account0,
        &ctx.accounts.token_vault,
        &amount,
        &ctx.accounts.token0,
        &ctx.accounts.maker,
        &ctx.accounts.token_program,
    );
    Ok(())
}

pub fn transfer_tokens<'info>(
    from: &InterfaceAccount<'info, TokenAccount>,
    to: &InterfaceAccount<'info, TokenAccount>,
    amount: &u64,
    mint: &InterfaceAccount<'info, Mint>,
    authority: &Signer<'info>,
    token_program: &Interface<'info, TokenInterface>,
) -> Result<()> {
    let transfer_accounts_options = TransferChecked {
        from: from.to_account_info(),
        mint: mint.to_account_info(),
        to: to.to_account_info(),
        authority: authority.to_account_info(),
    };

    let cpi_context = CpiContext::new(token_program.to_account_info(), transfer_accounts_options);
    transfer_checked(cpi_context, *amount, mint.decimals)
}
