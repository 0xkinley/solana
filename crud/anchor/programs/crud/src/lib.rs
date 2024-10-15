#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("GzurKGq4dorjxwyL4MMBqUksY7ZtDaZXQM8wTkfJzFpm");

#[program]
pub mod crud {

  use super::*;

  pub fn initialize_journal_entry(ctx: Context<CreateEntry>, title: String, message: String) -> Result<()> {

    let journal_entry = &mut ctx.accounts.journal;
    journal_entry.owner = *ctx.accounts.owner.key;
    journal_entry.title = title;
    journal_entry.message = message;

    Ok(())
  }  

  pub fn update_journal(ctx: Context<UpdateEntry>, _title: String, message: String) -> Result<()> {
    
    let journal_update = &mut ctx.accounts.update_journal;
    journal_update.message = message;

    Ok(())
    
  }

  pub fn delete_journal_entry(_ctx: Context<DeleteEntry>, _title: String) -> Result<()> {
    Ok(())
  }  
}

#[account]
#[derive(InitSpace)]
pub struct JournalEntryState {
  pub owner: Pubkey,

  #[max_len(100)]
  pub title: String,

  #[max_len(1000)]
  pub message: String,
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct CreateEntry<'info> {

  #[account(mut)]
  pub owner: Signer<'info>,

  #[account(
    init,
    payer = owner,
    space = 8 + JournalEntryState::INIT_SPACE,
    seeds = [title.as_bytes(), owner.key().as_ref()],
    bump
  )]
  pub journal: Account<'info, JournalEntryState>,

  pub system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct UpdateEntry<'info>{

  #[account(mut)]
  pub owner: Signer<'info>,

  #[account(
    mut,
    realloc = 8 + JournalEntryState::INIT_SPACE,
    realloc::payer = owner,
    realloc::zero = true,
    seeds = [title.as_bytes(), owner.key().as_ref()],
    bump
  )]
  pub update_journal: Account<'info, JournalEntryState>,

  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct DeleteEntry<'info> {

  #[account(mut)]
  pub owner: Signer<'info>,

  #[account(
    mut,
    seeds = [title.as_bytes(), owner.key().as_ref()],
    bump,
    close = owner
  )]
  pub delete_journal_entry: Account<'info, JournalEntryState>,

  pub system_program: Program<'info, System>
}


