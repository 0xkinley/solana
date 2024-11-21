use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("6MQGQmnSXoSJWFaEbk5An3zEeQ2kQikuzpN1QMQFqNRF");

#[error_code]
pub enum StorageError {
    #[msg("Numeric overflow occurred")]
    NumericOverflow,
    #[msg("Insufficient storage available")]
    InsufficientStorage,
    #[msg("Invalid pod data")]
    InvalidPodData,
    #[msg("Invalid argument provided")]
    InvalidArgument,
}

pub fn calculate_base_reward(
    stored_amount: u64,
    node_capacity: u64,
    total_network_storage: u64,
) -> Result<u64> {
    const BASE_EMISSION_RATE: u64 = 1_000_000; // 1 million tokens per epoch
    const EPOCH_LENGTH: i64 = 86400; // 1 day in seconds

    let storage_share = stored_amount
        .checked_mul(10000)
        .ok_or(StorageError::NumericOverflow)?
        .checked_div(total_network_storage)
        .ok_or(StorageError::NumericOverflow)?;

    let utilization_rate = stored_amount
        .checked_mul(10000)
        .ok_or(StorageError::NumericOverflow)?
        .checked_div(node_capacity)
        .ok_or(StorageError::NumericOverflow)?;

    let reward = BASE_EMISSION_RATE
        .checked_mul(storage_share)
        .ok_or(StorageError::NumericOverflow)?
        .checked_mul(utilization_rate)
        .ok_or(StorageError::NumericOverflow)?
        .checked_div(10000)
        .ok_or(StorageError::NumericOverflow)?
        .checked_div(10000)
        .ok_or(StorageError::NumericOverflow)?;

    Ok(reward)
}

#[program]
pub mod de_pin_storage {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let storage_pool = &mut ctx.accounts.storage_pool;
        storage_pool.authority = ctx.accounts.authority.key();
        storage_pool.total_storage = 0;
        storage_pool.active_nodes = 0;
        Ok(())
    }

    pub fn register_node(
        ctx: Context<RegisterNode>,
        storage_capacity: u64,
        location: String,
        endpoint: String,
    ) -> Result<()> {
        let node = &mut ctx.accounts.node;
        let storage_pool = &mut ctx.accounts.storage_pool;

        node.owner = ctx.accounts.owner.key();
        node.storage_capacity = storage_capacity;
        node.available_storage = storage_capacity;
        node.location = location;
        node.endpoint = endpoint;
        node.is_active = true;
        node.reputation = 0;
        node.total_stored = 0;
        node.last_reward_time = 0;

        storage_pool.total_storage = storage_pool.total_storage
            .checked_add(storage_capacity)
            .ok_or(StorageError::NumericOverflow)?;
        storage_pool.active_nodes = storage_pool.active_nodes
            .checked_add(1)
            .ok_or(StorageError::NumericOverflow)?;

        Ok(())
    }

    pub fn allocate_storage(
        ctx: Context<AllocateStorage>,
        size: u64,
        duration: i64,
    ) -> Result<()> {
        let storage_allocation = &mut ctx.accounts.storage_allocation;
        let node = &mut ctx.accounts.node;
        let client = &ctx.accounts.client;

        require!(
            node.available_storage >= size,
            StorageError::InsufficientStorage
        );

        storage_allocation.client = client.key();
        storage_allocation.node = node.key();
        storage_allocation.size = size;
        storage_allocation.start_time = Clock::get()?.unix_timestamp;
        storage_allocation.duration = duration;
        storage_allocation.is_active = true;

        node.available_storage = node.available_storage
            .checked_sub(size)
            .ok_or(StorageError::NumericOverflow)?;
        node.total_stored = node.total_stored
            .checked_add(size)
            .ok_or(StorageError::NumericOverflow)?;

        Ok(())
    }

    pub fn update_metrics(
        ctx: Context<UpdateMetrics>,
        uptime: u64,
        success_rate: u64,
    ) -> Result<()> {
        let node = &mut ctx.accounts.node;
        let pool = &ctx.accounts.storage_pool;
        
        let new_reputation = uptime
            .checked_mul(success_rate)
            .ok_or(StorageError::NumericOverflow)?
            .checked_div(100)
            .ok_or(StorageError::NumericOverflow)?;
        
        node.reputation = new_reputation;

        let base_reward = calculate_base_reward(
            node.total_stored,
            node.storage_capacity,
            pool.total_storage
        )?;

        let reputation_multiplier = (10000u64)
            .checked_add(
                node.reputation
                    .checked_mul(5000)
                    .ok_or(StorageError::NumericOverflow)?
                    .checked_div(100)
                    .ok_or(StorageError::NumericOverflow)?
            )
            .ok_or(StorageError::NumericOverflow)?
            .checked_div(10000)
            .ok_or(StorageError::NumericOverflow)?;

        let reward_amount = base_reward
            .checked_mul(reputation_multiplier)
            .ok_or(StorageError::NumericOverflow)?;

        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.reward_vault.to_account_info(),
                    to: ctx.accounts.node_token_account.to_account_info(),
                    authority: ctx.accounts.storage_pool.to_account_info(),
                },
                &[&[
                    b"storage_pool",
                    &[ctx.bumps.storage_pool]
                ]]
            ),
            reward_amount,
        )?;

        node.last_reward_time = Clock::get()?.unix_timestamp;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + StoragePool::INIT_SPACE,
        seeds = [b"storage_pool"],
        bump
    )]
    pub storage_pool: Account<'info, StoragePool>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RegisterNode<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + StorageNode::INIT_SPACE
    )]
    pub node: Account<'info, StorageNode>,
    
    #[account(
        mut,
        seeds = [b"storage_pool"],
        bump
    )]
    pub storage_pool: Account<'info, StoragePool>,
    
    #[account(mut)]
    pub owner: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AllocateStorage<'info> {
    #[account(
        init,
        payer = client,
        space = 8 + StorageAllocation::INIT_SPACE
    )]
    pub storage_allocation: Account<'info, StorageAllocation>,
    
    #[account(mut)]
    pub node: Account<'info, StorageNode>,
    
    #[account(mut)]
    pub client: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateMetrics<'info> {
    #[account(mut)]
    pub node: Account<'info, StorageNode>,

    #[account(
        mut,
        seeds = [b"storage_pool"],
        bump
    )]
    pub storage_pool: Account<'info, StoragePool>,

    #[account(
        mut,
        constraint = reward_vault.mint == pool_token_mint.key()
    )]
    pub reward_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = node_token_account.mint == pool_token_mint.key(),
        constraint = node_token_account.owner == node.owner
    )]
    pub node_token_account: Account<'info, TokenAccount>,

    pub pool_token_mint: Account<'info, token::Mint>,
    pub token_program: Program<'info, Token>,
    pub authority: Signer<'info>,
}

#[account]
#[derive(InitSpace)]
pub struct StorageAllocation {
    pub client: Pubkey,
    pub node: Pubkey,
    pub size: u64,
    pub start_time: i64,
    pub duration: i64,
    pub is_active: bool,
}

#[account]
#[derive(InitSpace)]
pub struct StorageNode {
    pub owner: Pubkey,
    pub storage_capacity: u64,
    pub available_storage: u64,
    #[max_len(64)]
    pub location: String,
    #[max_len(128)]
    pub endpoint: String,
    pub is_active: bool,
    pub reputation: u64,
    pub total_stored: u64,
    pub last_reward_time: i64,
}

#[account]
#[derive(InitSpace)]
pub struct StoragePool {
    pub authority: Pubkey,
    pub total_storage: u64,
    pub active_nodes: u64,
}