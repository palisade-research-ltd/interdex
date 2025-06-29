use anchor_lang::prelude::*;

#[error_code]
pub enum IndError {
    #[msg("Indicator parameters not initialized")]
    IndicatorNotInitialized,
    
    #[msg("Insufficient price history for calculations")]
    InsufficientPriceHistory,
    
    #[msg("Price data is too old")]
    StalePriceData,
    
    #[msg("Invalid price data received")]
    InvalidPriceData,
    
    #[msg("Indicator prediction failed")]
    PredictionFailed,
    
    #[msg("Feature calculation failed")]
    FeatureCalculationFailed,
    
    #[msg("Unauthorized access")]
    Unauthorized,
    
    #[msg("Indicator is not active")]
    IndicatorInactive,
    
    #[msg("Price confidence too low")]
    LowPriceConfidence,
    
    #[msg("Mathematical computation error")]
    ComputationError,
    
    #[msg("Array length mismatch")]
    ArrayLengthMismatch,
    
    #[msg("Price feed not found")]
    PriceFeedNotFound,
    
    #[msg("Invalid feed ID")]
    InvalidFeedId,
    
    #[msg("Training data incomplete")]
    IncompleteTrainingData,
    
    #[msg("Metrics update failed")]
    MetricsUpdateFailed,
}
