use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};

declare_id!("7KAJ1SJJZqN2Ut23kWkxSKc5kMgNeNYgTJV6zKJ2d7cd");

#[program]
pub mod soloshi {
    use anchor_lang::system_program::Transfer;
    use anchor_spl::token::Burn;

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        let pet = &mut ctx.accounts.pet;
        pet.hunger = 100;
        pet.owner = ctx.accounts.owner.key();
        pet.last_fed = Clock::get()?.unix_timestamp;
        Ok(())
    }

    pub fn feed_pet(ctx: Context<FeedPet>, amount: u64) -> Result<()> {
        // Burn tokens from user's token account
        let cpi_accounts = Burn {
            from: ctx.accounts.user_token_account.to_account_info(),
            mint: ctx.accounts.token_mint.to_account_info(),
            authority: ctx.accounts.owner.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        token::burn(CpiContext::new(cpi_program, cpi_accounts), amount)?;

        let pet = &mut ctx.accounts.pet;
        pet.hunger = (pet.hunger + amount as u8).min(100); // avoid overflow
        pet.last_fed = Clock::get()?.unix_timestamp;
        Ok(())
    }
}

#[account]
pub struct Pet {
    pub owner: Pubkey,
    pub hunger: u8,
    pub last_fed: i64,
}
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 1 + 8)]
    pub pet: Account<'info, Pet>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct FeedPet<'info> {
    #[account(mut, has_one = owner)]
    pub pet: Account<'info, Pet>,
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub pet_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub token_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
}
