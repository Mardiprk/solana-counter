use anchor_lang::prelude::*;
use std::mem::size_of;

declare_id!("BTHp3sf89ZWMGiMTSaZSWf8UC2a1oGwKtF2ZVhQgwa6q");

const PROFILE_SEED: &[*u8*] = b"profile";
const PROFILE_SPACE: usize = size_of::<UserProfile>();

#[program]
pub mod basic_account{
    use super::*;

    pub fn create_profile() -> Result<()>{
        
        Ok(())
    }

    pub fn update_profile() -> Result<()>{
        
        Ok(())
    }
    
    pub fn get_profile() -> Result<()>{
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateProfile<'info>{

}
#[derive(Accounts)]
pub struct UpdateProfile<'info>{

}
#[derive(Accounts)]
pub struct GetProfile<'info>{

}

#[account]
pub struct UserProfile{
    pub owner: 
}