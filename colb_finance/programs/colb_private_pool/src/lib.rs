use anchor_lang::prelude::*;

declare_id!("7wSRuQ1thJzxWmWrsukGJCBz5pE48hCczUpNeJoHw5WH");

#[program]
pub mod colb_private_pool {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
