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
    #[msg("Unauthorized: You are not the maker of this order")]
    UnauthorizedError,
    #[msg("Invalid order status: Order must be in CREATED status to cancel")]
    InvalidOrderStatusError,
}
