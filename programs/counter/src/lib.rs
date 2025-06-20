use anchor_lang::prelude::*;

declare_id!("BTHp3sf89ZWMGiMTSaZSWf8UC2a1oGwKtF2ZVhQgwa6q");

const SEEDS_AMOUNT: &[u8] = b"counter";

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
            HandleError::Unauthorized,
        );
        counter.count = counter
            .count
            .checked_add(1)
            .ok_or(HandleError::ArithmeticOverflow)?; // Added ?
        Ok(())
    }

    pub fn decrement(ctx: Context<Decrement>) -> Result<()> {
        // Fixed context type
        let counter = &mut ctx.accounts.counter;
        require!(
            counter.authority == ctx.accounts.user.key(),
            HandleError::Unauthorized,
        );
        counter.count = counter
            .count
            .checked_sub(1)
            .ok_or(HandleError::ArithmeticOverflow)?; // Added ?
        Ok(())
    }

    pub fn delegate(ctx: Context<Delegate>) -> Result<()> {
        // Fixed context type
        let counter = &mut ctx.accounts.counter;
        require!(
            counter.authority == ctx.accounts.user.key(),
            HandleError::Unauthorized,
        );
        msg!("Delegation Done (mock)");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = user,
        seeds = [SEEDS_AMOUNT, user.key().as_ref()],
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
        seeds = [SEEDS_AMOUNT, user.key().as_ref()],
        bump,
    )]
    pub counter: Account<'info, Counter>,

    #[account(mut)]
    pub user: Signer<'info>,
}

// Added missing context structs
#[derive(Accounts)]
pub struct Decrement<'info> {
    #[account(
        mut,
        seeds = [SEEDS_AMOUNT, user.key().as_ref()],
        bump,
    )]
    pub counter: Account<'info, Counter>,

    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct Delegate<'info> {
    #[account(
        mut,
        seeds = [SEEDS_AMOUNT, user.key().as_ref()],
        bump,
    )]
    pub counter: Account<'info, Counter>,

    #[account(mut)]
    pub user: Signer<'info>,
}

#[account]
pub struct Counter {
    pub count: u8,
    pub authority: Pubkey,
}

#[error_code]
enum HandleError {
    #[msg("Unauthorized!")]
    Unauthorized,
    #[msg("Arithmetic Overflow")] // Fixed typo
    ArithmeticOverflow,
}
