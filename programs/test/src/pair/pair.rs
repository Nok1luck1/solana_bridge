use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};
use crate::event::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
enum StatusOrder {
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
    #[max_length(20)]
    pub token1: String,
    #[max_length(20)]
    pub receiver:String,
    pub token0amount: u64,
    pub token1amount: u64,
    pub bump: u8,
}

#[derive(Accounts)]
pub struct BridgeOrder<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mint::token_program = token_program)]
    pub token0: InterfaceAccount<'info, Mint>,
    #[max_length(20)]
    pub token1: String,
    #[account(mut,
    associated_token::mint = token0,
    associated_token::authority = user,
    associated_token::token_program = token_program    
)]
    pub token_maker_account0: InterfaceAccount<'info, TokenAccount>,
    pub status: StatusOrder,
    #[account(
    init,
    payer = user,
    associated_token::mint = token0,
    associated_token::authority = order,
    associated_token::token_program = token_program
)]
    pub token_vault:InterfaceAccount<'info,TokenAccount>,
    pub system_program:Program<'info,System>,
    pub token_program:Interface<'info,TokenInterface>,
    pub associated_token:Program<'info,AssociatedToken>
}

pub fn create_order(ctx:&Context<BridgeOrder>,amount:u64)->Result<()>{
    transfer_tokens(
        &ctx.accounts.token_maker_account0,
        &ctx.accounts.token_vault,
        amount,
        &ctx.accounts.token0,
        &ctx.accounts.user,
        &ctx.accounts.token_program
    );
   emit!(OrderCreated{

   });
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


