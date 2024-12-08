use anchor_lang::prelude::*;

declare_id!("DMkMJucBK2QV32Tsbsb9HuUv59CBa5wr1tcah3AEKYV2");

#[program]
pub mod whitelist {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
