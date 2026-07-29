use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug, Serialize)]
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
    #[error("Failed to start activity")]
    InitError,
    #[error("Redis connection pool error")]
    RedisError,
    #[error("Already registed")]
    RegistrationError,
    #[error("Error in verifying siganture (expected {expected:?}, got {found:?})")]
    SignatureMismtachError { expected: String, found: String },
    #[error("Authorization error")]
    UnauthorizedError,
    #[error("JWT token error")]
    JWTokenError,
}
impl IntoResponse for FormatError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match &self {
            FormatError::BalanceError { has: _, neeed: _ } => (StatusCode::UPGRADE_REQUIRED, ""),
            FormatError::JWTokenError => (StatusCode::EXPECTATION_FAILED, ""),
            FormatError::MismatchAddressInDb {
                has: _,
                must_have: _,
            } => (StatusCode::NOT_FOUND, "address in DB error"),
            FormatError::InvalidHeader {
                expected: _,
                found: _,
            } => (StatusCode::NOT_ACCEPTABLE, ""),
            FormatError::MissingAttribute(_) => (StatusCode::UPGRADE_REQUIRED, ""),
            FormatError::ParseError => (StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS, ""),
            FormatError::OrderError => (StatusCode::CONFLICT, ""),
            FormatError::DBError => (StatusCode::INTERNAL_SERVER_ERROR, ""),
            FormatError::ScanError => (StatusCode::INTERNAL_SERVER_ERROR, ""),
            FormatError::BlockchainError => (StatusCode::INTERNAL_SERVER_ERROR, ""),
            FormatError::DecoderError => (StatusCode::INTERNAL_SERVER_ERROR, ""),
            FormatError::AppError => (StatusCode::INTERNAL_SERVER_ERROR, ""),
            FormatError::InitError => (StatusCode::INTERNAL_SERVER_ERROR, ""),
            FormatError::RedisError => (StatusCode::INTERNAL_SERVER_ERROR, ""),
            FormatError::RegistrationError => (StatusCode::METHOD_NOT_ALLOWED, ""),
            FormatError::SignatureMismtachError {
                expected: _,
                found: _,
            } => (StatusCode::NON_AUTHORITATIVE_INFORMATION, ""),
            FormatError::UnauthorizedError => (StatusCode::UNAUTHORIZED, ""),
        };
        (status, Json(json!({ "error": message }))).into_response()
    }
}
