use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReviewError {
    #[error("Account not initialized yet")]
    UninitializedAccount,

    #[error("PDA derived does not eual PDA passed in")]
    InvalidPDA,

    #[error("Input data exceeds max length")]
    InvalidDataLength,

    #[error("Rating greater than 5 or less than 1")]
    InvalidRating,
}

impl From<ReviewError> for ProgramError {
    fn from(value: ReviewError) -> Self {
        ProgramError::Custom(value as u32)
    }
}
