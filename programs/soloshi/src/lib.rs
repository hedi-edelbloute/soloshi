use anchor_lang::prelude::*;

declare_id!("7KAJ1SJJZqN2Ut23kWkxSKc5kMgNeNYgTJV6zKJ2d7cd");

#[program]
pub mod soloshi {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
