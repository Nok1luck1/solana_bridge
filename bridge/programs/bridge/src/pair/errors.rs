use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Custom error")]
    CustomError,
    #[msg("Oveflow")]
    OveflowError,
    #[msg("Cant be a zero")]
    ZeroAmountError,
    #[msg("Wrong ethereum address format")]
    AddressLengthError,
    #[msg("Insufficient funds")]
    InsufficientFundsError,
}
