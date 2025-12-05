use anchor_lang::prelude;
pub mod state;
//as i understand that this accounts will be placed inside main program
#[account]
#[derive(InitSpace, Debug)]
pub struct UserInfo {
    pub owner: Pubkey,         // Владелец (кошелек)
    pub eth_address: [u8; 20], // ETH адрес
    pub bump: u8,              // PDA bump
    pub order_count: u64,      // Счетчик заказов
                               // Больше не храним массивы здесь!
}

// Структура для отдельного заказа
#[account]
#[derive(InitSpace, Debug)]
pub struct Order {
    pub id: u64,             // ID заказа
    pub owner: Pubkey,       // Владелец (ссылка на UserInfo PDA)
    pub config: Pubkey,      // Конфиг (общий)
    pub token0: Pubkey,      // Токен в сети Solana
    pub token1: [u8; 20],    // Токен в ETH (адрес контракта)
    pub amount_token0: u128, // Сумма token0
    pub amount_token1: u128, // Сумма token1
    pub status_token0: bool, // Статус для token0
    pub status_token1: bool, // Статус для token1
    pub bump: u8,            // PDA bump для этого заказа
}

#[derive(Accounts)]
pub struct Initialize {
    #[account(mut)]
    pub admin: Signer<'info>,
    pub bump: u8,
    pub is_active: bool,
}

#[account]
#[derive(InitSpace)]
pub struct Bridge {
    pub admin: Pubkey,
    pub eth_bridge_address: [u8; 20],
    pub relayer: Pubkey, // Релеер, который может вызывать unlock
    pub bump: u8,
    pub is_active: bool,
}

//without #[account] because its used like signature of function
#[derive(Accounts)]
pub struct ChangeAdmin<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [b"bridge"],
        bump = bridge.bump,
        has_one = admin @ BridgeError::Unauthorized
    )]
    pub bridge: Account<'info, Bridge>,
}
// #[derive(AnchorSerialize,AnchorDeserialize,Clone,Copy,InitSpace)]
// pub struct BasicOrder{
//     pub token0:
// }
