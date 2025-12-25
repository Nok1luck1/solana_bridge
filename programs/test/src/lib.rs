#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;
pub mod pair;
pub use pair::*;
declare_id!("BGveEuTKJLqVRJc9sWaX3fjvhYGWesjsyXEe24nVVizA");

#[program]
pub mod test {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.bump = ctx.bumps.vault;
        vault.owner = ctx.accounts.authority.key();
        vault.count_orders = 0;
        Ok(())
    }
    pub fn increase(ctx: Context<VaultChange>, amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.count_orders += amount;
        Ok(())
    }
    pub fn decrease(ctx: Context<VaultChange>, amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.count_orders -= amount;
        Ok(())
    }

    ////fucntion signature
    #[derive(Accounts)]

    pub struct Initialize<'info> {
        #[account(
            init,
            payer = authority,
            space = 8 + Vault::INIT_SPACE,
            seeds = [b"counter-acc"],
            bump
        )]
        pub vault: Account<'info, Vault>,
        #[account(mut)]
        pub authority: Signer<'info>,
        pub system_program: Program<'info, System>,
    }
    //function signature
    #[derive(Accounts)]
    pub struct VaultChange<'info> {
        #[account(
            mut,
            seeds = [b"vault"],
            bump = vault.bump
        )]
        pub vault: Account<'info, Vault>,
    }

    #[account]
    #[derive(InitSpace)]
    pub struct Vault {
        pub count_orders: u64,
        pub owner: Pubkey,
        pub bump: u8,
    }
}
