use super::ErrorCode;
use anchor_lang::prelude::*;
#[account]
#[derive(InitSpace)]
pub struct AdminConfig {
    #[max_len(10)]
    pub admins: Vec<Pubkey>,
    pub settet: bool,
    pub bump: u8,
}
impl AdminConfig {
    pub fn is_admin(&self, pubkey: &Pubkey) -> bool {
        self.admins.contains(pubkey)
    }
}

#[derive(Accounts)]
pub struct ManageAdmins<'info> {
    #[account(mut)]
    pub current_admin: Signer<'info>,

    #[account(
        mut,
        seeds = [b"admin_config"],
        bump = admin_config.bump,
        constraint = admin_config.is_admin(&current_admin.key()) @ ErrorCode::UnauthorizedAdmin,
    )]
    pub admin_config: Account<'info, AdminConfig>,
}

pub fn add_admin(ctx: Context<ManageAdmins>, new_admin: Pubkey) -> Result<()> {
    let admin_config = &mut ctx.accounts.admin_config;

    require!(
        !admin_config.is_admin(&new_admin),
        ErrorCode::AdminAlreadyExists
    );
    require!(admin_config.admins.len() <= 10, ErrorCode::TooManyAdmins);

    admin_config.admins.push(new_admin);

    Ok(())
}

pub fn remove_admin(ctx: Context<ManageAdmins>, admin_to_remove: Pubkey) -> Result<()> {
    let admin_config = &mut ctx.accounts.admin_config;

    require!(
        admin_config.admins.len() > 1,
        ErrorCode::CannotRemoveLastAdmin
    );

    admin_config
        .admins
        .retain(|&admin| admin != admin_to_remove);

    Ok(())
}
