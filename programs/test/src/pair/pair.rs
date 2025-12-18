use anchor_lang::prelude::*;

use anchor_spl::associated_token::AssociatedToken;

enum StatusOrder {
    INITED,
    COMPLETED,
    CANCELED,
}

#[derive(Accounts)]
pub struct BridgeOrder<'info> {
    #[account(mint::token_program = token_program)]
    pub token0: InterfaceAccount<'info, Mint>,
    #[max_length(20)]
    pub token1: String,
    pub status: StatusOrder,
    pub bump: u8,
}
