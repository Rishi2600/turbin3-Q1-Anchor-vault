use anchor_lang::prelude::*;

declare_id!("6FkkMGktgzsNYfychBqrXq2VJH5nHbtbgDp6yhC4xSmV");

#[program]
pub mod anchor_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
