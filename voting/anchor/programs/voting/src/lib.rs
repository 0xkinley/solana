#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("AsjZ3kWAUSQRNt2pZVeJkywhZ6gpLpHZmJjduPmKZDZZ");

#[program]
pub mod voting {

  use super::*;

  pub fn initialize_poll(
    ctx: Context<InitializePoll>, 
    poll_id: u64, 
    poll_description: String, 
    poll_start: u64,
    poll_end: u64,
  ) -> Result<()>{

    let poll = &mut ctx.accounts.poll;
    poll.poll_id = poll_id;
    poll.poll_description = poll_description;
    poll.poll_start = poll_start;
    poll.poll_end = poll_end;
    poll.candidate_amount = 0;

    Ok(())
  }

  pub fn initialize_candidate(
    ctx: Context<InitializeCandidate>,
    _poll_id: u64,
    candidate_name: String
  ) -> Result<()>{

    let candidate = &mut ctx.accounts.candidate;
    candidate.candidate_name = candidate_name;
    candidate.candidate_votes = 0;

    Ok(())
  }

  pub fn vote(
    ctx: Context<Vote>,
    _poll_id: u64,
    _candidate_name: String
  ) -> Result<()>{

    let vote = &mut ctx.accounts.candidate;
    vote.candidate_votes += 1;

    Ok(())
  }
    
}

#[account]
#[derive(InitSpace)]
pub struct PollAcount{
  pub poll_id: u64,
  #[max_len(280)]
  pub poll_description: String,
  pub poll_start: u64,
  pub poll_end: u64,
  pub candidate_amount: u64
}

#[derive(Accounts)]
#[instruction(poll_id: u64)]
pub struct InitializePoll<'info>{

  #[account(mut)]
  pub signer: Signer<'info>,

  #[account(
    init_if_needed,
    payer = signer,
    space = 8 + PollAcount::INIT_SPACE,
    seeds = [poll_id.to_le_bytes().as_ref()],
    bump
  )]
  pub poll: Account<'info, PollAcount>,

  pub system_program: Program<'info, System>
}

#[account]
#[derive(InitSpace)]
pub struct Candidate{
  #[max_len(50)]
  pub candidate_name: String,

  pub candidate_votes: u64,
}

#[derive(Accounts)]
#[instruction(poll_id: u64, candidate_name: String)]
pub struct InitializeCandidate<'info>{
  #[account(mut)]
  pub signer: Signer<'info>,

  pub poll: Account<'info, PollAcount>,

  #[account(
    init,
    payer = signer,
    space = 8 + Candidate::INIT_SPACE,
    seeds = [poll_id.to_le_bytes().as_ref(), candidate_name.as_ref()],
    bump
  )]
  pub candidate: Account<'info, Candidate>,

  pub system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction(poll_id: u64, candidate_name: String)]
pub struct Vote<'info>{
  #[account(mut)]
    pub signer: Signer<'info>,

  #[account(
    mut,
    seeds = [poll_id.to_le_bytes().as_ref()],
    bump,
    )]
    pub poll: Account<'info, PollAcount>,

  #[account(
    mut,
    seeds = [poll_id.to_le_bytes().as_ref(), candidate_name.as_ref()],
    bump)]
    pub candidate: Account<'info, Candidate>,
}


#[error_code]
pub enum ErrorCode{
  #[msg("Voting has not started yet")]
  VotingNotStarted,
  #[msg("Voting has ended")]
  VotingEnded
}
