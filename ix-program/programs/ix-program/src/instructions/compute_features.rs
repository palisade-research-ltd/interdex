use anchor_lang::prelude::*;
use crate::ComputeFeatures;
use crate::operations::linear;
use crate::errors::IndError;

pub fn compute_features(
    ctx: Context<ComputeFeatures>
    ) -> Result<()> {

    let ind_features = &mut ctx.accounts.ind_features;
    let ind_data = &ctx.accounts.ind_data;
    
    // Get required number of prices for longest moving average
    let max_period = ind_features.price_periods.iter().max().unwrap_or(&50);
    
    // Get recent prices
    let recent_prices = ind_data.get_recent_prices(*max_period as usize);
    
    // Initialize moving average calculator
    let ma_calculator = linear::MovingAverageCalculator::new(&ind_features.price_periods);
    
    // Calculate simple moving averages
    let moving_averages = ma_calculator.calculate_sma(&recent_prices)
        .map_err(|_| IndError::FeatureCalculationFailed)?;
    
    // Update indicator results with calculated features
    ind_features.last_update = Clock::get()?.unix_timestamp;
    ind_features.computed_features = moving_averages;
    
    msg!("Features values = [{:.6}, {:.6}, {:.6}, {:.6}, {:.6}]",
         moving_averages[0], moving_averages[1], moving_averages[2],
         moving_averages[3], moving_averages[4]);
    
    Ok(())
}
