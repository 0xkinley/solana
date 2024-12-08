use anchor_lang::prelude::*;

declare_id!("Bt35z2yWaM1Lbs68pNLy7wCCdEfG5xCPaaoXTCwpMmhw");

#[program]
pub mod colb_usc {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
