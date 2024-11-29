use anchor_lang::prelude::*;

declare_id!("Ek4xHCupi9Mso6uGQU4XaTh5S8T5c66GmEHnmsX3rwq1");

#[program]
pub mod whitelist {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>
    ) -> Result<()> 
    {
        let whitelist_account = &mut ctx.accounts.whitelist_account;
        whitelist_account.authority = ctx.accounts.authority.key();
        Ok(())
    }

    pub fn add_to_whitelist(
        ctx: Context<ModifyWhitelist>, 
        address: Pubkey
    ) -> Result<()> 
    {
        require!(
            ctx.accounts.authority.key() == ctx.accounts.whitelist_account.authority,
            WhitelistError::Unauthorized
        );
        
        let whitelist_account = &mut ctx.accounts.whitelist_account;
        require!(
            !whitelist_account.addresses.contains(&address),
            WhitelistError::AlreadyWhitelisted
        );
        
        whitelist_account.addresses.push(address);
        Ok(())
    }

    pub fn remove_from_whitelist(
        ctx: Context<ModifyWhitelist>, 
        address: Pubkey
    ) -> Result<()> 
    {
        require!(
            ctx.accounts.authority.key() == ctx.accounts.whitelist_account.authority,
            WhitelistError::Unauthorized
        );
        
        let whitelist_account = &mut ctx.accounts.whitelist_account;
        let position = whitelist_account.addresses.iter()
            .position(|x| x == &address)
            .ok_or(WhitelistError::AddressNotFound)?;
            
        whitelist_account.addresses.remove(position);
        Ok(())
    }

    pub fn is_whitelisted(
        ctx: Context<CheckWhitelist>, 
        address: Pubkey
    ) -> Result<bool> 
    {
        let whitelist_account = &ctx.accounts.whitelist_account;
        Ok(whitelist_account.addresses.contains(&address))
    }
}

#[account]
pub struct WhitelistAccount {
    pub authority: Pubkey,
    pub addresses: Vec<Pubkey>,
}

#[derive(Accounts)]
pub struct CheckWhitelist<'info> {
    pub whitelist_account: Account<'info, WhitelistAccount>,
}

#[derive(Accounts)]
pub struct ModifyWhitelist<'info> {
    #[account(mut)]
    pub whitelist_account: Account<'info, WhitelistAccount>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init, 
        payer = authority, 
        space = 8 + 32 + 4 + (32 * 100))]
    pub whitelist_account: Account<'info, WhitelistAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum WhitelistError {
    #[msg("Only authority can modify whitelist")]
    Unauthorized,
    #[msg("Address already whitelisted")]
    AlreadyWhitelisted,
    #[msg("Address not found in whitelist")]
    AddressNotFound,
}
