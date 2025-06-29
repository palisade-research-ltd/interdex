use anchor_lang::prelude::*;
use crate::{
    InitializeParams,
    InitializeResults,
    InitializeFeatures,
    InitializeIndData,
};

pub fn initialize_params(
    ctx: Context<InitializeParams>,
    weights: [f32; 5],
    bias: f32,
    ) -> Result<()> {

    let ind_params = &mut ctx.accounts.ind_params;
    let bump = ctx.bumps.ind_params;
    
    ind_params.authority = ctx.accounts.authority.key();
    ind_params.last_update = Clock::get()?.unix_timestamp;
    ind_params.weights = weights;
    ind_params.bias = bias;
    ind_params.is_active = true;
    ind_params.bump = bump;
    
    msg!("Indicator parameters initialized");
    
    Ok(())

}

pub fn initialize_results(ctx: Context<InitializeResults>) -> Result<()> {

    let ind_results = &mut ctx.accounts.ind_results;
    let bump = ctx.bumps.ind_results;
    
    ind_results.authority = ctx.accounts.authority.key();
    ind_results.last_update = Clock::get()?.unix_timestamp;
    ind_results.latest_prediction = 0;
    ind_results.price_at_prediction = 0.0;
    ind_results.predictions_count = 0;
    ind_results.bump = bump;
    
    msg!("Indicator results account initialized");
    
    Ok(())

}

pub fn initialize_features(ctx: Context<InitializeFeatures>) -> Result<()> {

    let ind_features = &mut ctx.accounts.ind_features;
    let bump = ctx.bumps.ind_features;
    
    ind_features.authority = ctx.accounts.authority.key();
    ind_features.last_update = Clock::get()?.unix_timestamp;
    ind_features.price_periods = [1; 5];
    ind_features.computed_features = [0.1; 5];
    ind_features.bump = bump;
    
    msg!("Indicator Features account initialized");
    
    Ok(())

}

pub fn initialize_ind_data(ctx: Context<InitializeIndData>) -> Result<()> {

    let ind_data = &mut ctx.accounts.ind_data;
    let bump = ctx.bumps.ind_data;
    
    ind_data.authority = ctx.accounts.authority.key();
    ind_data.last_updated = 0;
    ind_data.current_index = 0;
    ind_data.prices = [0.0; 10];
    ind_data.timestamps = [0; 10];
    ind_data.is_full = false;
    ind_data.bump = bump;
    
    msg!("Indicator Data account initialized with capacity for 10 price points");
    
    Ok(())

}

