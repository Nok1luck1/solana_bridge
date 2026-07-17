use crate::db::{database, redis};
use crate::dto::auth::{self, RandomNonceReq};
use crate::errors::{self, FormatError};
use crate::{eth, state};
use crate::handlers::auth_routes::auth::{LoginRequest, RegisterRequestEVM, RegisterRequestSOL};
use crate::state::AppState;
use alloy::signers::k256::elliptic_curve::bigint::Random;
use axum::extract::State;
use axum::{http::StatusCode, Json};
// async fn register_evm(
//     Json(_input): Json<RegisterRequestEVM>,
//     State(state): State<AppState>,
// ) -> Json<StatusCode> {
//     if let number = database::get_user_id_by_address_evm(state.db, _input.address).await? > 0 {
//         Json(FormatError::DBError)
//     }
//     Json(StatusCode::ACCEPTED)
// }
// async fn register_solana(
//     Json(_input): Json<RegisterRequestSOL>,
//     State(state): State<AppState>,
// ) -> Json<StatusCode> {
//     //if _input.wallet
//     if let number database::get_user_id_by_address_solana(state.db, _input.address).await? > 0 {
//         FormatError::RegistrationError
//     }
//     Json(StatusCode::ACCEPTED)
// }
async fn login_evm(Json(_input): Json<LoginRequest>) -> Json<StatusCode> {
    Json(StatusCode::ACCEPTED)
}
async fn login_svm(Json(_input): Json<LoginRequest>) -> Json<StatusCode> {
    Json(StatusCode::ACCEPTED)
}
async fn generate_nonce(State(state):State<AppState>,Json(_input): Json<RandomNonceReq>) -> Json<RandomNonceReq> {
    let nonce = eth::get_address_nonce(_input.address.clone())
        .await
        .expect(&errors::FormatError::BlockchainError.to_string());
    let rand_bytes_arr = rand::random();
    let connection  = state::get_redis();
    connection
    Json(RandomNonceReq {
        address: _input.address,
        rand_nonce: nonce,
        rand_bytes_arr: rand_bytes_arr,
    })

    //let random = Random::random;
}
