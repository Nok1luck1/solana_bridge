use anchor_lang::prelude::*;
use anchor_spl::{
    token::{transfer_checked, TransferChecked},
    token_interface::{Mint, TokenAccount, TokenInterface},
};

#[account]
#[derive(InitSpace)]
pub struct Order {
    pub id: u64,
    pub maker: Pubkey,
    pub token0: Pubkey,
    #[max_len(42)]
    pub token1: String,
    #[max_len(42)]
    pub receiver: String,
    pub token0amount: u64,
    pub token1amount: u64,
    pub status: StatusOrder,
    pub locked: bool,
    pub bump: u8,
    pub timestart: i64,
}
#[account]
#[derive(InitSpace)]
pub struct OrderExecution {
    pub id: u64,
    #[max_len(42)]
    pub maker: String,
    #[max_len(42)]
    pub token0: String,
    pub token1: Pubkey,
    pub receiver: Pubkey,
    pub token0amount: u64,
    pub token1amount: u64,
    pub status: StatusOrder,
    pub bump: u8,
    pub timeend: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, InitSpace, Eq)]
pub enum StatusOrder {
    CREATED,
    COMPLETED,
    CANCELED,
}
#[account]
#[derive(InitSpace)]
pub struct OrderId {
    pub counter: u64,
    pub bump: u8,
}
pub const WSOL_MINT: Pubkey = pubkey!("So11111111111111111111111111111111111111112");
pub fn transfer_tokens<'info>(
    from: &InterfaceAccount<'info, TokenAccount>,
    to: &InterfaceAccount<'info, TokenAccount>,
    amount: &u64,
    mint: &InterfaceAccount<'info, Mint>,
    authority: &Signer<'info>,
    token_program: &Interface<'info, TokenInterface>,
) -> Result<()> {
    let transfer_accounts = TransferChecked {
        from: from.to_account_info(),
        mint: mint.to_account_info(),
        to: to.to_account_info(),
        authority: authority.to_account_info(),
    };

    let cpi_ctx = CpiContext::new(token_program.to_account_info(), transfer_accounts);

    transfer_checked(cpi_ctx, *amount, mint.decimals)
}
