use anchor_lang::prelude::*;

declare_id!("6XujACUJMDnN68Gk4iHpRCRAVeA8PfLZL2czW2SYyXdW");

#[program]
pub mod anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
