use anchor_lang::prelude::*;

declare_id!("BTHp3sf89ZWMGiMTSaZSWf8UC2a1oGwKtF2ZVhQgwa6q");

#[program]
pub mod counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Hello World");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
