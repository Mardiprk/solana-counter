use anchor_lang::prelude::*;

declare_id!("BTHp3sf89ZWMGiMTSaZSWf8UC2a1oGwKtF2ZVhQgwa6q");

const SEED_COUNT: &[u8] = b"counter";

#[program]
pub mod counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = 0;
        counter.authority = ctx.accounts.user.key();
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        require!(
            counter.authority == ctx.accounts.user.key(),
            ErrorCodes::Unauthorized
        );
        counter.count = counter
                    .count
                    .checked_add(1)
                    .ok_or(ErrorCodes::ArithmeticOverflow)?;

        Ok(())
    }

    pub fn delegate(ctx: Context<DelegateInput>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        require!(
            counter.authority == ctx.accounts.user.key(),
            ErrorCodes::Unauthorized
        );
        msg!("Delegation Complete");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = user,
        seeds = [SEED_COUNT, user.key().as_ref()],
        bump,
        space = 8 + 8 + 32,
    )]
    pub counter: Account<'info, Counter>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(
        mut,
        seeds = [SEED_COUNT, user.key().as_ref()],
        bump,
    )]
    pub counter: Account<'info, Counter>,

    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct DelegateInput<'info> {
    #[account(
        mut,
        seeds = [SEED_COUNT, user.key().as_ref()],
        bump,
    )]
    pub counter: Account<'info, Counter>,

    #[account(mut)]
    pub user: Signer<'info>,
}

#[account]
pub struct Counter {
    count: u64,
    authority: Pubkey,
}

#[error_code]
enum ErrorCodes {
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Arithmetic overflow")]
    ArithmeticOverflow,
}
