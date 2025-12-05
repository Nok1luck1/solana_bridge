use anchor_lang::prelude::*;

#[error_code]
pub enum BridgeError {
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Bridge is inactive")]
    BridgeInactive,
    #[msg("Invalid nonce")]
    InvalidNonce,
    #[msg("Transfer failed")]
    TransferFailed,
}
