//import statment
use anchor_lang::prelude::*;



declare_id!("7gEYEaNig8zdpg23eFpb7hvVM8py63WPEZPYxx9hrWCx");

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
