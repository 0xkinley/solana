use anchor_lang::prelude::*;

declare_id!("EQbygpUkAveiHQgyGpVtfJ7k22JyTReG1etBUZPZshYs");

#[program]
pub mod colb_swap {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
