
pub mod errors;

#[error_code]
pub enum ErrorCode {
    #[msg("Amount must be greater than zero")]
    InvalidAmount,
    #[msg("Insufficient funds for operation")]
    InsufficientFunds,
}

