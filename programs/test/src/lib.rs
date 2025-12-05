use anchor_lang::prelude::*;

declare_id!("8yARbjdaYfmV2EPTRAJBQTcq8SfcqeKCssmuXmfuus8k");

#[program]
pub mod test {

    use super::*;
    pub fn initialize(ctx: Context<Initialize>,admin:Pubkey) -> Result<()> {
        let bridge = &mut ctx.accounts.bridge
        Ok(())
    }
}
