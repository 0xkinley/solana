use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, Transfer};
use anchor_spl::token_interface::{Mint, TokenAccount};
use anchor_spl::associated_token::AssociatedToken;

declare_id!("9HyLh9MeLAzdgYdXPSwY4cSHEHHL1PBayhv1ecC81DjS");

#[program]
pub mod split_bill {
    use super::*;

    pub fn initialize_split(
        ctx: Context<InitializeSplit>, 
        authority: Pubkey,
        name: String, 
        total_amount: u64
    ) -> Result<()> {
        require!(!name.is_empty(), ErrorCode::InvalidBillName);
        require!(total_amount > 0, ErrorCode::InvalidAmount);

        let split_bill = &mut ctx.accounts.bill;
        split_bill.authority = authority;
        split_bill.bill_name = name;
        split_bill.total_amount = total_amount;
        split_bill.contributors = Vec::new();
        split_bill.is_settled = false;

        Ok(())
    }

    pub fn contribute(
        ctx: Context<Contribute>, 
        _authority: Pubkey,
        _name: String, 
        contribution: u64
    ) -> Result<()> {
        {
            let split_bill = &mut ctx.accounts.bill;
            
            require!(contribution > 0, ErrorCode::InvalidContribution);
            require!(!split_bill.is_settled, ErrorCode::BillAlreadySettled);
    
            let total_contribution = split_bill
                .contributors
                .iter()
                .map(|c| c.amount)
                .sum::<u64>();
    
            require!(
                total_contribution + contribution <= split_bill.total_amount,
                ErrorCode::OverContribution
            );
    
            require!(
                !split_bill.contributors.iter().any(|c| c.address == ctx.accounts.contributor.key()),
                ErrorCode::DuplicateContributor
            );
    
            let contributor = Contributor {
                address: ctx.accounts.contributor.key(),
                amount: contribution,
            };
    
            split_bill.contributors.push(contributor);
        } 
    
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.contributor_token_account.to_account_info(),
                    to: ctx.accounts.split_token_account.to_account_info(),
                    authority: ctx.accounts.contributor.to_account_info(),
                },
            ),
            contribution,
        )?;
    
        {
            let split_bill = &mut ctx.accounts.bill;
            let new_total = split_bill
                .contributors
                .iter()
                .map(|c| c.amount)
                .sum::<u64>();
                
            if new_total == split_bill.total_amount {
                split_bill.is_settled = true;
            }
        }
    
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        let split_bill = &ctx.accounts.bill;
        
        require!(split_bill.is_settled, ErrorCode::BillNotSettled);
        require!(
            split_bill.authority == ctx.accounts.authority.key(),
            ErrorCode::UnauthorizedWithdrawal
        );

        let amount = ctx.accounts.split_token_account.amount;
        token::transfer(
            ctx.accounts.into_transfer_context(),
            amount,
        )?;

        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct SplitBill {
    pub authority: Pubkey,
    
    #[max_len(50)]
    pub bill_name: String,
    pub total_amount: u64,
    
    #[max_len(10)]
    pub contributors: Vec<Contributor>,
    pub is_settled: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct Contributor {
    pub address: Pubkey,
    pub amount: u64,
}

#[derive(Accounts)]
#[instruction(authority: Pubkey, bill_name: String)]
pub struct InitializeSplit<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,

    #[account(
        init,
        space = 8 + SplitBill::INIT_SPACE,
        payer = initializer,
        seeds = [authority.key().as_ref(), bill_name.as_ref()],
        bump,
    )]
    pub bill: Account<'info, SplitBill>,

    /// CHECK: This account is not loaded in this instruction
    pub receiver: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(authority: Pubkey, bill_name: String)]
pub struct Contribute<'info> {
    #[account(
        mut,
        seeds = [authority.as_ref(), bill_name.as_ref()],
        bump
    )]
    pub bill: Account<'info, SplitBill>,

    #[account(mint::token_program = token_program)]
    pub token: InterfaceAccount<'info, Mint>,

    #[account(mut)]
    pub contributor: Signer<'info>,

    #[account(
        init_if_needed,
        payer = contributor,
        associated_token::mint = token,
        associated_token::authority = contributor,
        associated_token::token_program = token_program
    )]
    pub contributor_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(mut)]
    pub split_token_account: InterfaceAccount<'info, TokenAccount>,
    
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub bill: Account<'info, SplitBill>,

    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub split_token_account: InterfaceAccount<'info, TokenAccount>,
    
    #[account(mut)]
    pub receiver_token_account: InterfaceAccount<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,
}

impl<'info> Withdraw<'info> {
    fn into_transfer_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.split_token_account.to_account_info(),
            to: self.receiver_token_account.to_account_info(),
            authority: self.authority.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("Contribution amount exceeds total required.")]
    OverContribution,
    #[msg("Contributions are incomplete.")]
    IncompleteContributions,
    #[msg("Invalid bill name provided.")]
    InvalidBillName,
    #[msg("Invalid amount provided.")]
    InvalidAmount,
    #[msg("Invalid contribution amount.")]
    InvalidContribution,
    #[msg("Contributor has already contributed.")]
    DuplicateContributor,
    #[msg("Bill is already settled.")]
    BillAlreadySettled,
    #[msg("Bill is not fully settled yet.")]
    BillNotSettled,
    #[msg("Only the authority can withdraw funds.")]
    UnauthorizedWithdrawal,
}