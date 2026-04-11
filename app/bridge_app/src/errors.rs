use thiserror::Error;

#[derive(Error, Debug)]
pub enum FormatError {
    #[error("Invalid header (expected {expected:?}, got {found:?})")]
    InvalidHeader { expected: String, found: String },
    #[error("Missing attribute: {0}")]
    MissingAttribute(String),
    #[error("Bridge has insufficient balance, has {has:?}, needed {neeed:?}")]
    BalanceError { has: String, neeed: String },
}
