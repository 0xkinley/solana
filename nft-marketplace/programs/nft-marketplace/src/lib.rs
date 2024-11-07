use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("4XERgPMNdrcW6y2is8entYWUHodnx9eURozV9YbKsquM");

#[program]
pub mod nft_marketplace {
    use super::*;

    pub fn initialize_marketplace(
        ctx: Context<InitializeMarketplace>,
        marketplace_fee: u64,
    ) -> Result<()> {
        let marketplace = &mut ctx.accounts.marketplace;
        marketplace.authority = ctx.accounts.authority.key();
        marketplace.fee = marketplace_fee;
        marketplace.total_sales = 0;
        Ok(())
    }

    pub fn list_token(
        ctx: Context<ListToken>,
        price: u64,
    ) -> Result<()> {
        let listing = &mut ctx.accounts.listing;
        listing.seller = ctx.accounts.seller.key();
        listing.token_account = ctx.accounts.token_account.key();
        listing.price = price;
        listing.active = true;

        // Transfer token custody to program
        let transfer_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.token_account.to_account_info(),
                to: ctx.accounts.vault_account.to_account_info(),
                authority: ctx.accounts.seller.to_account_info(),
            },
        );
        token::transfer(transfer_ctx, 1)?;

        Ok(())
    }

    pub fn purchase_token(
        ctx: Context<PurchaseToken>,
    ) -> Result<()> {
        let listing = &mut ctx.accounts.listing;
        require!(listing.active, ErrorCode::ListingNotActive);

        // Calculate fees
        let marketplace = &ctx.accounts.marketplace;
        let fee_amount = listing.price
            .checked_mul(marketplace.fee)
            .unwrap()
            .checked_div(10000)
            .unwrap();
        let seller_amount = listing.price.checked_sub(fee_amount).unwrap();

        // Transfer SOL payment
        let transfer_ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.buyer.key(),
            &listing.seller,
            seller_amount,
        );
        anchor_lang::solana_program::program::invoke(
            &transfer_ix,
            &[
                ctx.accounts.buyer.to_account_info(),
                ctx.accounts.seller.to_account_info(),
            ],
        )?;

        // Transfer token to buyer
        let transfer_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.vault_account.to_account_info(),
                to: ctx.accounts.buyer_token_account.to_account_info(),
                authority: ctx.accounts.marketplace.to_account_info(),
            },
        );
        token::transfer(transfer_ctx, 1)?;

        listing.active = false;
        Ok(())
    }

}

#[account]
pub struct Marketplace {
    pub authority: Pubkey,
    pub fee: u64,
    pub total_sales: u64,
}

#[account]
pub struct TokenListing {
    pub seller: Pubkey,
    pub token_account: Pubkey,
    pub price: u64,
    pub active: bool,
}

#[derive(Accounts)]
pub struct InitializeMarketplace<'info> {
    #[account(init, payer = authority, space = 8 + 32 + 8 + 8)]
    pub marketplace: Account<'info, Marketplace>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ListToken<'info> {
    #[account(mut)]
    pub marketplace: Account<'info, Marketplace>,
    #[account(
        init,
        payer = seller,
        space = 8 + 32 + 32 + 8 + 1
    )]
    pub listing: Account<'info, TokenListing>,
    #[account(mut)]
    pub seller: Signer<'info>,
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub vault_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PurchaseToken<'info> {
    #[account(mut)]
    pub marketplace: Account<'info, Marketplace>,
    #[account(mut)]
    pub listing: Account<'info, TokenListing>,
    #[account(mut)]
    pub buyer: Signer<'info>,

    /// CHECK: Safe because we check that this account's key matches the seller stored in the listing
    #[account(mut)]
    pub seller: AccountInfo<'info>,
    #[account(mut)]
    pub vault_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub buyer_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}


#[error_code]
pub enum ErrorCode {
    #[msg("Listing is not active")]
    ListingNotActive,
}
