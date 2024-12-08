use anchor_lang::prelude::*;

declare_id!("HxQhUN65mxvfhDm1UpPiaL4Xa833uGWyT2snTXGG1ZSi");

#[program]
pub mod colb_private_factory {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
