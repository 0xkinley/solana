use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("7wQx44y6ggAPVLBgPm7RuZCpTbYsqerJ4v7Yw2x3MyuU");

#[program]
pub mod split_bill_pay {
    use super::*;

    pub fn initialize_split(ctx: Context<InitializeSplit>, total_amount: u64) -> Result<()> {

        let split_bill = &mut ctx.accounts.split_bill;
        split_bill.authority = ctx.accounts.initializer.key();
        split_bill.total_amount = total_amount;
        split_bill.contributors = Vec::new();
        Ok(())
    }

    pub fn contribute(ctx: Context<Contribute>, contribution: u64) -> Result<()> {

        let split_bill = &mut ctx.accounts.split_bill;

        let total_contribution = split_bill.contributors.iter().map(|c| c.amount).sum::<u64>();

        if total_contribution + contribution > split_bill.total_amount {
            return Err(ErrorCode::OverContribution.into());
        };

        let contributor = Contributor {
            address: ctx.accounts.contributor.key(),
            amount: contribution,
        };
        split_bill.contributors.push(contributor);

        token::transfer(
            ctx.accounts.into_transfer_to_split_context(),
            contribution,
        )?;

        Ok(())
    }

    pub fn finalize(ctx: Context<FinalizeSplit>) -> Result<()> {

        let split_bill = &mut ctx.accounts.split_bill;
        let total_contribution = split_bill.contributors.iter().map(|c| c.amount).sum::<u64>();

        if total_contribution == split_bill.total_amount {
            token::transfer(ctx.accounts.into_transfer_to_receiver_context(), total_contribution)?;
        }else{
            return Err(ErrorCode::IncompleteContributions.into());
        }

        Ok(())
    }


}

#[account]
pub struct SplitBill {
    pub authority: Pubkey,
    pub total_amount: u64,
    pub contributors: Vec<Contributor>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Contributor{
    pub address: Pubkey,
    pub amount: u64,
}

#[derive(Accounts)]
pub struct InitializeSplit<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,

    #[account(
        init,
        space = 8 + 32 + 8 + 20*(32+8),
        payer = initializer,
        seeds = [b"split_bill".as_ref(), initializer.key().as_ref()],
        bump,    
    )]
    pub split_bill: Account<'info, SplitBill>,

    /// CHECK: This account is not loaded in this instruction and serves only as a destination for the final transfer.
    pub receiver: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Contribute<'info>{
    #[account(mut)]
    pub split_bill: Account<'info, SplitBill>,

    #[account(mut)]
    pub contributor: Signer<'info>,

    #[account(mut)]
    pub contributor_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub split_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

impl<'info> Contribute<'info>{
    fn into_transfer_to_split_context(&self) -> CpiContext<'info, 'info, 'info, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {from: self.contributor_token_account.to_account_info().clone(),
        to: self.split_token_account.to_account_info().clone(),
        authority: self.contributor.to_account_info().clone(),
        };
        CpiContext::new(self.token_program.to_account_info().clone(), cpi_accounts)
    }
}

#[derive(Accounts)]
pub struct FinalizeSplit<'info>{
    #[account(mut)]
    pub split_bill: Account<'info, SplitBill>,

    #[account(mut)]
    pub receiver_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub split_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

impl<'info> FinalizeSplit<'info>{
    fn into_transfer_to_receiver_context(&self) -> CpiContext<'info, 'info, 'info, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
        from: self.split_token_account.to_account_info().clone(),
        to: self.receiver_token_account.to_account_info().clone(),
        authority: self.split_bill.to_account_info().clone(),
        };
        CpiContext::new(self.token_program.to_account_info().clone(), cpi_accounts)
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("Contribution amount exceeds total required.")]
    OverContribution,
    #[msg("Contributions are incomplete.")]
    IncompleteContributions,
}


