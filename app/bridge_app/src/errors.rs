use anchor_client::solana_sdk::message;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FormatError {
    #[error("Invalid header (expected {expected:?}, got {found:?})")]
    InvalidHeader { expected: String, found: String },
    #[error("Missing attribute: {0}")]
    MissingAttribute(String),
    #[error("Bridge has insufficient balance, has {has:?}, needed {neeed:?}")]
    BalanceError { has: String, neeed: String },
    #[error("mismatch address user, has {has:?}, needed {must_have:?}")]
    MismatchAddressInDb { has: String, must_have: String },
    #[error("Parse error")]
    ParseError,
    #[error("Order Error")]
    OrderError,
    #[error("App error")]
    AppError,
    #[error("DatabaseError")]
    DBError,
    #[error("Scanner error")]
    ScanError,
    #[error("Blockchain call errror")]
    BlockchainError,
    #[error("Decoder Error")]
    DecoderError,
}
impl IntoResponse for FormatError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match &self {
            FormatError::BalanceError { has: _, neeed: _ } => (StatusCode::UPGRADE_REQUIRED, ""),
            FormatError::MismatchAddressInDb {
                has: _,
                must_have: _,
            } => (StatusCode::NOT_FOUND, "address in DB error"),
            FormatError::InvalidHeader {
                expected: _,
                found: _,
            } => (StatusCode::NOT_ACCEPTABLE, ""),
            FormatError::MissingAttribute(_) => (StatusCode::UPGRADE_REQUIRED, ""),
            FormatError::ParseError(_) => (StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS, ""),
            FormatError::OrderError(_) => (StatusCode::CONFLICT, ""),
            FormatError::DBError(_) => (StatusCode::INTERNAL_SERVER_ERROR, ""),
            FormatError::ScanError(_) => (StatusCode::INTERNAL_SERVER_ERROR, ""),
            FormatError::BlockchainError(_) => (StatusCode::INTERNAL_SERVER_ERROR, ""),
            FormatError::DecoderError(_) => (StatusCode::INTERNAL_SERVER_ERROR, ""),
        };
        (status, Json(json!({ "error": message }))).into_response()
    }
}
