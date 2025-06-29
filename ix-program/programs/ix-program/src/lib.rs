#![allow(unexpected_cfgs)]
#![allow(deprecated)]

use anchor_lang::prelude::*;
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;

/// To initialize state accounts
use crate::state::{
    ind_params::IndParameters,
    ind_results::IndResults,
    ind_features::IndFeatures,
    ind_data::IndData,
};

// Program ID
declare_id!("7kdisgriKw2idW8o6jJeCFVATiKwzyGb8F4wwhatDein");

/// To execute instructions
pub use instructions::initialize;

/// OnChain Instructions
pub mod instructions;

/// OnChain State Trackers
pub mod state;

/// OnChain compatible perations 
pub mod operations;

/// OnChain Data IO
pub mod data;

/// SVM operations related errors
pub mod errors;

#[program]
pub mod ix_program {

    use super::*;

    /// Initialize indicator parameters account
    pub fn initialize_params(ctx: Context<InitializeParams>, weights: [f32; 5], bias: f32,
    ) -> Result<()> {
        instructions::initialize::initialize_params(ctx, weights, bias)
    }
     
    /// Initialize indicator results account
    pub fn initialize_results(ctx: Context<InitializeResults>) -> Result<()> {
        instructions::initialize::initialize_results(ctx)
    }
    
    /// Initialize indicator features account
    pub fn initialize_features(ctx: Context<InitializeFeatures>) -> Result<()> {
        instructions::initialize::initialize_features(ctx)
    }
    
    /// Initialize price history account
    pub fn initialize_ind_data(ctx: Context<InitializeIndData>) -> Result<()> {
        instructions::initialize::initialize_ind_data(ctx)
    }

    /// Fetch latest prices from Pyth oracle and update price history
    pub fn fetch_and_store_price(ctx: Context<FetchPrice>) -> Result<()> {
        instructions::fetch_data::fetch_and_store_price(ctx)
    }

    /// Calculate features from price history
    pub fn compute_features(ctx: Context<ComputeFeatures>) -> Result<()> {
        instructions::compute_features::compute_features(ctx)
    }

}

#[derive(Accounts)]
pub struct InitializeParams<'info> {

    #[account(
        init,
        payer = authority,
        space = IndParameters::LEN,
        seeds = [b"ind_params", authority.key().as_ref()],
        bump
    )]
    pub ind_params: Account<'info, IndParameters>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,

}

#[derive(Accounts)]
pub struct InitializeResults<'info> {

    #[account(
        init,
        payer = authority,
        space = IndResults::LEN,
        seeds = [b"ind_results", authority.key().as_ref()],
        bump
    )]
    pub ind_results: Account<'info, IndResults>,
    
    #[account(
        seeds = [b"ind_params", authority.key().as_ref()],
        bump
    )]
    pub ind_params: Account<'info, IndParameters>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,

}

#[derive(Accounts)]
pub struct InitializeFeatures<'info> {

    #[account(
        init,
        payer = authority,
        space = IndFeatures::LEN,
        seeds = [b"ind_features", authority.key().as_ref()],
        bump
    )]
    pub ind_features: Account<'info, IndFeatures>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,

}

#[derive(Accounts)]
pub struct InitializeIndData<'info> {

    #[account(
        init,
        payer = authority,
        space = IndData::LEN,
        seeds = [b"ind_data", authority.key().as_ref()],
        bump
    )]
    pub ind_data: Account<'info, IndData>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,

}

#[derive(Accounts)]
pub struct FetchPrice<'info> {

    #[account(
        mut,
        seeds = [b"ind_data", authority.key().as_ref()],
        bump
    )]

    pub ind_data: Account<'info, IndData>,
    pub price_update: Account<'info, PriceUpdateV2>,
    
    #[account(mut)]
    pub authority: Signer<'info>,

}

#[derive(Accounts)]
pub struct ComputeFeatures<'info> {
    
    #[account(
        mut,
        seeds = [b"ind_features", authority.key().as_ref()],
        bump
    )]
    pub ind_features: Account<'info, IndFeatures>,

    #[account(
        seeds = [b"ind_data", authority.key().as_ref()],
        bump
    )]
    pub ind_data: Account<'info, IndData>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct IndInference<'info> {
    #[account(
        seeds = [b"ind_params", authority.key().as_ref()],
        bump
    )]
    pub ind_params: Account<'info, IndParameters>,
    
    #[account(
        mut,
        seeds = [b"ind_results", authority.key().as_ref()],
        bump
    )]
    pub ind_results: Account<'info, IndResults>,
    
    #[account(
        seeds = [b"ind_features", authority.key().as_ref()],
        bump
    )]
    
    pub ind_features: Account<'info, IndFeatures>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
}

