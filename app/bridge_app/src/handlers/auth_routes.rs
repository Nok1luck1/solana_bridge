use crate::db::database;
use crate::dto::auth::{self, RandomNonceReq};
use crate::errors::FormatError;
use crate::eth;
use crate::handlers::auth_routes::auth::{LoginRequest, RegisterRequest};
use crate::state::AppState;
use alloy::signers::k256::elliptic_curve::bigint::Random;
use axum::extract::State;
use axum::{http::StatusCode, Json};
async fn register_evm(
    Json(_input): Json<RegisterRequestEVM>,
    State(state): State<AppState>,
) -> Json<StatusCode> {
    if let number = database::get_user_id_by_address_evm(state.db, _input.address).await? > 0 {
        FormatError::RegistrationError
    }
    Json(StatusCode::ACCEPTED)
}
async fn register_solana(
    Json(_input): Json<RegisterRequestSOL>,
    State(state): State<AppState>,
) -> Json<StatusCode> {
    //if _input.wallet
    if let number database::get_user_id_by_address_solana(state.db, _input.address).await? > 0 {
        FormatError::RegistrationError
    }
    Json(StatusCode::ACCEPTED)
}
async fn login_evm(Json(_input): Json<LoginRequest>) -> Json<StatusCode> {
    Json(StatusCode::ACCEPTED)
}
async fn login_svm(Json(_input): Json<LoginRequest>) -> Json<StatusCode> {
    Json(StatusCode::ACCEPTED)
}
async fn generate_nonce(Json(_input):Json<RandomNonceReq>)->Json<RandomNonceReq>{
    if let length = _input.address.len() == 42 {
        eth::get_address_nonce(_input.address)
    }
    //let random = Random::random;
}
