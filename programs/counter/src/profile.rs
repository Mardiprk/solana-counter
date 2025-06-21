use anchor_lang::prelude::*;
use std::mem::size_of;

declare_id!("BTHp3sf89ZWMGiMTSaZSWf8UC2a1oGwKtF2ZVhQgwa6q");

const PROFILE_SEED: &[u8] = b"profile"; 
const MAX_NAME_LEN: usize = 32; // max name length  
const PROFILE_SPACE: usize = 8 + 32 + 4 + MAX_NAME_LEN + 1 + 8;

#[program]
pub mod basic_account{
    use super::*;

    pub fn create_profile(ctx: Context<CreateProfile>, name: String, age: u8) -> Result<()>{
        //checking name length so it doesn't  exceeds the limit
        require!(
            name.len() <= MAX_NAME_LEN,
            ProfileError::NameTooLong,
        );
        //checking age bounds
        require!(
            age >= 18 && age <= 100 ,
            ProfileError::InvalidAge,
        );
        let profile = &mut ctx.accounts.profile;

        profile.owner = ctx.accounts.user.key();
        profile.name = name;
        profile.age = age;
        profile.created_at = Clock::get()?.unix_timestamp;

        msg!("Profile Created for User: {}", profile.owner);
        Ok(())
    }

    pub fn update_profile(ctx: Context<UpdateProfile>, name: String, age:u8) -> Result<()>{
        let profile = &mut ctx.accounts.profile;
        require!(
            profile.owner == ctx.accounts.user.key(),
            ProfileError::Unauthorized,
        );
        //Again Checking name length
        require!(
            name.len() <= MAX_NAME_LEN,
            ProfileError::NameTooLong,
        );
        //Again checking age bounds
        require!(
            age >= 18 && age <= 100 ,
            ProfileError::InvalidAge,
        );
        profile.name = name;
        profile.age = age;

        msg!("Profile updated for user: {}", profile.owner);
        Ok(())
    }
    
    pub fn get_profile(ctx: Context<GetProfile>) -> Result<()>{
        let profile = &ctx.accounts.profile;
        msg!(
            "Profile: {}, Age: {}, Owner: {}, Created At: {}",
            profile.name,
            profile.age,
            profile.owner,
            profile.created_at
        );

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateProfile<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer = user,
        space = PROFILE_SPACE,
        seeds = [PROFILE_SEED, user.key().as_ref()],
        bump,
    )]
    pub profile: Account<'info, UserProfile>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateProfile<'info> {
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [PROFILE_SEED, user.key().as_ref()],
        bump,
    )]
    pub profile: Account<'info, UserProfile>,
}

#[derive(Accounts)]
pub struct GetProfile<'info> {
    #[account(
        seeds = [PROFILE_SEED, user.key().as_ref()],
        bump,
    )]
    pub profile: Account<'info, UserProfile>,
}

#[account]
pub struct UserProfile {
    pub owner: Pubkey,
    pub name: String,
    pub age: u8,
    pub created_at: i64,
}

#[error_code]
pub enum ProfileError {
    #[msg("You're not authorized to perform this action")]
    Unauthorized,
    #[msg("Name is too long")]
    NameTooLong,
    #[msg("Please Select an Appropriate Age")]
    InvalidAge,
    #[msg("Profile Already Exists")]
    ProfileAlreadyExists,
    #[msg("Profile Too")]
    ProfileToo,
}