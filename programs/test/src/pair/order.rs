use anchor_lang::prelude::*;

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
