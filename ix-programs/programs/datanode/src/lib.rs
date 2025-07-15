#![allow(unexpected_cfgs)]
#![allow(deprecated)]

use anchor_lang::prelude::*;

/// To initialize state accounts
use crate::state::{
    pf_buffer::PriorityFeesBuffer,
    pf_stats::PriorityFeesStats,
};

// Program ID
declare_id!("AHpDZgjEwGmcmRBqbMtW6KsXvBAtxAQeRjj8fd5NQNMc");

/// To execute instructions
pub use instructions::initialize;

/// OnChain Instructions
pub mod instructions;

/// Indicator's formulations
pub mod indicators;

/// OnChain State Trackers
pub mod state;

/// SVM operations related errors
pub mod errors;

#[program]
pub mod datanode {

    use super::*;

    /// Initialize Priority Fees Buffer
    pub fn initialize_pf_buffer(ctx: Context<InitializePFBuffer>) -> Result<()> {
        instructions::initialize::initialize_pf_buffer(ctx)
    }
    
    /// Initialize Priority Fees Aggregated Stats
    pub fn initialize_pf_stats(ctx: Context<InitializePFStats>) -> Result<()> {
        instructions::initialize::initialize_pf_stats(ctx)
    }
}

#[derive(Accounts)]
pub struct InitializePFBuffer<'info> {

    #[account(
        init,
        payer = authority,
        space = PriorityFeesBuffer::LEN,
        seeds = [b"pf_buffer", authority.key().as_ref()],
        bump
    )]

    pub pf_buffer: Account<'info, PriorityFeesBuffer>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,

}

#[derive(Accounts)]
pub struct InitializePFStats<'info> {

    #[account(
        init,
        payer = authority,
        space = PriorityFeesStats::LEN,
        seeds = [b"pf_stats", authority.key().as_ref()],
        bump
    )]

    pub pf_stats: Account<'info, PriorityFeesStats>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,

}

