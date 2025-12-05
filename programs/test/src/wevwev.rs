use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("YourProgramIdHere");

#[program]
pub mod solana_bridge {
    use super::*;

    // Инициализация бриджа - только админ
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let bridge = &mut ctx.accounts.bridge;
        bridge.admin = ctx.accounts.admin.key();
        bridge.bump = ctx.bumps.bridge;
        bridge.is_active = true;
        Ok(())
    }

    // Админ блокирует токены для отправки в Ethereum
    pub fn lock_tokens(
        ctx: Context<LockTokens>,
        amount: u64,
        recipient_eth: [u8; 20], // Куда отправить в Ethereum
        nonce: u64,
    ) -> Result<()> {
        // Проверяем, что отправитель - админ
        require!(
            ctx.accounts.admin.key() == ctx.accounts.bridge.admin,
            BridgeError::Unauthorized
        );

        require!(ctx.accounts.bridge.is_active, BridgeError::BridgeInactive);

        // Переводим токены на PDA бриджа (замораживаем)
        let cpi_accounts = Transfer {
            from: ctx.accounts.admin_token_account.to_account_info(),
            to: ctx.accounts.bridge_vault.to_account_info(),
            authority: ctx.accounts.admin.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        token::transfer(cpi_ctx, amount)?;

        emit!(TokensLocked {
            amount,
            token_mint: ctx.accounts.token_mint.key(),
            recipient_eth,
            nonce,
        });

        Ok(())
    }

    // Админ разблокирует токены для получателя в Solana
    pub fn unlock_tokens(
        ctx: Context<UnlockTokens>,
        amount: u64,
        recipient_solana: Pubkey, // Кому отправить в Solana
        nonce: u64,
    ) -> Result<()> {
        // Только админ может разблокировать токены
        require!(
            ctx.accounts.admin.key() == ctx.accounts.bridge.admin,
            BridgeError::Unauthorized
        );

        // Переводим токены из хранилища бриджа получателю
        let seeds = &[b"bridge", &[ctx.accounts.bridge.bump]];
        let signer = &[&seeds[..]];

        let cpi_accounts = Transfer {
            from: ctx.accounts.bridge_vault.to_account_info(),
            to: ctx.accounts.recipient_token_account.to_account_info(),
            authority: ctx.accounts.bridge.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);

        token::transfer(cpi_ctx, amount)?;

        emit!(TokensUnlocked {
            amount,
            recipient_solana,
            nonce,
        });

        Ok(())
    }

    // Админ включает/выключает бридж
    pub fn toggle_bridge(ctx: Context<ToggleBridge>, is_active: bool) -> Result<()> {
        require!(
            ctx.accounts.admin.key() == ctx.accounts.bridge.admin,
            BridgeError::Unauthorized
        );

        ctx.accounts.bridge.is_active = is_active;
        Ok(())
    }

    // Админ меняет себя (передача прав)
    pub fn change_admin(ctx: Context<ChangeAdmin>, new_admin: Pubkey) -> Result<()> {
        require!(
            ctx.accounts.admin.key() == ctx.accounts.bridge.admin,
            BridgeError::Unauthorized
        );

        ctx.accounts.bridge.admin = new_admin;
        Ok(())
    }
}

// Структуры аккаунтов
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        space = 8 + Bridge::INIT_SPACE,
        seeds = [b"bridge"],
        bump
    )]
    pub bridge: Account<'info, Bridge>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct LockTokens<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        seeds = [b"bridge"],
        bump = bridge.bump,
        constraint = admin.key() == bridge.admin @ BridgeError::Unauthorized
    )]
    pub bridge: Account<'info, Bridge>,

    #[account(
        mut,
        constraint = admin_token_account.owner == admin.key()
    )]
    pub admin_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = bridge_vault.owner == bridge.key()
    )]
    pub bridge_vault: Account<'info, TokenAccount>,

    pub token_mint: Account<'info, anchor_spl::token::Mint>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct UnlockTokens<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        seeds = [b"bridge"],
        bump = bridge.bump,
        constraint = admin.key() == bridge.admin @ BridgeError::Unauthorized
    )]
    pub bridge: Account<'info, Bridge>,

    #[account(
        mut,
        constraint = bridge_vault.owner == bridge.key()
    )]
    pub bridge_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = recipient_token_account.owner == recipient_solana.key()
    )]
    pub recipient_token_account: Account<'info, TokenAccount>,

    /// CHECK: Это просто получатель, проверяем в constraint выше
    pub recipient_solana: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct ToggleBridge<'info> {
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

// Структура данных бриджа - теперь проще
#[account]
#[derive(InitSpace)]
pub struct Bridge {
    pub admin: Pubkey,
    pub bump: u8,
    pub is_active: bool,
}

// События
#[event]
pub struct TokensLocked {
    pub amount: u64,
    pub token_mint: Pubkey,
    pub recipient_eth: [u8; 20],
    pub nonce: u64,
}

#[event]
pub struct TokensUnlocked {
    pub amount: u64,
    pub recipient_solana: Pubkey,
    pub nonce: u64,
}

#[error_code]
pub enum BridgeError {
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Bridge is inactive")]
    BridgeInactive,
}
