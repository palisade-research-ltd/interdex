use anchor_lang::prelude::*;
use crate::{
    InitializePFBuffer,
    InitializePFStats,
};

pub fn initialize_pf_buffer(ctx: Context<InitializePFBuffer>) -> Result<()> {

    let pf_buffer = &mut ctx.accounts.pf_buffer;
    let bump = ctx.bumps.pf_buffer;
    
    pf_buffer.authority = ctx.accounts.authority.key();
    pf_buffer.last_updated = Clock::get()?.unix_timestamp;
    pf_buffer.current_index = 0;
    pf_buffer.slots = [0; 16];
    pf_buffer.fees = [0; 16];
    pf_buffer.is_full = false;
    pf_buffer.bump = bump;
    
    msg!("Data Prices account initialized with capacity for 10 price points");
    
    Ok(())

}

pub fn initialize_pf_stats(ctx: Context<InitializePFStats>) -> Result<()> {

    let pf_stats = &mut ctx.accounts.pf_stats;
    let bump = ctx.bumps.pf_stats;
    
    pf_stats.authority = ctx.accounts.authority.key();
    pf_stats.last_updated = Clock::get()?.unix_timestamp;
    pf_stats.sample_count = 0;
    pf_stats.sum = 0;
    pf_stats.min = 0;
    pf_stats.max = 0;
    pf_stats.last_slot = 0;
    pf_stats.bump = bump;
    
    msg!("Data Prices account initialized with capacity for 10 price points");
    
    Ok(())

}


